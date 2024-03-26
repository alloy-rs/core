//! Expanders convert [`SolInputKind`] into [`SolOutput`].

use std::{collections::BTreeMap, path::PathBuf};

use crate::{SolAttrs, SolInputKind};
use ast::{
    File, ItemContract, ItemEnum, ItemError, ItemEvent, ItemFunction, ItemStruct, ItemUdt,
    SolIdent, Type, VariableDefinition,
};
use proc_macro2::TokenStream;

fn same_borrow<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}

trait ExpansionOutput: Sized {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &Self) -> bool;
    /// Merge two expansions.
    ///
    /// ## Behavior
    ///
    /// - If the two expansions do not have the same item, return `Err(other)`.
    /// - If the two expansions have the same item, merge the two expansions and return `Ok(())`.
    fn merge(&mut self, other: Self) -> Result<(), Self>;
}

/// Expansion of [`ItemContract`].
#[derive(Debug, Clone)]
pub struct ContractExpansion<'a> {
    item: &'a ItemContract,
    body: BTreeMap<SolIdent, SolExpansionOutput<'a>>,
}

impl<'a> ExpansionOutput for ContractExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &ContractExpansion<'a>) -> bool {
        same_borrow(self.item, other.item)
    }

    /// Merge two contract expansions by combining their bodies.
    fn merge(&mut self, mut other: ContractExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        // assume same keys
        self.body.iter_mut().for_each(|(k, v)| {
            let other_v = other.body.remove(k).expect("key not found");
            v.merge(other_v).expect("same item");
        });

        Ok(())
    }
}

/// Expansion of [`ItemEnum`].
#[derive(Debug, Clone)]
pub struct EnumExpansion<'a>(pub &'a ItemEnum, pub TokenStream);

impl<'a> ExpansionOutput for EnumExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &EnumExpansion<'a>) -> bool {
        same_borrow(self.0, other.0)
    }

    /// Merge two expansions.
    fn merge(&mut self, other: EnumExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        self.1.extend(other.1);
        Ok(())
    }
}

/// Expansion of [`ItemError`].
#[derive(Debug, Clone)]
pub struct EventExpansion<'a>(pub &'a ItemEnum, pub TokenStream);

impl<'a> ExpansionOutput for EventExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &EventExpansion<'a>) -> bool {
        same_borrow(self.0, other.0)
    }

    /// Merge two expansions.
    fn merge(&mut self, other: EventExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        self.1.extend(other.1);
        Ok(())
    }
}

/// Expansion of [`ItemError`].
#[derive(Debug, Clone)]
pub struct ErrorExpansion<'a>(pub &'a ItemError, pub TokenStream);

impl<'a> ExpansionOutput for ErrorExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &ErrorExpansion<'a>) -> bool {
        same_borrow(self.0, other.0)
    }

    /// Merge two expansions.
    fn merge(&mut self, other: ErrorExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        self.1.extend(other.1);
        Ok(())
    }
}

/// Expansion of [`ItemFunction`].
#[derive(Debug, Clone)]
pub struct FunctionExpansion<'a>(pub &'a ItemFunction, pub TokenStream);

impl<'a> ExpansionOutput for FunctionExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &FunctionExpansion<'a>) -> bool {
        same_borrow(self.0, other.0)
    }

    /// Merge two expansions.
    fn merge(&mut self, other: FunctionExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        self.1.extend(other.1);
        Ok(())
    }
}

/// Expansion of [`ItemStruct`].
#[derive(Debug, Clone)]
pub struct StructExpansion<'a>(pub &'a ItemStruct, pub TokenStream);

impl<'a> ExpansionOutput for StructExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &StructExpansion<'a>) -> bool {
        same_borrow(self.0, other.0)
    }

    /// Merge two expansions.
    fn merge(&mut self, other: StructExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        self.1.extend(other.1);
        Ok(())
    }
}

/// Expansion of [`ItemUdt`].
#[derive(Debug, Clone)]
pub struct UdtExpansion<'a>(pub &'a ItemUdt, pub TokenStream);

