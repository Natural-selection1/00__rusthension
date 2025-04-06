use crate::iter_clause::IterClause;
use crate::mapping::{Mapping, MappingElse};

use quote::quote;
use syn::parse::ParseStream;

/*-----------------BTreeMapComprehension------------------- */
#[derive(Debug)]
pub struct BTreeMapComprehension {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for BTreeMapComprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let BTreeMapComprehension {
            mapping:
                Mapping {
                    left_key,
                    left_value,
                    right_expr,
                },
            iter_clauses,
        } = self;

        let nested_code = match right_expr {
            None => quote! {
                __rusthension_b_tree_map.insert(#left_key, #left_value);
            },
            Some(MappingElse {
                conditions,
                else_key,
                else_value,
            }) => {
                quote! {
                    if #conditions {
                        __rusthension_b_tree_map.insert(#left_key, #left_value);
                    } else {
                        __rusthension_b_tree_map.insert(#else_key, #else_value);
                    }
                }
            }
        };

        let nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);

        let output_code = {
            quote! {
                {
                    use ::std::collections::BTreeMap;
                    let mut __rusthension_b_tree_map = BTreeMap::new();
                    #nested_code
                    __rusthension_b_tree_map
                }
            }
        };

        tokens.extend(output_code);
    }
}

impl syn::parse::Parse for BTreeMapComprehension {
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
