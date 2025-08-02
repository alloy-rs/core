use crate::{Expr, Spanned, kw, utils::ParseNested};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

/// A unary operation: `!x`, `-x`.
#[derive(Clone, Debug)]
pub struct ExprUnary {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl Parse for ExprUnary {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { op: input.parse()?, expr: input.parse()? })
    }
}

impl fmt::Display for ExprUnary {
    /// Formats a prefix unary expression as valid Solidity source code.
    ///
    /// This implementation formats prefix unary operations by placing the operator
    /// directly before the operand expression without any spacing. This follows
    /// standard Solidity syntax for unary prefix operators.
    ///
    /// # Format Pattern
    /// ```text
    /// <operator><operand_expr>
    /// ```
    ///
    /// # Supported Operators
    ///
    /// **Logical negation:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("!condition").unwrap();
    /// assert_eq!(format!("{}", expr), "!condition");
    ///
    /// let expr: Expr = parse_str("!isValid").unwrap();
    /// assert_eq!(format!("{}", expr), "!isValid");
    /// ```
    ///
    /// **Arithmetic negation:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("-amount").unwrap();
    /// assert_eq!(format!("{}", expr), "-amount");
    ///
    /// let expr: Expr = parse_str("-balance").unwrap();
    /// assert_eq!(format!("{}", expr), "-balance");
    /// ```
    ///
    /// **Bitwise complement:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("~mask").unwrap();
    /// assert_eq!(format!("{}", expr), "~mask");
    ///
    /// let expr: Expr = parse_str("~bits").unwrap();
    /// assert_eq!(format!("{}", expr), "~bits");
    /// ```
    ///
    /// **Pre-increment:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("++counter").unwrap();
    /// assert_eq!(format!("{}", expr), "++counter");
    ///
    /// let expr: Expr = parse_str("++index").unwrap();
    /// assert_eq!(format!("{}", expr), "++index");
    /// ```
    ///
    /// **Pre-decrement:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("--counter").unwrap();
    /// assert_eq!(format!("{}", expr), "--counter");
    ///
    /// let expr: Expr = parse_str("--index").unwrap();
    /// assert_eq!(format!("{}", expr), "--index");
    /// ```
    ///
    /// # Complex Expressions
    ///
    /// Unary operators can be applied to complex expressions:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Negation of member access
    /// let expr: Expr = parse_str("!msg.sender").unwrap();
    /// assert_eq!(format!("{}", expr), "!msg.sender");
    ///
    /// // Negation of function call
    /// let expr: Expr = parse_str("!isAuthorized()").unwrap();
    /// assert_eq!(format!("{}", expr), "!isAuthorized()");
    ///
    /// // Arithmetic negation of array access
    /// let expr: Expr = parse_str("-balances[user]").unwrap();
    /// assert_eq!(format!("{}", expr), "-balances[user]");
    /// ```
    ///
    /// # Operator Precedence
    ///
    /// Unary operators have high precedence and bind tightly to their operands.
    /// When combined with other operators, the precedence is preserved:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Unary minus has higher precedence than addition
    /// let expr: Expr = parse_str("-a + b").unwrap();
    /// assert_eq!(format!("{}", expr), "-a + b");
    ///
    /// // Multiple unary operators can be chained
    /// let expr: Expr = parse_str("!!flag").unwrap();
    /// assert_eq!(format!("{}", expr), "!!flag");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.op, self.expr)
    }
}

impl Spanned for ExprUnary {
    fn span(&self) -> Span {
        let span = self.op.span();
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.op.set_span(span);
        self.expr.set_span(span);
    }
}

/// A unary `delete` expression: `delete vector`.
#[derive(Clone)]
pub struct ExprDelete {
    pub delete_token: kw::delete,
    pub expr: Box<Expr>,
}

impl fmt::Debug for ExprDelete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprDelete").field("expr", &self.expr).finish()
    }
}

impl Parse for ExprDelete {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { delete_token: input.parse()?, expr: input.parse()? })
    }
}

