use heck::{ToKebabCase, ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use syn::{Attribute, Error, LitStr, Result};

pub fn docs(attrs: &[Attribute]) -> impl Iterator<Item = &Attribute> {
    attrs.iter().filter(|attr| attr.path().is_ident("doc"))
}

pub fn derives(attrs: &[Attribute]) -> impl Iterator<Item = &Attribute> {
    attrs.iter().filter(|attr| attr.path().is_ident("derive"))
}

/// `#[sol(...)]` attributes.
///
/// When adding a new attribute:
/// 1. add a field to this struct,
/// 2. add a match arm in the `parse` function below,
/// 3. add test cases in the `tests` module at the bottom of this file,
/// 4. implement the attribute in the `expand` module,
/// 5. document the attribute in the [`crate::sol!`] macro docs.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SolAttrs {
    pub all_derives: Option<()>,
    // TODO: Implement
    pub rename: Option<LitStr>,
    // TODO: Implement
    pub rename_all: Option<CasingStyle>,

    pub bytecode: Option<LitStr>,
    pub deployed_bytecode: Option<LitStr>,
}

impl SolAttrs {
    pub fn parse(attrs: &[Attribute]) -> Result<(Self, Vec<Attribute>)> {
        let mut this = Self::default();
        let mut others = Vec::with_capacity(attrs.len());
        for attr in attrs {
            if !attr.path().is_ident("sol") {
                others.push(attr.clone());
                continue
            }

            attr.meta.require_list()?.parse_nested_meta(|meta| {
                let path = meta
                    .path
                    .get_ident()
                    .ok_or_else(|| meta.error("expected ident"))?;
                let s = path.to_string();

                macro_rules! match_ {
                    ($($l:ident => $e:expr),* $(,)?) => {
                        match s.as_str() {
                            $(
                                stringify!($l) => if this.$l.is_some() {
                                    return Err(meta.error("duplicate attribute"))
                                } else {
                                    this.$l = Some($e);
                                },
                            )*
                            _ => return Err(meta.error("unknown `sol` attribute")),
                        }
                    };
                }

                let lit = || meta.value()?.parse::<LitStr>();
                let bytes = || {
                    let lit = lit()?;
                    let v = lit.value();
                    let v = v.strip_prefix("0x").unwrap_or(&v);
                    if v.contains(|c: char| !c.is_ascii_hexdigit()) {
                        return Err(Error::new(lit.span(), "expected hex literal"))
                    }
                    if v.len() % 2 != 0 {
                        return Err(Error::new(lit.span(), "expected even number of hex digits"))
                    }
                    Ok(LitStr::new(v, lit.span()))
                };

                match_! {
                    all_derives => (),
                    rename => lit()?,
                    rename_all => CasingStyle::from_lit(&lit()?)?,

                    bytecode => bytes()?,
                    deployed_bytecode => bytes()?,
                };
                Ok(())
            })?;
        }
        Ok((this, others))
    }
}

/// Defines the casing for the attributes long representation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CasingStyle {
    /// Indicate word boundaries with uppercase letter, excluding the first
    /// word.
    Camel,
    /// Keep all letters lowercase and indicate word boundaries with hyphens.
    Kebab,
    /// Indicate word boundaries with uppercase letter, including the first
    /// word.
    Pascal,
    /// Keep all letters uppercase and indicate word boundaries with
    /// underscores.
    ScreamingSnake,
    /// Keep all letters lowercase and indicate word boundaries with
    /// underscores.
    Snake,
    /// Keep all letters lowercase and remove word boundaries.
    Lower,
    /// Keep all letters uppercase and remove word boundaries.
    Upper,
    /// Use the original attribute name defined in the code.
    Verbatim,
}

impl CasingStyle {
    fn from_lit(name: &LitStr) -> Result<Self> {
        let normalized = name.value().to_upper_camel_case().to_lowercase();
        let s = match normalized.as_ref() {
            "camel" | "camelcase" => Self::Camel,
            "kebab" | "kebabcase" => Self::Kebab,
            "pascal" | "pascalcase" => Self::Pascal,
            "screamingsnake" | "screamingsnakecase" => Self::ScreamingSnake,
            "snake" | "snakecase" => Self::Snake,
            "lower" | "lowercase" => Self::Lower,
            "upper" | "uppercase" => Self::Upper,
            "verbatim" | "verbatimcase" => Self::Verbatim,
            s => return Err(Error::new(name.span(), format!("unsupported casing: {s}"))),
        };
        Ok(s)
    }

