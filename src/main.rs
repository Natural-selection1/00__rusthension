use rusthension::{
    b_tree_map, b_tree_set, binary_heap, hash_map, hash_set, linked_list, vec_,
    vec_deque,
};
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
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
}

fn test_vec() {
    {
        println!("测试返回元组_1迭代器_有条件");
        let vec = vec![("a", 1), ("b", 2), ("c", 3)];
        let result = vec_![(x, y) for (x, y) in vec if y >= 2];
        assert_eq!(result, [("b", 2), ("c", 3)]);
    }

    {
        println!("测试返回String_1迭代器_有条件");
        let vec = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("c".to_string(), 3),
        ];
        let result = vec_![(x, y) for (x, y) in vec if y >= 2];

        assert_eq!(result, [("b".to_string(), 2), ("c".to_string(), 3)]);
    }
    {
        let result = vec_![
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
        let _result = vec_![
            y if x > *z else y
            for x in vec_comprehension1.clone()
            for y in vec_comprehension2
            for z in &vec_comprehension3
        ];
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
