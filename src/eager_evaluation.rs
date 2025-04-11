pub mod b_tree_map;
pub mod b_tree_set;
pub mod binary_heap;
pub mod hash_map;
pub mod hash_set;
pub mod linked_list;
pub mod vec_deque;
pub mod vector;

pub use b_tree_map::BTreeMapComprehension;
pub use b_tree_set::BTreeSetComprehension;
pub use binary_heap::BinaryHeapComprehension;
pub use hash_map::HashMapComprehension;
pub use hash_set::HashSetComprehension;
pub use linked_list::LinkedListComprehension;
pub use vec_deque::VecDequeComprehension;
pub use vector::VecComprehension;

use crate::iter_clause::{BareIfClause, ForInClause, IterClause, LetClause};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

pub(crate) fn handle_nested_loops(
    iter_clauses: &[IterClause],
    mut nested_code: TokenStream,
) -> TokenStream {
    let mut need_to_shadow: Vec<&Expr> = vec![];

    // 遍历iter_clauses(因为越向后层次越深, 所以直接pop就行了)
    let mut iter_clauses: Vec<&IterClause> = iter_clauses.iter().collect();

    while let Some(iter_clause) = iter_clauses.pop() {
        let IterClause {
            for_in_clause: ForInClause { pat, iterable },
            if_clause,
            let_clauses,
        } = iter_clause;

        let iterable_code = match iterable {
            Expr::Reference(_) | Expr::Range(_) => quote! { #iterable },
            Expr::Path(_) => {
                need_to_shadow.push(iterable);
                quote! { &#iterable }
            }
            Expr::MethodCall(_) => match is_iter(iterable) {
                true => quote! { #iterable },
                _ => panic!(
                    "please ensure the first method call is iter(): \n{:#?}",
                    iterable
                ),
            },
            Expr::Paren(expr) => {
                let iterable = &*expr.expr;
                quote! { #iterable }
            }
            _ => panic!("type is not supported: \n{:#?}", iterable),
        };

        let mut let_clauses: Vec<&LetClause> = let_clauses.iter().collect();
        while let Some(LetClause { let_expr }) = let_clauses.pop() {
            nested_code = quote! {
                #let_expr;
                #nested_code
            };
        }

        // 根据是否有if条件生成循环代码
        nested_code = match if_clause {
            Some(BareIfClause { conditions }) => {
                quote! {
                    for #pat in #iterable_code {
                        if #conditions {
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
    }

    // 为需要影子变量的变量添加声明
    while let Some(shadowed) = need_to_shadow.pop() {
        nested_code = quote! {
            let #shadowed = #shadowed;
            #nested_code
        };
    }

    nested_code
}

struct IterMethodCallFinder {
    is_iter: bool,
}

use syn::{ExprMethodCall, visit::Visit};
impl<'ast> Visit<'ast> for IterMethodCallFinder {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        match *node.receiver {
            syn::Expr::Path(_) | syn::Expr::Field(_) => {
                if node.method == "iter" {
                    self.is_iter = true;
                }
            }
            _ => syn::visit::visit_expr(&mut *self, &node.receiver),
        }
    }
}

fn is_iter(expr: &syn::Expr) -> bool {
    let mut finder = IterMethodCallFinder { is_iter: false };
    finder.visit_expr(expr);

    finder.is_iter
}

#[test]
fn test_is_iter() {
    // 最右侧是iter方法
    let expr = syn::parse_quote!(some.method_1().method_2().iter());
    assert!(!is_iter(&expr));
    eprintln!("--------------------------------");
    // 最左侧是iter方法
    let expr = syn::parse_quote!(some.iter().method_3().method_4());
    assert!(is_iter(&expr));
    eprintln!("--------------------------------");

    let expr = syn::parse_quote!(some.method_5().iter().method_6());
    assert!(!is_iter(&expr));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapping::{Mapping, MappingElse};
    use syn::Expr;
    use syn::parse_quote;
    #[allow(unused_variables)]
    #[test]
    fn test_comprehension_parse() {
        // 测试基本的列表推导式解析
        let comprehension: VecComprehension = parse_quote! {
            x * 2 for x in vec![1, 2, 3]
        };
        let Mapping {
            left_key,
            left_value,
            right_expr,
        } = &comprehension.mapping;
        assert!(matches!(left_key, Expr::Binary(_)));
        assert!(right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        let iter_clause = &comprehension.iter_clauses[0];
        let pat = &iter_clause.for_in_clause.pat;
        let iterable = &iter_clause.for_in_clause.iterable;
        assert!(matches!(pat, syn::Pat::Ident(_)));
        assert!(matches!(iterable, Expr::Macro(_)));
        assert!(iter_clause.if_clause.is_none());
        eprintln!("Comprehension基本列表推导式测试通过");

        // 测试带if条件的列表推导式解析
        let comprehension: VecComprehension = parse_quote! {
            x * 2 for x in 1..10 if x > 0
        };
        let Mapping {
            left_key,
            left_value,
            right_expr,
        } = &comprehension.mapping;
        assert!(matches!(left_key, Expr::Binary(_)));
        assert!(right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        let iter_clause = &comprehension.iter_clauses[0];
        let pat = &iter_clause.for_in_clause.pat;
        let iterable = &iter_clause.for_in_clause.iterable;
        assert!(matches!(pat, syn::Pat::Ident(_)));
        assert!(matches!(iterable, Expr::Range(_)));
        assert!(iter_clause.if_clause.is_some());
        if let Some(if_clause) = &iter_clause.if_clause {
            assert!(matches!(if_clause.conditions, syn::Expr::Binary(_)));
        }
        eprintln!("Comprehension带if条件的列表推导式测试通过");

        // 测试带条件表达式的列表推导式解析
        let comprehension: VecComprehension = parse_quote! {
            x * 2 if x > 0 || x < 10 && x % 2 == 0 else 0 for x in items
        };
        let Mapping {
            left_key,
            left_value,
            right_expr,
        } = &comprehension.mapping;
        assert!(matches!(left_key, Expr::Binary(_)));
        assert!(right_expr.is_some());
        if let Some(mapping_else) = right_expr {
            let MappingElse {
                conditions,
                else_key,
                else_value,
            } = mapping_else;
            assert!(matches!(conditions, Expr::Binary(_)));
            assert!(matches!(else_key, Expr::Lit(_)));
        }
        assert_eq!(comprehension.iter_clauses.len(), 1);
        eprintln!("Comprehension带条件表达式的列表推导式测试通过");

        // 测试多层嵌套的列表推导式解析
        let comprehension: VecComprehension = parse_quote! {
            x + y for x in outer for y in inner
        };
        let Mapping {
            left_key,
            left_value,
            right_expr,
        } = &comprehension.mapping;
        assert!(matches!(left_key, Expr::Binary(_)));
        assert!(right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 2);
        let first_clause = &comprehension.iter_clauses[0];
        let second_clause = &comprehension.iter_clauses[1];
        assert!(matches!(first_clause.for_in_clause.pat, syn::Pat::Ident(_)));
        assert!(matches!(
            first_clause.for_in_clause.iterable,
            syn::Expr::Path(_)
        ));
        assert!(matches!(
            second_clause.for_in_clause.pat,
            syn::Pat::Ident(_)
        ));
        assert!(matches!(
            second_clause.for_in_clause.iterable,
            syn::Expr::Path(_)
        ));
        eprintln!("Comprehension多层嵌套的列表推导式测试通过");

        // 测试复杂的多层嵌套带条件的列表推导式解析
        let comprehension: VecComprehension = parse_quote! {
            [x, y] if x > y else (y, x)
            for x in (0..10) if x % 2 == 0
            for y in (0..x) if y % 3 == 0
        };
        let Mapping {
            left_key,
            left_value,
            right_expr,
        } = &comprehension.mapping;
        assert!(matches!(left_key, Expr::Array(_)));
        assert!(right_expr.is_some());
        if let Some(mapping_else) = right_expr {
            let MappingElse {
                conditions,
                else_key,
                else_value,
            } = mapping_else;
            assert!(matches!(conditions, Expr::Binary(_)));
            assert!(matches!(else_key, Expr::Tuple(_)));
        }

        assert_eq!(comprehension.iter_clauses.len(), 2);
        let first_clause = &comprehension.iter_clauses[0];
        let second_clause = &comprehension.iter_clauses[1];
        assert!(matches!(
            second_clause.for_in_clause.iterable,
            Expr::Paren(_)
        ));
        // eprintln!(
        //     "{:#?}",
        //     comprehension.iter_clauses[1].for_in_clause.iterable
        // );
        assert!(first_clause.if_clause.is_some());
        assert!(second_clause.if_clause.is_some());

        eprintln!("Comprehension复杂的多层嵌套带条件的列表推导式测试通过");

        // 测试使用复杂表达式的列表推导式解析
        let comprehension: VecComprehension = parse_quote! {
            x.method().call() for x in items.iter().filter(|i| i.is_valid())
        };
        let Mapping {
            left_key,
            left_value,
            right_expr,
        } = &comprehension.mapping;
        assert!(matches!(left_key, Expr::MethodCall(_)));
        assert!(right_expr.is_none());
        assert_eq!(comprehension.iter_clauses.len(), 1);
        let iter_clause = &comprehension.iter_clauses[0];
        assert!(matches!(
            iter_clause.for_in_clause.iterable,
            Expr::MethodCall(_)
        ));
        eprintln!("Comprehension使用复杂表达式的列表推导式测试通过");
    }
}
