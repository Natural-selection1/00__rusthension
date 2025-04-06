use crate::iter_clause::IterClause;
use crate::mapping::{Mapping, MappingElse};

use quote::quote;
use syn::parse::ParseStream;

/*-----------------BinaryHeapComprehension------------------- */
#[derive(Debug)]
pub struct BinaryHeapComprehension {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for BinaryHeapComprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let BinaryHeapComprehension {
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
                __rusthension_binary_heap.push(#left_key);
            },
            Some(MappingElse {
                conditions,
                else_key,
                ..
            }) => {
                quote! {
                    if #conditions {
                        __rusthension_binary_heap.push(#left_key);
                    } else {
                        __rusthension_binary_heap.push(#else_key);
                    }
                }
            }
        };

        let nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);

        let output_code = {
            quote! {
                {
                    use ::std::collections::BinaryHeap;
                    let mut __rusthension_binary_heap = BinaryHeap::new();
                    #nested_code
                    __rusthension_binary_heap
                }
            }
        };

        tokens.extend(output_code);
    }
}

impl syn::parse::Parse for BinaryHeapComprehension {
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
