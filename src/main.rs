use rusthension::{
    b_tree_map_comprehension, b_tree_set_comprehension,
    binary_heap_comprehension, linked_list_comprehension, vec_comprehension,
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList};

fn main() {
    test_vec();
    test_binary_heap();
    test_linked_list();
    test_b_tree_set();
    test_b_tree_map();
}

fn test_vec() {
    {
        println!("测试返回元组_1迭代器_有条件");
        let vec = vec![("a", 1), ("b", 2), ("c", 3)];
        let result = vec_comprehension![(x, y) for (x, y) in vec if y >= 2];
        assert_eq!(result, [("b", 2), ("c", 3)]);
    }

    {
        println!("测试返回String_1迭代器_有条件");
        let vec = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("c".to_string(), 3),
        ];
        let result = vec_comprehension![(x, y) for (x, y) in vec if y >= 2];

        assert_eq!(result, [("b".to_string(), 2), ("c".to_string(), 3)]);
    }
    {
        let result = vec_comprehension![
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
        let result = vec_comprehension![
            y if x > *z else y
            for x in vec_comprehension1.clone()
            for y in vec_comprehension2
            for z in &vec_comprehension3
        ];

        println!("{:?}", vec_comprehension1);
        // println!("{:?}", vec_comprehension2);
        println!("{:?}", result);
    }
}

fn test_binary_heap() {
    let vec = vec![1, 2, 3];
    let result = binary_heap_comprehension![x for x in vec];
    assert_eq!(result.into_sorted_vec(), vec![1, 2, 3]);
}

fn test_linked_list() {
    let vec = vec![1, 2, 3];
    let result = linked_list_comprehension![x for x in vec];
    assert_eq!(result, LinkedList::from([1, 2, 3]));
}

fn test_b_tree_set() {
    let vec = vec![1, 2, 3];
    let result = b_tree_set_comprehension![x for x in vec];
    assert_eq!(result, BTreeSet::from([1, 2, 3]));
}

fn test_b_tree_map() {
    let vec_key = vec![("a", 1), ("b", 2), ("c", 3)];
    let vec_value = vec![("a", 1), ("b", 2), ("c", 3)];

    let result =
        b_tree_map_comprehension![x , y for x in vec_key for y in vec_value];
    assert_eq!(
        result,
        BTreeMap::from([
            (("a", 1), ("a", 1)),
            (("b", 2), ("b", 2)),
            (("c", 3), ("c", 3))
        ])
    );
}
