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

    let files: Vec<_> = fs::read_dir(PATH)
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
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
        if !s.success() {
            panic!("failed to apply patch: {s}")
        }
    }

    let mut failed = false;
    for file in files {
        let path = file.path();
        match parse_file(&path) {
            Ok(()) => {}
            Err(e) => {
                let name = path.file_name().unwrap().to_str().unwrap();
                eprintln!("failed to parse {name}: {e} ({e:?})");
                failed = true;
            }
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
        if !s.success() {
            panic!("failed to reset patch: {s}")
        }
    }

    if failed {
        panic!();
    }
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
