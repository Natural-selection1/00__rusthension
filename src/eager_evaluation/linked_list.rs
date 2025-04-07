use crate::iter_clause::IterClause;
use crate::mapping::{Mapping, MappingElse};
use quote::quote;
use syn::parse::ParseStream;

/*-----------------LinkedListComprehension------------------- */
#[derive(Debug)]
pub struct LinkedListComprehension {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for LinkedListComprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let LinkedListComprehension {
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
                __rusthension_linked_list.push_back(#left_key);
            },
            Some(MappingElse {
                conditions,
                else_key,
                ..
            }) => {
                quote! {
                    if #conditions {
                        __rusthension_linked_list.push_back(#left_key);
                    } else {
                        __rusthension_linked_list.push_back(#else_key);
                    }
                }
            }
        };

        nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);
        nested_code = quote! {
            {
                use ::std::collections::LinkedList;
                let mut __rusthension_linked_list = LinkedList::new();
                #nested_code
                __rusthension_linked_list
            }
        };

        tokens.extend(nested_code);
    }
}

impl syn::parse::Parse for LinkedListComprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mapping, iter_clauses) = crate::common_parse(input);

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
