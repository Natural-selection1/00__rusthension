use crate::iter_clause::{BareIfClause, ForInClause, IterClause};
use crate::mapping::{Mapping, MappingElse};

use quote::quote;
use syn::Expr;
use syn::parse::ParseStream;

/*-----------------LazyRefIterator------------------- */
#[derive(Debug)]
pub struct LazyRefIterator {
    pub mapping: Mapping,
    pub iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for LazyRefIterator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // 解构以获得变量
        let LazyRefIterator {
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

        let mut need_to_clone_and_ref = vec![];
        // let mut is_deepest = true;
        // 得到借用的iter_clauses
        let mut iter_clauses: Vec<&IterClause> = iter_clauses.iter().collect();

        // 从尾部遍历iter_clauses
        while let Some(iter_clause) = iter_clauses.pop() {
            let IterClause {
                for_in_clause: ForInClause { pat, iterable },
                if_clause,
                ..
            } = iter_clause;

            //
            let current_loop =
            // 开始处理嵌套
            match (iter_clauses.is_empty(), iterable, if_clause) {
                // 最外层
                (true, Expr::Range(iterable), Some(BareIfClause { expr })) => {
                    quote! {
                        (#iterable)
                        .into_iter()
                        .filter_map(move |#pat| { #expr })
                        .then(|| {
                            #nested_code
                        })
                    }
                }
                // 最外层
                (true, Expr::Range(iterable), None) => {
                    quote! {
                        (#iterable)
                        .into_iter()
                        .filter_map(move |#pat| { true })
                        .then(|| {
                            #nested_code
                        })
                    }
                }
                (false, Expr::Range(iterable), Some(BareIfClause { expr })) => {
                    nested_code = quote! {
                        (#iterable)
                        .into_iter()
                        .filter_map(move |#pat| { #expr })
                        .then(|| {
                            #nested_code
                        })
                    };
                    // 处理需要克隆的iterable
                    for (_ , iterable, _) in &need_to_clone_and_ref {
                        nested_code = quote! {
                            let #iterable = #iterable.clone();
                            #nested_code
                        }
                    }

                    nested_code
                }
                (false, Expr::Range(iterable), None) => {
                    nested_code = quote! {
                        (#iterable)
                        .into_iter()
                        .filter_map(move |#pat| { true })
                        .then(|| {
                            #nested_code
                        })
                    };
                    // 处理需要克隆的iterable
                    for (_ , iterable, _) in &need_to_clone_and_ref {
                        nested_code = quote! {
                            let #iterable = #iterable.clone();
                            #nested_code
                        }
                    }

                    nested_code
                }

                // 最外层
                (true, Expr::Path(iterable), Some(BareIfClause { expr })) => {
                    // 需要引用化的iterable
                    need_to_clone_and_ref.push((pat, iterable, if_clause));

                    //处理嵌套
                    quote! {
                        (#iterable)
                        .into_iter()
                        .filter_map(move |#pat| { #expr })
                        .then(|| {
                            #nested_code
                        })
                    }
                }
                // 最外层
                (true, Expr::Path(iterable), None) => {
                    // 需要引用化的iterable
                    need_to_clone_and_ref.push((pat, iterable, if_clause));

                    //处理嵌套
                    quote! {
                        (#iterable)
                        .into_iter()
                        .filter_map(move |#pat| { true })
                        .then(|| {
                            #nested_code
                        })
                    }
                }
                (false, Expr::Path(iterable), Some(BareIfClause { expr })) => {
                    // 需要引用化的iterable
                    need_to_clone_and_ref.push((pat, iterable, if_clause));

                    nested_code = quote! {
                        (#iterable)
                        .into_iter()
                        .filter_map(move |#pat| { #expr })
                        .then(|| {
                            #nested_code
                        })
                    };
                    // 处理需要克隆的iterable
                    for (_,iterable,_) in &need_to_clone_and_ref {
                        nested_code = quote! {
                            let #iterable = #iterable.clone();
                            #nested_code
                        }
                    }

                    nested_code
                }
                (false, Expr::Path(iterable), None) => {
                    // 需要引用化的iterable
                    need_to_clone_and_ref.push((pat, iterable, if_clause));

                    nested_code = quote! {
                        (#iterable)
                        .into_iter()
                        .filter_map(move |#pat| { true })
                        .then(|| {
                            #nested_code
                        })
                    };
                    // 处理需要克隆的iterable
                    for (_,iterable,_) in &need_to_clone_and_ref {
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

        // 为需要影子变量的变量添加声明
        while let Some(shadowed) = need_to_clone_and_ref.pop() {
            let (_, iterable, _) = shadowed;
            nested_code = quote! {
                let #iterable = #iterable.iter().collect::<Vec<_>>();
                #nested_code
            }
        }

        let output_code = {
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
