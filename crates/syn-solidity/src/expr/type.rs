use crate::{Spanned, Type, kw};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result, Token, parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
};

/// A `type()` expression: `type(uint256)`
#[derive(Clone)]
pub struct ExprTypeCall {
    pub type_token: Token![type],
    pub paren_token: Paren,
    pub ty: Type,
}

impl fmt::Debug for ExprTypeCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprTypeCall").field("ty", &self.ty).finish()
    }
}

impl Parse for ExprTypeCall {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            type_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            ty: content.parse()?,
        })
    }
}

impl fmt::Display for ExprTypeCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type({})", self.ty)
    }
}

impl Spanned for ExprTypeCall {
    fn span(&self) -> Span {
        let span = self.type_token.span;
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.type_token.span = span;
        self.paren_token = Paren(span);
    }
}

/// A `new` expression: `new Contract`.
///
/// i.e. a contract creation or the allocation of a dynamic memory array.
#[derive(Clone)]
pub struct ExprNew {
    pub new_token: kw::new,
    pub ty: Type,
}

impl fmt::Debug for ExprNew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprNew").field("ty", &self.ty).finish()
    }
}

impl Parse for ExprNew {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { new_token: input.parse()?, ty: input.parse()? })
    }
}

impl fmt::Display for ExprNew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "new {}", self.ty)
    }
}

impl Spanned for ExprNew {
    fn span(&self) -> Span {
        let span = self.new_token.span;
        span.join(self.ty.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.new_token.span = span;
        self.ty.set_span(span);
    }
}

#[cfg(test)]
mod tests {
    use crate::Expr;
    use syn::parse_str;

    fn parse_expr(input: &str) -> Expr {
        parse_str(input).expect(&format!("Failed to parse: {}", input))
    }

    #[test]
    fn test_display_type_call_builtin_types() {
        let expr = parse_expr("type(uint256)");
        assert_eq!(format!("{}", expr), "type(uint256)");

        let expr = parse_expr("type(int256)");
        assert_eq!(format!("{}", expr), "type(int256)");

        let expr = parse_expr("type(address)");
        assert_eq!(format!("{}", expr), "type(address)");

        let expr = parse_expr("type(bool)");
        assert_eq!(format!("{}", expr), "type(bool)");

        let expr = parse_expr("type(bytes32)");
        assert_eq!(format!("{}", expr), "type(bytes32)");
    }

    #[test]
    fn test_display_type_call_array_types() {
        let expr = parse_expr("type(uint256[])");
        assert_eq!(format!("{}", expr), "type(uint256[])");

        let expr = parse_expr("type(address[10])");
        assert_eq!(format!("{}", expr), "type(address[10])");

        let expr = parse_expr("type(bytes[])");
        assert_eq!(format!("{}", expr), "type(bytes[])");

        let expr = parse_expr("type(string[])");
        assert_eq!(format!("{}", expr), "type(string[])");
    }

    #[test]
    fn test_display_type_call_custom_types() {
        let expr = parse_expr("type(MyContract)");
        assert_eq!(format!("{}", expr), "type(MyContract)");

        let expr = parse_expr("type(CustomStruct)");
        assert_eq!(format!("{}", expr), "type(CustomStruct)");

        let expr = parse_expr("type(MyEnum)");
        assert_eq!(format!("{}", expr), "type(MyEnum)");

        let expr = parse_expr("type(Token)");
        assert_eq!(format!("{}", expr), "type(Token)");
    }

    #[test]
    fn test_display_type_call_mapping_types() {
        let expr = parse_expr("type(mapping(address => uint256))");
        assert_eq!(format!("{}", expr), "type(mapping(address => uint256))");

        let expr = parse_expr("type(mapping(bytes32 => bool))");
        assert_eq!(format!("{}", expr), "type(mapping(bytes32 => bool))");

        let expr = parse_expr("type(mapping(uint256 => string))");
        assert_eq!(format!("{}", expr), "type(mapping(uint256 => string))");
    }

    #[test]
    fn test_display_new_expression_contracts() {
        let expr = parse_expr("new MyContract");
        assert_eq!(format!("{}", expr), "new MyContract");

        let expr = parse_expr("new Token");
        assert_eq!(format!("{}", expr), "new Token");

        let expr = parse_expr("new Implementation");
        assert_eq!(format!("{}", expr), "new Implementation");

        let expr = parse_expr("new Factory");
        assert_eq!(format!("{}", expr), "new Factory");
    }

    #[test]
    fn test_display_new_expression_arrays() {
        let expr = parse_expr("new uint256[]");
        assert_eq!(format!("{}", expr), "new uint256[]");

        let expr = parse_expr("new address[]");
        assert_eq!(format!("{}", expr), "new address[]");

        let expr = parse_expr("new bytes[]");
        assert_eq!(format!("{}", expr), "new bytes[]");

        let expr = parse_expr("new string[]");
        assert_eq!(format!("{}", expr), "new string[]");
    }

    #[test]
    fn test_display_new_expression_fixed_arrays() {
        let expr = parse_expr("new uint256[10]");
        assert_eq!(format!("{}", expr), "new uint256[10]");

        let expr = parse_expr("new address[5]");
        assert_eq!(format!("{}", expr), "new address[5]");

        let expr = parse_expr("new bytes32[100]");
        assert_eq!(format!("{}", expr), "new bytes32[100]");

        let expr = parse_expr("new bool[3]");
        assert_eq!(format!("{}", expr), "new bool[3]");
    }

    #[test]
    fn test_display_new_expression_multidimensional_arrays() {
        let expr = parse_expr("new uint256[][]");
        assert_eq!(format!("{}", expr), "new uint256[][]");

        let expr = parse_expr("new address[][10]");
        assert_eq!(format!("{}", expr), "new address[][10]");

        let expr = parse_expr("new bytes32[5][10]");
        assert_eq!(format!("{}", expr), "new bytes32[5][10]");

        let expr = parse_expr("new string[][][]");
        assert_eq!(format!("{}", expr), "new string[][][]");
    }

    #[test]
    fn test_display_new_expression_custom_types() {
        let expr = parse_expr("new CustomStruct");
        assert_eq!(format!("{}", expr), "new CustomStruct");

        let expr = parse_expr("new MyToken");
        assert_eq!(format!("{}", expr), "new MyToken");

        let expr = parse_expr("new LibraryContract");
        assert_eq!(format!("{}", expr), "new LibraryContract");

        let expr = parse_expr("new ImplementationV2");
        assert_eq!(format!("{}", expr), "new ImplementationV2");
    }

    #[test]
    fn test_display_combined_type_expressions() {
        // Test that type() and new expressions work correctly in complex contexts
        // Note: These may not all be valid Solidity, but test Display formatting

        let expr = parse_expr("type(uint256)");
        assert_eq!(format!("{}", expr), "type(uint256)");

        let expr = parse_expr("new Contract");
        assert_eq!(format!("{}", expr), "new Contract");

        // These would be parts of larger expressions in practice
        let expr = parse_expr("type(MyContract)");
        assert_eq!(format!("{}", expr), "type(MyContract)");

        let expr = parse_expr("new DynamicArray[]");
        assert_eq!(format!("{}", expr), "new DynamicArray[]");
    }
}
