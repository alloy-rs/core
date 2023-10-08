use alloy_json_abi::{AbiItem, EventParam, JsonAbi, Param};
use std::collections::HashMap;

#[test]
#[cfg_attr(miri, ignore = "no fs")]
fn abi() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/abi");
    for file in std::fs::read_dir(path).unwrap() {
        let path = file.unwrap().path();
        assert_eq!(path.extension(), Some("json".as_ref()));
        if path.file_name() == Some("LargeFunction.json".as_ref()) {
            continue
        }
        parse_test(
            &std::fs::read_to_string(&path).unwrap(),
            path.to_str().unwrap(),
        );
    }
}

fn parse_test(s: &str, path: &str) {
    eprintln!("{path}");
    let abi_items: Vec<AbiItem<'_>> = serde_json::from_str(s).unwrap();
    let len = abi_items.len();

    let json: String = serde_json::to_string(&abi_items).unwrap();
    let abi1: JsonAbi = serde_json::from_str(&json).unwrap();

    let abi2: JsonAbi = serde_json::from_str(s).unwrap();

    assert_eq!(len, abi2.len());
    assert_eq!(abi1, abi2);

    load_test(path, &abi1);

    let json: String = serde_json::to_string(&abi2).unwrap();
    let abi3: JsonAbi = serde_json::from_str(&json).unwrap();
    assert_eq!(abi2, abi3);

    param_tests(&abi1);
    method_tests(&abi1);

    iterator_test(abi1.items(), abi1.items().rev(), len);
    iterator_test(abi1.items().skip(1), abi1.items().skip(1).rev(), len - 1);
    iterator_test(abi1.clone().into_items(), abi1.into_items().rev(), len);
}

#[allow(unused_variables)]
fn load_test(path: &str, abi: &JsonAbi) {
    #[cfg(all(feature = "std", feature = "serde_json", not(miri)))]
    {
        use std::{fs::File, io::BufReader};
        let file: File = File::open(path).unwrap();
        let buffer: BufReader<File> = BufReader::new(file);
        let loaded_abi: JsonAbi = JsonAbi::load(buffer).unwrap();

        assert_eq!(*abi, loaded_abi);
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
    abi.errors()
        .for_each(|e| e.inputs.iter().for_each(test_param));

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
    abi.events()
        .for_each(|e| e.inputs.iter().for_each(test_event_param));

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
        return
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
        return
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
