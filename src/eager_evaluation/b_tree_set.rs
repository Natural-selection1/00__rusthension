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

        let nested_code = match right_expr {
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

        let nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);

        let output_code = {
            quote! {
                {
                    use ::std::collections::BTreeSet;
                    let mut __rusthension_b_tree_set = BTreeSet::new();
                    #nested_code
                    __rusthension_b_tree_set
                }
            }
        };

        tokens.extend(output_code);
    }
}

impl syn::parse::Parse for BTreeSetComprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut iter_clauses = Vec::new();

        let mapping = input.parse::<Mapping>()?;
        while let Ok(iter_clause) = input.parse::<IterClause>() {
            iter_clauses.push(iter_clause);
        }

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
