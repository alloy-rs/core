use std::{fs, path::Path, process::Command};
use syn_solidity::{File, Item};

#[test]
fn contracts() {
    static PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/contracts");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut files: Vec<_> = fs::read_dir(PATH)
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    files.sort_by_key(std::fs::DirEntry::path);
    let patches = files
        .iter()
        .filter(|p| p.path().extension() == Some("patch".as_ref()));
    let files = files
        .iter()
        .filter(|p| p.path().extension() == Some("sol".as_ref()));

    for patch in patches.clone() {
        let s = Command::new("git")
            .current_dir(root)
            .arg("apply")
            .arg(patch.path())
            .status()
            .unwrap();
        assert!(s.success(), "failed to apply patch: {s}");
    }

    let mut failed = false;
    for file in files {
        let path = file.path();
        let name = path.file_name().unwrap().to_str().unwrap();

        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| parse_file(&path))) {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                eprintln!("failed to parse {name}: {e} ({e:?})");
                failed = true;
            }
            Err(_) => {
                eprintln!("panicked while parsing {name}");
                failed = true;
            }
        }
        if failed {
            break
        }
    }

    for patch in patches {
        let s = Command::new("git")
            .current_dir(root)
            .arg("apply")
            .arg("--reverse")
            .arg(patch.path())
            .status()
            .unwrap();
        assert!(s.success(), "failed to reset patch: {s}");
    }

    assert!(!failed);
}

fn parse_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let solidity = fs::read_to_string(path)?;
    let file: File = syn::parse_str(&solidity)?;
    assert!(!file.items.is_empty());
    file.items.iter().try_for_each(assert_item)
}

fn assert_item(item: &Item) -> Result<(), Box<dyn std::error::Error>> {
    match item {
        Item::Contract(contract) => contract.body.iter().try_for_each(assert_item),
        Item::Enum(e) if e.variants.is_empty() => Err("empty enum".into()),
        Item::Struct(s) if s.fields.is_empty() => Err("empty struct".into()),
        _ => Ok(()),
    }
}
