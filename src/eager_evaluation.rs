#![allow(unused_imports)]
pub mod _b_tree_map;
pub mod _b_tree_set;
pub mod _binary_heap;
pub mod _hash_map;
pub mod _hash_set;
pub mod _linked_list;
pub mod _vec;
pub mod _vec_deque;

pub use _b_tree_map::BTreeMapComprehension;
pub use _b_tree_set::BTreeSetComprehension;
pub use _binary_heap::BinaryHeapComprehension;
pub use _hash_map::HashMapComprehension;
pub use _hash_set::HashSetComprehension;
pub use _linked_list::LinkedListComprehension;
pub use _vec::VecComprehension;
pub use _vec_deque::VecDequeComprehension;

use crate::iter_clause::{BareIfClause, ForInClause, IterClause};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

pub(crate) fn handle_nested_loops<'a>(
    iter_clauses: &'a [IterClause],
    mut nested_code: TokenStream,
) -> (Vec<&'a Expr>, TokenStream) {
    let mut need_to_shadow: Vec<&'a Expr> = vec![];

    // 得到借用并反序的iter_clauses
    let mut iter_clauses: Vec<&'a IterClause> =
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

    (need_to_shadow, nested_code)
}
