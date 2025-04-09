use crate::iter_clause::IterClause;
use crate::mapping::{Mapping, MappingElse};
use quote::quote;
use syn::parse::ParseStream;

/*-----------------HashSetComprehension------------------- */
#[derive(Debug)]
pub struct HashSetComprehension {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for HashSetComprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HashSetComprehension {
            mapping:
                Mapping {
                    left_key,
                    left_value,
                    right_expr,
                },
            iter_clauses,
        } = self;

        if left_value.is_some() {
            panic!("HashSet isn't key-value collection");
        }

        let mut nested_code = match right_expr {
            None => quote! {
                __hash_set_comprehension.insert(#left_key);
            },
            Some(MappingElse {
                conditions,
                else_key,
                else_value,
            }) => {
                if else_value.is_some() {
                    panic!("HashSet isn't key-value collection");
                }

                quote! {
                    if #conditions {
                        __hash_set_comprehension.insert(#left_key);
                    } else {
                        __hash_set_comprehension.insert(#else_key);
                    }
                }
            }
        };

        nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);
        nested_code = quote! {
            {
                use ::std::collections::HashSet;
                let mut __hash_set_comprehension = HashSet::new();
                #nested_code
                __hash_set_comprehension
            }
        };

        tokens.extend(nested_code);
    }
}

impl syn::parse::Parse for HashSetComprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mapping, iter_clauses) = crate::common_parse(input);

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
