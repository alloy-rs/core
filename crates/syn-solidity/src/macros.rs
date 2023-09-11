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
                fn visit_lit_str(&mut v, lit: &'ast $($mut)? LitStr) {
                    // nothing to do
                }

                fn visit_ident(&mut v, ident: &'ast $($mut)? SolIdent) {
                    // nothing to do
                }

                fn visit_path(&mut v, ident: &'ast $($mut)? SolPath) {
                    // nothing to do
                }

                fn visit_type(&mut v, ty: &'ast $($mut)? Type) {
                    match ty {
                        Type::Address(..)
                        | Type::Bool(_)
                        | Type::Uint(..)
                        | Type::Int(..)
                        | Type::String(_)
                        | Type::Bytes(_)
                        | Type::FixedBytes(..) => {},
                        Type::Array(TypeArray { ty, .. }) => v.visit_type(ty),
                        Type::Tuple(TypeTuple { types, .. }) => {
                            for ty in types {
                                v.visit_type(ty);
                            }
                        },
                        Type::Function(TypeFunction { arguments, returns, .. }) => {
                            v.visit_parameter_list(arguments);
                            if let Some(returns) = returns {
                                v.visit_parameter_list(& $($mut)? returns.returns);
                            }
                        },
                        Type::Mapping(TypeMapping { key, key_name, value, value_name, .. }) => {
                            v.visit_type(key);
                            if let Some(key_name) = key_name {
                                v.visit_ident(key_name);
                            }
                            v.visit_type(value);
                            if let Some(value_name) = value_name {
                                v.visit_ident(value_name);
                            }
                        },
                        Type::Custom(name) => v.visit_path(name),
                    }
                }

                fn visit_variable_declaration(&mut v, var: &'ast $($mut)? VariableDeclaration) {
                    v.visit_type(& $($mut)? var.ty);
                    if let Some(name) = & $($mut)? var.name {
                        v.visit_ident(name);
                    }
                }

                fn visit_variable_definition(&mut v, var: &'ast $($mut)? VariableDefinition) {
                    v.visit_type(& $($mut)? var.ty);
                    v.visit_ident(& $($mut)? var.name);
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
                        Item::Enum(enumm) => v.visit_item_enum(enumm),
                        Item::Error(error) => v.visit_item_error(error),
                        Item::Event(event) => v.visit_item_event(event),
                        Item::Function(function) => v.visit_item_function(function),
                        Item::Import(import) => v.visit_import_directive(import),
                        Item::Pragma(pragma) => v.visit_pragma_directive(pragma),
                        Item::Struct(strukt) => v.visit_item_struct(strukt),
                        Item::Udt(udt) => v.visit_item_udt(udt),
                        Item::Using(using) => v.visit_using_directive(using),
                        Item::Variable(variable) => v.visit_variable_definition(variable),
                    }
                }

                fn visit_item_contract(&mut v, contract: &'ast $($mut)? ItemContract) {
                    v.visit_ident(& $($mut)? contract.name);
                    for item in & $($mut)? contract.body {
                        v.visit_item(item);
                    }
                }

                fn visit_item_enum(&mut v, enumm: &'ast $($mut)? ItemEnum) {
                    v.visit_ident(& $($mut)? enumm.name);
                    for Variant { ident, .. } in & $($mut)? enumm.variants {
                        v.visit_ident(ident);
                    }
                }

                fn visit_item_error(&mut v, error: &'ast $($mut)? ItemError) {
                    v.visit_ident(& $($mut)? error.name);
                    v.visit_parameter_list(& $($mut)? error.parameters);
                }

                fn visit_item_event(&mut v, event: &'ast $($mut)? ItemEvent) {
                    v.visit_ident(& $($mut)? event.name);
                    for EventParameter { name, ty, .. } in & $($mut)? event.parameters {
                        v.visit_type(ty);
                        if let Some(name) = name {
                            v.visit_ident(name);
                        }
                    }
                }

                fn visit_item_function(&mut v, function: &'ast $($mut)? ItemFunction) {
                    if let Some(name) = & $($mut)? function.name {
                        v.visit_ident(name);
                    }
                    v.visit_parameter_list(& $($mut)? function.arguments);
                    if let Some(returns) = & $($mut)? function.returns {
                        v.visit_parameter_list(& $($mut)? returns.returns);
                    }
                }

                fn visit_import_directive(&mut v, import: &'ast $($mut)? ImportDirective) {
                    match & $($mut)? import.path {
                        ImportPath::Plain(ImportPlain { path, alias }) => {
                            v.visit_lit_str(path);
                            if let Some(ImportAlias { alias, .. }) = alias {
                                v.visit_ident(alias);
                            }
                        }
                        ImportPath::Aliases(ImportAliases { imports, path, .. }) => {
                            for (name, alias) in imports {
                                v.visit_ident(name);
                                if let Some(ImportAlias { alias, .. }) = alias {
                                    v.visit_ident(alias);
                                }
                            }
                            v.visit_lit_str(path);
                        }
                        ImportPath::Glob(ImportGlob { alias, path, .. }) => {
                            if let Some(ImportAlias { alias, .. }) = alias {
                                v.visit_ident(alias);
                            }
                            v.visit_lit_str(path);
                        }
                    }
                }

                fn visit_pragma_directive(&mut v, pragma: &'ast $($mut)? PragmaDirective) {
                    // nothing to do
                }

                fn visit_item_struct(&mut v, strukt: &'ast $($mut)? ItemStruct) {
                    v.visit_ident(& $($mut)? strukt.name);
                    v.visit_field_list(& $($mut)? strukt.fields);
                }

                fn visit_item_udt(&mut v, udt: &'ast $($mut)? ItemUdt) {
                    v.visit_ident(& $($mut)? udt.name);
                    v.visit_type(& $($mut)? udt.ty);
                }

                fn visit_using_directive(&mut v, using: &'ast $($mut)? UsingDirective) {
                    // nothing to do
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
            #[doc = concat!("`", stringify!($kw), "`")]
            ///
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
                f.write_str(self.as_debug_str())
            }
        }

        impl ::core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
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

        impl $crate::Spanned for $name {
            fn span(&self) -> ::proc_macro2::Span {
                match self {$(
                    Self::$variant(kw) => kw.span,
                )+}
            }

            fn set_span(&mut self, span: ::proc_macro2::Span) {
                match self {$(
                    Self::$variant(kw) => kw.span = span,
                )+}
            }
        }

        impl $name {
            ::paste::paste! {
                $(
                    #[doc = concat!("Creates a new `", stringify!($variant), "` keyword with the given `span`.")]
                    #[inline]
                    pub fn [<new_ $variant:snake>](span: ::proc_macro2::Span) -> Self {
                        Self::$variant(kw::$kw(span))
                    }
                )+
            }

            pub fn parse_opt(input: ::syn::parse::ParseStream<'_>) -> ::syn::Result<Option<Self>> {
                $(
                    if input.peek($crate::kw::$kw) {
                        input.parse::<$crate::kw::$kw>().map(|kw| Some(Self::$variant(kw)))
                    } else
                )+
                {
                    Ok(None)
                }
            }

            pub fn peek(lookahead: &::syn::parse::Lookahead1<'_>) -> bool {
                $( lookahead.peek($crate::kw::$kw) )||+
            }

            pub const fn as_str(self) -> &'static str {
                match self {$(
                    Self::$variant(_) => stringify!($kw),
                )+}
            }

            pub const fn as_debug_str(self) -> &'static str {
                match self {$(
                    Self::$variant(_) => stringify!($variant),
                )+}
            }

            ::paste::paste! {
                $(
                    #[doc = concat!("Returns true if `self` matches `Self::", stringify!($variant), "`.")]
                    #[inline]
                    pub const fn [<is_ $variant:snake>](self) -> bool {
                        matches!(self, Self::$variant(_))
                    }
                )+
            }
        }
    };
}

