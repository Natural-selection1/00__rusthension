use proc_macro::TokenStream as TS;

mod eager_evaluation;
mod iter_clause;
mod lazy_evaluation;
mod mapping;

pub(crate) use eager_evaluation::{
    BTreeMapComprehension, BTreeSetComprehension, BinaryHeapComprehension, HashMapComprehension,
    HashSetComprehension, LinkedListComprehension, VecComprehension, VecDequeComprehension,
};
pub(crate) use lazy_evaluation::LazyRefIterator;

#[proc_macro]
pub fn vector(token_stream: TS) -> TS {
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

#[proc_macro]
pub fn iterator_ref(token_stream: TS) -> TS {
    process_comprehension::<LazyRefIterator>(token_stream)
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

pub(crate) fn common_parse(
    input: syn::parse::ParseStream,
) -> (crate::mapping::Mapping, Vec<iter_clause::IterClause>) {
    let mut iter_clauses = Vec::new();

    let mapping = input
        .parse::<crate::mapping::Mapping>()
        .unwrap_or_else(|e| panic!("{}", e));

    while let Ok(iter_clause) = input.parse::<iter_clause::IterClause>() {
        iter_clauses.push(iter_clause);
    }

    (mapping, iter_clauses)
}