impl fmt::Display for ExprDelete {
    /// Formats a delete expression as valid Solidity source code.
    ///
    /// This implementation formats delete operations by placing the `delete` keyword
    /// followed by a single space and the operand expression. This follows standard
    /// Solidity syntax for delete statements.
    ///
    /// # Format Pattern
    /// ```text
    /// delete <operand_expr>
    /// ```
    ///
    /// # Examples
    ///
    /// **Delete array element:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("delete array[index]").unwrap();
    /// assert_eq!(format!("{}", expr), "delete array[index]");
    ///
    /// let expr: Expr = parse_str("delete myArray[0]").unwrap();
    /// assert_eq!(format!("{}", expr), "delete myArray[0]");
    /// ```
    ///
    /// **Delete mapping value:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("delete balances[account]").unwrap();
    /// assert_eq!(format!("{}", expr), "delete balances[account]");
    ///
    /// let expr: Expr = parse_str("delete userProfiles[userId]").unwrap();
    /// assert_eq!(format!("{}", expr), "delete userProfiles[userId]");
    /// ```
    ///
    /// **Delete struct field:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("delete user.profile").unwrap();
    /// assert_eq!(format!("{}", expr), "delete user.profile");
    ///
    /// let expr: Expr = parse_str("delete storage.data").unwrap();
    /// assert_eq!(format!("{}", expr), "delete storage.data");
    /// ```
    ///
    /// **Delete variable:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("delete myVariable").unwrap();
    /// assert_eq!(format!("{}", expr), "delete myVariable");
    ///
    /// let expr: Expr = parse_str("delete temp").unwrap();
    /// assert_eq!(format!("{}", expr), "delete temp");
    /// ```
    ///
    /// # Complex Expressions
    ///
    /// The delete operator can be applied to complex expressions involving
    /// nested member access, array indexing, and function calls:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Delete nested array element
    /// let expr: Expr = parse_str("delete users[id].tokens[index]").unwrap();
    /// assert_eq!(format!("{}", expr), "delete users[id].tokens[index]");
    ///
    /// // Delete result of function call indexing
    /// let expr: Expr = parse_str("delete getArray()[position]").unwrap();
    /// assert_eq!(format!("{}", expr), "delete getArray()[position]");
    /// ```
    ///
    /// # Solidity Semantics
    ///
    /// The `delete` operator in Solidity resets the value to its default:
    /// - For integers: resets to 0
    /// - For booleans: resets to false
    /// - For arrays: resets length to 0
    /// - For mappings: resets the key to default value
    /// - For structs: resets all members to their default values
    ///
    /// The Display implementation preserves the original syntax without
    /// interpreting the semantic effects of the delete operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "delete {}", self.expr)
    }
}

impl Spanned for ExprDelete {
    fn span(&self) -> Span {
        let span = self.delete_token.span;
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.delete_token.span = span;
        self.expr.set_span(span);
    }
}

/// A postfix unary expression: `foo++`.
#[derive(Clone, Debug)]
pub struct ExprPostfix {
    pub expr: Box<Expr>,
    pub op: PostUnOp,
}

impl ParseNested for ExprPostfix {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { expr, op: input.parse()? })
    }
}

derive_parse!(ExprPostfix);

impl fmt::Display for ExprPostfix {
    /// Formats a postfix unary expression as valid Solidity source code.
    ///
    /// This implementation formats postfix unary operations by placing the operator
    /// directly after the operand expression without any spacing. This follows
    /// standard Solidity syntax for postfix operators.
    ///
    /// # Format Pattern
    /// ```text
    /// <operand_expr><operator>
    /// ```
    ///
    /// # Supported Operators
    ///
    /// **Post-increment:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("counter++").unwrap();
    /// assert_eq!(format!("{}", expr), "counter++");
    ///
    /// let expr: Expr = parse_str("index++").unwrap();
    /// assert_eq!(format!("{}", expr), "index++");
    /// ```
    ///
    /// **Post-decrement:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("counter--").unwrap();
    /// assert_eq!(format!("{}", expr), "counter--");
    ///
    /// let expr: Expr = parse_str("index--").unwrap();
    /// assert_eq!(format!("{}", expr), "index--");
    /// ```
    ///
    /// # Complex Expressions
    ///
    /// Postfix operators can be applied to complex expressions:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Postfix increment of array element
    /// let expr: Expr = parse_str("counters[i]++").unwrap();
    /// assert_eq!(format!("{}", expr), "counters[i]++");
    ///
    /// // Postfix decrement of struct field
    /// let expr: Expr = parse_str("user.balance--").unwrap();
    /// assert_eq!(format!("{}", expr), "user.balance--");
    ///
    /// // Postfix increment of member access
    /// let expr: Expr = parse_str("storage.count++").unwrap();
    /// assert_eq!(format!("{}", expr), "storage.count++");
    /// ```
    ///
    /// # Operator Precedence and Semantics
    ///
    /// Postfix operators have the highest precedence in Solidity and are
    /// evaluated before most other operations:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // The increment happens after the current value is used
    /// let expr: Expr = parse_str("arr[index++]").unwrap();
    /// assert_eq!(format!("{}", expr), "arr[index++]");
    ///
    /// // Complex expression with postfix and other operators
    /// let expr: Expr = parse_str("value++ * 2").unwrap();
    /// assert_eq!(format!("{}", expr), "value++ * 2");
    /// ```
    ///
    /// # Difference from Prefix Operators
    ///
    /// Unlike prefix operators (`++var`, `--var`), postfix operators return
    /// the value before the operation is applied:
    ///
    /// - `var++`: returns current value of `var`, then increments
    /// - `++var`: increments `var`, then returns new value
    /// - `var--`: returns current value of `var`, then decrements
    /// - `--var`: decrements `var`, then returns new value
    ///
    /// The Display implementation preserves the original syntax without
    /// interpreting the semantic differences between pre and post operations.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.expr, self.op)
    }
}

