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
    /// Formats a ternary (conditional) expression as valid Solidity source code.
    ///
    /// This implementation formats ternary expressions using the standard
    /// conditional operator syntax with proper spacing around the `?` and `:`
    /// operators. This follows standard Solidity formatting conventions for
    /// conditional expressions.
    ///
    /// # Format Pattern
    /// ```text
    /// <condition_expr> ? <true_expr> : <false_expr>
    /// ```
    ///
    /// # Examples
    ///
    /// **Simple conditionals:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("condition ? trueValue : falseValue").unwrap();
    /// assert_eq!(format!("{}", expr), "condition ? trueValue : falseValue");
    ///
    /// let expr: Expr = parse_str("isValid ? result : error").unwrap();
    /// assert_eq!(format!("{}", expr), "isValid ? result : error");
    /// ```
    ///
    /// **Numeric conditionals:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("balance > 0 ? balance : 0").unwrap();
    /// assert_eq!(format!("{}", expr), "balance > 0 ? balance : 0");
    ///
    /// let expr: Expr = parse_str("x >= y ? x : y").unwrap();
    /// assert_eq!(format!("{}", expr), "x >= y ? x : y");
    /// ```
    ///
    /// **Authorization checks:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("msg.sender == owner ? allowed : denied").unwrap();
    /// assert_eq!(format!("{}", expr), "msg.sender == owner ? allowed : denied");
    ///
    /// let expr: Expr = parse_str("hasPermission ? proceed : revert").unwrap();
    /// assert_eq!(format!("{}", expr), "hasPermission ? proceed : revert");
    /// ```
    ///
    /// **Complex conditions:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("balance >= amount && approved ? success : failure").unwrap();
    /// assert_eq!(format!("{}", expr), "balance >= amount && approved ? success : failure");
    ///
    /// let expr: Expr = parse_str("value > 0 || emergency ? continue : stop").unwrap();
    /// assert_eq!(format!("{}", expr), "value > 0 || emergency ? continue : stop");
    /// ```
    ///
    /// **Function call results:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("checkCondition() ? getValue() : getDefault()").unwrap();
    /// assert_eq!(format!("{}", expr), "checkCondition() ? getValue() : getDefault()");
    ///
    /// let expr: Expr = parse_str("isAuthorized() ? transfer(amount) : revert()").unwrap();
    /// assert_eq!(format!("{}", expr), "isAuthorized() ? transfer(amount) : revert()");
    /// ```
    ///
    /// # Nested Ternary Expressions
    ///
    /// Ternary expressions can be nested for complex conditional logic:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Nested condition in the true branch
    /// let expr: Expr = parse_str("outer ? (inner ? value1 : value2) : value3").unwrap();
    /// assert_eq!(format!("{}", expr), "outer ? (inner ? value1 : value2) : value3");
    ///
    /// // Nested condition in the false branch
    /// let expr: Expr = parse_str("primary ? result : (secondary ? backup : default)").unwrap();
    /// assert_eq!(format!("{}", expr), "primary ? result : (secondary ? backup : default)");
    ///
    /// // Chained ternary operations
    /// let expr: Expr = parse_str("a ? b : c ? d : e").unwrap();
    /// assert_eq!(format!("{}", expr), "a ? b : c ? d : e");
    /// ```
    ///
    /// # Complex Expression Components
    ///
    /// Any part of the ternary can be a complex expression:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Complex condition with member access
    /// let expr: Expr = parse_str("user.balance >= cost ? purchase : cancel").unwrap();
    /// assert_eq!(format!("{}", expr), "user.balance >= cost ? purchase : cancel");
    ///
    /// // Complex true/false expressions
    /// let expr: Expr = parse_str("valid ? users[id].balance : accounts[id].funds").unwrap();
    /// assert_eq!(format!("{}", expr), "valid ? users[id].balance : accounts[id].funds");
    ///
    /// // All complex components
    /// let expr: Expr =
    ///     parse_str("getUser().isActive() ? calculateReward(amount) : processRefund(fee)").unwrap();
    /// assert_eq!(
    ///     format!("{}", expr),
    ///     "getUser().isActive() ? calculateReward(amount) : processRefund(fee)"
    /// );
    /// ```
    ///
    /// # Operator Precedence
    ///
    /// The ternary operator has relatively low precedence. Most other operators
    /// are evaluated before the ternary condition is evaluated:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Arithmetic is evaluated before ternary
    /// let expr: Expr = parse_str("a + b > c ? x * 2 : y / 3").unwrap();
    /// assert_eq!(format!("{}", expr), "a + b > c ? x * 2 : y / 3");
    ///
    /// // Comparison is evaluated before ternary
    /// let expr: Expr = parse_str("balance >= amount ? approved : rejected").unwrap();
    /// assert_eq!(format!("{}", expr), "balance >= amount ? approved : rejected");
    /// ```
    ///
    /// # Right-Associative Behavior
    ///
    /// Ternary operators are right-associative, meaning chained ternary
    /// expressions are evaluated from right to left:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Equivalent to: a ? b : (c ? d : e)
    /// let expr: Expr = parse_str("a ? b : c ? d : e").unwrap();
    /// assert_eq!(format!("{}", expr), "a ? b : c ? d : e");
    /// ```
    ///
    /// # Use Cases
    ///
    /// Ternary expressions are commonly used for:
    /// - Conditional assignments
    /// - Default value selection
    /// - Authorization-based execution paths
    /// - Mathematical operations (min/max selection)
    /// - Error handling and fallback values
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
