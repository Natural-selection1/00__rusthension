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
                    left_value,
                    right_expr,
                },
            iter_clauses,
        } = self;

        if left_value.is_some() {
            panic!("BinaryHeap isn't key-value collection");
        }

        let mut nested_code = match right_expr {
            None => quote! {
                __binary_heap_comprehension.push(#left_key);
            },
            Some(MappingElse {
                conditions,
                else_key,
                ..
            }) => {
                quote! {
                    if #conditions {
                        __binary_heap_comprehension.push(#left_key);
                    } else {
                        __binary_heap_comprehension.push(#else_key);
                    }
                }
            }
        };

        nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);
        nested_code = quote! {
            {
                use ::std::collections::BinaryHeap;
                let mut __binary_heap_comprehension = BinaryHeap::new();
                #nested_code
                __binary_heap_comprehension
            }
        };

        tokens.extend(nested_code);
    }
}

impl syn::parse::Parse for BinaryHeapComprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mapping, iter_clauses) = crate::common_parse(input);

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
