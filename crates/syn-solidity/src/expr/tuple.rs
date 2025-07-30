use crate::{Expr, Spanned, utils::DebugPunctuated};
use proc_macro2::Span;
use std::fmt;
use syn::{
    Result, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
};

/// A tuple expression: `(a, b, c, d)`.
#[derive(Clone)]
pub struct ExprTuple {
    pub paren_token: Paren,
    pub elems: Punctuated<Expr, Token![,]>,
}

impl fmt::Debug for ExprTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprTuple").field("elems", DebugPunctuated::new(&self.elems)).finish()
    }
}

impl Parse for ExprTuple {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            elems: content.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

impl fmt::Display for ExprTuple {
    /// Formats a tuple expression as valid Solidity source code.
    ///
    /// This implementation formats tuple expressions using parentheses with
    /// comma-separated elements. Elements are separated by a comma followed by
    /// a single space, following standard Solidity formatting conventions.
    ///
    /// # Format Pattern
    /// ```text
    /// (<element1>, <element2>, ..., <elementN>)
    /// ```
    ///
    /// # Examples
    ///
    /// **Empty tuple:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("()").unwrap();
    /// assert_eq!(format!("{}", expr), "()");
    /// ```
    ///
    /// **Single element (parenthesized expression):**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("(value)").unwrap();
    /// assert_eq!(format!("{}", expr), "(value)");
    ///
    /// let expr: Expr = parse_str("(42)").unwrap();
    /// assert_eq!(format!("{}", expr), "(42)");
    /// ```
    ///
    /// **Multiple elements:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("(a, b)").unwrap();
    /// assert_eq!(format!("{}", expr), "(a, b)");
    ///
    /// let expr: Expr = parse_str("(x, y, z)").unwrap();
    /// assert_eq!(format!("{}", expr), "(x, y, z)");
    ///
    /// let expr: Expr = parse_str("(1, 2, 3, 4, 5)").unwrap();
    /// assert_eq!(format!("{}", expr), "(1, 2, 3, 4, 5)");
    /// ```
    ///
    /// **Mixed types:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("(address, amount, true)").unwrap();
    /// assert_eq!(format!("{}", expr), "(address, amount, true)");
    ///
    /// let expr: Expr = parse_str("(\"hello\", 42, false)").unwrap();
    /// assert_eq!(format!("{}", expr), "(\"hello\", 42, false)");
    /// ```
    ///
    /// **Complex expressions as elements:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Tuple with arithmetic expressions
    /// let expr: Expr = parse_str("(a + b, x * y, c - d)").unwrap();
    /// assert_eq!(format!("{}", expr), "(a + b, x * y, c - d)");
    ///
    /// // Tuple with member access
    /// let expr: Expr = parse_str("(user.balance, msg.sender, block.timestamp)").unwrap();
    /// assert_eq!(format!("{}", expr), "(user.balance, msg.sender, block.timestamp)");
    ///
    /// // Tuple with function calls
    /// let expr: Expr = parse_str("(getBalance(), getAmount(), getTotal())").unwrap();
    /// assert_eq!(format!("{}", expr), "(getBalance(), getAmount(), getTotal())");
    /// ```
    ///
    /// **Nested tuples:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("((a, b), (c, d))").unwrap();
    /// assert_eq!(format!("{}", expr), "((a, b), (c, d))");
    ///
    /// let expr: Expr = parse_str("((), (1, 2), (x, y, z))").unwrap();
    /// assert_eq!(format!("{}", expr), "((), (1, 2), (x, y, z))");
    /// ```
    ///
    /// **Tuples with array and other complex expressions:**
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// let expr: Expr = parse_str("([1, 2, 3], mapping, struct)").unwrap();
    /// assert_eq!(format!("{}", expr), "([1, 2, 3], mapping, struct)");
    ///
    /// let expr: Expr = parse_str("(condition ? true : false, array[index])").unwrap();
    /// assert_eq!(format!("{}", expr), "(condition ? true : false, array[index])");
    /// ```
    ///
    /// # Common Use Cases in Solidity
    ///
    /// **Multiple return values:**
    /// ```solidity
    /// // Function returning multiple values
    /// function getBalances() returns (uint256, uint256, bool) {
    ///     return (balance1, balance2, isActive);
    /// }
    ///
    /// // Destructuring assignment
    /// (uint256 bal1, uint256 bal2, bool active) = getBalances();
    /// ```
    ///
    /// **Tuple assignments:**
    /// ```solidity
    /// // Swapping variables
    /// (a, b) = (b, a);
    ///
    /// // Multiple assignments
    /// (x, y, z) = (1, 2, 3);
    /// ```
    ///
    /// **Function arguments grouping:**
    /// ```solidity
    /// // Passing multiple related values
    /// processData((startIndex, endIndex, batchSize));
    ///
    /// // Complex parameter passing
    /// initializeContract((owner, treasury, fee), (startTime, endTime));
    /// ```
    ///
    /// **Event emissions:**
    /// ```solidity
    /// // Emitting events with multiple parameters
    /// emit Transfer(from, to, amount);
    /// // Often written as: emit Transfer((from, to, amount));
    /// ```
    ///
    /// # Parentheses vs Tuples
    ///
    /// In Solidity syntax parsing, parentheses serve dual purposes:
    ///
    /// 1. **Grouping expressions** (changing precedence): `(a + b) * c`
    /// 2. **Tuple literals**: `(a, b, c)`
    ///
    /// The Display implementation preserves parentheses regardless of their semantic
    /// purpose, maintaining the original expression structure.
    ///
    /// # Trailing Commas
    ///
    /// The formatter handles tuples that may have been parsed with trailing commas,
    /// but the output always omits the trailing comma for clean formatting:
    ///
    /// ```rust
    /// # use syn_solidity::Expr;
    /// # use syn::parse_str;
    /// // Input may have trailing comma, output will not
    /// let expr: Expr = parse_str("(a, b, c,)").unwrap();
    /// assert_eq!(format!("{}", expr), "(a, b, c)");
    /// ```
    ///
    /// # Memory and Performance Considerations
    ///
    /// While the Display implementation doesn't affect runtime behavior, tuple
    /// expressions in Solidity can have memory implications:
    ///
    /// - Multiple return values create temporary tuple structures
    /// - Large tuples may consume significant stack space
    /// - Tuple destructuring involves multiple memory operations
    ///
    /// The formatted output helps developers understand the structure and complexity
    /// of tuple expressions in their contracts.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        for (i, elem) in self.elems.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            elem.fmt(f)?;
        }
        f.write_str(")")
    }
}

