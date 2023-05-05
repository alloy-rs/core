use core::cmp::Ordering;

use ethers_abi_enc::SolStruct;
use ethers_primitives::{keccak256, B256};
use serde::{Deserialize, Serialize};

use crate::{
    eip712::typed_data::Eip712Types,
    eip712_parser::EncodeType,
    no_std_prelude::*,
    parser::{RootType, TypeSpecifier, TypeStem},
    DynAbiError, DynSolType, DynSolValue,
};

/// An EIP-712 property definition
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct PropertyDef {
    /// Typename
    #[serde(rename = "type")]
    type_name: String,
    /// Property Name
    name: String,
}

impl<'de> serde::Deserialize<'de> for PropertyDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "type")]
            type_name: String,
            name: String,
        }
        let h: Helper = serde::Deserialize::deserialize(deserializer)?;
        Self::new(h.type_name, h.name).map_err(serde::de::Error::custom)
    }
}

impl PropertyDef {
    /// Instantiate a new name-type pair
    pub fn new(type_name: impl AsRef<str>, name: impl AsRef<str>) -> Result<Self, DynAbiError> {
        let type_name: TypeSpecifier<'_> = type_name.as_ref().try_into()?;
        Ok(Self::new_unchecked(type_name, name))
    }

    /// Instantiate a new name-type pair, without checking that the type name
    /// is a valid root type
    pub fn new_unchecked(type_name: impl AsRef<str>, name: impl AsRef<str>) -> Self {
        Self {
            type_name: type_name.as_ref().to_owned(),
            name: name.as_ref().to_owned(),
        }
    }

    /// Returns the type name of the property
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns the root type of the name/type pair, stripping any array
    pub fn root_type_name(&self) -> &str {
        self.type_name
            .split_once('[')
            .map(|t| t.0)
            .unwrap_or(&self.type_name)
    }

    /// Returns the name of the property
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDef {
    /// Must always be a ROOT type name with any array stripped
    type_name: String,
    /// A list of property definitions
    props: Vec<PropertyDef>,
}

impl Ord for TypeDef {
    // This is not a logic error because we know type names cannot be duplicated in the resolver map
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_name.cmp(&other.type_name)
    }
}

impl PartialOrd for TypeDef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for TypeDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.encode_type())
    }
}

impl TypeDef {
    /// Instantiate a new type definition, checking that the type name is a
    /// valid root type
    pub fn new(type_name: impl AsRef<str>, props: Vec<PropertyDef>) -> Result<Self, DynAbiError> {
        let _rt: RootType<'_> = type_name.as_ref().try_into()?;
        Ok(Self {
            type_name: type_name.as_ref().to_owned(),
            props,
        })
    }

    /// Instantiate a new type definition, without checking that the type name
    /// is a valid root type. This may result in bad behavior in a resolver
    pub fn new_unchecked(type_name: String, props: Vec<PropertyDef>) -> Self {
        Self { type_name, props }
    }

    /// Returns the type name of the type definition
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns the property definitions of the type definition
    pub fn props(&self) -> &[PropertyDef] {
        &self.props
    }

    /// Returns the property names of the type definition
    pub fn prop_names(&self) -> impl Iterator<Item = &str> + '_ {
        self.props.iter().map(|p| p.name())
    }

    /// Returns the root property types of the type definition
    pub fn prop_root_types(&self) -> impl Iterator<Item = &str> + '_ {
        self.props.iter().map(|p| p.root_type_name())
    }

    /// Returns the property types of the type definition
    pub fn prop_types(&self) -> impl Iterator<Item = &str> + '_ {
        self.props.iter().map(|p| p.type_name())
    }

    /// Produces the EIP-712 `encodeType` typestring for this type definition
    pub fn encode_type(&self) -> String {
        let mut encoded = format!("{}(", self.type_name);
        for prop in self.props.iter() {
            encoded.push_str(&format!("{} {},", prop.type_name(), prop.name()));
        }
        // if the props are not empty, there is a trailing comma
        if !self.props.is_empty() {
            encoded.pop();
        }
        encoded.push(')');
        encoded
    }

    /// Return the root type
    pub fn root_type(&self) -> RootType<'_> {
        self.type_name
            .as_str()
            .try_into()
            .expect("checked in instantiation")
    }
}

#[derive(Debug, Default)]
struct DfsContext<'a> {
    visited: BTreeSet<&'a TypeDef>,
    stack: BTreeSet<&'a str>,
}

