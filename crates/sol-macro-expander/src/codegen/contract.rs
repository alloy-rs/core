//! Contract instance code generation.
//!
//! Generates the `{Contract}Instance<P, N>` struct and its impl blocks for
//! interacting with deployed contracts via RPC.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

/// Reserved method names that conflict with built-in instance methods.
/// Contract functions with these names get a `_call` suffix.
const RESERVED_METHOD_NAMES: &[&str] = &[
    "new",
    "deploy",
    "deploy_builder",
    "address",
    "set_address",
    "at",
    "provider",
    "call_builder",
    "event_filter",
];

/// Returns true if the given name conflicts with a built-in instance method.
pub fn is_reserved_method_name(name: &str) -> bool {
    RESERVED_METHOD_NAMES.contains(&name)
}

/// Function call builder method layout (determines struct construction syntax).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallLayout {
    /// Empty parameters: `CallName`
    Unit,
    /// Single unnamed parameter: `CallName(_0)`
    Tuple,
    /// Named parameters: `CallName { field1, field2, ... }`
    Named,
}

/// Info for a single function call builder method.
#[derive(Debug)]
pub struct ContractFunctionInfo {
    /// Method name on instance (may have `_call` suffix for reserved names).
    pub method_name: Ident,
    /// Call struct name, e.g., `transferCall`.
    pub call_name: Ident,
    /// Parameter names (e.g., `[to, amount]`).
    pub param_names: Vec<Ident>,
    /// Rust types as TokenStreams (e.g., `[Address, U256]`).
    pub rust_types: Vec<TokenStream>,
    /// How to construct the call struct.
    pub layout: CallLayout,
}

/// Info for a single event filter method.
#[derive(Debug)]
pub struct ContractEventInfo {
    /// Event name (e.g., `Transfer`).
    pub event_name: Ident,
}

/// Constructor info for deploy methods.
#[derive(Debug)]
pub struct ConstructorInfo {
    /// Parameter names.
    pub param_names: Vec<Ident>,
    /// Rust types as TokenStreams.
    pub rust_types: Vec<TokenStream>,
}

/// Main codegen struct for contract instance generation.
///
/// Generates the `{Contract}Instance<P, N>` struct with:
/// - Module-level `new()`, `deploy()`, `deploy_builder()` functions
/// - Instance struct with Debug impl
/// - Instantiation methods (new, address, set_address, at, provider)
/// - Function call builder methods
/// - Event filter methods
#[derive(Debug)]
pub struct ContractCodegen {
    /// Contract name (used for doc comments, e.g., `ERC20`).
    pub contract_name: Ident,
    /// Functions to generate call builder methods for.
    pub functions: Vec<ContractFunctionInfo>,
    /// Events to generate filter methods for.
    pub events: Vec<ContractEventInfo>,
    /// Whether bytecode is available (enables deploy methods).
    pub has_bytecode: bool,
    /// Constructor info (None if no constructor or empty params).
    pub constructor: Option<ConstructorInfo>,
}

impl ContractCodegen {
    /// Creates a new contract codegen.
    pub fn new(
        contract_name: Ident,
        functions: Vec<ContractFunctionInfo>,
        events: Vec<ContractEventInfo>,
        has_bytecode: bool,
        constructor: Option<ConstructorInfo>,
    ) -> Self {
        Self { contract_name, functions, events, has_bytecode, constructor }
    }

    /// Generates the full contract instance code.
    ///
    /// NOTE: The generated code assumes `alloy_sol_types` and `alloy_contract` are in scope.
    pub fn expand(self) -> TokenStream {
        let Self { contract_name, functions, events, has_bytecode, constructor } = self;

        let instance_name = format_ident!("{contract_name}Instance");
        let instance_name_s = instance_name.to_string();

        // Generate call builder methods
        let methods = functions.iter().map(gen_call_builder_method);

        // Generate event filter methods
        let filter_methods = events.iter().map(|e| {
            let event_name = &e.event_name;
            let filter_name = format_ident!("{event_name}_filter");
            let doc = format!("Creates a new event filter for the [`{event_name}`] event.");
            quote! {
                #[doc = #doc]
                pub fn #filter_name(&self) -> alloy_contract::Event<&P, #event_name, N> {
                    self.event_filter::<#event_name>()
                }
            }
        });

