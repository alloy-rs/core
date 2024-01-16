use alloy_json_abi::{AbiItem, ContractObject, EventParam, JsonAbi, Param};
use pretty_assertions::assert_eq;
use std::{
    collections::HashMap,
    fs,
    path::Path,
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
};

const JSON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/abi");

const TESTDATA_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/testdata");

static UPDATED: AtomicBool = AtomicBool::new(false);

#[test]
#[cfg_attr(miri, ignore = "no fs")]
fn abi() {
    let run_solc = run_solc();
    for file in std::fs::read_dir(JSON_PATH).unwrap() {
        let path = file.unwrap().path();
        if path.extension() != Some("json".as_ref()) {
            continue;
        }

        let fname = path.file_name().unwrap().to_str().unwrap();
        // Not an ABI sequence, just one function object.
        if fname == "LargeFunction.json" {
            continue;
        }

        abi_test(&std::fs::read_to_string(&path).unwrap(), path.to_str().unwrap(), run_solc);
    }
    if UPDATED.load(Ordering::Relaxed) {
        panic!("some file was not up to date and has been updated, simply re-run the tests");
    }
}

fn abi_test(s: &str, path: &str, run_solc: bool) {
    eprintln!("{path}");
    let abi_items: Vec<AbiItem<'_>> = serde_json::from_str(s).unwrap();
    let len = abi_items.len();

    let json: String = serde_json::to_string(&abi_items).unwrap();
    let abi1: JsonAbi = serde_json::from_str(&json).unwrap();

    let abi2: JsonAbi = serde_json::from_str(s).unwrap();

    assert_eq!(len, abi2.len());
    assert_eq!(abi1, abi2);

    #[cfg(all(feature = "std", feature = "serde_json"))]
    load_test(path, &abi1);
    to_sol_test(path, &abi1, run_solc);

    let json: String = serde_json::to_string(&abi2).unwrap();
    let abi3: JsonAbi = serde_json::from_str(&json).unwrap();
    assert_eq!(abi2, abi3);

    param_tests(&abi1);
    method_tests(&abi1);

    iterator_test(abi1.items(), abi1.items().rev(), len);
    iterator_test(abi1.items().skip(1), abi1.items().skip(1).rev(), len - 1);
    iterator_test(abi1.clone().into_items(), abi1.into_items().rev(), len);
}

#[cfg(all(feature = "std", feature = "serde_json"))]
fn load_test(path: &str, abi: &JsonAbi) {
    use std::{fs::File, io::BufReader};
    let file: File = File::open(path).unwrap();
    let buffer: BufReader<File> = BufReader::new(file);
    let loaded_abi: JsonAbi = JsonAbi::load(buffer).unwrap();

    assert_eq!(*abi, loaded_abi);
}

fn to_sol_test(path: &str, abi: &JsonAbi, run_solc: bool) {
    let path = Path::new(path);
    let sol_path = path.with_extension("sol");
    let name = path.file_stem().unwrap().to_str().unwrap();

    let mut abi = abi.clone();
    abi.dedup();
    let actual = abi.to_sol(name);

    ensure_file_contents(&sol_path, &actual);

    if matches!(
        name,
        // https://github.com/alloy-rs/core/issues/349
        "ZeroXExchange" | "GaugeController" | "DoubleExponentInterestSetter"
    ) {
        return;
    }

    if run_solc {
        let out = Command::new("solc").arg("--abi").arg(&sol_path).output().unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        let panik = |s| -> ! { panic!("{s}\n\nstdout:\n{stdout}\n\nstderr:\n{stderr}") };
        if !out.status.success() {
            panik("solc failed");
        }
        let Some(json_str_start) = stdout.find("[{") else {
            panik("no JSON");
        };
        let json_str = &stdout[json_str_start..];
        let solc_abi = match serde_json::from_str::<JsonAbi>(json_str) {
            Ok(solc_abi) => solc_abi,
            Err(e) => panik(&format!("invalid JSON: {e}")),
        };

        // Constructor is ignored.
        abi.constructor = None;

        // Note that we don't compare the ABIs directly since the conversion is lossy, e.g.
        // `internalType` fields change.
        if solc_abi.len() != abi.len() {
            assert_eq!(solc_abi, abi, "ABI length mismatch");
        }
    }
}

fn iterator_test<T, I, R>(items: I, rev: R, len: usize)
where
    T: PartialEq + std::fmt::Debug,
    I: Iterator<Item = T> + DoubleEndedIterator + ExactSizeIterator,
    R: Iterator<Item = T>,
{
    assert_eq!(items.len(), len);
    assert_eq!(items.size_hint(), (len, Some(len)));

    let mut items2: Vec<_> = items.collect();
    items2.reverse();
    assert_eq!(items2, rev.collect::<Vec<_>>());
}

