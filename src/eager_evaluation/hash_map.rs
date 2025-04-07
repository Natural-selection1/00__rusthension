use crate::iter_clause::IterClause;
use crate::mapping::{Mapping, MappingElse};
use quote::quote;
use syn::parse::ParseStream;

/*-----------------HashMapComprehension------------------- */
#[derive(Debug)]
pub struct HashMapComprehension {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for HashMapComprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HashMapComprehension {
            mapping:
                Mapping {
                    left_key,
                    left_value,
                    right_expr,
                },
            iter_clauses,
        } = self;

        if left_value.is_none() {
            panic!("missing key-value pair");
        }

        let mut nested_code = match right_expr {
            None => quote! {
                __rusthension_hash_map.insert(#left_key, #left_value);
            },
            Some(MappingElse {
                conditions,
                else_key,
                else_value,
            }) => {
                if else_value.is_none() {
                    panic!("missing key-value pair");
                }

                quote! {
                    if #conditions {
                        __rusthension_hash_map.insert(#left_key, #left_value);
                    } else {
                        __rusthension_hash_map.insert(#else_key, #else_value);
                    }
                }
            }
        };

        nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);
        nested_code = quote! {
            {
                use ::std::collections::HashMap;
                let mut __rusthension_hash_map = HashMap::new();
                #nested_code
                __rusthension_hash_map
            }
        };

        tokens.extend(nested_code);
    }
}

impl syn::parse::Parse for HashMapComprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mapping, iter_clauses) = crate::common_parse(input);

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