macro_rules! op_enum {
    (@skip $($tt:tt)*) => {};
    (@first $first:tt $($rest:tt)*) => { ::syn::Token![$first] };

    (@peek $input:ident, $lookahead:ident, $a:tt) => {
        $lookahead.peek(::syn::Token![$a])
    };
    // can't use `peek2` for `BinOp::Sar` (`>>>`) since the first token is 2 characters,
    // so take it in as input
    (@peek $input:ident, $lookahead:ident, $a:tt $b:tt $peek:ident) => {
        $lookahead.peek(::syn::Token![$a])
            && $input.$peek(::syn::Token![$b])
    };

    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {$(
            $(#[$variant_attr:meta])*
            $variant:ident($($op:tt)+) $($peek:ident)?
        ),+ $(,)?}
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy)]
        $vis enum $name {$(
            #[doc = concat!("`", $(stringify!($op),)+ "`")]
            ///
            $(#[$variant_attr])*
            $variant($(::syn::Token![$op]),+),
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
                f.write_str(self.as_debug_str())
            }
        }

        impl ::core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::syn::parse::Parse for $name {
            fn parse(input: ::syn::parse::ParseStream<'_>) -> ::syn::Result<Self> {
                let lookahead = input.lookahead1();
                $(
                    if op_enum!(@peek input, lookahead, $($op)+ $($peek)?) {
                        Ok(Self::$variant(
                            $(input.parse::<::syn::Token![$op]>()?),+
                        ))
                    } else
                )+
                {
                    Err(lookahead.error())
                }
            }
        }

        impl $crate::Spanned for $name {
            fn span(&self) -> ::proc_macro2::Span {
                match self {$(
                    Self::$variant(kw, ..) => kw.span(),
                )+}
            }

            fn set_span(&mut self, span: ::proc_macro2::Span) {
                match self {$(
                    Self::$variant(kw, ..) => kw.set_span(span),
                )+}
            }
        }

        impl $name {
            ::paste::paste! {
                $(
                    #[doc = concat!("Creates a new `", stringify!($variant), "` operator with the given `span`.")]
                    #[inline]
                    pub fn [<new_ $variant:snake>](span: ::proc_macro2::Span) -> Self {
                        Self::$variant($(::syn::Token![$op](span)),+)
                    }
                )+
            }

            #[allow(unused_parens, unused_variables)]
            pub fn peek(input: syn::parse::ParseStream<'_>, lookahead: &::syn::parse::Lookahead1<'_>) -> bool {
                $(
                    (op_enum!(@peek input, lookahead, $($op)+ $($peek)?))
                )||+
            }

            pub const fn as_str(self) -> &'static str {
                match self {$(
                    Self::$variant(..) => concat!($(stringify!($op)),+),
                )+}
            }

            pub const fn as_debug_str(self) -> &'static str {
                match self {$(
                    Self::$variant(..) => stringify!($variant),
                )+}
            }

            ::paste::paste! {
                $(
                    #[doc = concat!("Returns true if `self` matches `Self::", stringify!($variant), "`.")]
                    #[inline]
                    pub const fn [<is_ $variant:snake>](self) -> bool {
                        matches!(self, Self::$variant(..))
                    }
                )+
            }
        }
    };
}

macro_rules! derive_parse {
    ($($t:ty),+ $(,)?) => {$(
        impl Parse for $t {
            fn parse(input: ParseStream<'_>) -> Result<Self> {
                <Self as $crate::utils::ParseNested>::parse_nested(
                    input.parse()?,
                    input,
                )
            }
        }
    )+};
}

macro_rules! debug {
    ($($t:tt)*) => {
        if $crate::DEBUG {
            eprintln!($($t)*)
        }
    };
}
