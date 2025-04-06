#![allow(
    unused,
    clippy::filter_map_bool_then,
    clippy::nonminimal_bool,
    clippy::unnecessary_lazy_evaluations,
    clippy::if_same_then_else,
    clippy::useless_conversion
)]
use better_comprehension::{
    b_tree_map, b_tree_set, binary_heap, hash_map, hash_set, iterator_ref, linked_list, vec_deque,
    vector,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

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
    test_ref_iterator();
}

fn test_vec() {
    {
        let result = vector![
            [x, y] if x > y else [y, x]
            for y in 0..7 if y % 3 == 0
            for x in (0..y+2) if x % 2 == 0
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
        let _result = vector![
            y if x > *z else y
            for z in &vec_comprehension3
            for x in vec_comprehension1.clone()
            for y in vec_comprehension2
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
    let _result = vector![
        MyType { x, y }
        for y in vec_y
        for x in vec_x if y == x + 1
    ];
    println!("{:#?}", _result);
}

fn test_ref_iterator() {
    let vec_1 = ["123".to_string(), "456".to_string(), "789".to_string()];
    let vec_2 = ["ABC".to_string(), "DEF".to_string(), "GHI".to_string()];

    // let result = iterator_ref![x for x in vec_1 if x.contains("1") for i in 1..=9]; // 范围最外层

    // let result2 = iterator_ref![x for i in 1..=9 for x in vec_1 if x.contains("123")]; // 范围最内层

    // let result3 =
    // {
    //     let vec_2 = vec_2.iter().collect::<Vec<_>>();
    //     let vec_1 = vec_1.iter().collect::<Vec<_>>();
    //     (vec_1)
    //         .into_iter()
    //         .filter_map(move |x|  x.contains("1") || x.contains("7") )
    //         .then(|| {
    //             let vec_2 = vec_2.clone();
    //             (1..=9)
    //                 .into_iter()
    //                 .filter_map(move |i| true)
    //                 .then(|| {
    //                     let vec_2 = vec_2.clone();
    //                     (vec_2)
    //                         .into_iter()
    //                         .filter_map(move |y|  y.contains("A") || y.contains("D") )
    //                         .then(|| { (x, y) })
    //                 })
    //         })
    // };

    let result4 =


    // {
    //     let vec_2 = vec_2.iter().collect::<Vec<_>>();
    //     let vec_1 = vec_1.iter().collect::<Vec<_>>();

    //     (vec_1)
    //         .into_iter()
    //         .filter_map(move |x| {
    //             (true && (x.contains("1") || x.contains("7")))
    //             .then(|| {
    //                 // 进入第二层

    //                 let vec_2 = vec_2.clone();
    //                 (1..=9).into_iter().filter_map(move |i| {
    //                     (true && (x.contains("1") && i >= 8)).then(|| {
    //                         // 进入第三层

    //                         let vec_2 = vec_2.clone();
    //                         (vec_2).into_iter().filter_map(move |y| {
    //                             (true && (y.contains("A") || y.contains("D"))).then(|| {
    //                                 // 进入第四层

    //                                 if 1 > 2 { (y) } else { (y) }

    //                                 //
    //                             })
    //                         })

    //                         // 离开最内层
    //                     })
    //                 })

    //                 //第二层结束
    //             })
    //         })
    //         .flatten()
    //         .flatten() // 有 n 层就要 n-1 次 flatten()
    // };

    iterator_ref![
    (x, y)
    for x in vec_1 if x.contains("1") || x.contains("7")
    for i in 1..=9
    for j in 1..=i
    for k in 1..=j
    for y in vec_2 if y.contains("A") || y.contains("D") || x.contains("3")];

    // println!("{:#?}", result2);
    // for (x, y) in result3 {
    //     println!("{:#?}", x);
    //     println!("{:#?}", y);
    //     println!("--------------------------------");
    // }
}
