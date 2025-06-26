use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Fields, FieldsNamed, parse_macro_input};

#[proc_macro_attribute]
pub fn tree_builder(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let expanded = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let fields = named.iter();

                quote! {
                    #(#attrs)*
                    #vis struct #name<'a> {
                        #(#fields,)*
                        pub arena: ::indextree::Arena<::oxc_ast::AstKind<'a>>,
                        pub node_stack: ::std::vec::Vec<::indextree::NodeId>,
                        pub allocator: &'a ::oxc_allocator::Allocator,
                        pub scoping: &'a ::oxc_semantic::Scoping,
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let original_fields = fields.unnamed.iter();
                quote! {
                    #(#attrs)*
                    #vis struct #name<'a>(
                        #(#original_fields,)*
                        pub arena: ::indextree::Arena<::oxc_ast::AstKind<'a>>,
                        pub node_stack: ::std::vec::Vec<::indextree::NodeId>,
                        pub allocator: &'a ::oxc_allocator::Allocator,
                        pub scoping: &'a ::oxc_semantic::Scoping,
                    );
                }
            }
            Fields::Unit => {
                quote! {
                    #(#attrs)*
                    #vis struct #name<'a> {
                        pub arena: ::indextree::Arena<::oxc_ast::AstKind<'a>>,
                        pub node_stack: ::std::vec::Vec<::indextree::NodeId>,
                        pub allocator: &'a ::oxc_allocator::Allocator,
                        pub scoping: &'a ::oxc_semantic::Scoping,
                    }
                }
            }
        },
        _ => syn::Error::new(name.span(), "tree_builder can only be applied to structs")
            .to_compile_error(),
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn tree_builder_mut(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let expanded = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let fields = named.iter();

                quote! {
                    #(#attrs)*
                    #vis struct #name<'a> {
                        #(#fields,)*
                        pub arena: ::indextree::Arena<::oxc_ast::AstType>,
                        pub node_stack: ::std::vec::Vec<::indextree::NodeId>,
                        pub allocator: &'a ::oxc_allocator::Allocator,
                        pub scoping: &'a ::oxc_semantic::Scoping,
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let original_fields = fields.unnamed.iter();
                quote! {
                    #(#attrs)*
                    #vis struct #name<'a>(
                        #(#original_fields,)*
                        pub arena: ::indextree::Arena<::oxc_ast::AstType>,
                        pub node_stack: ::std::vec::Vec<::indextree::NodeId>,
                        pub allocator: &'a ::oxc_allocator::Allocator,
                        pub scoping: &'a ::oxc_semantic::Scoping,
                    );
                }
            }
            Fields::Unit => {
                quote! {
                    #(#attrs)*
                    #vis struct #name<'a> {
                        pub arena: ::indextree::Arena<::oxc_ast::AstType>,
                        pub node_stack: ::std::vec::Vec<::indextree::NodeId>,
                        pub allocator: &'a ::oxc_allocator::Allocator,
                        pub scoping: &'a ::oxc_semantic::Scoping,
                    }
                }
            }
        },
        _ => syn::Error::new(name.span(), "tree_builder can only be applied to structs")
            .to_compile_error(),
    };

    TokenStream::from(expanded)
}



#[proc_macro_derive(TreeBuilder)]
pub fn derive_tree_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_tree_builder(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn impl_tree_builder(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;

    // Only implement the trait, assuming fields exist
    let expanded = quote! {
        #[automatically_derived]
        impl<'a> ::oxc_ast_visit::Visit<'a> for #name<'a> {
            fn enter_node(&mut self, node: ::oxc_ast::AstKind<'a>) {
                let id = self.arena.new_node(node);
                if let Some(parent) = self.current_parent {
                    parent.append(id, &mut self.arena);
                }
                self.current_parent = Some(id);
            }

            fn leave_node(&mut self, _: ::oxc_ast::AstKind<'a>) {
                if let Some(current) = self.current_parent {
                    if let Some(parent) = current.ancestors(&self.arena).nth(1) {
                        self.current_parent = Some(parent);
                    } else {
                        self.current_parent = None;
                    }
                }
            }
        }
    };

    Ok(expanded)
}
