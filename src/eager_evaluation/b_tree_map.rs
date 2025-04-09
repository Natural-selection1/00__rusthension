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

        if left_value.is_none() {
            panic!("missing key-value pair");
        }

        let mut nested_code = match right_expr {
            None => quote! {
                __b_tree_map_comprehension.insert(#left_key, #left_value);
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
                        __b_tree_map_comprehension.insert(#left_key, #left_value);
                    } else {
                        __b_tree_map_comprehension.insert(#else_key, #else_value);
                    }
                }
            }
        };

        nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);
        nested_code = quote! {
            {
                use ::std::collections::BTreeMap;
                let mut __b_tree_map_comprehension = BTreeMap::new();
                #nested_code
                __b_tree_map_comprehension
            }
        };

        tokens.extend(nested_code);
    }
}

impl syn::parse::Parse for BTreeMapComprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mapping, iter_clauses) = crate::common_parse(input);

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