        // Doc strings
        let new_fn_doc = format!(
            "Creates a new wrapper around an on-chain [`{contract_name}`](self) contract instance.\n\
             \n\
             See the [wrapper's documentation](`{instance_name}`) for more details."
        );
        let struct_doc = format!(
            "A [`{contract_name}`](self) instance.\n\
             \n\
             Contains type-safe methods for interacting with an on-chain instance of the\n\
             [`{contract_name}`](self) contract located at a given `address`, using a given\n\
             provider `P`.\n\
             \n\
             If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)\n\
             documentation on how to provide it), the `deploy` and `deploy_builder` methods can\n\
             be used to deploy a new instance of the contract.\n\
             \n\
             See the [module-level documentation](self) for all the available methods."
        );

        // Generic bounds
        let generic_p_n =
            quote!(<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>);

        // Deploy methods (only if bytecode is available)
        let (deploy_fn, deploy_method) = if has_bytecode {
            gen_deploy_methods(&instance_name, constructor.as_ref())
        } else {
            (quote! {}, quote! {})
        };

        quote! {
            #[doc = #new_fn_doc]
            #[inline]
            pub const fn new #generic_p_n(
                address: alloy_sol_types::private::Address,
                __provider: P,
            ) -> #instance_name<P, N> {
                #instance_name::<P, N>::new(address, __provider)
            }

            #deploy_fn

            #[doc = #struct_doc]
            #[derive(Clone)]
            pub struct #instance_name<P, N = alloy_contract::private::Ethereum> {
                address: alloy_sol_types::private::Address,
                provider: P,
                _network: ::core::marker::PhantomData<N>,
            }

            #[automatically_derived]
            impl<P, N> ::core::fmt::Debug for #instance_name<P, N> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_tuple(#instance_name_s).field(&self.address).finish()
                }
            }

            /// Instantiation and getters/setters.
            impl #generic_p_n #instance_name<P, N> {
                #[doc = #new_fn_doc]
                #[inline]
                pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
                    Self { address, provider: __provider, _network: ::core::marker::PhantomData }
                }

                #deploy_method

                /// Returns a reference to the address.
                #[inline]
                pub const fn address(&self) -> &alloy_sol_types::private::Address {
                    &self.address
                }

                /// Sets the address.
                #[inline]
                pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
                    self.address = address;
                }

                /// Sets the address and returns `self`.
                pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
                    self.set_address(address);
                    self
                }

                /// Returns a reference to the provider.
                #[inline]
                pub const fn provider(&self) -> &P {
                    &self.provider
                }
            }

            impl<P: ::core::clone::Clone, N> #instance_name<&P, N> {
                /// Clones the provider and returns a new instance with the cloned provider.
                #[inline]
                pub fn with_cloned_provider(self) -> #instance_name<P, N> {
                    #instance_name { address: self.address, provider: ::core::clone::Clone::clone(&self.provider), _network: ::core::marker::PhantomData }
                }
            }

            /// Function calls.
            impl #generic_p_n #instance_name<P, N> {
                /// Creates a new call builder using this contract instance's provider and address.
                ///
                /// Note that the call can be any function call, not just those defined in this
                /// contract. Prefer using the other methods for building type-safe contract calls.
                pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C)
                    -> alloy_contract::SolCallBuilder<&P, C, N>
                {
                    alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
                }

                #(#methods)*
            }

            /// Event filters.
            impl #generic_p_n #instance_name<P, N> {
                /// Creates a new event filter using this contract instance's provider and address.
                ///
                /// Note that the type can be any event, not just those defined in this contract.
                /// Prefer using the other methods for building type-safe event filters.
                pub fn event_filter<E: alloy_sol_types::SolEvent>(&self)
                    -> alloy_contract::Event<&P, E, N>
                {
                    alloy_contract::Event::new_sol(&self.provider, &self.address)
                }

                #(#filter_methods)*
            }
        }
    }
}

