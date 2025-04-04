use quote::quote;
use syn::Expr;
use syn::parse::ParseStream;

use crate::iter_clause::{BareIfClause, ForInClause, IterClause};
use crate::mapping::{Mapping, MappingElse};

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

        let mut nested_code = match right_expr {
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

        let mut need_to_shadow: Vec<&Expr> = vec![];

        // 得到借用并反序的iter_clauses
        let mut iter_clauses: Vec<&IterClause> =
            iter_clauses.iter().rev().collect();

        // 遍历已经反序的iter_clauses
        while let Some(iter_clause) = iter_clauses.pop() {
            let IterClause {
                for_in_clause: ForInClause { pat, iterable },
                if_clause,
            } = iter_clause;

            let iterable_code = if iter_clauses.is_empty()
                || matches!(iterable, Expr::MethodCall(node) if node.method == "clone")
            {
                quote! { #iterable }
            } else {
                match iterable {
                    Expr::Reference(_) => {
                        panic!(
                            "can't use reference in inner loop, \
                            maybe change &iterable<T> to iterable<&T>"
                        );
                    }
                    Expr::Path(_) => {
                        need_to_shadow.push(iterable);
                        quote! { #iterable.clone() }
                    }
                    Expr::Range(_) | _ => quote! { #iterable.clone() },
                }
            };

            // 根据是否有if条件生成循环代码
            let current_loop = match if_clause {
                Some(BareIfClause { expr }) => {
                    quote! {
                        for #pat in #iterable_code {
                            if #expr {
                                #nested_code
                            }
                        }
                    }
                }
                None => {
                    quote! {
                        for #pat in #iterable_code {
                            #nested_code
                        }
                    }
                }
            };

            nested_code = current_loop;
        }

        let output_code = {
            // 为需要影子变量的变量添加声明
            while let Some(shadowed) = need_to_shadow.pop() {
                nested_code = quote! {
                    let #shadowed = #shadowed;
                    #nested_code
                };
            }

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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_comprehension_parse() {
        // 测试基本的列表推导式解析
        let comprehension: BinaryHeapComprehension = parse_quote! {
            x * 2 for x in vec![1, 2, 3]
        };
        assert!(matches!(comprehension.mapping.left_key, Expr::Binary(_)));
        assert!(comprehension.mapping.right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.pat,
            syn::Pat::Ident(_)
        ));
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.iterable,
            Expr::Macro(_)
        ));
        assert!(comprehension.iter_clauses[0].if_clause.is_none());
        eprintln!("Comprehension基本列表推导式测试通过");

        // 测试带if条件的列表推导式解析
        let comprehension: BinaryHeapComprehension = parse_quote! {
            x * 2 for x in 1..10 if x > 0
        };
        assert!(matches!(comprehension.mapping.left_key, Expr::Binary(_)));
        assert!(comprehension.mapping.right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.pat,
            syn::Pat::Ident(_)
        ));
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.iterable,
            Expr::Range(_)
        ));
        assert!(comprehension.iter_clauses[0].if_clause.is_some());
        if let Some(if_clause) = &comprehension.iter_clauses[0].if_clause {
            assert!(matches!(if_clause.expr, syn::Expr::Binary(_)));
        }
        eprintln!("Comprehension带if条件的列表推导式测试通过");

        // 测试带条件表达式的列表推导式解析
        let comprehension: BinaryHeapComprehension = parse_quote! {
            x * 2 if x > 0 || x < 10 && x % 2 == 0 else 0 for x in items
        };
        assert!(matches!(comprehension.mapping.left_key, Expr::Binary(_)));
        assert!(comprehension.mapping.right_expr.is_some());
        if let Some(mapping_else) = &comprehension.mapping.right_expr {
            assert!(matches!(mapping_else.conditions, Expr::Binary(_)));
            assert!(matches!(mapping_else.else_key, Expr::Lit(_)));
        }
        assert_eq!(comprehension.iter_clauses.len(), 1);
        eprintln!("Comprehension带条件表达式的列表推导式测试通过");

        // 测试多层嵌套的列表推导式解析
        let comprehension: BinaryHeapComprehension = parse_quote! {
            x + y for x in outer for y in inner
        };
        assert!(matches!(comprehension.mapping.left_key, Expr::Binary(_)));
        assert!(comprehension.mapping.right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 2);
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.pat,
            syn::Pat::Ident(_)
        ));
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.iterable,
            syn::Expr::Path(_)
        ));
        assert!(matches!(
            comprehension.iter_clauses[1].for_in_clause.pat,
            syn::Pat::Ident(_)
        ));
        assert!(matches!(
            comprehension.iter_clauses[1].for_in_clause.iterable,
            syn::Expr::Path(_)
        ));
        eprintln!("Comprehension多层嵌套的列表推导式测试通过");

        // 测试复杂的多层嵌套带条件的列表推导式解析
        let comprehension: BinaryHeapComprehension = parse_quote! { [x, y]
            if x > y else (y, x)
            for x in (0..10) if x % 2 == 0
            for y in (0..x) if y % 3 == 0
        };
        assert!(matches!(comprehension.mapping.left_key, Expr::Array(_)));
        assert!(comprehension.mapping.right_expr.is_some());
        if let Some(mapping_else) = &comprehension.mapping.right_expr {
            assert!(matches!(mapping_else.conditions, Expr::Binary(_)));
            assert!(matches!(mapping_else.else_key, Expr::Tuple(_)));
        }

        assert_eq!(comprehension.iter_clauses.len(), 2);
        assert!(comprehension.iter_clauses[0].if_clause.is_some());
        assert!(comprehension.iter_clauses[1].if_clause.is_some());
        eprintln!("Comprehension复杂的多层嵌套带条件的列表推导式测试通过");

        // 测试使用复杂表达式的列表推导式解析
        let comprehension: BinaryHeapComprehension = parse_quote! {
            x.method().call() for x in items.iter().filter(|i| i.is_valid())
        };
        assert!(matches!(
            comprehension.mapping.left_key,
            Expr::MethodCall(_)
        ));
        assert!(comprehension.mapping.right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.iterable,
            Expr::MethodCall(_)
        ));
        eprintln!("Comprehension使用复杂表达式的列表推导式测试通过");
    }
}