impl Spanned for ExprPostfix {
    fn span(&self) -> Span {
        let span = self.op.span();
        span.join(self.expr.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.op.set_span(span);
        self.expr.set_span(span);
    }
}

op_enum! {
    /// Unary operators.
    pub enum UnOp {
        Increment(++) peek2,
        Decrement(--) peek2,
        Not(!),
        BitNot(~),
        Neg(-),
    }
}

op_enum! {
    /// Postfix unary operators.
    pub enum PostUnOp {
        Increment(++) peek2,
        Decrement(--) peek2,
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
    fn test_display_unary_prefix_expressions() {
        // expr "!flag" should display "!flag"
        let expr = parse_expr("!flag");
        assert_eq!(format!("{}", expr), "!flag");

        // expr "-value" should display "-value"
        let expr = parse_expr("-value");
        assert_eq!(format!("{}", expr), "-value");

        // expr "~bits" should display "~bits"
        let expr = parse_expr("~bits");
        assert_eq!(format!("{}", expr), "~bits");

        // expr "++counter" should display "++counter"
        let expr = parse_expr("++counter");
        assert_eq!(format!("{}", expr), "++counter");

        // expr "--index" should display "--index"
        let expr = parse_expr("--index");
        assert_eq!(format!("{}", expr), "--index");
    }

    #[test]
    fn test_display_postfix_expressions() {
        // expr "counter++" should display "counter++"
        let expr = parse_expr("counter++");
        assert_eq!(format!("{}", expr), "counter++");

        // expr "index--" should display "index--"
        let expr = parse_expr("index--");
        assert_eq!(format!("{}", expr), "index--");

        // expr "variable++" should display "variable++"
        let expr = parse_expr("variable++");
        assert_eq!(format!("{}", expr), "variable++");

        // expr "array[i]++" should display "array[i]++"
        let expr = parse_expr("array[i]++");
        assert_eq!(format!("{}", expr), "array[i]++");
    }

    #[test]
    fn test_display_delete_expressions() {
        // expr "delete myArray" should display "delete myArray"
        let expr = parse_expr("delete myArray");
        assert_eq!(format!("{}", expr), "delete myArray");

        // expr "delete storage.field" should display "delete storage.field"
        let expr = parse_expr("delete storage.field");
        assert_eq!(format!("{}", expr), "delete storage.field");

        // expr "delete mapping[key]" should display "delete mapping[key]"
        let expr = parse_expr("delete mapping[key]");
        assert_eq!(format!("{}", expr), "delete mapping[key]");

        // expr "delete data" should display "delete data"
        let expr = parse_expr("delete data");
        assert_eq!(format!("{}", expr), "delete data");
    }

    #[test]
    fn test_complex_unary_expressions() {
        let expr = parse_expr("!condition && flag");
        assert_eq!(format!("{}", expr), "!condition && flag");

        let expr = parse_expr("-(a + b)");
        assert_eq!(format!("{}", expr), "-(a + b)");

        let expr = parse_expr("~value & mask");
        assert_eq!(format!("{}", expr), "~value & mask");

        let expr = parse_expr("++array[index]");
        assert_eq!(format!("{}", expr), "++array[index]");
    }

    #[test]
    fn test_nested_unary_expressions() {
        let expr = parse_expr("!!flag");
        assert_eq!(format!("{}", expr), "!!flag");

        let expr = parse_expr("--counter");
        assert_eq!(format!("{}", expr), "--counter");

        let expr = parse_expr("~-value");
        assert_eq!(format!("{}", expr), "~-value");

        let expr = parse_expr("!~bits");
        assert_eq!(format!("{}", expr), "!~bits");
    }

    #[test]
    fn test_unary_with_member_access() {
        let expr = parse_expr("!obj.flag");
        assert_eq!(format!("{}", expr), "!obj.flag");

        let expr = parse_expr("-contract.balance");
        assert_eq!(format!("{}", expr), "-contract.balance");

        let expr = parse_expr("msg.sender++");
        assert_eq!(format!("{}", expr), "msg.sender++");

        let expr = parse_expr("delete storage.data");
        assert_eq!(format!("{}", expr), "delete storage.data");
    }

    #[test]
    fn test_unary_with_function_calls() {
        let expr = parse_expr("!isValid()");
        assert_eq!(format!("{}", expr), "!isValid()");

        let expr = parse_expr("-getValue()");
        assert_eq!(format!("{}", expr), "-getValue()");

        let expr = parse_expr("getCounter()++");
        assert_eq!(format!("{}", expr), "getCounter()++");

        let expr = parse_expr("delete getMapping()");
        assert_eq!(format!("{}", expr), "delete getMapping()");
    }
}