/// A dependency graph built from the `Eip712Types` object. This is used to
/// safely resolve JSON into a [`crate::DynSolType`] by detecting cycles in the
/// type graph and traversing the dep graph.
#[derive(Debug, Clone, Default)]
pub struct Resolver {
    /// Nodes in the graph
    ///
    // NOTE: Non-duplication of names must be enforced. See note on impl of Ord
    // for TypeDef
    nodes: BTreeMap<String, TypeDef>,
    /// Edges from a type name to its dependencies
    edges: BTreeMap<String, Vec<String>>,
}

impl serde::Serialize for Resolver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let types: Eip712Types = self.into();
        types.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Resolver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let types: Eip712Types = serde::Deserialize::deserialize(deserializer)?;
        Ok(types.into())
    }
}

impl From<Eip712Types> for Resolver {
    fn from(types: Eip712Types) -> Self {
        let mut graph = Resolver::default();
        graph.ingest_types(&types);
        graph
    }
}

impl From<&Eip712Types> for Resolver {
    fn from(types: &Eip712Types) -> Self {
        let mut graph = Resolver::default();
        graph.ingest_types(types);
        graph
    }
}

impl From<&Resolver> for Eip712Types {
    fn from(resolver: &Resolver) -> Self {
        let mut types = Eip712Types::default();
        for (name, ty) in resolver.nodes.iter() {
            types.insert(name.clone(), ty.props.clone());
        }
        types
    }
}

impl Resolver {
    /// Instantiate a new resolver from a `SolStruct` type.
    pub fn from_struct<S>() -> Self
    where
        S: SolStruct,
    {
        let mut resolver = Resolver::default();
        resolver.ingest_sol_struct::<S>();
        resolver
    }