    /// Apply the casing style to the given string.
    #[allow(dead_code)]
    pub fn apply(self, s: &str) -> String {
        match self {
            Self::Pascal => s.to_upper_camel_case(),
            Self::Kebab => s.to_kebab_case(),
            Self::Camel => s.to_lower_camel_case(),
            Self::ScreamingSnake => s.to_shouty_snake_case(),
            Self::Snake => s.to_snake_case(),
            Self::Lower => s.to_snake_case().replace('_', ""),
            Self::Upper => s.to_shouty_snake_case().replace('_', ""),
            Self::Verbatim => s.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    macro_rules! test_sol_attrs {
        ($( $(#[$attr:meta])* => $expected:expr ),+ $(,)?) => {$(
            run_test(
                &[$(stringify!(#[$attr])),*],
                $expected
            );
        )+};
    }

    macro_rules! sol_attrs {
        ($($id:ident : $e:expr),* $(,)?) => {
            SolAttrs {
                $($id: Some($e),)*
                ..Default::default()
            }
        };
    }

    struct OuterAttribute(Vec<Attribute>);

    impl syn::parse::Parse for OuterAttribute {
        fn parse(input: syn::parse::ParseStream<'_>) -> Result<Self> {
            input.call(Attribute::parse_outer).map(Self)
        }
    }

    fn run_test(
        attrs_s: &'static [&'static str],
        expected: std::result::Result<SolAttrs, &'static str>,
    ) {
        let attrs: Vec<Attribute> = attrs_s
            .iter()
            .flat_map(|s| syn::parse_str::<OuterAttribute>(s).unwrap().0)
            .collect();
        match (SolAttrs::parse(&attrs), expected) {
            (Ok((actual, _)), Ok(expected)) => assert_eq!(actual, expected, "{attrs_s:?}"),
            (Err(actual), Err(expected)) => assert_eq!(actual.to_string(), expected, "{attrs_s:?}"),
            (a, b) => panic!("assertion failed: `{a:?} != {b:?}`: {attrs_s:?}"),
        }
    }

    #[test]
    fn sol_attrs() {
        test_sol_attrs! {
            #[cfg()] => Ok(SolAttrs::default()),
            #[derive()] #[sol()] => Ok(SolAttrs::default()),
            #[sol()] => Ok(SolAttrs::default()),
            #[sol()] #[sol()] => Ok(SolAttrs::default()),
            #[sol = ""] => Err("expected `(`"),
            #[sol] => Err("expected attribute arguments in parentheses: `sol(...)`"),

            #[sol(() = "")] => Err("unexpected token in nested attribute, expected ident"),
            #[sol(? = "")] => Err("unexpected token in nested attribute, expected ident"),
            #[sol(a::b = "")] => Err("expected ident"),

            #[sol(all_derives)] => Ok(sol_attrs! { all_derives: () }),
            #[sol(all_derives)] #[sol(all_derives)] => Err("duplicate attribute"),

            #[sol(rename = "foo")] => Ok(sol_attrs! { rename: parse_quote!("foo") }),

            #[sol(rename_all = "foo")] => Err("unsupported casing: foo"),
            #[sol(rename_all = "camelcase")] => Ok(sol_attrs! { rename_all: CasingStyle::Camel }),
            #[sol(rename_all = "camelCase")] #[sol(rename_all = "PascalCase")] => Err("duplicate attribute"),

            #[sol(deployed_bytecode = "0x1234")] => Ok(sol_attrs! { deployed_bytecode: parse_quote!("1234") }),
            #[sol(bytecode = "0x1234")] => Ok(sol_attrs! { bytecode: parse_quote!("1234") }),
            #[sol(bytecode = "1234")] => Ok(sol_attrs! { bytecode: parse_quote!("1234") }),
            #[sol(bytecode = "0x123xyz")] => Err("expected hex literal"),
            #[sol(bytecode = "12 34")] => Err("expected hex literal"),
            #[sol(bytecode = "xyz")] => Err("expected hex literal"),
            #[sol(bytecode = "123")] => Err("expected even number of hex digits"),
        }
    }
}
