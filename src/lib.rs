use crate::iter_clause::BareIfClause;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::ParseStream;

#[proc_macro]
pub fn rusthension(token_stream: TokenStream) -> TokenStream {
    let comprehension = syn::parse_macro_input!(token_stream as Comprehension);

    let tokens = quote::quote! {
        #comprehension
    };

    tokens.into()
}

/*-----------------Comprehension------------------- */
#[derive(Debug)]
struct Comprehension {
    mapping: Mapping,
    iter_clauses: Vec<IterClause>,
}

impl quote::ToTokens for Comprehension {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // 创建Mapping
        let left_expr = &self.mapping.left_expr;
        let mut nested_code = match &self.mapping.right_expr {
            None => quote! {
                __rusthension_list_result.push(#left_expr);
            },
            Some(MappingElse {
                conditions,
                else_expr,
            }) => {
                quote! {
                    if #conditions {
                        __rusthension_list_result.push(#left_expr);
                    } else {
                        __rusthension_list_result.push(#else_expr);
                    }
                }
            }
        };

        // fixme: 只使用引用, 不克隆本体
        let mut need_to_shadow = vec![];

        // 克隆并反序iter_clauses (不知道是不是能直接取反序引用的列表?)
        let mut iter_clauses = self.iter_clauses.clone();
        iter_clauses.reverse();

        // 遍历已经反序的iter_clauses
        while let Some(iter_clause) = iter_clauses.pop() {
            let iterable = &iter_clause.for_in_clause.iterable;

            // 生成可迭代对象的代码
            let iterable_code = if iter_clauses.is_empty()
                || matches!(iterable, syn::Expr::MethodCall(node) if node.method == "clone")
            {
                quote! { #iterable }
            } else {
                match iterable {
                    syn::Expr::Reference(_) => {
                        panic!(
                            "can't use reference in inner loop, maybe change &iterable<T> to iterable<&T>"
                        );
                    }
                    syn::Expr::Range(_) => quote! { #iterable.clone() },
                    syn::Expr::Path(_) => {
                        need_to_shadow.push(iterable.clone());

                        quote! { #iterable.clone() }
                    }
                    _ => quote! { #iterable.clone() },
                }
            };

            // 根据是否有if条件生成循环代码
            let pat = &iter_clause.for_in_clause.pat;
            let current_loop = match &iter_clause.if_clause {
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
                    let mut __rusthension_list_result = Vec::new();
                    #nested_code
                    __rusthension_list_result
                }
            }
        };

        tokens.extend(output_code);
    }
}

impl syn::parse::Parse for Comprehension {
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

mod iter_clause;
mod mapping;

use iter_clause::IterClause;
use mapping::{Mapping, MappingElse};

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_comprehension_parse() {
        // 测试基本的列表推导式解析
        let comprehension: Comprehension = parse_quote! {
            x * 2 for x in vec![1, 2, 3]
        };
        assert!(matches!(
            comprehension.mapping.left_expr,
            syn::Expr::Binary(_)
        ));
        assert!(comprehension.mapping.right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.pat,
            syn::Pat::Ident(_)
        ));
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.iterable,
            syn::Expr::Macro(_)
        ));
        assert!(comprehension.iter_clauses[0].if_clause.is_none());
        eprintln!("Comprehension基本列表推导式测试通过");

        // 测试带if条件的列表推导式解析
        let comprehension: Comprehension = parse_quote! {
            x * 2 for x in 1..10 if x > 0
        };
        assert!(matches!(
            comprehension.mapping.left_expr,
            syn::Expr::Binary(_)
        ));
        assert!(comprehension.mapping.right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.pat,
            syn::Pat::Ident(_)
        ));
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.iterable,
            syn::Expr::Range(_)
        ));
        assert!(comprehension.iter_clauses[0].if_clause.is_some());
        if let Some(if_clause) = &comprehension.iter_clauses[0].if_clause {
            assert!(matches!(if_clause.expr, syn::Expr::Binary(_)));
        }
        eprintln!("Comprehension带if条件的列表推导式测试通过");

        // 测试带条件表达式的列表推导式解析
        let comprehension: Comprehension = parse_quote! {
            x * 2 if x > 0 || x < 10 && x % 2 == 0 else 0 for x in items
        };
        assert!(matches!(
            comprehension.mapping.left_expr,
            syn::Expr::Binary(_)
        ));
        assert!(comprehension.mapping.right_expr.is_some());
        if let Some(mapping_else) = &comprehension.mapping.right_expr {
            assert!(matches!(mapping_else.conditions, syn::Expr::Binary(_)));
            assert!(matches!(mapping_else.else_expr, syn::Expr::Lit(_)));
        }
        assert_eq!(comprehension.iter_clauses.len(), 1);
        eprintln!("Comprehension带条件表达式的列表推导式测试通过");

        // 测试多层嵌套的列表推导式解析
        let comprehension: Comprehension = parse_quote! {
            x + y for x in outer for y in inner
        };
        assert!(matches!(
            comprehension.mapping.left_expr,
            syn::Expr::Binary(_)
        ));
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
        let comprehension: Comprehension = parse_quote! { [x, y]
            if x > y else (y, x)
            for x in (0..10) if x % 2 == 0
            for y in (0..x) if y % 3 == 0
        };
        assert!(matches!(
            comprehension.mapping.left_expr,
            syn::Expr::Array(_)
        ));
        assert!(comprehension.mapping.right_expr.is_some());
        if let Some(mapping_else) = &comprehension.mapping.right_expr {
            assert!(matches!(mapping_else.conditions, syn::Expr::Binary(_)));
            assert!(matches!(mapping_else.else_expr, syn::Expr::Tuple(_)));
        }

        assert_eq!(comprehension.iter_clauses.len(), 2);
        assert!(comprehension.iter_clauses[0].if_clause.is_some());
        assert!(comprehension.iter_clauses[1].if_clause.is_some());
        eprintln!("Comprehension复杂的多层嵌套带条件的列表推导式测试通过");

        // 测试使用复杂表达式的列表推导式解析
        let comprehension: Comprehension = parse_quote! {
            x.method().call() for x in items.iter().filter(|i| i.is_valid())
        };
        assert!(matches!(
            comprehension.mapping.left_expr,
            syn::Expr::MethodCall(_)
        ));
        assert!(comprehension.mapping.right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        assert!(matches!(
            comprehension.iter_clauses[0].for_in_clause.iterable,
            syn::Expr::MethodCall(_)
        ));
        eprintln!("Comprehension使用复杂表达式的列表推导式测试通过");
    }
}
