use crate::{
    item::{Error, Event, Fallback, Function, Receive},
    EventParam, InternalType, JsonAbi, Param, StateMutability,
};
use alloc::{collections::BTreeSet, string::String, vec::Vec};
use core::cmp::Ordering;

const INDENT: &str = "    ";

pub(crate) trait ToSol {
    fn to_sol(&self, out: &mut String);
}

impl ToSol for JsonAbi {
    #[allow(unknown_lints, for_loops_over_fallibles)]
    #[inline]
    fn to_sol(&self, out: &mut String) {
        macro_rules! fmt {
            ($e:expr) => {
                fmt!($e, true)
            };
            ($iter:expr, $sep:expr) => {
                let mut any = false;
                for x in $iter {
                    any = true;
                    out.push_str(INDENT);
                    x.to_sol(out);
                    out.push('\n');
                }
                if $sep && any {
                    out.push('\n');
                }
            };
        }

        let mut its = InternalTypes::new();
        its.visit_abi(self);
        fmt!(its.0);
        fmt!(self.errors());
        fmt!(self.events());
        fmt!(self.fallback);
        fmt!(self.receive);
        fmt!(self.functions(), false);
    }
}

/// Recursively collects internal structs, enums, and udvts from an ABI's items.
struct InternalTypes<'a>(BTreeSet<It<'a>>);

impl<'a> InternalTypes<'a> {
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    fn new() -> Self {
        Self(BTreeSet::new())
    }

    fn visit_abi(&mut self, abi: &'a JsonAbi) {
        if let Some(constructor) = &abi.constructor {
            self.visit_params(&constructor.inputs);
        }
        for function in abi.functions() {
            self.visit_params(&function.inputs);
            self.visit_params(&function.outputs);
        }
        for error in abi.errors() {
            self.visit_params(&error.inputs);
        }
        for event in abi.events() {
            self.visit_event_params(&event.inputs);
        }
    }

    fn visit_params(&mut self, params: &'a [Param]) {
        for param in params {
            self.visit_param(param);
        }
    }

    fn visit_param(&mut self, param: &'a Param) {
        self.extend(param.internal_type.as_ref(), &param.components, &param.ty);
        self.visit_params(&param.components);
    }

    fn visit_event_params(&mut self, params: &'a [EventParam]) {
        for param in params {
            self.visit_event_param(param);
        }
    }

    fn visit_event_param(&mut self, param: &'a EventParam) {
        self.extend(param.internal_type.as_ref(), &param.components, &param.ty);
        self.visit_params(&param.components);
    }

    fn extend(
        &mut self,
        internal_type: Option<&'a InternalType>,
        components: &'a Vec<Param>,
        real_ty: &'a String,
    ) {
        match internal_type {
            None | Some(InternalType::AddressPayable(_) | InternalType::Contract(_)) => {}
            Some(InternalType::Struct { contract: _, ty }) => {
                self.0.insert(It::new(ty, ItKind::Struct(components)));
            }
            Some(InternalType::Enum { contract: _, ty }) => {
                self.0.insert(It::new(ty, ItKind::Enum));
            }
            Some(it @ InternalType::Other { contract: _, ty }) => {
                // `Other` is a UDVT if it's not a basic Solidity type and not an array
                if let Some(it) = it.other_specifier() {
                    if it.try_basic_solidity().is_err() && !it.is_array() {
                        self.0.insert(It::new(ty, ItKind::Udvt(real_ty)));
                    }
                }
            }
        }
    }
}

/// An internal ABI type.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct It<'a> {
    // kind must come before name for `Ord`
    kind: ItKind<'a>,
    name: &'a str,
}

#[derive(PartialEq, Eq)]
enum ItKind<'a> {
    Enum,
    Udvt(&'a String),
    Struct(&'a Vec<Param>),
}

// implemented manually because `Param: !Ord`
impl PartialOrd for ItKind<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ItKind<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Enum, Self::Enum) => Ordering::Equal,
            (Self::Enum, _) => Ordering::Less,
            (_, Self::Enum) => Ordering::Greater,

            (Self::Udvt(_), Self::Udvt(_)) => Ordering::Equal,
            (Self::Udvt(_), _) => Ordering::Less,
            (_, Self::Udvt(_)) => Ordering::Greater,

            (Self::Struct(_), Self::Struct(_)) => Ordering::Equal,
        }
    }
}

impl<'a> It<'a> {
    #[inline]
    fn new(ty_name: &'a str, kind: ItKind<'a>) -> Self {
        Self {
            kind,
            // `ty_name` might be an array, we just want the identifier
            name: ty_name.split('[').next().unwrap(),
        }
    }
}

impl ToSol for It<'_> {
    fn to_sol(&self, out: &mut String) {
        match self.kind {
            ItKind::Enum => {
                out.push_str("type ");
                out.push_str(self.name);
                out.push_str(" is uint8;");
            }
            ItKind::Udvt(ty) => {
                out.push_str("type ");
                out.push_str(self.name);
                out.push_str(" is ");
                out.push_str(ty);
                out.push(';');
            }
            ItKind::Struct(components) => {
                out.push_str("struct ");
                out.push_str(self.name);
                out.push_str(" {\n");
                for component in components {
                    out.push_str(INDENT);
                    out.push_str(INDENT);
                    component.to_sol(out);
                    out.push_str(";\n");
                }
                out.push_str(INDENT);
                out.push('}');
            }
        }
    }
}