fn param_tests(abi: &JsonAbi) {
    abi.items().for_each(|item| match item {
        AbiItem::Constructor(c) => c.inputs.iter().for_each(test_param),
        AbiItem::Function(f) => {
            f.inputs.iter().for_each(test_param);
            f.outputs.iter().for_each(test_param);
        }
        AbiItem::Event(e) => e.inputs.iter().for_each(test_event_param),
        AbiItem::Error(e) => e.inputs.iter().for_each(test_param),
        _ => {}
    });
}

fn method_tests(abi: &JsonAbi) {
    test_functions(abi);
    test_events(abi);
    test_errors(abi);
}

fn test_functions(abi: &JsonAbi) {
    abi.functions().for_each(|f| {
        f.inputs.iter().for_each(test_param);
        f.outputs.iter().for_each(test_param);
    });

    abi.functions()
        .map(|f| f.name.clone())
        .fold(HashMap::new(), |mut freq_count, name| {
            *freq_count.entry(name).or_insert(0) += 1;
            freq_count
        })
        .into_iter()
        .for_each(|(name, freq)| {
            assert_eq!(abi.function(&name).unwrap().len(), freq);
        });
}

fn test_errors(abi: &JsonAbi) {
    abi.errors().for_each(|e| e.inputs.iter().for_each(test_param));

    abi.errors()
        .map(|e| e.name.clone())
        .fold(HashMap::new(), |mut freq_count, name| {
            *freq_count.entry(name).or_insert(0) += 1;
            freq_count
        })
        .into_iter()
        .for_each(|(name, freq)| {
            assert_eq!(abi.error(&name).unwrap().len(), freq);
        });
}

fn test_events(abi: &JsonAbi) {
    abi.events().for_each(|e| e.inputs.iter().for_each(test_event_param));

    abi.events()
        .map(|e| e.name.clone())
        .fold(HashMap::new(), |mut freq_count, name| {
            *freq_count.entry(name).or_insert(0) += 1;
            freq_count
        })
        .into_iter()
        .for_each(|(name, freq)| {
            assert_eq!(abi.event(&name).unwrap().len(), freq);
        });
}

fn test_event_param(param: &EventParam) {
    if param.components.is_empty() {
        assert!(!param.ty.contains("tuple"));
        return;
    }

    if param.is_struct() {
        assert!(param.ty.contains("tuple"));
        assert!(param.struct_specifier().is_some());
    }
    param.components.iter().for_each(test_param);
}

fn test_param(param: &Param) {
    if param.components.is_empty() {
        assert!(!param.ty.contains("tuple"));
        return;
    }

    if param.is_struct() {
        assert!(param.ty.contains("tuple"));
        if param.struct_specifier().is_none() {
            println!("{:#?}", param.internal_type);
        }
        assert!(param.struct_specifier().is_some());
    }

    param.components.iter().for_each(test_param);
}

/// Checks that the `file` has the specified `contents`. If that is not the
/// case, updates the file and then fails the test.
fn ensure_file_contents(file: &Path, contents: &str) {
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == normalize_newlines(contents) {
            // File is already up to date.
            return;
        }
    }

    eprintln!("\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n", file.display());
    if std::env::var("CI").is_ok() {
        eprintln!("    NOTE: run `cargo test` locally and commit the updated files\n");
    }
    if let Some(parent) = file.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(file, contents).unwrap();
    UPDATED.store(true, Ordering::Relaxed);
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}

fn run_solc() -> bool {
    let Some(v) = get_solc_version() else {
        return false;
    };
    // UDVTs: https://soliditylang.org/blog/2021/09/27/user-defined-value-types/
    v >= (0, 8, 8)
}

fn get_solc_version() -> Option<(u16, u16, u16)> {
    let output = Command::new("solc").arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;

    let start = stdout.find(": 0.")?;
    let version = &stdout[start + 2..];
    let end = version.find('+')?;
    let version = &version[..end];

    let mut iter = version.split('.').map(|s| s.parse::<u16>().expect("bad solc version"));
    let major = iter.next().unwrap();
    let minor = iter.next().unwrap();
    let patch = iter.next().unwrap();
    Some((major, minor, patch))
}

// <https://github.com/foundry-rs/foundry/issues/6815>
#[test]
#[cfg_attr(miri, ignore = "no fs")]
#[cfg(all(feature = "std", feature = "serde_json"))]
fn parse_unlinked_contract() {
    // unlinked placeholder __$7233c33f2e1e35848c685b0eb24649959e$__
    let content = fs::read_to_string(Path::new(TESTDATA_PATH).join("UnlinkedNouns.json")).unwrap();
    let res = serde_json::from_str::<ContractObject>(&content);
    let err = res.unwrap_err();
    assert!(err.to_string().contains("expected bytecode, found unlinked bytecode with placeholder: 7233c33f2e1e35848c685b0eb24649959e"));
}
