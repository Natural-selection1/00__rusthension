#![allow(unused)]
use rusthension::{
    b_tree_map, b_tree_set, binary_heap, hash_map, hash_set, lazy_ref_iterator,
    linked_list, vec_deque,
};
use std::collections::{
    BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque,
};

fn main() {
    test_vec();
    test_binary_heap();
    test_linked_list();
    test_b_tree_set();
    test_b_tree_map();
    test_vec_deque();
    test_hash_set();
    test_hash_map();
    test_自建类型();
    test_lazy_ref_iterator();
}

fn test_vec() {
    {
        let result = rusthension::vec![
            [x, y] if x > y else [y, x]
            for x in (0..y+2) if x % 2 == 0
            for y in 0..7 if y % 3 == 0
        ];
        assert_eq!(
            result,
            [
                [0, 0],
                [3, 0],
                [3, 2],
                [4, 3],
                [6, 0],
                [6, 2],
                [6, 4],
                [6, 6]
            ]
        );
        println!("测试返回元组_2迭代器_有条件");
    }
    {
        let vec_comprehension1 = vec![("a", 1), ("b", 2), ("c", 3)];
        let vec_comprehension2 = vec![("a", 1), ("b", 2), ("c", 3)];
        let vec_comprehension3 = vec![("a", 1), ("b", 2), ("c", 3)];
        let _result = rusthension::vec![
            y if x > *z else y
            for x in vec_comprehension1.clone()
            for y in vec_comprehension2
            for z in &vec_comprehension3
        ];

        println!("{:#?}", vec_comprehension1);
        // println!("{:#?}", vec_comprehension2);
        println!("{:#?}", vec_comprehension3);
        println!("{:#?}", _result);
    }
}

fn test_binary_heap() {
    let vec = vec![1, 2, 3];
    let result = binary_heap![x for x in vec];
    assert_eq!(result.into_sorted_vec(), vec![1, 2, 3]);
}

fn test_linked_list() {
    let vec = vec![1, 2, 3];
    let result = linked_list![x for x in vec];
    assert_eq!(result, LinkedList::from([1, 2, 3]));
}

fn test_b_tree_set() {
    let vec = vec![1, 2, 3];
    let result = b_tree_set![x for x in vec];
    assert_eq!(result, BTreeSet::from([1, 2, 3]));
}

fn test_b_tree_map() {
    let vec_key = vec!["key_1", "key_2", "key_3"];
    let vec_value = [1, 2, 3];

    let result = b_tree_map![x , y for y in vec_value for x in vec_key];
    assert_eq!(
        result,
        BTreeMap::from([("key_1", 3), ("key_2", 3), ("key_3", 3)])
    );
}

fn test_vec_deque() {
    let vec = vec![1, 2, 3];
    let result = vec_deque![x for x in vec];
    assert_eq!(result, VecDeque::from([1, 2, 3]));
}

fn test_hash_set() {
    let vec = vec![1, 2, 3];
    let result = hash_set![x for x in vec];
    assert_eq!(result, HashSet::from([1, 2, 3]));
}

fn test_hash_map() {
    let vec_key = vec!["key_1", "key_2", "key_3"];
    let vec_value = [1, 2, 3];

    let result = hash_map![x , y for y in vec_value for x in vec_key];
    assert_eq!(
        result,
        HashMap::from([("key_1", 3), ("key_2", 3), ("key_3", 3)])
    );
}

fn test_自建类型() {
    #[derive(Debug, PartialEq, Eq)]
    struct MyType {
        x: i32,
        y: i32,
    }

    let vec_y = vec![2, 4, 6];
    let vec_x = vec![1, 3, 5];
    let _result = rusthension::vec![
        MyType { x, y }
        for x in vec_x if y == x + 1
        for y in vec_y
    ];
    println!("{:#?}", _result);
}

fn test_lazy_ref_iterator() {
    let vec_1 = vec!["123".to_string(), "456".to_string(), "789".to_string()];
    let vec_2 = vec!["123".to_string(), "456".to_string(), "789".to_string()];

    let result =
        lazy_ref_iterator![x for x in vec_1 if x.contains("1") for i in 1..=9]; // 范围最外层

    let result2 = lazy_ref_iterator![x for i in 1..=9 for x in vec_1 if x.contains("123")]; // 范围最内层

    let result3 =//双重迭代器
        lazy_ref_iterator![x.clone() for x in vec_1 if x.contains("1") for y in vec_2 if y.contains("4")];

    for x in result3 {

        println!("{:#?}", x);
    }
    println!("--------------------------------");
    for x in result2 {
        println!("{:#?}", x);
    }
    println!("--------------------------------");
    for x in result {
        println!("{:#?}", x);
    }
}