impl<'a> ExpansionOutput for UdtExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &UdtExpansion<'a>) -> bool {
        same_borrow(self.0, other.0)
    }

    /// Merge two expansions.
    fn merge(&mut self, other: UdtExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        self.1.extend(other.1);
        Ok(())
    }
}

/// Expansion of [`VariableDefinition`].
#[derive(Debug, Clone)]
pub struct VariableExpansion<'a>(pub &'a VariableDefinition, pub TokenStream);

impl<'a> ExpansionOutput for VariableExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &VariableExpansion<'a>) -> bool {
        same_borrow(self.0, other.0)
    }

    /// Merge two expansions.
    fn merge(&mut self, other: VariableExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        self.1.extend(other.1);
        Ok(())
    }
}

/// Expansion of [`File`].
#[derive(Debug, Clone)]
pub enum SolExpansionOutput<'a> {
    /// Expansion of [`ItemContract`].
    Contract(ContractExpansion<'a>),

    /// Expansion of [`ItemEnum`].
    Enum(EnumExpansion<'a>),
    /// Expansion of [`ItemError`].
    Error(ErrorExpansion<'a>),
    /// Expansion of [`ItemEvent`].
    Event(EventExpansion<'a>),
    /// Expansion of [`ItemFunction`].
    Function(FunctionExpansion<'a>),
    /// Expansion of [`ItemStruct`].
    Struct(StructExpansion<'a>),
    /// Expansion of [`ItemUdt`].
    Udt(UdtExpansion<'a>),
    /// Expansion of [`VariableDefinition`].
    Variable(VariableExpansion<'a>),
}

impl<'a> ExpansionOutput for SolExpansionOutput<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &SolExpansionOutput<'a>) -> bool {
        match (self, other) {
            (Self::Contract(a), Self::Contract(b)) => a.same_item(b),
            (Self::Enum(a), Self::Enum(b)) => a.same_item(b),
            (Self::Error(a), Self::Error(b)) => a.same_item(b),
            (Self::Event(a), Self::Event(b)) => a.same_item(b),
            (Self::Function(a), Self::Function(b)) => a.same_item(b),
            (Self::Struct(a), Self::Struct(b)) => a.same_item(b),
            (Self::Udt(a), Self::Udt(b)) => a.same_item(b),
            (Self::Variable(a), Self::Variable(b)) => a.same_item(b),
            _ => false,
        }
    }

    /// Merge two expansions.
    fn merge(&mut self, other: SolExpansionOutput<'a>) -> Result<(), Self> {
        match (self, other) {
            (Self::Contract(a), Self::Contract(b)) => a.merge(b).map_err(Self::Contract),
            (Self::Enum(a), Self::Enum(b)) => a.merge(b).map_err(Self::Enum),
            (Self::Error(a), Self::Error(b)) => a.merge(b).map_err(Self::Error),
            (Self::Event(a), Self::Event(b)) => a.merge(b).map_err(Self::Event),
            (Self::Function(a), Self::Function(b)) => a.merge(b).map_err(Self::Function),
            (Self::Struct(a), Self::Struct(b)) => a.merge(b).map_err(Self::Struct),
            (Self::Udt(a), Self::Udt(b)) => a.merge(b).map_err(Self::Udt),
            (Self::Variable(a), Self::Variable(b)) => a.merge(b).map_err(Self::Variable),
            (_, other) => Err(other),
        }
    }
}

/// Expansion of a [`Type`].
#[derive(Debug, Clone)]
pub struct TypeExpansion<'a>(pub &'a Type, pub TokenStream);

impl<'a> ExpansionOutput for TypeExpansion<'a> {
    /// True if the two expansions have the same item.
    fn same_item(&self, other: &TypeExpansion<'a>) -> bool {
        same_borrow(self.0, other.0)
    }

    /// Merge two expansions.
    fn merge(&mut self, other: TypeExpansion<'a>) -> Result<(), Self> {
        if !self.same_item(&other) {
            return Err(other);
        }

        self.1.extend(other.1);
        Ok(())
    }
}

/// Expansion of [`SolInput`].
#[derive(Debug, Clone)]
pub enum SolOutput<'a> {
    /// Expansion of a [`Type`].
    Type(TypeExpansion<'a>),
    /// Expansion of a [`File`].
    Sol(SolExpansionOutput<'a>),
}
impl<'a> ExpansionOutput for SolOutput<'a> {
    fn same_item(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Type(a), Self::Type(b)) => a.same_item(b),
            (Self::Sol(a), Self::Sol(b)) => a.same_item(b),
            _ => false,
        }
    }

    fn merge(&mut self, other: Self) -> Result<(), Self> {
        match (self, other) {
            (Self::Type(a), Self::Type(b)) => a.merge(b).map_err(Self::Type),
            (Self::Sol(a), Self::Sol(b)) => a.merge(b).map_err(Self::Sol),
            (_, other) => Err(other),
        }
    }
}

/// Context for the expander.
#[derive(Debug, Clone)]
pub struct Context {
    /// The path of the file being expanded.
    pub path: Option<PathBuf>,
    /// Crate overrides
    pub crates: BTreeMap<String, syn::Path>,
    /// Non-sol attributes.
    pub root_attrs: Vec<syn::Attribute>,
    /// Sol attributes.
    pub sol_attrs: SolAttrs,
}

/// Expands a [`SolInput`] into a [`SolOutput`].
pub trait SolInputExpander {
    /// Create a new expander from a context.
    fn from_context(ctx: Context) -> Self;

    /// Get the expansion context.
    fn ctx(&self) -> &Context;

    /// Expand an [`ItemEnum`] into a [`EnumExpansion`].
    fn expand_enum<'a>(&mut self, item: &'a ItemEnum) -> EnumExpansion<'a>;

    /// Expand an [`ItemError`] into a [`ErrorExpansion`].
    fn expand_error<'a>(&mut self, item: &'a ItemError) -> ErrorExpansion<'a>;

    /// Expand an [`ItemEvent`] into a [`EventExpansion`].
    fn expand_event<'a>(&mut self, item: &'a ItemEvent) -> EventExpansion<'a>;

    /// Expand an [`ItemFunction`] into a [`FunctionExpansion`].
    fn expand_function<'a>(&mut self, item: &'a ItemFunction) -> FunctionExpansion<'a>;

    /// Expand an [`ItemStruct`] into a [`StructExpansion`].
    fn expand_struct<'a>(&mut self, item: &'a ItemStruct) -> StructExpansion<'a>;

    /// Expand an [`ItemUdt`] into a [`UdtExpansion`].
    fn expand_udt<'a>(&mut self, item: &'a ItemUdt) -> UdtExpansion<'a>;

    /// Expand a [`VariableDefinition`] into a [`VariableExpansion`].
    fn expand_variable<'a>(&mut self, item: &'a VariableDefinition) -> VariableExpansion<'a>;

    /// Expand an [`ItemContract`] into a [`ContractExpansion`].
    fn expand_contract<'a>(&mut self, item: &'a ItemContract) -> ContractExpansion<'a>;

    /// Expand a [`File`] into an [`SolOutput`].
    fn expand_file<'a>(&mut self, file: &'a File) -> SolOutput<'a>;

    /// Expand a [`Type`] into an [`SolOutput`].
    fn expand_type<'a>(&mut self, ty: &'a Type) -> SolOutput<'a>;

    /// Expand a `SolInputKind` into a `TokenStream`.
    fn expand<'a>(&mut self, input: &'a SolInputKind) -> syn::Result<SolOutput<'a>> {
        match input {
            crate::SolInputKind::Type(ty) => self.expand_type(ty),
            crate::SolInputKind::Sol(sol) => self.expand_file(sol),
            #[cfg(feature = "json")]
            crate::SolInputKind::Json(_, _) => {
                panic!("unnormalized JSON");
            }
        };

        todo!()
    }
}