impl Spanned for ExprTuple {
    fn span(&self) -> Span {
        self.paren_token.span.join()
    }

    fn set_span(&mut self, span: Span) {
        self.paren_token = Paren(span);
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
    fn test_display_empty_tuple() {
        // expr "()" should display "()"
        let expr = parse_expr("()");
        assert_eq!(format!("{}", expr), "()");
    }

    #[test]
    fn test_display_single_element_tuple() {
        // expr "(single)" should display "(single)"
        let expr = parse_expr("(single)");
        assert_eq!(format!("{}", expr), "(single)");

        let expr = parse_expr("(42)");
        assert_eq!(format!("{}", expr), "(42)");

        let expr = parse_expr("(variable)");
        assert_eq!(format!("{}", expr), "(variable)");

        let expr = parse_expr("(\"string\")");
        assert_eq!(format!("{}", expr), "(\"string\")");
    }

    #[test]
    fn test_display_two_element_tuple() {
        let expr = parse_expr("(a, b)");
        assert_eq!(format!("{}", expr), "(a, b)");

        let expr = parse_expr("(first, second)");
        assert_eq!(format!("{}", expr), "(first, second)");

        let expr = parse_expr("(1, 2)");
        assert_eq!(format!("{}", expr), "(1, 2)");

        let expr = parse_expr("(true, false)");
        assert_eq!(format!("{}", expr), "(true, false)");
    }

    #[test]
    fn test_display_multiple_element_tuple() {
        let expr = parse_expr("(a, b, c)");
        assert_eq!(format!("{}", expr), "(a, b, c)");

        let expr = parse_expr("(x, y, z, w)");
        assert_eq!(format!("{}", expr), "(x, y, z, w)");

        let expr = parse_expr("(1, 2, 3, 4, 5)");
        assert_eq!(format!("{}", expr), "(1, 2, 3, 4, 5)");

        let expr = parse_expr("(first, second, third, fourth)");
        assert_eq!(format!("{}", expr), "(first, second, third, fourth)");
    }

    #[test]
    fn test_display_tuple_with_complex_expressions() {
        let expr = parse_expr("(a + b, c * d)");
        assert_eq!(format!("{}", expr), "(a + b, c * d)");

        let expr = parse_expr("(func(), variable)");
        assert_eq!(format!("{}", expr), "(func(), variable)");

        let expr = parse_expr("(array[index], obj.field)");
        assert_eq!(format!("{}", expr), "(array[index], obj.field)");

        let expr = parse_expr("(condition ? true : false, another.value)");
        assert_eq!(format!("{}", expr), "(condition ? true : false, another.value)");
    }

    #[test]
    fn test_display_nested_tuples() {
        let expr = parse_expr("((a, b), c)");
        assert_eq!(format!("{}", expr), "((a, b), c)");

        let expr = parse_expr("(x, (y, z))");
        assert_eq!(format!("{}", expr), "(x, (y, z))");

        let expr = parse_expr("((a, b), (c, d))");
        assert_eq!(format!("{}", expr), "((a, b), (c, d))");

        let expr = parse_expr("(((inner)), outer)");
        assert_eq!(format!("{}", expr), "(((inner)), outer)");
    }

    #[test]
    fn test_display_tuple_with_function_calls() {
        let expr = parse_expr("(getValue(), getOther())");
        assert_eq!(format!("{}", expr), "(getValue(), getOther())");

        let expr = parse_expr("(func(arg1, arg2), simple)");
        assert_eq!(format!("{}", expr), "(func(arg1, arg2), simple)");

        let expr = parse_expr("(contract.method(), variable.field)");
        assert_eq!(format!("{}", expr), "(contract.method(), variable.field)");

        let expr = parse_expr("(a.b.c(), d.e.f())");
        assert_eq!(format!("{}", expr), "(a.b.c(), d.e.f())");
    }

    #[test]
    fn test_display_tuple_with_array_access() {
        let expr = parse_expr("(array[0], array[1])");
        assert_eq!(format!("{}", expr), "(array[0], array[1])");

        let expr = parse_expr("(matrix[i][j], vector[k])");
        assert_eq!(format!("{}", expr), "(matrix[i][j], vector[k])");

        let expr = parse_expr("(data[key], fallback)");
        assert_eq!(format!("{}", expr), "(data[key], fallback)");

        let expr = parse_expr("(mapping[user], defaultValue)");
        assert_eq!(format!("{}", expr), "(mapping[user], defaultValue)");
    }

    #[test]
    fn test_display_tuple_with_mixed_types() {
        let expr = parse_expr("(42, \"string\", true)");
        assert_eq!(format!("{}", expr), "(42, \"string\", true)");

        let expr = parse_expr("(address, balance, active)");
        assert_eq!(format!("{}", expr), "(address, balance, active)");

        let expr = parse_expr("(getValue(), 100, flag)");
        assert_eq!(format!("{}", expr), "(getValue(), 100, flag)");

        let expr = parse_expr("(user.id, user.name, user.isActive)");
        assert_eq!(format!("{}", expr), "(user.id, user.name, user.isActive)");
    }

    #[test]
    fn test_display_tuple_assignments_and_destructuring() {
        // Note: These test the display of tuple expressions, not assignment syntax
        let expr = parse_expr("(a, b, c)");
        assert_eq!(format!("{}", expr), "(a, b, c)");

        let expr = parse_expr("(first, second, third)");
        assert_eq!(format!("{}", expr), "(first, second, third)");

        let expr = parse_expr("(x, y)");
        assert_eq!(format!("{}", expr), "(x, y)");

        let expr = parse_expr("(result1, result2, result3)");
        assert_eq!(format!("{}", expr), "(result1, result2, result3)");
    }
}
