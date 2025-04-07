use crate::iter_clause::IterClause;
use crate::mapping::{Mapping, MappingElse};
use quote::quote;
use syn::parse::ParseStream;

/*-----------------VecDequeComprehension------------------- */
#[derive(Debug)]
pub struct VecDequeComprehension {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for VecDequeComprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let VecDequeComprehension {
            mapping:
                Mapping {
                    left_key,
                    left_value,
                    right_expr,
                },
            iter_clauses,
        } = self;

        if left_value.is_some() {
            panic!("VecDeque isn't key-value collection");
        }

        let mut nested_code = match right_expr {
            None => quote! {
                __rusthension_vec_deque.push_back(#left_key);
            },
            Some(MappingElse {
                conditions,
                else_key,
                else_value,
            }) => {
                if else_value.is_some() {
                    panic!("VecDeque isn't key-value collection");
                }

                quote! {
                    if #conditions {
                        __rusthension_vec_deque.push_back(#left_key);
                    } else {
                        __rusthension_vec_deque.push_back(#else_key);
                    }
                }
            }
        };

        nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);
        nested_code = quote! {
            {
                use ::std::collections::VecDeque;
                let mut __rusthension_vec_deque = VecDeque::new();
                #nested_code
                __rusthension_vec_deque
            }
        };

        tokens.extend(nested_code);
    }
}

impl syn::parse::Parse for VecDequeComprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mapping, iter_clauses) = crate::common_parse(input);

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