    /// Detect cycles in the subgraph rooted at `type_name`
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
        false
    }

    /// Ingest types from an EIP-712 `encodeType`
    pub fn ingest_string(&mut self, s: impl AsRef<str>) -> Result<(), DynAbiError> {
        let encode_type: EncodeType<'_> = s.as_ref().try_into()?;
        for type_def in encode_type.types.into_iter().map(|t| t.to_owned()) {
            self.ingest(type_def);
        }
        Ok(())
    }

    /// Ingest a sol struct typedef
    pub fn ingest_sol_struct<S>(&mut self)
    where
        S: SolStruct,
    {
        self.ingest_string(S::encode_type()).unwrap();
    }

    /// Ingest a type
    pub fn ingest(&mut self, type_def: TypeDef) {
        let type_name = type_def.type_name.to_owned();
        // Insert the edges into the graph
        {
            let entry = self.edges.entry(type_name.clone()).or_insert_with(Vec::new);
            type_def
                .props
                .iter()
                .for_each(|prop| entry.push(prop.root_type_name().to_owned()));
        } // entry dropped here

        // Insert the node into the graph
        self.nodes.insert(type_name, type_def);
    }

    /// Ingest a `Types` object into the resolver, discarding any invalid types
    pub fn ingest_types(&mut self, types: &Eip712Types) {
        for (type_name, props) in types.iter() {
            if let Ok(ty) = TypeDef::new(type_name.clone(), props.to_vec()) {
                self.ingest(ty);
            }
        }
    }

    // This function assumes that the graph is acyclic.
    fn linearize_into<'a>(
        &'a self,
        resolution: &mut Vec<&'a TypeDef>,
        root_type: RootType<'_>,
    ) -> Result<(), DynAbiError> {
        if root_type.try_basic_solidity().is_ok() {
            return Ok(());
        }

        let this_type = self
            .nodes
            .get(root_type.as_str())
            .ok_or_else(|| DynAbiError::missing_type(root_type.as_str()))?;

        let edges: &Vec<String> = self.edges.get(root_type.as_str()).unwrap();

        if !resolution.contains(&this_type) {
            resolution.push(this_type);
            for edge in edges.iter() {
                let rt = edge.as_str().try_into()?;
                self.linearize_into(resolution, rt)?;
            }
        }

        Ok(())
    }

    /// This function linearizes a type into a list of typedefs of its
    /// dependencies
    pub fn linearize(&self, type_name: &str) -> Result<Vec<&TypeDef>, DynAbiError> {
        let mut context = DfsContext::default();
        if self.detect_cycle(type_name, &mut context) {
            return Err(DynAbiError::circular_dependency(type_name));
        }
        let root_type = type_name.try_into()?;
        let mut resolution = vec![];
        self.linearize_into(&mut resolution, root_type)?;
        Ok(resolution)
    }

    /// Resolves a root solidity type into either a basic type or a custom
    /// struct
    fn resolve_root_type(&self, root_type: RootType<'_>) -> Result<DynSolType, DynAbiError> {
        if root_type.try_basic_solidity().is_ok() {
            return root_type.resolve_basic_solidity();
        }

        let ty = self
            .nodes
            .get(root_type.as_str())
            .ok_or_else(|| DynAbiError::missing_type(root_type.as_str()))?;

        let prop_names = ty.prop_names().map(str::to_string).collect();
        let tuple = ty
            .prop_types()
            .map(|ty| self.unchecked_resolve(ty.try_into()?))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DynSolType::CustomStruct {
            name: ty.type_name.clone(),
            prop_names,
            tuple,
        })
    }

    /// Resolve a type into a [`crate::DynSolType`] without checking for cycles
    fn unchecked_resolve(&self, type_spec: TypeSpecifier<'_>) -> Result<DynSolType, DynAbiError> {
        let ty = match type_spec.root {
            TypeStem::Root(root) => self.resolve_root_type(root)?,
            TypeStem::Tuple(tuple) => {
                let tuple = tuple
                    .types
                    .into_iter()
                    .map(|ty| self.unchecked_resolve(ty))
                    .collect::<Result<Vec<_>, _>>()?;
                DynSolType::Tuple(tuple)
            }
        };

        let ty = type_spec.sizes.iter().fold(ty, |acc, item| match item {
            Some(size) => DynSolType::FixedArray(Box::new(acc), *size),
            None => DynSolType::Array(Box::new(acc)),
        });

        Ok(ty)
    }

    /// Resolve a typename into a [`crate::DynSolType`] or return an error if
    /// the type is mising, or contains a circular dependency.
    pub fn resolve(&self, type_name: &str) -> Result<DynSolType, DynAbiError> {
        if self.detect_cycle(type_name, &mut Default::default()) {
            return Err(DynAbiError::circular_dependency(type_name));
        }
        self.unchecked_resolve(type_name.try_into()?)
    }

    /// Encode the type into an EIP-712 `encodeType` string
    ///
    /// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
    pub fn encode_type(&self, name: &str) -> Result<String, DynAbiError> {
        let linear = self.linearize(name)?;
        let first = linear.first().unwrap().encode_type();

        // Sort references by name (eip-712 encodeType spec)
        let mut sorted_refs = linear[1..]
            .iter()
            .map(|t| t.encode_type())
            .collect::<Vec<String>>();
        sorted_refs.sort();

        Ok(sorted_refs.iter().fold(first, |mut acc, s| {
            acc.push_str(s);
            acc
        }))
    }

    /// Compute the keccak256 hash of the EIP-712 `encodeType` string
    pub fn type_hash(&self, name: &str) -> Result<B256, DynAbiError> {
        self.encode_type(name).map(keccak256)
    }

    /// Encode the data according to EIP-712 `encodeData` rules
    pub fn encode_data(&self, value: &DynSolValue) -> Result<Option<Vec<u8>>, DynAbiError> {
        match value {
            DynSolValue::CustomStruct { tuple: inner, .. }
            | DynSolValue::Array(inner)
            | DynSolValue::FixedArray(inner) => {
                let inner = inner.iter().try_fold(Vec::new(), |mut acc, v| {
                    acc.extend(self.eip712_data_word(v)?.as_slice());
                    Ok::<_, DynAbiError>(acc)
                })?;
                Ok(Some(inner))
            }
            DynSolValue::Bytes(buf) => Ok(Some(buf.to_vec())),
            DynSolValue::String(s) => Ok(Some(s.as_bytes().to_vec())),
            _ => Ok(None),
        }
    }

    /// Encode the data as a struct property according to EIP-712 `encodeData`
    /// rules. Atomic types are encoded as-is, while non-atomic types are
    /// encoded as their `encodeData` hash.
    pub fn eip712_data_word(&self, value: &DynSolValue) -> Result<B256, DynAbiError> {
        if let Some(word) = value.as_word() {
            return Ok(word);
        }

        match value {
            DynSolValue::CustomStruct { name, tuple, .. } => {
                let type_hash = self.type_hash(name)?.to_vec();
                let inner = tuple.iter().try_fold(type_hash, |mut acc, v| {
                    acc.extend(self.eip712_data_word(v)?.as_slice());
                    Ok::<_, DynAbiError>(acc)
                })?;
                Ok(keccak256(inner))
            }
            DynSolValue::Array(inner) | DynSolValue::FixedArray(inner) => {
                let inner = inner.iter().try_fold(Vec::new(), |mut acc, v| {
                    acc.extend(self.eip712_data_word(v)?.as_slice());
                    Ok::<_, DynAbiError>(acc)
                })?;
                Ok(keccak256(inner))
            }
            DynSolValue::Bytes(buf) => Ok(keccak256(buf)),
            DynSolValue::String(s) => Ok(keccak256(s.as_bytes())),
            _ => unreachable!("all types are words or covered in the match"),
        }
    }
}

