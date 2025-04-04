use proc_macro::TokenStream as TS;

mod _b_tree_map;
mod _b_tree_set;
mod _binary_heap;
mod _linked_list;
mod _vec;
mod iter_clause;
mod mapping;

pub(crate) use _b_tree_map::BTreeMapComprehension;
pub(crate) use _b_tree_set::BTreeSetComprehension;
pub(crate) use _binary_heap::BinaryHeapComprehension;
pub(crate) use _linked_list::LinkedListComprehension;
pub(crate) use _vec::VecComprehension;

#[proc_macro]
pub fn vec_comprehension(token_stream: TS) -> TS {
    process_comprehension::<VecComprehension>(token_stream)
}

#[proc_macro]
pub fn binary_heap_comprehension(token_stream: TS) -> TS {
    process_comprehension::<BinaryHeapComprehension>(token_stream)
}

#[proc_macro]
pub fn linked_list_comprehension(token_stream: TS) -> TS {
    process_comprehension::<LinkedListComprehension>(token_stream)
}

#[proc_macro]
pub fn b_tree_set_comprehension(token_stream: TS) -> TS {
    process_comprehension::<BTreeSetComprehension>(token_stream)
}

#[proc_macro]
pub fn b_tree_map_comprehension(token_stream: TS) -> TS {
    process_comprehension::<BTreeMapComprehension>(token_stream)
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
