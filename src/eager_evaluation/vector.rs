use crate::iter_clause::IterClause;
use crate::mapping::{Mapping, MappingElse};

use quote::quote;
use syn::parse::ParseStream;

/*-----------------VecComprehension------------------- */
#[derive(Debug)]
pub struct VecComprehension {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for VecComprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let VecComprehension {
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
                __rusthension_vec.push(#left_key);
            },
            Some(MappingElse {
                conditions,
                else_key,
                ..
            }) => {
                quote! {
                    if #conditions {
                        __rusthension_vec.push(#left_key);
                    } else {
                        __rusthension_vec.push(#else_key);
                    }
                }
            }
        };

        let nested_code = crate::eager_evaluation::handle_nested_loops(iter_clauses, nested_code);

        let output_code = {
            quote! {
                {
                    let mut __rusthension_vec = Vec::new();
                    #nested_code
                    __rusthension_vec
                }
            }
        };

        tokens.extend(output_code);
    }
}

impl syn::parse::Parse for VecComprehension {
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
