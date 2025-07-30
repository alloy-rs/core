use crate::{Expr, Spanned, utils::ParseNested};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result, Token,
    parse::{Parse, ParseStream},
};

/// Access of a named member: `obj.k`.
#[derive(Clone)]
pub struct ExprMember {
    pub expr: Box<Expr>,
    pub dot_token: Token![.],
    pub member: Box<Expr>,
}

impl fmt::Debug for ExprMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprMember")
            .field("expr", &self.expr)
            .field("member", &self.member)
            .finish()
    }
}

impl ParseNested for ExprMember {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { expr, dot_token: input.parse()?, member: input.parse()? })
    }
}

derive_parse!(ExprMember);

impl fmt::Display for ExprMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.expr, self.member)
    }
}

impl Spanned for ExprMember {
    fn span(&self) -> Span {
        self.expr.span().join(self.member.span()).unwrap_or_else(|| {
            self.dot_token.span.join(self.member.span()).unwrap_or_else(|| self.expr.span())
        })
    }

    fn set_span(&mut self, span: Span) {
        self.expr.set_span(span);
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
    fn test_display_simple_member_access() {
        let expr = parse_expr("obj.field");
        assert_eq!(format!("{}", expr), "obj.field");

        let expr = parse_expr("contract.balance");
        assert_eq!(format!("{}", expr), "contract.balance");

        let expr = parse_expr("msg.sender");
        assert_eq!(format!("{}", expr), "msg.sender");

        let expr = parse_expr("block.timestamp");
        assert_eq!(format!("{}", expr), "block.timestamp");
    }

    #[test]
    fn test_display_chained_member_access() {
        let expr = parse_expr("a.b.c");
        assert_eq!(format!("{}", expr), "a.b.c");

        let expr = parse_expr("contract.storage.data");
        assert_eq!(format!("{}", expr), "contract.storage.data");

        let expr = parse_expr("user.profile.settings.theme");
        assert_eq!(format!("{}", expr), "user.profile.settings.theme");

        let expr = parse_expr("nested.object.property.value");
        assert_eq!(format!("{}", expr), "nested.object.property.value");
    }

    #[test]
    fn test_display_member_access_with_function_calls() {
        let expr = parse_expr("obj.method()");
        assert_eq!(format!("{}", expr), "obj.method()");

        let expr = parse_expr("contract.getBalance()");
        assert_eq!(format!("{}", expr), "contract.getBalance()");

        let expr = parse_expr("instance.getValue().toString()");
        assert_eq!(format!("{}", expr), "instance.getValue().toString()");

        let expr = parse_expr("service.api.getData().result");
        assert_eq!(format!("{}", expr), "service.api.getData().result");
    }

    #[test]
    fn test_display_member_access_with_array_indexing() {
        let expr = parse_expr("arr[0].field");
        assert_eq!(format!("{}", expr), "arr[0].field");

        let expr = parse_expr("matrix[i][j].value");
        assert_eq!(format!("{}", expr), "matrix[i][j].value");

        let expr = parse_expr("data[key].properties.length");
        assert_eq!(format!("{}", expr), "data[key].properties.length");

        let expr = parse_expr("users[index].profile.name");
        assert_eq!(format!("{}", expr), "users[index].profile.name");
    }

    #[test]
    fn test_display_member_access_with_complex_expressions() {
        let expr = parse_expr("(a + b).field");
        assert_eq!(format!("{}", expr), "(a + b).field");

        let expr = parse_expr("getValue().result.data");
        assert_eq!(format!("{}", expr), "getValue().result.data");

        let expr = parse_expr("contract.methods[\"transfer\"].call");
        assert_eq!(format!("{}", expr), "contract.methods[\"transfer\"].call");

        let expr = parse_expr("(condition ? obj1 : obj2).property");
        assert_eq!(format!("{}", expr), "(condition ? obj1 : obj2).property");
    }

    #[test]
    fn test_display_builtin_member_access() {
        let expr = parse_expr("msg.value");
        assert_eq!(format!("{}", expr), "msg.value");

        let expr = parse_expr("msg.data");
        assert_eq!(format!("{}", expr), "msg.data");

        let expr = parse_expr("msg.gas");
        assert_eq!(format!("{}", expr), "msg.gas");

        let expr = parse_expr("block.number");
        assert_eq!(format!("{}", expr), "block.number");

        let expr = parse_expr("block.difficulty");
        assert_eq!(format!("{}", expr), "block.difficulty");

        let expr = parse_expr("tx.origin");
        assert_eq!(format!("{}", expr), "tx.origin");

        let expr = parse_expr("tx.gasprice");
        assert_eq!(format!("{}", expr), "tx.gasprice");
    }

    #[test]
    fn test_display_member_access_with_operations() {
        let expr = parse_expr("balance.add(amount)");
        assert_eq!(format!("{}", expr), "balance.add(amount)");

        let expr = parse_expr("value.mul(rate).div(100)");
        assert_eq!(format!("{}", expr), "value.mul(rate).div(100)");

        let expr = parse_expr("token.balanceOf(address)");
        assert_eq!(format!("{}", expr), "token.balanceOf(address)");

        let expr = parse_expr("library.math.sqrt(number)");
        assert_eq!(format!("{}", expr), "library.math.sqrt(number)");
    }
}
