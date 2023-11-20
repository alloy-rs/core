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
                fn visit_block(&mut v, block: &'ast $($mut)? Block) {
                    for stmt in & $($mut)? block.stmts {
                        v.visit_stmt(stmt);
                    }
                }

                fn visit_stmt(&mut v, stmt: &'ast $($mut)? Stmt) {
                    match stmt {
                        Stmt::Assembly(asm) => v.visit_stmt_asm(asm),
                        Stmt::Block(block) => v.visit_block(block),
                        Stmt::Break(brk) => v.visit_stmt_break(brk),
                        Stmt::Continue(cont) => v.visit_stmt_continue(cont),
                        Stmt::DoWhile(dowhile) => v.visit_stmt_dowhile(dowhile),
                        Stmt::Emit(emit) => v.visit_stmt_emit(emit),
                        Stmt::Expr(expr) => v.visit_expr(& $($mut)? expr.expr),
                        Stmt::For(f) => v.visit_stmt_for(f),
                        Stmt::If(ifstmt) => v.visit_stmt_if(ifstmt),
                        Stmt::Return(ret) => v.visit_stmt_return(ret),
                        Stmt::Revert(revert) => v.visit_stmt_revert(revert),
                        Stmt::Try(try_stmt) => v.visit_stmt_try(try_stmt),
                        Stmt::UncheckedBlock(ublock) => v.visit_unchecked_block(ublock),
                        Stmt::VarDecl(vard) => v.visit_stmt_var_decl(vard),
                        Stmt::While(w) => v.visit_stmt_while(w),
                    }
                }

                fn visit_stmt_asm(&mut v, _i: &'ast $($mut)? StmtAssembly) {
                    // nothing to do
                }

                fn visit_stmt_break(&mut v, _i: &'ast $($mut)? StmtBreak) {
                    // nothing to do
                }

                fn visit_stmt_continue(&mut v, _i: &'ast $($mut)? StmtContinue) {
                    // nothing to do
                }

                fn visit_stmt_dowhile(&mut v, stmt_dowhile: &'ast $($mut)? StmtDoWhile) {
                    v.visit_expr(& $($mut)? stmt_dowhile.cond);
                    v.visit_stmt(& $($mut)? stmt_dowhile.body);
                }

                fn visit_stmt_emit(&mut v, emit: &'ast $($mut)? StmtEmit) {
                    v.visit_expr(& $($mut)? emit.expr);
                }

                fn visit_stmt_for(&mut v, stmt_for: &'ast $($mut)? StmtFor) {
                    match & $($mut)? stmt_for.init {
                        ForInitStmt::Expr(expr) => v.visit_expr(& $($mut)? expr.expr),
                        ForInitStmt::VarDecl(vard) => v.visit_stmt_var_decl(vard),
                        ForInitStmt::Empty(_) => {}
                    }

                    v.visit_stmt(& $($mut)? stmt_for.body);
                    if let Some(cond) = & $($mut)? stmt_for.cond {
                        v.visit_expr(cond);
                    }
                    if let Some(post) = & $($mut)? stmt_for.post {
                        v.visit_expr(post);
                    }
                }

                fn visit_stmt_if(&mut v, stmt_if: &'ast $($mut)? StmtIf) {
                    v.visit_expr(& $($mut)? stmt_if.cond);
                    v.visit_stmt(& $($mut)? stmt_if.then_branch);
                    if let Some((_, stmt)) = & $($mut)? stmt_if.else_branch {
                        v.visit_stmt(stmt);
                    }
                }

                fn visit_stmt_return(&mut v, ret: &'ast $($mut)? StmtReturn) {
                    if let Some(ret_expr) = & $($mut)? ret.expr {
                        v.visit_expr(ret_expr);
                    }
                }

                fn visit_stmt_revert(&mut v, rvert: &'ast $($mut)? StmtRevert) {
                    v.visit_expr(& $($mut)? rvert.expr);
                }

                fn visit_stmt_try(&mut v, stmt_try: &'ast $($mut)? StmtTry) {
                    v.visit_block(& $($mut)? stmt_try.block);
                    v.visit_expr(& $($mut)? stmt_try.expr);

                    for catch in & $($mut)? stmt_try.catch {
                        v.visit_block(& $($mut)? catch.block);
                        for iden in & $($mut)? catch.list {
                            v.visit_variable_declaration(iden);
                        }
                    }
                    if let Some(ret) = & $($mut)? stmt_try.returns {
                        v.visit_parameter_list(& $($mut)? ret.returns);
                    }
                }

                fn visit_unchecked_block(&mut v, ublock: &'ast $($mut)? UncheckedBlock) {
                    v.visit_block(& $($mut)? ublock.block);
                }

                fn visit_stmt_var_decl(&mut v, stmt_var_decl: &'ast $($mut)? StmtVarDecl) {
                    if let Some((_, expr)) = & $($mut)? stmt_var_decl.assignment {
                        v.visit_expr(expr);
                    }
                    match & $($mut)? stmt_var_decl.declaration {
                        VarDeclDecl::VarDecl(vard) => v.visit_variable_declaration(vard),
                        VarDeclDecl::Tuple(tuple) => {
                            for var_opt in & $($mut)? tuple.vars {
                                if let Some(var_decl) = var_opt {
                                    v.visit_variable_declaration(var_decl);
                                }
                            }
                        }
                    }
                }

                fn visit_stmt_while(&mut v, stmt_while: &'ast $($mut)? StmtWhile) {
                    v.visit_expr(& $($mut)? stmt_while.cond);
                    v.visit_stmt(& $($mut)? stmt_while.body);
                }

                fn visit_expr(&mut v, expr: &'ast $($mut)? Expr) {
                    match expr {
                        Expr::Array(array) => v.visit_expr_array(array),
                        Expr::Binary(binary) => v.visit_expr_binary(binary),
                        Expr::Call(call) => v.visit_expr_call(call),
                        Expr::CallOptions(call_options) => v.visit_expr_call_options(call_options),
                        Expr::Delete(delete) => v.visit_expr_delete(delete),
                        Expr::Ident(ident) => v.visit_ident(ident),
                        Expr::Index(index) => v.visit_expr_index(index),
                        Expr::Lit(lit) => v.visit_lit(lit),
                        Expr::LitDenominated(lit_denominated) => v.visit_lit_denominated(lit_denominated),
                        Expr::Member(member) => v.visit_expr_member(member),
                        Expr::New(new) => v.visit_expr_new(new),
                        Expr::Payable(payable) => v.visit_expr_payable(payable),
                        Expr::Postfix(postfix) => v.visit_expr_postfix(postfix),
                        Expr::Ternary(ternary) => v.visit_expr_ternary(ternary),
                        Expr::Tuple(tuple) => v.visit_expr_tuple(tuple),
                        Expr::Type(typ) => v.visit_type(typ),
                        Expr::TypeCall(type_call) => v.visit_expr_type_call(type_call),
                        Expr::Unary(unary) => v.visit_expr_unary(unary),
                    }
                }

                fn visit_expr_array(&mut v, i: &'ast $($mut)? ExprArray) {
                    for expr in & $($mut)? i.elems {
                        v.visit_expr(expr);
                    }
                }

                fn visit_expr_binary(&mut v, i: &'ast $($mut)? ExprBinary) {
                    v.visit_expr(& $($mut)? i.left);
                    v.visit_expr(& $($mut)? i.right);
                }

                fn visit_expr_call(&mut v, i: &'ast $($mut)? ExprCall) {
                    v.visit_expr(& $($mut)? i.expr);
                    match & $($mut)? i.args.list {
                        ArgListImpl::Unnamed(args) => {
                            for arg in args {
                                v.visit_expr(arg);
                            }
                        },
                        ArgListImpl::Named(args) => {
                            for arg in & $($mut)? args.list {
                                v.visit_ident(& $($mut)? arg.name);
                                v.visit_expr(& $($mut)? arg.arg);
                            }
                        },
                    }
                }

                fn visit_expr_call_options(&mut v, i: &'ast $($mut)? ExprCallOptions) {
                    v.visit_expr(& $($mut)? i.expr);
                    for arg in & $($mut)? i.args.list {
                        v.visit_ident(& $($mut)? arg.name);
                        v.visit_expr(& $($mut)? arg.arg);
                    }
                }

                fn visit_expr_delete(&mut v, i: &'ast $($mut)? ExprDelete) {
                    v.visit_expr(& $($mut)? i.expr);
                }

                fn visit_expr_index(&mut v, i: &'ast $($mut)? ExprIndex) {
                    v.visit_expr(& $($mut)? i.expr);

                    if let Some(index) = & $($mut)? i.start {
                        v.visit_expr(index);
                    }
                    if let Some(index) = & $($mut)? i.end {
                        v.visit_expr(index);
                    }
                }

                fn visit_lit(&mut v, i: &'ast $($mut)? Lit) {
                    // nothing to do
                }

                fn visit_lit_denominated(&mut v, i: &'ast $($mut)? LitDenominated) {
                    // nothing to do
                }

                fn visit_expr_member(&mut v, i: &'ast $($mut)? ExprMember) {
                    v.visit_expr(& $($mut)? i.expr);
                    v.visit_expr(& $($mut)? i.member);
                }

                fn visit_expr_new(&mut v, i: &'ast $($mut)? ExprNew) {
                    v.visit_type(& $($mut)? i.ty);
                }

                fn visit_expr_payable(&mut v, i: &'ast $($mut)? ExprPayable) {
                    match & $($mut)? i.args.list {
                        ArgListImpl::Unnamed(exprs) => {
                            for expr in exprs {
                                v.visit_expr(expr);
                            }
                        }
                        ArgListImpl::Named(named) => {
                            for a in & $($mut)? named.list {
                                v.visit_ident(& $($mut)? a.name);
                                v.visit_expr(& $($mut)? a.arg);
                            }
                        }
                    }
                }

                fn visit_expr_postfix(&mut v, i: &'ast $($mut)? ExprPostfix) {
                    v.visit_expr(& $($mut)? i.expr);
                }

                fn visit_expr_ternary(&mut v, i: &'ast $($mut)? ExprTernary) {
                    v.visit_expr(& $($mut)? i.cond);
                    v.visit_expr(& $($mut)? i.if_true);
                    v.visit_expr(& $($mut)? i.if_false);
                }

                fn visit_expr_tuple(&mut v, i: &'ast $($mut)? ExprTuple) {
                    for expr in & $($mut)? i.elems {
                        v.visit_expr(expr);
                    }
                }

                fn visit_expr_type_call(&mut v, i: &'ast $($mut)? ExprTypeCall) {
                    v.visit_type(& $($mut)? i.ty);
                }

                fn visit_expr_unary(&mut v, i: &'ast $($mut)? ExprUnary) {
                    v.visit_expr(& $($mut)? i.expr);
                }

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
                    v.visit_parameter_list(& $($mut)? function.parameters);
                    if let Some(returns) = & $($mut)? function.returns {
                        v.visit_parameter_list(& $($mut)? returns.returns);
                    }
                    if let FunctionBody::Block(block) = & $($mut)? function.body {
                        v.visit_block(block);
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
            #[doc = concat!("`", stringify!($kw), "`\n\n")]
            $(#[$variant_attr])*
            $variant($crate::kw::$kw),
        )+}

        impl std::cmp::PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                std::mem::discriminant(self) == std::mem::discriminant(other)
            }
        }

        impl std::cmp::Eq for $name {}

        impl std::hash::Hash for $name {
            #[inline]
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::hash::Hash::hash(&std::mem::discriminant(self), state)
            }
        }

        impl std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_debug_str())
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
            #[doc = concat!("`", $(stringify!($op),)+ "`\n\n")]
            $(#[$variant_attr])*
            $variant($(::syn::Token![$op]),+),
        )+}

        impl std::cmp::PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                std::mem::discriminant(self) == std::mem::discriminant(other)
            }
        }

        impl std::cmp::Eq for $name {}

        impl std::hash::Hash for $name {
            #[inline]
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::hash::Hash::hash(&std::mem::discriminant(self), state)
            }
        }

        impl std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_debug_str())
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
