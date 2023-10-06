use crate::{
    kw, Block, FunctionAttribute, FunctionAttributes, Mutability, ParameterList, Parameters,
    SolIdent, Spanned, Stmt, Type, VariableDeclaration, VariableDefinition, Visibility,
};
use proc_macro2::Span;
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU16,
};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
    Attribute, Error, Result, Token,
};

/// A function, constructor, fallback, receive, or modifier definition:
/// `function helloWorld() external pure returns(string memory);`.
///
/// Solidity reference:
/// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.functionDefinition>
#[derive(Clone)]
pub struct ItemFunction {
    /// The `syn` attributes of the function.
    pub attrs: Vec<Attribute>,
    pub kind: FunctionKind,
    pub name: Option<SolIdent>,
    /// Parens are optional for modifiers:
    /// <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.modifierDefinition>
    pub paren_token: Option<Paren>,
    pub arguments: ParameterList,
    /// The Solidity attributes of the function.
    pub attributes: FunctionAttributes,
    /// The optional return types of the function.
    pub returns: Option<Returns>,
    pub body: FunctionBody,
}

impl fmt::Display for ItemFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.kind.as_str())?;
        if let Some(name) = &self.name {
            f.write_str(" ")?;
            name.fmt(f)?;
        }
        write!(f, "({})", self.arguments)?;

        if !self.attributes.is_empty() {
            write!(f, " {}", self.attributes)?;
        }

        if let Some(returns) = &self.returns {
            write!(f, " {returns}")?;
        }

        if !self.body.is_empty() {
            f.write_str(" ")?;
        }
        f.write_str(self.body.as_str())
    }
}

impl fmt::Debug for ItemFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemFunction")
            .field("attrs", &self.attrs)
            .field("kind", &self.kind)
            .field("name", &self.name)
            .field("arguments", &self.arguments)
            .field("attributes", &self.attributes)
            .field("returns", &self.returns)
            .field("body", &self.body)
            .finish()
    }
}

impl Parse for ItemFunction {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let kind: FunctionKind = input.parse()?;
        let name = input.call(SolIdent::parse_opt)?;

        let (paren_token, arguments) = if kind.is_modifier() && !input.peek(Paren) {
            (None, ParameterList::new())
        } else {
            let content;
            (Some(parenthesized!(content in input)), content.parse()?)
        };

        let attributes = input.parse()?;
        let returns = input.call(Returns::parse_opt)?;
        let body = input.parse()?;

        Ok(Self {
            attrs,
            kind,
            name,
            paren_token,
            arguments,
            attributes,
            returns,
            body,
        })
    }
}

impl Spanned for ItemFunction {
    fn span(&self) -> Span {
        if let Some(name) = &self.name {
            name.span()
        } else {
            self.kind.span()
        }
    }

    fn set_span(&mut self, span: Span) {
        self.kind.set_span(span);
        if let Some(name) = &mut self.name {
            name.set_span(span);
        }
    }
}

impl ItemFunction {
    /// Create a new function of the given kind.
    pub fn new(kind: FunctionKind, name: Option<SolIdent>) -> Self {
        let span = name
            .as_ref()
            .map_or_else(|| kind.span(), |name| name.span());
        Self {
            attrs: Vec::new(),
            kind,
            name,
            paren_token: Some(Paren(span)),
            arguments: Parameters::new(),
            attributes: FunctionAttributes::new(),
            returns: None,
            body: FunctionBody::Empty(Token![;](span)),
        }
    }

    /// Create a new function with the given name and arguments.
    ///
    /// Note that:
    /// - the type is not validated
    /// - structs/array of structs in return position are not expanded
    /// - the body is not set
    ///
    /// The attributes are set to `public view`.
    ///
    /// See [the Solidity documentation][ref] for more details on how getters
    /// are generated.
    ///
    /// [ref]: https://docs.soliditylang.org/en/latest/contracts.html#getter-functions
    pub fn new_getter(name: SolIdent, ty: Type) -> Self {
        let span = name.span();
        let kind = FunctionKind::new_function(span);
        let mut function = Self::new(kind, Some(name));

        // `public view`
        function.attributes.0 = vec![
            FunctionAttribute::Visibility(Visibility::new_public(span)),
            FunctionAttribute::Mutability(Mutability::new_view(span)),
        ];

        // Recurse into mappings and arrays to generate arguments and the return type
        let mut ty = ty;
        let mut return_name = None;
        loop {
            match ty {
                // mapping(k => v) -> arguments += k, ty = v
                Type::Mapping(map) => {
                    let key = VariableDeclaration::new_with(*map.key, None, map.key_name);
                    function.arguments.push(key);
                    return_name = map.value_name;
                    ty = *map.value;
                }
                // inner[] -> arguments += uint256, ty = inner
                Type::Array(array) => {
                    let uint256 = Type::Uint(span, NonZeroU16::new(256));
                    function.arguments.push(VariableDeclaration::new(uint256));
                    ty = *array.ty;
                }
                _ => break,
            }
        }
        let mut returns = ParameterList::new();
        returns.push(VariableDeclaration::new_with(ty, None, return_name));
        function.returns = Some(Returns::new(span, returns));

        function
    }

