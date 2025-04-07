use crate::iter_clause::IterClause;
use crate::mapping::{Mapping, MappingElse};
use quote::quote;
use syn::parse::ParseStream;

/*-----------------BTreeSetComprehension------------------- */
#[derive(Debug)]
pub struct BTreeSetComprehension {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for BTreeSetComprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let BTreeSetComprehension {
            mapping:
                Mapping {
                    left_key,
                    right_expr,
                    ..
                },
            iter_clauses,
        } = self;

        let mut nested_code = match right_expr {
            None => quote! {
                __rusthension_b_tree_set.insert(#left_key);
            },
            Some(MappingElse {
                conditions,
                else_key,
                ..
            }) => {
                quote! {
                    if #conditions {
                        __rusthension_b_tree_set.insert(#left_key);
                    } else {
                        __rusthension_b_tree_set.insert(#else_key);
                    }
                }
            }
        };

        nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);
        nested_code = quote! {
            {
                use ::std::collections::BTreeSet;
                let mut __rusthension_b_tree_set = BTreeSet::new();
                #nested_code
                __rusthension_b_tree_set
            }
        };

        tokens.extend(nested_code);
    }
}

impl syn::parse::Parse for BTreeSetComprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mapping, iter_clauses) = crate::common_parse(input);

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
