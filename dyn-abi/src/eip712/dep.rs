use crate::{
    eip712::{
        typed_data::{NameTypePair, Types},
        Parse712Error,
    },
    no_std_prelude::{BTreeMap, HashSet},
    DynSolType,
};

fn is_atomic(type_name: &str) -> bool {
    match type_name {
        "address" | "bool" => true,
        _ => {
            if type_name.starts_with("bytes") {
                if let Some(len) = type_name.strip_prefix("bytes") {
                    if let Ok(len) = len.parse::<usize>() {
                        return len <= 32;
                    }
                }
            }
            if type_name.starts_with("uint") {
                if let Some(len) = type_name.strip_prefix("uint") {
                    if let Ok(len) = len.parse::<usize>() {
                        return len <= 256 && len % 8 == 0;
                    }
                }
            }
            if type_name.starts_with("int") {
                if let Some(len) = type_name.strip_prefix("int") {
                    if let Ok(len) = len.parse::<usize>() {
                        return len <= 256 && len % 8 == 0;
                    }
                }
            }
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDef {
    /// Must always be a ROOT type name with any array stripped
    type_name: String,
    props: Vec<NameTypePair>,
}

impl TypeDef {
    fn new(type_name: String, props: Vec<NameTypePair>) -> Self {
        Self { type_name, props }
    }

    fn type_name(&self) -> &str {
        &self.type_name
    }

    fn props(&self) -> &[NameTypePair] {
        &self.props
    }

    /// Produces the EIP-712 `encodeType` typestring for this type definition
    fn encode_type(&self) -> String {
        let mut encoded = format!("{}(", self.type_name);
        for prop in self.props.iter() {
            encoded.push_str(&format!("{} {},", prop.type_name(), prop.name()));
        }
        if self.props.len() > 0 {
            encoded.pop();
        }
        encoded.push(')');
        encoded
    }
}

/// A dependency graph built from the EIP-712 `Types` object. This is used to
/// safely resolve JSON into a [`crate::DynSolType`] by detecting cycles in the
/// type graph and traversing the dep graph.
#[derive(Debug, Clone, Default)]
pub struct DepGraph {
    /// Nodes in the graph
    nodes: BTreeMap<String, TypeDef>,
    /// Edges from a type name to its dependencies
    edges: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Default)]
struct DfsContext<'a> {
    visited: HashSet<&'a TypeDef>,
    stack: HashSet<&'a str>,
}

impl DepGraph {
    /// Detect cycles in the subgraph rooted at `ty`
    fn detect_cycle<'a>(&'a self, type_name: &'_ str, context: &mut DfsContext<'a>) -> bool {
        let ty = match self.nodes.get(type_name) {
            Some(ty) => ty,
            None => return false,
        };

        if context.stack.contains(type_name) {
            return true;
        }
        if context.visited.contains(ty) {
            return false;
        }

        // update visited and stack
        context.visited.insert(ty);
        context.stack.insert(&ty.type_name);

        if self
            .edges
            .get(&ty.type_name)
            .unwrap()
            .iter()
            .any(|edge| self.detect_cycle(edge, context))
        {
            return true;
        }

        context.stack.remove(type_name);
        return false;
    }

    pub fn ingest(&mut self, type_def: TypeDef) {
        let type_name = type_def.type_name.to_owned();
        self.nodes.insert(type_name.clone(), type_def);

        self.nodes
            .get(&type_name)
            .unwrap()
            .props
            .iter()
            .for_each(|prop| {
                self.edges
                    .entry(type_name.clone())
                    .or_insert_with(Vec::new)
                    .push(prop.root_type_name().to_owned());
            });
    }

    pub fn ingest_types(&mut self, types: Types) {
        for (type_name, props) in types.into_iter() {
            self.ingest(TypeDef::new(type_name, props));
        }
    }

    // This function assumes that the graph is acyclic.
    fn resolve_into<'a>(
        &'a self,
        resolution: &mut Vec<&'a TypeDef>,
        type_name: &str,
    ) -> Result<(), Parse712Error> {
        if is_atomic(type_name) {
            return Ok(());
        }

        println!("Resolving into {}", type_name);

        let this_type = self
            .nodes
            .get(type_name)
            .ok_or_else(|| Parse712Error::missing_type(type_name))?;

        println!("Found type {:?}", this_type);
        let edges: &Vec<String> = self.edges.get(type_name).unwrap();

        if !resolution.contains(&this_type) {
            resolution.push(this_type);
            for edge in edges.iter() {
                self.resolve_into(resolution, edge)?;
            }
        }

        println!("Resolution: {:?}", resolution);

        Ok(())
    }

    /// This function resolves a type into a list of typedefs
    pub fn resolve(&self, type_name: &str) -> Result<Vec<&TypeDef>, Parse712Error> {
        println!("Resolving {}", type_name);
        let mut context = DfsContext::default();
        if self.detect_cycle(type_name, &mut context) {
            return Err(Parse712Error::circular_dependency(type_name));
        }
        let mut resolution = vec![];
        self.resolve_into(&mut resolution, type_name)?;
        Ok(resolution)
    }

    pub fn encode_type(&self, name: &str) -> Result<String, Parse712Error> {
        println!("Encoding type {}", name);
        let resolution = self.resolve(name)?;
        println!("Resolved");
        let first = resolution.first().unwrap().encode_type();

        // Sort references by name (eip-712 encodeType spec)
        let mut sorted_refs = resolution[1..]
            .iter()
            .map(|t| t.encode_type())
            .collect::<Vec<String>>();
        sorted_refs.sort_by(|a, b| a.cmp(b));

        Ok(sorted_refs.iter().fold(first, |mut acc, s| {
            acc.push_str(&s);
            acc
        }))
    }

    /// Resolve the type into a [`crate::DynSolType`]
    pub fn resolve_type(&self, name: &str) -> Result<DynSolType, Parse712Error> {
        self.encode_type(name)
            .and_then(|s| crate::eip712::parser::parse_structs(&s))
            .and_then(|m| {
                println!("Resolved {} to {:?}", name, m);
                m.get(name)
                    .map(Clone::clone)
                    .ok_or(Parse712Error::missing_type(name))
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_detects_cycles() {
        let mut graph = DepGraph::default();
        graph.ingest(TypeDef::new(
            "A".to_string(),
            vec![NameTypePair::new("B", "myB".to_string())],
        ));
        graph.ingest(TypeDef::new(
            "B".to_string(),
            vec![NameTypePair::new("C", "myC".to_string())],
        ));
        graph.ingest(TypeDef::new(
            "C".to_string(),
            vec![NameTypePair::new("A", "myA".to_string())],
        ));

        assert!(graph.detect_cycle("A", &mut DfsContext::default()));
    }

    #[test]
    fn it_produces_encode_type_strings() {
        let mut graph = DepGraph::default();
        graph.ingest(TypeDef::new(
            "A".to_string(),
            vec![
                NameTypePair::new("C", "myC".to_string()),
                NameTypePair::new("B", "myB".to_string()),
            ],
        ));
        graph.ingest(TypeDef::new(
            "B".to_string(),
            vec![NameTypePair::new("C", "myC".to_string())],
        ));
        graph.ingest(TypeDef::new(
            "C".to_string(),
            vec![
                NameTypePair::new("uint256", "myUint".to_string()),
                NameTypePair::new("uint256", "myUint2".to_string()),
            ],
        ));

        // This tests specific adherence to EIP-712 specified ordering.
        // Referenced types are sorted by name, the Primary type is at the
        // start of the string
        assert_eq!(
            graph.encode_type("A").unwrap(),
            "A(C myC,B myB)B(C myC)C(uint256 myUint,uint256 myUint2)"
        );
    }

    #[test]
    fn it_resolves_types() {
        let mut graph = DepGraph::default();
        graph.ingest(TypeDef::new(
            "A".to_string(),
            vec![NameTypePair::new("B", "myB".to_string())],
        ));
        graph.ingest(TypeDef::new(
            "B".to_string(),
            vec![NameTypePair::new("C", "myC".to_string())],
        ));
        graph.ingest(TypeDef::new(
            "C".to_string(),
            vec![NameTypePair::new("uint256", "myUint".to_string())],
        ));

        let c = DynSolType::CustomStruct {
            name: "C".to_string(),
            prop_names: vec!["myUint".to_string()],
            tuple: vec![DynSolType::Uint(256)],
        };
        let b = DynSolType::CustomStruct {
            name: "B".to_string(),
            prop_names: vec!["myC".to_string()],
            tuple: vec![c.clone()],
        };
        let a = DynSolType::CustomStruct {
            name: "A".to_string(),
            prop_names: vec!["myB".to_string()],
            tuple: vec![b.clone()],
        };
        assert_eq!(graph.resolve_type("A").unwrap(), a);
        assert_eq!(graph.resolve_type("B").unwrap(), b);
        assert_eq!(graph.resolve_type("C").unwrap(), c);
    }

    #[test]
    fn it_resolves_types_with_arrays() {
        let mut graph = DepGraph::default();
        graph.ingest(TypeDef::new(
            "A".to_string(),
            vec![NameTypePair::new("B", "myB".to_string())],
        ));
        graph.ingest(TypeDef::new(
            "B".to_string(),
            vec![NameTypePair::new("C[]", "myC".to_string())],
        ));
        graph.ingest(TypeDef::new(
            "C".to_string(),
            vec![NameTypePair::new("uint256", "myUint".to_string())],
        ));

        let c = DynSolType::CustomStruct {
            name: "C".to_string(),
            prop_names: vec!["myUint".to_string()],
            tuple: vec![DynSolType::Uint(256)],
        };
        let b = DynSolType::CustomStruct {
            name: "B".to_string(),
            prop_names: vec!["myC".to_string()],
            tuple: vec![DynSolType::Array(Box::new(c.clone()))],
        };
        let a = DynSolType::CustomStruct {
            name: "A".to_string(),
            prop_names: vec!["myB".to_string()],
            tuple: vec![b.clone()],
        };
        assert_eq!(graph.resolve_type("A").unwrap(), a);
        assert_eq!(graph.resolve_type("B").unwrap(), b);
        assert_eq!(graph.resolve_type("C").unwrap(), c);
    }
}
