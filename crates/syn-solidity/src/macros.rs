#[cfg(any(feature = "visit", feature = "visit-mut"))]
macro_rules! make_visitor {
    (
        $(#[$attr:meta])*
        trait $trait_name:ident $(is $mut:ident)?;
    ) => {
        make_visitor! {
            @impl
            $(#[$attr])*
            pub trait $trait_name<'ast> {
                fn visit_ident(&mut v, ident: &'ast $($mut)? SolIdent) {
                    // nothing to do
                }

                fn visit_path(&mut v, ident: &'ast $($mut)? SolPath) {
                    // nothing to do
                }

                fn visit_storage(&mut v, storage: &'ast $($mut)? Storage) {
                    // nothing to do
                }

                fn visit_type(&mut v, ty: &'ast $($mut)? Type) {
                    match ty {
                        Type::Array(array) => v.visit_type(& $($mut)? array.ty),
                        Type::Tuple(tuple) => {
                            for ty in & $($mut)? tuple.types {
                                v.visit_type(ty);
                            }
                        },
                        Type::Custom(name) => v.visit_path(name),
                        _ => {}
                    }
                }

                fn visit_variable_declaration(&mut v, field: &'ast $($mut)? VariableDeclaration) {
                    v.visit_type(& $($mut)? field.ty);
                    if let Some(storage) = & $($mut)? field.storage {
                        v.visit_storage(storage);
                    }
                    if let Some(name) = & $($mut)? field.name {
                        v.visit_ident(name);
                    }
                }

                fn visit_parameter_list(&mut v, params: &'ast $($mut)? ParameterList) {
                    for param in params {
                        v.visit_variable_declaration(param);
                    }
                }

                fn visit_field_list(&mut v, params: &'ast $($mut)? FieldList) {
                    for param in params {
                        v.visit_variable_declaration(param);
                    }
                }

                fn visit_file(&mut v, file: &'ast $($mut)? File) {
                    for item in & $($mut)? file.items {
                        v.visit_item(item);
                    }
                }

                fn visit_item(&mut v, item: &'ast $($mut)? Item) {
                    match item {
                        Item::Contract(contract) => v.visit_item_contract(contract),
                        Item::Error(error) => v.visit_item_error(error),
                        Item::Function(function) => v.visit_item_function(function),
                        Item::Struct(strukt) => v.visit_item_struct(strukt),
                        Item::Udt(udt) => v.visit_item_udt(udt),
                    }
                }

                fn visit_item_contract(&mut v, contract: &'ast $($mut)? ItemContract) {
                    v.visit_ident(& $($mut)? contract.name);
                    for item in & $($mut)? contract.body {
                        v.visit_item(item);
                    }
                }

                fn visit_item_error(&mut v, error: &'ast $($mut)? ItemError) {
                    v.visit_ident(& $($mut)? error.name);
                    v.visit_parameter_list(& $($mut)? error.fields);
                }

                fn visit_item_function(&mut v, function: &'ast $($mut)? ItemFunction) {
                    v.visit_ident(& $($mut)? function.name);
                    v.visit_parameter_list(& $($mut)? function.arguments);
                    if let Some(returns) = & $($mut)? function.returns {
                        v.visit_parameter_list(& $($mut)? returns.returns);
                    }
                }

                fn visit_item_struct(&mut v, strukt: &'ast $($mut)? ItemStruct) {
                    v.visit_ident(& $($mut)? strukt.name);
                    v.visit_field_list(& $($mut)? strukt.fields);
                }

                fn visit_item_udt(&mut v, udt: &'ast $($mut)? ItemUdt) {
                    v.visit_ident(& $($mut)? udt.name);
                    v.visit_type(& $($mut)? udt.ty);
                }
            }
        }
    };

    (
        @impl
        $(#[$attr:meta])*
        $vis:vis trait $trait_name:ident<'ast> {$(
            $(#[$fn_attr:meta])*
            fn $fn_name:ident(&mut $v:ident $(, $arg_name:ident : $arg_ty:ty)*) { $($impl:tt)* }
        )*}
    ) => {
        $(#[$attr])*
        $vis trait $trait_name<'ast> {$(
            $(#[$fn_attr])*
            fn $fn_name(&mut self $(, $arg_name: $arg_ty)*) { $fn_name(self $(, $arg_name)*) }
        )*}

        $(
            $(#[$fn_attr])*
            pub fn $fn_name<'ast, V: ?Sized + $trait_name<'ast>>($v: &mut V $(, $arg_name: $arg_ty)*) {
                $($impl)*
            }
        )*
    };
}

macro_rules! kw_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {$(
            $(#[$variant_attr:meta])*
            $variant:ident(kw::$kw:ident)
        ),+ $(,)?}
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy)]
        $vis enum $name {$(
            $(#[$variant_attr])*
            $variant($crate::kw::$kw),
        )+}

        impl ::core::cmp::PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                ::core::mem::discriminant(self) == ::core::mem::discriminant(other)
            }
        }

        impl ::core::cmp::Eq for $name {}

        impl ::core::hash::Hash for $name {
            #[inline]
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(&::core::mem::discriminant(self), state)
            }
        }

        impl ::core::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(self.as_debug_str(), f)
            }
        }

        impl ::core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(self.as_str(), f)
            }
        }

        impl ::syn::parse::Parse for $name {
            fn parse(input: ::syn::parse::ParseStream<'_>) -> ::syn::Result<Self> {
                let lookahead = input.lookahead1();
                $(
                    if lookahead.peek($crate::kw::$kw) {
                        input.parse::<$crate::kw::$kw>().map(Self::$variant)
                    } else
                )+
                {
                    Err(lookahead.error())
                }
            }
        }

        impl $name {
            pub fn peek(lookahead: &::syn::parse::Lookahead1<'_>) -> bool {
                $( lookahead.peek($crate::kw::$kw) )||+
            }

            pub const fn span(self) -> ::proc_macro2::Span {
                match self {$(
                    Self::$variant(kw) => kw.span,
                )+}
            }

            pub fn set_span(&mut self, span: ::proc_macro2::Span) {
                match self {$(
                    Self::$variant(kw) => kw.span = span,
                )+}
            }

            pub const fn as_str(self) -> &'static str {
                match self {$(
                    Self::$variant(..) => stringify!($kw),
                )+}
            }

            pub const fn as_debug_str(self) -> &'static str {
                match self {$(
                    Self::$variant(..) => stringify!($variant),
                )+}
            }
        }
    };
}
