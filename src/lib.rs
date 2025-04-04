use proc_macro::TokenStream as TS;

mod _b_tree_map;
mod _b_tree_set;
mod _binary_heap;
mod _hash_map;
mod _hash_set;
mod _linked_list;
mod _vec;
mod _vec_deque;
mod iter_clause;
mod mapping;

pub(crate) use _b_tree_map::BTreeMapComprehension;
pub(crate) use _b_tree_set::BTreeSetComprehension;
pub(crate) use _binary_heap::BinaryHeapComprehension;
pub(crate) use _hash_map::HashMapComprehension;
pub(crate) use _hash_set::HashSetComprehension;
pub(crate) use _linked_list::LinkedListComprehension;
pub(crate) use _vec::VecComprehension;
pub(crate) use _vec_deque::VecDequeComprehension;

#[proc_macro]
pub fn vec(token_stream: TS) -> TS {
    process_comprehension::<VecComprehension>(token_stream)
}

#[proc_macro]
pub fn binary_heap(token_stream: TS) -> TS {
    process_comprehension::<BinaryHeapComprehension>(token_stream)
}

#[proc_macro]
pub fn linked_list(token_stream: TS) -> TS {
    process_comprehension::<LinkedListComprehension>(token_stream)
}

#[proc_macro]
pub fn b_tree_set(token_stream: TS) -> TS {
    process_comprehension::<BTreeSetComprehension>(token_stream)
}

#[proc_macro]
pub fn b_tree_map(token_stream: TS) -> TS {
    process_comprehension::<BTreeMapComprehension>(token_stream)
}

#[proc_macro]
pub fn vec_deque(token_stream: TS) -> TS {
    process_comprehension::<VecDequeComprehension>(token_stream)
}

#[proc_macro]
pub fn hash_set(token_stream: TS) -> TS {
    process_comprehension::<HashSetComprehension>(token_stream)
}

#[proc_macro]
pub fn hash_map(token_stream: TS) -> TS {
    process_comprehension::<HashMapComprehension>(token_stream)
}

fn process_comprehension<T>(token_stream: TS) -> TS
where
    T: syn::parse::Parse + quote::ToTokens,
{
    let comprehension = syn::parse_macro_input!(token_stream as T);
    let tokens = quote::quote! {
        #comprehension
    };
    tokens.into()
}
