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

use crate::iter_clause::{BareIfClause, ForInClause, IterClause};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

pub(crate) fn handle_nested_loops<'a>(
    iter_clauses: &'a [IterClause],
    mut nested_code: TokenStream,
) -> TokenStream {
    let mut need_to_shadow: Vec<&'a Expr> = vec![];

    // 遍历iter_clauses(因为越向后层次越深, 所以直接pop就行了)
    let mut iter_clauses: Vec<&'a IterClause> = iter_clauses.iter().collect();

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
                    panic!("can't use reference in inner loop");
                }
                Expr::Path(_) => {
                    need_to_shadow.push(iterable);
                    quote! { #iterable.clone() }
                }
                Expr::Range(_) | _ => quote! { #iterable.clone() },
            }
        };

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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Expr;
    use syn::parse_quote;

    #[test]
    fn test_comprehension_parse() {
        // 测试基本的列表推导式解析
        let comprehension: VecComprehension = parse_quote! {
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
        let comprehension: VecComprehension = parse_quote! {
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
            assert!(matches!(if_clause.conditions, syn::Expr::Binary(_)));
        }
        eprintln!("Comprehension带if条件的列表推导式测试通过");

        // 测试带条件表达式的列表推导式解析
        let comprehension: VecComprehension = parse_quote! {
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
        let comprehension: VecComprehension = parse_quote! {
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
        let comprehension: VecComprehension = parse_quote! { [x, y]
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
        let comprehension: VecComprehension = parse_quote! {
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
