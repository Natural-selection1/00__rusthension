use quote::quote;
use syn::Expr;
use syn::parse::ParseStream;

use crate::iter_clause::{BareIfClause, ForInClause, IterClause};
use crate::mapping::{Mapping, MappingElse};

/*-----------------LazyRefIterator------------------- */
#[derive(Debug)]
pub struct LazyRefIterator {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for LazyRefIterator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // 创建Mapping
        let left_key = &self.mapping.left_key;

        let mut nested_code = match &self.mapping.right_expr {
            None => quote! {
                #left_key
            },
            Some(MappingElse {
                conditions,
                else_key,
                ..
            }) => {
                quote! {
                    if #conditions {
                        #left_key
                    } else {
                        #else_key
                    }
                }
            }
        };

        let mut need_to_clone_and_filter = vec![];
        let mut is_deepest = true;
        // 得到借用并反序的iter_clauses
        let mut iter_clauses: Vec<&IterClause> =
            self.iter_clauses.iter().rev().collect();

        // 遍历已经反序的iter_clauses
        while let Some(iter_clause) = iter_clauses.pop() {
            let IterClause {
                for_in_clause: ForInClause { pat, iterable },
                if_clause,
                ..
            } = &iter_clause;

            //
            let current_loop =
                match (iter_clauses.is_empty(), is_deepest, iterable) {
                    // 最外层, 最内层, 是range
                    (true, true, Expr::Range(_)) => {
                        is_deepest = false;
                        quote! {
                            (#iterable).map(move |#pat| {
                                #nested_code
                            }).collect::<Vec<_>>()
                        }
                    }
                    // 最外层, 最内层, 是path
                    (true, true, Expr::Path(_)) => {
                        is_deepest = false;
                        // 需要克隆的iterable
                        need_to_clone_and_filter
                            .push((iterable, if_clause, pat));

                        //处理嵌套
                        nested_code = quote! {
                            (#iterable)
                            .clone()
                            .into_iter()
                            .map(move |#pat| {
                                #nested_code
                            }).collect::<Vec<_>>()
                        };

                        // 处理需要克隆的iterable
                        for (iterable, ..) in &need_to_clone_and_filter {
                            nested_code = quote! {
                                let #iterable = #iterable.clone();
                                #nested_code
                            }
                        }
                        nested_code
                    }
                    // 最外层, 不是最内层, 是range
                    (true, false, Expr::Range(_)) => quote! {
                        (#iterable).flat_map(move |#pat| {
                            #nested_code
                        }).collect::<Vec<_>>()
                    },
                    // 最外层, 非最内层, 是path
                    (true, false, Expr::Path(_)) => {
                        // 需要克隆的iterable
                        need_to_clone_and_filter
                            .push((iterable, if_clause, pat));

                        quote! {
                            (#iterable)
                            .clone()
                        .into_iter()
                        .flat_map(move |#pat| {
                            #nested_code
                            }).collect::<Vec<_>>()
                        }
                    }
                    // 不是最外层, 是最内层, 是range
                    (false, true, Expr::Range(_)) => {
                        // 需要克隆的iterable
                        is_deepest = false;
                        need_to_clone_and_filter
                            .push((iterable, if_clause, pat));

                        nested_code = quote! {
                            (#iterable).map(move |#pat| {
                                #nested_code
                            }).collect::<Vec<_>>()
                        };

                        // 处理需要克隆的iterable
                        for (iterable, ..) in &need_to_clone_and_filter {
                            nested_code = quote! {
                                let #iterable = #iterable.clone();
                                #nested_code
                            }
                        }
                        nested_code
                    }
                    // 非最外层, 最内层, 是path
                    (false, true, Expr::Path(_)) => {
                        is_deepest = false;
                        // 需要克隆的iterable
                        need_to_clone_and_filter
                            .push((iterable, if_clause, pat));

                        nested_code = quote! {
                            (#iterable).into_iter().map(move |#pat| {
                                #nested_code
                            }).collect::<Vec<_>>()
                        };

                        // 处理需要克隆的iterable
                        for (iterable, ..) in &need_to_clone_and_filter {
                            nested_code = quote! {
                                let #iterable = #iterable.clone();
                                #nested_code
                            }
                        }
                        nested_code
                    }
                    //
                    (false, false, Expr::Range(_)) => {
                        nested_code = quote! {
                            (#iterable).flat_map(move |#pat| {
                                #nested_code
                            }).collect::<Vec<_>>()
                        };

                        for (iterable, ..) in &need_to_clone_and_filter {
                            nested_code = quote! {
                                let #iterable = #iterable.clone();
                                #nested_code
                            }
                        }
                        nested_code
                    }
                    //
                    (false, false, Expr::Path(_)) => {
                        // 需要克隆的iterable
                        need_to_clone_and_filter
                            .push((iterable, if_clause, pat));

                        nested_code = quote! {
                            let #iterable = #iterable.clone();
                            (#iterable)
                            .into_iter()
                            .flat_map(move |#pat| {
                                #nested_code
                            }).collect::<Vec<_>>()
                        };

                        // 处理需要克隆的iterable
                        for (iterable, ..) in &need_to_clone_and_filter {
                            nested_code = quote! {
                                let #iterable = #iterable.clone();
                                #nested_code
                            }
                        }
                        nested_code
                    }
                    _ => panic!("unreachable"),
                };

            nested_code = current_loop;
        }

        let output_code = {
            // 为需要影子变量的变量添加声明
            while let Some(shadowed) = need_to_clone_and_filter.pop() {
                //
                nested_code = match shadowed {
                    (iterable, Some(BareIfClause { expr }), pat) => quote! {
                        let #iterable = #iterable
                        .iter()
                        .filter(|&#pat| #expr)
                        .collect::<Vec<_>>();
                        #nested_code
                    },
                    (iterable, None, _) => quote! {
                        let #iterable = #iterable
                        .iter()
                        .collect::<Vec<_>>();
                        #nested_code
                    },
                }
            }
            quote! {
                { #nested_code }
            }
        };

        tokens.extend(output_code);
    }
}

impl syn::parse::Parse for LazyRefIterator {
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
