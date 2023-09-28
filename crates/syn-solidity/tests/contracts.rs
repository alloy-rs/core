use std::{
    fs::{self, DirEntry},
    path::Path,
    process::Command,
};
use syn_solidity::{File, Item};

#[test]
fn contracts() {
    static PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/contracts");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut entries: Vec<_> = fs::read_dir(PATH)
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    entries.sort_by_key(std::fs::DirEntry::path);
    let mut patcher = GitPatcher::new(entries, root);
    patcher.patch();
    for file in patcher.files() {
        let path = file.path();
        eprintln!("parsing {}", path.display());
        parse_file(&path).unwrap();
    }
    patcher.unpatch();
}

/// Runs `unpatch` on drop. This ensures that the patch is always reset even if
/// the test panics.
struct GitPatcher<'a> {
    entries: Vec<DirEntry>,
    root: &'a Path,
    patched: bool,
}

impl<'a> GitPatcher<'a> {
    fn new(entries: Vec<DirEntry>, root: &'a Path) -> Self {
        Self {
            entries,
            root,
            patched: false,
        }
    }

    fn patches(&self) -> impl Iterator<Item = &DirEntry> {
        self.entries
            .iter()
            .filter(|p| p.path().extension() == Some("patch".as_ref()))
    }

    fn files(&self) -> impl Iterator<Item = &DirEntry> {
        self.entries
            .iter()
            .filter(|p| p.path().extension() == Some("sol".as_ref()))
    }

    fn patch(&mut self) {
        self.patched = true;
        for patch in self.patches() {
            let path = patch.path();
            let s = Command::new("git")
                .current_dir(self.root)
                .arg("apply")
                .arg(&path)
                .status()
                .unwrap();
            assert!(
                s.success(),
                "failed to apply patch at {}: {s}",
                path.display()
            );
        }
    }

    fn unpatch(&mut self) {
        if !self.patched {
            return
        }
        self.patched = false;
        for patch in self.patches() {
            let path = patch.path();
            match Command::new("git")
                .current_dir(self.root)
                .arg("apply")
                .arg("--reverse")
                .arg(&path)
                .status()
            {
                Ok(s) if s.success() => {}
                e => {
                    eprintln!("failed to reset patch at {}: {e:?}", path.display())
                }
            }
        }
    }
}

impl Drop for GitPatcher<'_> {
    fn drop(&mut self) {
        self.unpatch();
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