impl ToSol for Event {
    fn to_sol(&self, out: &mut String) {
        AbiFunction::<'_, EventParam> {
            kw: "event",
            name: Some(&self.name),
            inputs: &self.inputs,
            visibility: None,
            state_mutability: None,
            anonymous: self.anonymous,
            outputs: &[],
        }
        .to_sol(out);
    }
}

impl ToSol for Error {
    fn to_sol(&self, out: &mut String) {
        AbiFunction::<'_, Param> {
            kw: "error",
            name: Some(&self.name),
            inputs: &self.inputs,
            visibility: None,
            state_mutability: None,
            anonymous: false,
            outputs: &[],
        }
        .to_sol(out);
    }
}

impl ToSol for Fallback {
    fn to_sol(&self, out: &mut String) {
        AbiFunction::<'_, Param> {
            kw: "fallback",
            name: None,
            inputs: &[],
            visibility: Some("external"),
            state_mutability: Some(self.state_mutability),
            anonymous: false,
            outputs: &[],
        }
        .to_sol(out);
    }
}

impl ToSol for Receive {
    fn to_sol(&self, out: &mut String) {
        AbiFunction::<'_, Param> {
            kw: "receive",
            name: None,
            inputs: &[],
            visibility: Some("external"),
            state_mutability: Some(self.state_mutability),
            anonymous: false,
            outputs: &[],
        }
        .to_sol(out);
    }
}

impl ToSol for Function {
    fn to_sol(&self, out: &mut String) {
        AbiFunction::<'_, Param> {
            kw: "function",
            name: Some(&self.name),
            inputs: &self.inputs,
            visibility: Some("external"),
            state_mutability: Some(self.state_mutability),
            anonymous: false,
            outputs: &self.outputs,
        }
        .to_sol(out);
    }
}

struct AbiFunction<'a, IN> {
    kw: &'static str,
    name: Option<&'a str>,
    inputs: &'a [IN],
    visibility: Option<&'static str>,
    state_mutability: Option<StateMutability>,
    anonymous: bool,
    outputs: &'a [Param],
}

impl<IN: ToSol> ToSol for AbiFunction<'_, IN> {
    fn to_sol(&self, out: &mut String) {
        out.push_str(self.kw);
        if let Some(name) = self.name {
            out.push(' ');
            out.push_str(name);
        }

        out.push('(');
        for (i, input) in self.inputs.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            input.to_sol(out);
        }
        out.push(')');

        if let Some(visibility) = self.visibility {
            out.push(' ');
            out.push_str(visibility);
        }

        if let Some(state_mutability) = self.state_mutability {
            if let Some(state_mutability) = state_mutability.as_str() {
                out.push(' ');
                out.push_str(state_mutability);
            }
        }

        if !self.outputs.is_empty() {
            out.push_str(" returns (");
            for (i, output) in self.outputs.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                output.to_sol(out);
            }
            out.push(')');
        }

        if self.anonymous {
            out.push_str(" anonymous");
        }

        out.push(';');
    }
}

impl ToSol for Param {
    fn to_sol(&self, out: &mut String) {
        param(&self.ty, self.internal_type.as_ref(), false, &self.name, &self.components, out);
    }
}

impl ToSol for EventParam {
    fn to_sol(&self, out: &mut String) {
        param(
            &self.ty,
            self.internal_type.as_ref(),
            self.indexed,
            &self.name,
            &self.components,
            out,
        );
    }
}

fn param<'a>(
    mut type_name: &'a str,
    internal_type: Option<&'a InternalType>,
    indexed: bool,
    name: &str,
    components: &[Param],
    out: &mut String,
) {
    if let Some(it) = internal_type {
        type_name = match it {
            InternalType::AddressPayable(_) => "address payable",
            InternalType::Contract(_) => "address",
            InternalType::Struct { ty, .. }
            | InternalType::Enum { ty, .. }
            | InternalType::Other { ty, .. } => ty,
        };
    };

    match type_name.strip_prefix("tuple") {
        // This condition is met only for JSON ABIs emitted by Solc 0.4.X which don't contain
        // `internalType` fields and instead all structs are emitted as unnamed tuples.
        // See https://github.com/alloy-rs/core/issues/349
        Some(rest) if rest.is_empty() || rest.starts_with('[') => {
            // note: this does not actually emit valid Solidity because there are no inline
            // tuple types `(T, U, V, ...)`, but it's valid for `sol!`.
            out.push('(');
            for (i, component) in components.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                param(
                    &component.ty,
                    component.internal_type.as_ref(), // this is probably always None
                    false,
                    "", // don't emit names in types
                    &component.components,
                    out,
                );
            }
            // trailing comma for single-element tuples
            if components.len() == 1 {
                out.push(',');
            }
            out.push(')');
            // could be array sizes
            out.push_str(rest);
        }
        // primitive type
        _ => out.push_str(type_name),
    }

    if indexed {
        out.push_str(" indexed");
    }
    if !name.is_empty() {
        out.push(' ');
        out.push_str(name);
    }
}