/// Generates a single call builder method.
fn gen_call_builder_method(f: &ContractFunctionInfo) -> TokenStream {
    let method_name = &f.method_name;
    let call_name = &f.call_name;
    let param_names = &f.param_names;
    let rust_types = &f.rust_types;

    let doc = format!("Creates a new call builder for the [`{method_name}`] function.");

    // Build the call struct construction based on layout
    let call_struct = match f.layout {
        CallLayout::Unit => {
            quote! { #call_name }
        }
        CallLayout::Tuple => {
            let arg = &param_names[0];
            quote! { #call_name(#arg) }
        }
        CallLayout::Named => {
            quote! { #call_name { #(#param_names),* } }
        }
    };

    quote! {
        #[doc = #doc]
        pub fn #method_name(&self, #(#param_names: #rust_types),*) -> alloy_contract::SolCallBuilder<&P, #call_name, N> {
            self.call_builder(&#call_struct)
        }
    }
}

/// Generates the deploy functions (both module-level and instance methods).
fn gen_deploy_methods(
    instance_name: &Ident,
    constructor: Option<&ConstructorInfo>,
) -> (TokenStream, TokenStream) {
    let deploy_doc_str = "Deploys this contract using the given `provider` and constructor arguments, if any.\n\
         \n\
         Returns a new instance of the contract, if the deployment was successful.\n\
         \n\
         For more fine-grained control over the deployment process, use [`deploy_builder`] instead.";

    let deploy_builder_doc_str = "Creates a `RawCallBuilder` for deploying this contract using the given `provider`\n\
         and constructor arguments, if any.\n\
         \n\
         This is a simple wrapper around creating a `RawCallBuilder` with the data set to\n\
         the bytecode concatenated with the constructor's ABI-encoded arguments.";

    let (params, args) = constructor
        .and_then(|c| {
            if c.param_names.is_empty() {
                return None;
            }
            let names = &c.param_names;
            let types = &c.rust_types;
            Some((quote!(#(#names: #types),*), quote!(#(#names,)*)))
        })
        .unzip();

    let deploy_builder_data = if constructor.is_some_and(|c| !c.param_names.is_empty()) {
        quote! {
            [
                &BYTECODE[..],
                &alloy_sol_types::SolConstructor::abi_encode(&constructorCall { #args })[..]
            ].concat().into()
        }
    } else {
        quote! {
            ::core::clone::Clone::clone(&BYTECODE)
        }
    };

    let deploy_fn = quote! {
        #[doc = #deploy_doc_str]
        #[inline]
        pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(__provider: P, #params)
            -> impl ::core::future::Future<Output = alloy_contract::Result<#instance_name<P, N>>>
        {
            #instance_name::<P, N>::deploy(__provider, #args)
        }

        #[doc = #deploy_builder_doc_str]
        #[inline]
        pub fn deploy_builder<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(__provider: P, #params)
            -> alloy_contract::RawCallBuilder<P, N>
        {
            #instance_name::<P, N>::deploy_builder(__provider, #args)
        }
    };

    let deploy_method = quote! {
        #[doc = #deploy_doc_str]
        #[inline]
        pub async fn deploy(__provider: P, #params)
            -> alloy_contract::Result<#instance_name<P, N>>
        {
            let call_builder = Self::deploy_builder(__provider, #args);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }

        #[doc = #deploy_builder_doc_str]
        #[inline]
        pub fn deploy_builder(__provider: P, #params)
            -> alloy_contract::RawCallBuilder<P, N>
        {
            alloy_contract::RawCallBuilder::new_raw_deploy(__provider, #deploy_builder_data)
        }
    };

    (deploy_fn, deploy_method)
}