    /// Creates a new function from a variable definition.
    ///
    /// The function will have the same name and the variable type's will be the
    /// return type. The variable attributes are ignored, and instead will
    /// always generate `public returns`.
    ///
    /// See [`new_getter`](Self::new_getter) for more details.
    pub fn from_variable_definition(var: VariableDefinition) -> Self {
        Self::new_getter(var.name, var.ty)
    }

    /// Returns the name of the function.
    ///
    /// # Panics
    ///
    /// Panics if the function has no name. This is the case when `kind` is not
    /// `Function`.
    pub fn name(&self) -> &SolIdent {
        match &self.name {
            Some(name) => name,
            None => panic!("function has no name: {self:?}"),
        }
    }

    /// Returns true if the function returns nothing.
    pub fn is_void(&self) -> bool {
        match &self.returns {
            None => true,
            Some(returns) => returns.returns.is_empty(),
        }
    }

    /// Returns true if the function has a body.
    pub fn has_implementation(&self) -> bool {
        matches!(self.body, FunctionBody::Block(_))
    }

    /// Returns the function's arguments tuple type.
    pub fn call_type(&self) -> Type {
        Type::Tuple(self.arguments.types().cloned().collect())
    }

    /// Returns the function's return tuple type.
    pub fn return_type(&self) -> Option<Type> {
        self.returns
            .as_ref()
            .map(|returns| Type::Tuple(returns.returns.types().cloned().collect()))
    }

    /// Returns a reference to the function's body, if any.
    pub fn body(&self) -> Option<&[Stmt]> {
        match &self.body {
            FunctionBody::Block(block) => Some(&block.stmts),
            _ => None,
        }
    }

    /// Returns a mutable reference to the function's body, if any.
    pub fn body_mut(&mut self) -> Option<&mut Vec<Stmt>> {
        match &mut self.body {
            FunctionBody::Block(block) => Some(&mut block.stmts),
            _ => None,
        }
    }

    pub fn into_body(self) -> std::result::Result<Vec<Stmt>, Self> {
        match self.body {
            FunctionBody::Block(block) => Ok(block.stmts),
            _ => Err(self),
        }
    }
}

kw_enum! {
    /// The kind of function.
    pub enum FunctionKind {
        Constructor(kw::constructor),
        Function(kw::function),
        Fallback(kw::fallback),
        Receive(kw::receive),
        Modifier(kw::modifier),
    }
}

/// The `returns` attribute of a function.
#[derive(Clone)]
pub struct Returns {
    pub returns_token: kw::returns,
    pub paren_token: Paren,
    /// The returns of the function. This cannot be parsed empty.
    pub returns: ParameterList,
}

impl PartialEq for Returns {
    fn eq(&self, other: &Self) -> bool {
        self.returns == other.returns
    }
}

impl Eq for Returns {}

impl Hash for Returns {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.returns.hash(state);
    }
}

impl fmt::Display for Returns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("returns (")?;
        self.returns.fmt(f)?;
        f.write_str(")")
    }
}

impl fmt::Debug for Returns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Returns").field(&self.returns).finish()
    }
}

impl Parse for Returns {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        let this = Self {
            returns_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            returns: content.parse()?,
        };
        if this.returns.is_empty() {
            Err(Error::new(
                this.paren_token.span.join(),
                "expected at least one return type",
            ))
        } else {
            Ok(this)
        }
    }
}

impl Spanned for Returns {
    fn span(&self) -> Span {
        let span = self.returns_token.span;
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.returns_token.span = span;
        self.paren_token = Paren(span);
    }
}

impl Returns {
    pub fn new(span: Span, returns: ParameterList) -> Self {
        Self {
            returns_token: kw::returns(span),
            paren_token: Paren(span),
            returns,
        }
    }