#[cfg(test)]
mod test {
    use ethers_abi_enc::sol;

    use super::*;

    #[test]
    fn it_detects_cycles() {
        let mut graph = Resolver::default();
        graph.ingest(TypeDef::new_unchecked(
            "A".to_string(),
            vec![PropertyDef::new_unchecked("B", "myB")],
        ));
        graph.ingest(TypeDef::new_unchecked(
            "B".to_string(),
            vec![PropertyDef::new_unchecked("C", "myC")],
        ));
        graph.ingest(TypeDef::new_unchecked(
            "C".to_string(),
            vec![PropertyDef::new_unchecked("A", "myA")],
        ));

        assert!(graph.detect_cycle("A", &mut DfsContext::default()));
    }

    #[test]
    fn it_produces_encode_type_strings() {
        let mut graph = Resolver::default();
        graph.ingest(TypeDef::new_unchecked(
            "A".to_string(),
            vec![
                PropertyDef::new_unchecked("C", "myC"),
                PropertyDef::new_unchecked("B", "myB"),
            ],
        ));
        graph.ingest(TypeDef::new_unchecked(
            "B".to_string(),
            vec![PropertyDef::new_unchecked("C", "myC")],
        ));
        graph.ingest(TypeDef::new_unchecked(
            "C".to_string(),
            vec![
                PropertyDef::new_unchecked("uint256", "myUint"),
                PropertyDef::new_unchecked("uint256", "myUint2"),
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
        let mut graph = Resolver::default();
        graph.ingest(TypeDef::new_unchecked(
            "A".to_string(),
            vec![PropertyDef::new_unchecked("B", "myB")],
        ));
        graph.ingest(TypeDef::new_unchecked(
            "B".to_string(),
            vec![PropertyDef::new_unchecked("C", "myC")],
        ));
        graph.ingest(TypeDef::new_unchecked(
            "C".to_string(),
            vec![PropertyDef::new_unchecked("uint256", "myUint")],
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
        assert_eq!(graph.resolve("A").unwrap(), a);
        assert_eq!(graph.resolve("B").unwrap(), b);
        assert_eq!(graph.resolve("C").unwrap(), c);
    }

    #[test]
    fn it_resolves_types_with_arrays() {
        let mut graph = Resolver::default();
        graph.ingest(TypeDef::new_unchecked(
            "A".to_string(),
            vec![PropertyDef::new_unchecked("B", "myB")],
        ));
        graph.ingest(TypeDef::new_unchecked(
            "B".to_string(),
            vec![PropertyDef::new_unchecked("C[]", "myC")],
        ));
        graph.ingest(TypeDef::new_unchecked(
            "C".to_string(),
            vec![PropertyDef::new_unchecked("uint256", "myUint")],
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
        assert_eq!(graph.resolve("C").unwrap(), c);
        assert_eq!(graph.resolve("B").unwrap(), b);
        assert_eq!(graph.resolve("A").unwrap(), a);
    }

    #[test]
    fn encode_type_round_trip() {
        const ENCODE_TYPE: &str = "A(C myC,B myB)B(C myC)C(uint256 myUint,uint256 myUint2)";
        let mut graph = Resolver::default();
        graph.ingest_string(ENCODE_TYPE).unwrap();
        assert_eq!(graph.encode_type("A").unwrap(), ENCODE_TYPE);

        const ENCODE_TYPE_2: &str = "Transaction(Person from,Person to,Asset tx)Asset(address token,uint256 amount)Person(address wallet,string name)";
        let mut graph = Resolver::default();
        graph.ingest_string(ENCODE_TYPE_2).unwrap();
        assert_eq!(graph.encode_type("Transaction").unwrap(), ENCODE_TYPE_2);
    }

    sol!(
        struct MyStruct {
            uint256 a;
        }
    );

    #[test]
    fn it_ingests_sol_structs() {
        let mut graph = Resolver::default();
        graph.ingest_sol_struct::<MyStruct>();
        assert_eq!(
            graph.encode_type("MyStruct").unwrap(),
            MyStruct::encode_type()
        );
    }
}
