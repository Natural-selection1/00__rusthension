use crate::iter_clause::{BareIfClause, ForInClause, IterClause};
use crate::mapping::{Mapping, MappingElse};

use quote::quote;
use syn::Expr;
use syn::parse::ParseStream;

/*-----------------RefIterator------------------- */
#[derive(Debug)]
pub struct IteratorRef {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

struct InfoContainer<'a> {
    depth: usize,
    paths: Vec<&'a Expr>,
}

impl quote::ToTokens for IteratorRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // 解构以获得变量
        let IteratorRef {
            mapping:
                Mapping {
                    left_key,
                    left_value,
                    right_expr,
                },
            iter_clauses,
        } = self;

        if left_value.is_some() {
            panic!("IteratorRef isn't key-value collection");
        }

        let mut nested_code = match right_expr {
            None => quote! {
                #left_key
            },
            Some(MappingElse {
                conditions,
                else_key,
                else_value,
            }) => {
                if else_value.is_some() {
                    panic!("IteratorRef isn't key-value collection");
                }

                quote! {
                    if #conditions {
                        #left_key
                    } else {
                        #else_key
                    }
                }
            }
        };

        let mut info_container = InfoContainer {
            depth: 0,
            paths: vec![],
        };

        // 从尾部遍历iter_clauses(从最内层开始)
        let mut iter_clauses: Vec<&IterClause> = iter_clauses.iter().collect();
        while let Some(iter_clause) = iter_clauses.pop() {
            let IterClause {
                for_in_clause: ForInClause { pat, iterable },
                if_clause,
            } = iter_clause;
            info_container.depth += 1;

            nested_code = {
                //
                match iterable {
                    Expr::Range(_) => {}
                    Expr::Path(_) => {
                        info_container.paths.push(iterable);
                    }
                    _ => panic!("unreachable"),
                }

                let conditions = match if_clause {
                    Some(BareIfClause { conditions }) => quote! { #conditions },
                    None => quote! { true },
                };

                nested_code = quote! {
                    (#iterable)
                    .into_iter()
                    .filter_map(move |#pat| {
                        ( #conditions ).then(|| {
                            #nested_code
                        })
                    })
                };

                match iter_clauses.is_empty() {
                    true => nested_code,
                    false => {
                        // 非最外层，需要处理克隆
                        for iterable in &info_container.paths {
                            nested_code = quote! {
                                let #iterable = #iterable.clone();
                                #nested_code
                            }
                        }
                        nested_code
                    }
                }
            };
        }

        // 将fliter_map的Some展开
        for _ in 0..(info_container.depth - 1) {
            nested_code = quote! { #nested_code.flatten() }
        }

        // 为需要引用化的容器添加声明
        while let Some(iterable) = info_container.paths.last().copied() {
            info_container.paths.pop();
            nested_code = quote! {
                let #iterable = #iterable.iter().collect::<Vec<_>>();
                #nested_code
            }
        }

        tokens.extend(quote! { { #nested_code } });
    }
}

impl syn::parse::Parse for IteratorRef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mapping, iter_clauses) = crate::common_parse(input);

        Ok(Self {
            mapping,
            iter_clauses,
        })
    }
}
