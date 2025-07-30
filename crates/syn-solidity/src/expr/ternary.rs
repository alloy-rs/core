use crate::{Expr, Spanned, utils::ParseNested};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result, Token,
    parse::{Parse, ParseStream},
};

/// A ternary (AKA conditional) expression: `foo ? bar : baz`.
#[derive(Clone)]
pub struct ExprTernary {
    pub cond: Box<Expr>,
    pub question_token: Token![?],
    pub if_true: Box<Expr>,
    pub colon_token: Token![:],
    pub if_false: Box<Expr>,
}

impl fmt::Debug for ExprTernary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprTernary")
            .field("cond", &self.cond)
            .field("if_true", &self.if_true)
            .field("if_false", &self.if_false)
            .finish()
    }
}

impl ParseNested for ExprTernary {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            cond: expr,
            question_token: input.parse()?,
            if_true: input.parse()?,
            colon_token: input.parse()?,
            if_false: input.parse()?,
        })
    }
}

derive_parse!(ExprTernary);

impl fmt::Display for ExprTernary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ? {} : {}", self.cond, self.if_true, self.if_false)
    }
}

impl Spanned for ExprTernary {
    fn span(&self) -> Span {
        let span = self.cond.span();
        span.join(self.if_false.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.cond.set_span(span);
        self.if_false.set_span(span);
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
    fn test_display_simple_ternary_expressions() {
        // expr "condition ? trueValue : falseValue" should display "condition ? trueValue :
        // falseValue"
        let expr = parse_expr("condition ? trueValue : falseValue");
        assert_eq!(format!("{}", expr), "condition ? trueValue : falseValue");

        let expr = parse_expr("x > 0 ? positive : negative");
        assert_eq!(format!("{}", expr), "x > 0 ? positive : negative");

        let expr = parse_expr("flag ? 1 : 0");
        assert_eq!(format!("{}", expr), "flag ? 1 : 0");

        let expr = parse_expr("isValid ? \"yes\" : \"no\"");
        assert_eq!(format!("{}", expr), "isValid ? \"yes\" : \"no\"");
    }

    #[test]
    fn test_display_nested_ternary_expressions() {
        let expr = parse_expr("a ? b ? c : d : e");
        assert_eq!(format!("{}", expr), "a ? b ? c : d : e");

        let expr = parse_expr("condition1 ? (condition2 ? value1 : value2) : value3");
        assert_eq!(format!("{}", expr), "condition1 ? (condition2 ? value1 : value2) : value3");

        let expr = parse_expr("x > 0 ? (y > 0 ? \"positive\" : \"zero\") : \"negative\"");
        assert_eq!(format!("{}", expr), "x > 0 ? (y > 0 ? \"positive\" : \"zero\") : \"negative\"");

        let expr = parse_expr("a ? b : c ? d : e");
        assert_eq!(format!("{}", expr), "a ? b : c ? d : e");
    }

    #[test]
    fn test_display_ternary_with_function_calls() {
        let expr = parse_expr("isOwner() ? getValue() : getDefault()");
        assert_eq!(format!("{}", expr), "isOwner() ? getValue() : getDefault()");

        let expr = parse_expr("condition ? func(arg1) : func(arg2)");
        assert_eq!(format!("{}", expr), "condition ? func(arg1) : func(arg2)");

        let expr = parse_expr("hasPermission() ? execute() : reject()");
        assert_eq!(format!("{}", expr), "hasPermission() ? execute() : reject()");

        let expr = parse_expr("checkBalance() ? transfer(amount) : revert()");
        assert_eq!(format!("{}", expr), "checkBalance() ? transfer(amount) : revert()");
    }

    #[test]
    fn test_display_ternary_with_member_access() {
        let expr = parse_expr("user.isActive ? user.balance : 0");
        assert_eq!(format!("{}", expr), "user.isActive ? user.balance : 0");

        let expr = parse_expr("msg.sender == owner ? contract.balance : 0");
        assert_eq!(format!("{}", expr), "msg.sender == owner ? contract.balance : 0");

        let expr = parse_expr("obj.flag ? obj.value.toString() : \"default\"");
        assert_eq!(format!("{}", expr), "obj.flag ? obj.value.toString() : \"default\"");

        let expr = parse_expr("token.paused ? 0 : token.totalSupply");
        assert_eq!(format!("{}", expr), "token.paused ? 0 : token.totalSupply");
    }

    #[test]
    fn test_display_ternary_with_array_access() {
        let expr = parse_expr("index < length ? array[index] : defaultValue");
        assert_eq!(format!("{}", expr), "index < length ? array[index] : defaultValue");

        let expr = parse_expr("hasKey ? mapping[key] : 0");
        assert_eq!(format!("{}", expr), "hasKey ? mapping[key] : 0");

        let expr = parse_expr("valid ? data[i][j] : matrix[0][0]");
        assert_eq!(format!("{}", expr), "valid ? data[i][j] : matrix[0][0]");

        let expr = parse_expr("inBounds ? values[position] : fallback");
        assert_eq!(format!("{}", expr), "inBounds ? values[position] : fallback");
    }

    #[test]
    fn test_display_ternary_with_complex_conditions() {
        let expr = parse_expr("a && b ? trueCase : falseCase");
        assert_eq!(format!("{}", expr), "a && b ? trueCase : falseCase");

        let expr = parse_expr("x > 0 && y < 10 ? compute() : skip()");
        assert_eq!(format!("{}", expr), "x > 0 && y < 10 ? compute() : skip()");

        let expr = parse_expr("balance >= amount && isApproved ? transfer : reject");
        assert_eq!(format!("{}", expr), "balance >= amount && isApproved ? transfer : reject");

        let expr = parse_expr("(condition1 || condition2) && condition3 ? result1 : result2");
        assert_eq!(
            format!("{}", expr),
            "(condition1 || condition2) && condition3 ? result1 : result2"
        );
    }

    #[test]
    fn test_display_ternary_with_arithmetic() {
        let expr = parse_expr("positive ? a + b : a - b");
        assert_eq!(format!("{}", expr), "positive ? a + b : a - b");

        let expr = parse_expr("multiply ? x * rate : x / rate");
        assert_eq!(format!("{}", expr), "multiply ? x * rate : x / rate");

        let expr = parse_expr("condition ? (value + bonus) : (value - penalty)");
        assert_eq!(format!("{}", expr), "condition ? (value + bonus) : (value - penalty)");

        let expr = parse_expr("useMax ? max(a, b) : min(a, b)");
        assert_eq!(format!("{}", expr), "useMax ? max(a, b) : min(a, b)");
    }

    #[test]
    fn test_display_complex_ternary_expressions() {
        let expr = parse_expr("owner == msg.sender ? (paused ? 0 : balance) : publicBalance");
        assert_eq!(
            format!("{}", expr),
            "owner == msg.sender ? (paused ? 0 : balance) : publicBalance"
        );

        let expr = parse_expr("condition ? func(arg1, arg2) : other.method(param)");
        assert_eq!(format!("{}", expr), "condition ? func(arg1, arg2) : other.method(param)");

        let expr = parse_expr("valid ? array[getIndex()] : fallbackArray[0]");
        assert_eq!(format!("{}", expr), "valid ? array[getIndex()] : fallbackArray[0]");

        let expr = parse_expr("isContract(addr) ? addr.call(data) : addr.transfer(value)");
        assert_eq!(
            format!("{}", expr),
            "isContract(addr) ? addr.call(data) : addr.transfer(value)"
        );
    }
}
