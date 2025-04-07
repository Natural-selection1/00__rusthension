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
                    right_expr,
                    ..
                },
            iter_clauses,
        } = self;

        let nested_code = match right_expr {
            None => quote! {
                __rusthension_hash_set.insert(#left_key);
            },
            Some(MappingElse {
                conditions,
                else_key,
                ..
            }) => {
                quote! {
                    if #conditions {
                        __rusthension_hash_set.insert(#left_key);
                    } else {
                        __rusthension_hash_set.insert(#else_key);
                    }
                }
            }
        };

        let nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);

        let output_code = {
            quote! {
                {
                    use ::std::collections::HashSet;
                    let mut __rusthension_hash_set = HashSet::new();
                    #nested_code
                    __rusthension_hash_set
                }
            }
        };

        tokens.extend(output_code);
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
