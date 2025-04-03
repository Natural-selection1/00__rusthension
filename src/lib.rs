#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::ParseStream;

#[proc_macro]
pub fn rusthension(token_stream: TokenStream) -> TokenStream {
    // 解析输入的token流为Comprehension结构
    let comprehension = syn::parse_macro_input!(token_stream as Comprehension);

    // 将解析后的结构转换为token流
    let tokens = quote::quote! {
        #comprehension
    };

    // 将quote生成的token流转换为proc_macro的TokenStream
    tokens.into()
}

/*-----------------Comprehension------------------- */
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

        // 得到一份iter_clauses的副本
        let mut iter_clauses = self.iter_clauses.clone();

        // 从后向前遍历iter_clauses
        while let Some(iter_clause) = iter_clauses.pop() {
            // 一些检查
            let is_range = is_range(&iter_clause.for_in_clause.iterable);
            let is_ref = is_ref(&iter_clause.for_in_clause.iterable);
            let length = self.iter_clauses.len();

            let iterable = &iter_clause.for_in_clause.iterable;

            let iterable_code = if is_range {
                quote! { #iterable.clone() }
            } else {
                quote! { #iterable }
            };

            // 根据是否有if条件生成循环代码
            let current_loop = match &iter_clause.if_clause {
                Some(if_clause) => {
                    let pat = &iter_clause.for_in_clause.pat;
                    let expr = &if_clause.expr;
                    quote! {
                        for #pat in #iterable_code {
                            if #expr {
                                #nested_code
                            }
                        }
                    }
                }
                None => {
                    let pat = &iter_clause.for_in_clause.pat;
                    quote! {
                        for #pat in #iterable_code {
                            #nested_code
                        }
                    }
                }
            };

            nested_code = current_loop;
        }

        let output_code = quote! {
            {
                let mut __rusthension_list_result = Vec::new();
                #nested_code
                __rusthension_list_result
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

fn is_range(iterable: &syn::Expr) -> bool {
    matches!(iterable, syn::Expr::Range(_))
}

fn is_ref(iterable: &syn::Expr) -> bool {
    matches!(iterable, syn::Expr::Reference(_))
}

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

use syn::{ExprMethodCall, visit_mut::VisitMut};

struct CloneCallFinder {
    has_clone_call: bool,
}

impl VisitMut for CloneCallFinder {
    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        // 检查方法名是否为 "clone"
        eprintln!("node.method: {}", node.method);
        // 打印现在的时间
        println!("now: {}", chrono::Local::now());
        // 时停1秒
        std::thread::sleep(std::time::Duration::from_secs(1));
        if node.method == "clone" {
            self.has_clone_call = true;
        }

        // 继续访问子节点
        syn::visit_mut::visit_expr_method_call_mut(self, node);
    }
}

// 使用方法
fn check_has_clone(expr: &syn::Expr) -> bool {
    let mut finder = CloneCallFinder {
        has_clone_call: false,
    };
    finder.visit_expr_mut(&mut expr.clone());

    finder.has_clone_call
}

#[test]
fn test_check_has_clone() {
    let expr: syn::Expr = syn::parse_quote!(some_var.as_str().clone());
    assert!(check_has_clone(&expr));
    let expr: syn::Expr = syn::parse_quote!(some_var.other_method());
    assert!(!check_has_clone(&expr));
}