    pub fn parse_opt(input: ParseStream<'_>) -> Result<Option<Self>> {
        if input.peek(kw::returns) {
            input.parse().map(Some)
        } else {
            Ok(None)
        }
    }
}

/// The body of a function.
#[derive(Clone)]
pub enum FunctionBody {
    /// A function without implementation.
    Empty(Token![;]),
    /// A function body delimited by curly braces.
    Block(Block),
}

impl fmt::Display for FunctionBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Debug for FunctionBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FunctionBody::")?;
        match self {
            Self::Empty(_) => f.write_str("Empty"),
            Self::Block(block) => block.fmt(f),
        }
    }
}

impl Parse for FunctionBody {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else if lookahead.peek(Token![;]) {
            input.parse().map(Self::Empty)
        } else {
            Err(lookahead.error())
        }
    }
}

impl FunctionBody {
    /// Returns `true` if the function body is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }

    /// Returns a string representation of the function body.
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Empty(_) => ";",
            // TODO: fmt::Display for Stmt
            Self::Block(_) => "{ <stmts> }",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::{
        error::Error,
        io::Write,
        process::{Command, Stdio},
    };

    #[test]
    #[cfg_attr(miri, ignore = "takes too long")]
    fn getters() {
        macro_rules! test_getters {
            ($($var:literal => $f:literal),* $(,)?) => {
                let vars: &[&str] = &[$($var),*];
                let fns: &[&str] = &[$($f),*];
                let run_solc = run_solc();
                for (var, f) in std::iter::zip(vars, fns) {
                    test_getter(var, f, run_solc);
                }
            };
        }

        test_getters! {
            "bool public getter;"
                => "function getter() public view returns (bool);",
            "bool public constant publicConstantAttr = false;"
                => "function publicConstantAttr() public view returns (bool);",

            "mapping(address => bool) public map;"
                => "function map(address) public view returns (bool);",
            "mapping(address a => bool b) public mapWithNames;"
                => "function mapWithNames(address a) public view returns (bool b);",
            "mapping(uint256 k1 => mapping(uint256 k2 => bool v) ignored) public nested2;"
                => "function nested2(uint256 k1, uint256 k2) public view returns (bool v);",
            "mapping(uint256 k1 => mapping(uint256 k2 => mapping(uint256 k3 => bool v) ignored1) ignored2) public nested3;"
                => "function nested3(uint256 k1, uint256 k2, uint256 k3) public view returns (bool v);",

            "bool[] public boolArray;"
                => "function boolArray(uint256) public view returns(bool);",
            "mapping(bool => bytes2)[] public mapArray;"
                => "function mapArray(uint256, bool) public view returns(bytes2);",
            "mapping(bool => mapping(address => int[])[])[][] public nestedMapArray;"
                => "function nestedMapArray(uint256, uint256, bool, uint256, address, uint256) public view returns(int);",
        }
    }

    fn test_getter(var_s: &str, fn_s: &str, run_solc: bool) {
        let var = syn::parse_str::<VariableDefinition>(var_s).unwrap();
        let getter = ItemFunction::from_variable_definition(var);
        let f = syn::parse_str::<ItemFunction>(fn_s).unwrap();
        assert_eq!(format!("{getter:#?}"), format!("{f:#?}"));
        // test that the ABIs are the same
        if run_solc {
            match (wrap_and_compile(var_s, true), wrap_and_compile(fn_s, false)) {
                (Ok(a), Ok(b)) => {
                    assert_eq!(a.trim(), b.trim(), "\nleft:  {var_s:?}\nright: {fn_s:?}")
                }
                (Err(e), _) | (_, Err(e)) => panic!("{e}"),
            }
        }
    }

    fn run_solc() -> bool {
        let Ok(status) = Command::new("solc").arg("--version").status() else {
            return false
        };
        status.success()
    }

    fn wrap_and_compile(s: &str, var: bool) -> std::result::Result<String, Box<dyn Error>> {
        let contract = if var {
            format!("contract C {{ {s} }}")
        } else {
            format!(
                "abstract contract C {{ {} }}",
                s.replace("returns", "virtual returns")
            )
        };
        let mut cmd = Command::new("solc")
            .args(["--abi", "--pretty-json", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        cmd.stdin.as_mut().unwrap().write_all(contract.as_bytes())?;
        let output = cmd.wait_with_output()?;
        if output.status.success() {
            String::from_utf8(output.stdout).map_err(Into::into)
        } else {
            Err(String::from_utf8(output.stderr)?.into())
        }
    }
}
