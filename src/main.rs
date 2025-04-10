#![allow(unused)]
use better_comprehension::{
    b_tree_map, b_tree_set, binary_heap, hash_map, hash_set, iterator_ref, linked_list, vec_deque,
    vector,
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    test_vec();
    test_binary_heap();
    test_linked_list();
    test_b_tree_set();
    test_b_tree_map();
    test_vec_deque();
    test_hash_set();
    test_hash_map();
    test_custom_type();
    test_ref_iterator();
    test_pattern_matching();
    test_nested_comprehension();
    test_ownership_handling();
    test_option();
    some_real_example_2();
}

fn test_vec() {
    {
        let vec_1 = vec![Some("123".to_string()), Some("456".to_string())];
        let vec_2 = vec!["abc".to_string(), "def".to_string()];
        let vec = vector![
            {
                let some = x_1.clone() + y;
                println!("{}", some);

                (x.clone(), y.clone())
            }
            for x in vec_1 if let Some(x_1) = x
            for y in vec_2 if y.contains("d")
        ];
    }

    // 简单示例
    {
        let vec_1 = vec!["AB".to_string(), "CD".to_string()];
        let vec: Vec<String> = vector![{println!("{}", x);
                                        x.clone()}
                                        for x in vec_1];
        assert_eq!(vec, vec!["AB".to_string(), "CD".to_string()]);
    }

    // 条件返回不同值
    {
        let result = vector![
            {println!("{}", x); [x, y]} if x > y
            else {println!("{}", y); [y, x]}
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
    }

    // 多层循环和引用
    {
        let vec_comprehension1 = vec![("a", 1), ("b", 2), ("c", 3)];
        let vec_comprehension2 = vec![("a", 1), ("b", 2), ("c", 3)];
        let vec_comprehension3 = vec![("a", 1), ("b", 2), ("c", 3)];
        let _result = vector![
            y if x > z else y
            for z in &vec_comprehension3
            for x in vec_comprehension1
            for y in &vec_comprehension2
        ];
    }
}

fn test_binary_heap() {
    // 简单示例
    let vec = vec![1, 2, 3];
    let result = binary_heap![*x for x in vec];
    assert_eq!(result.into_sorted_vec(), vec![1, 2, 3]);

    // 条件返回不同值
    let binary_heap = binary_heap![
        i if i-1 == 0 || j-2 == 0 else i+10
        for i in 1..=3 if i != 2
        for j in 1..=3 if j+i != 4];
    assert_eq!(binary_heap.into_sorted_vec(), vec![1, 1, 3, 13]);
}

fn test_linked_list() {
    // 简单示例
    let vec = vec![1, 2, 3];
    let result = linked_list![*x for x in vec];
    assert_eq!(result, LinkedList::from([1, 2, 3]));

    // 过滤值
    let linked_list = linked_list![i*2 for i in 1..=3 if i != 2];
    assert_eq!(linked_list, LinkedList::from([2, 6]));
}

fn test_b_tree_set() {
    // 简单示例
    let vec = vec![1, 2, 3];
    let result = b_tree_set![*x for x in vec];
    assert_eq!(result, BTreeSet::from([1, 2, 3]));

    // 条件返回不同值
    let b_tree_set = b_tree_set! {
        i if i-1 == 0 else i+10
        for i in 1..=3 if i != 2
    };
    assert_eq!(b_tree_set, BTreeSet::from([1, 13]));
}

fn test_b_tree_map() {
    // 简单示例
    let vec_key = vec!["key_1", "key_2", "key_3"];
    let vec_value = [1, 2, 3];

    let result = b_tree_map![*x , *y for y in vec_value for x in vec_key];
    assert_eq!(
        result,
        BTreeMap::from([("key_1", 3), ("key_2", 3), ("key_3", 3)])
    );

    // 键值对分隔符测试
    let vec_key = [
        "key_1".to_string(),
        "key_2".to_string(),
        "key_3".to_string(),
    ];
    let vec_value = [1, 2, 3];

    // 使用冒号分隔
    let b_tree_map1 = b_tree_map! {
        key.clone() : *value
        for key in vec_key.iter()
        for value in vec_value
    };

    // 使用箭头分隔
    let vec_key = [
        "key_1".to_string(),
        "key_2".to_string(),
        "key_3".to_string(),
    ];
    let b_tree_map2 = b_tree_map! {
        key.clone() => *value
        for key in vec_key.iter()
        for value in vec_value
    };

    // 使用逗号分隔
    let vec_key = [
        "key_1".to_string(),
        "key_2".to_string(),
        "key_3".to_string(),
    ];
    let b_tree_map3 = b_tree_map! {
        key.clone() , *value
        for key in vec_key.iter()
        for value in vec_value
    };

    let expected = BTreeMap::from([
        ("key_1".to_string(), 3),
        ("key_2".to_string(), 3),
        ("key_3".to_string(), 3),
    ]);

    assert_eq!(b_tree_map1, expected);
    assert_eq!(b_tree_map2, expected);
    assert_eq!(b_tree_map3, expected);
}

fn test_vec_deque() {
    // 简单示例
    let vec = vec![1, 2, 3];
    let result = vec_deque![*x for x in vec];
    assert_eq!(result, VecDeque::from([1, 2, 3]));

    // 模式匹配示例
    #[derive(Debug, PartialEq, Eq)]
    struct Person {
        name: String,
        age: i32,
    }
    let people = [
        Person {
            name: "Joe".to_string(),
            age: 20,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
    ];
    let vec_deque = vec_deque![name.clone() for Person { name, .. } in people];
    assert_eq!(
        vec_deque,
        VecDeque::from(["Joe".to_string(), "Bob".to_string()])
    );
}

fn test_hash_set() {
    // 简单示例
    let vec = vec![1, 2, 3];
    let result = hash_set![*x for x in vec];
    assert_eq!(result, HashSet::from([1, 2, 3]));

    // 过滤值
    let hash_set = hash_set![i*2 for i in 1..=3 if i != 2];
    assert_eq!(hash_set, HashSet::from([2, 6]));
}

fn test_hash_map() {
    // 简单示例
    let vec_key = vec!["key_1", "key_2", "key_3"];
    let vec_value = [1, 2, 3];

    let result = hash_map![*x , *y for y in vec_value for x in vec_key];
    assert_eq!(
        result,
        HashMap::from([("key_1", 3), ("key_2", 3), ("key_3", 3)])
    );

    // 键值对分隔符测试
    let vec_key = vec![
        "key_1".to_string(),
        "key_2".to_string(),
        "key_3".to_string(),
    ];
    let vec_value = [1, 2, 3];

    let hash_map = hash_map! {
        key.clone() : *value
        for key in vec_key
        for value in vec_value
    };

    assert_eq!(
        hash_map,
        HashMap::from([
            ("key_1".to_string(), 3),
            ("key_2".to_string(), 3),
            ("key_3".to_string(), 3)
        ])
    );
}

fn test_custom_type() {
    #[derive(Debug, PartialEq, Eq)]
    struct MyType {
        x: i32,
        y: i32,
    }

    let vec_y = vec![2, 4, 6];
    let vec_x = vec![1, 3, 5];
    let _result = vector![
        MyType { x: *x, y: *y }
        for y in vec_y
        for x in vec_x if *y == *x + 1
    ];
}

fn test_pattern_matching() {
    #[derive(Debug, PartialEq, Eq)]
    struct Person {
        name: String,
        age: i32,
    }

    let people = [
        Person {
            name: "Joe".to_string(),
            age: 20,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
    ];

    // 使用模式匹配提取字段
    let names = vector![name.clone() for Person { name, .. } in &people];
    assert_eq!(
        names,
        vec!["Joe".to_string(), "Bob".to_string(), "Alice".to_string()]
    );

    // 使用模式匹配和条件过滤
    let adult_names = vector![
        name.clone()
        for Person { name, age } in people
        if *age >= 25
    ];
    assert_eq!(adult_names, vec!["Bob".to_string(), "Alice".to_string()]);
}

fn test_nested_comprehension() {
    // 嵌套推导式示例
    let vec = vector![
        (top, bottom)
        for top in 1..=3 if top != 2
        for bottom in 4..=6 if bottom+top != 4
    ];
    assert_eq!(vec, vec![(1, 4), (1, 5), (1, 6), (3, 4), (3, 5), (3, 6)]);

    // 多层条件嵌套
    let vec = vector![
        (i, j, k)
        for i in 1..=2
        for j in 3..=4 if i+j > 4
        for k in 5..=6 if i+j+k > 10
    ];
    assert_eq!(vec, vec![(1, 4, 6), (2, 3, 6), (2, 4, 5), (2, 4, 6)]);
}

fn test_ownership_handling() {
    // 测试所有权处理
    let vec_1 = vec!["ABC".to_string(), "DEF".to_string()];
    let vec_2 = ["abc".to_string(), "def".to_string()];
    let vec_3 = vec![123, 456];

    let vec = vector![
        (i.clone(), j.clone(), *k)
        for i in vec_1 if i == "ABC"
        for j in vec_2.iter() if j == "abc"
        for k in vec_3 if k == &123
    ];

    assert_eq!(vec, vec![("ABC".to_string(), "abc".to_string(), 123)]);
}

fn test_ref_iterator() {
    let vec_1 = ["123".to_string(), "456".to_string(), "789".to_string()];
    let vec_2 = ["ABC".to_string(), "DEF".to_string(), "GHI".to_string()];

    // 基于引用的迭代器推导式
    let mut result3 = iterator_ref![
        (x.clone(), y.clone()) if x.contains("1") else (y.clone(), x.clone())
        for x in vec_1 if x.contains("1") || x.contains("7")
        for _ in 1..=2
        for y in vec_2 if y.contains("D") || x.contains("3")
    ];

    // 验证迭代器已耗尽
    for _ in 0..=9 {
        result3.next();
    }
    assert_eq!(result3.next(), None);

    // 验证原始集合未被消耗
    assert_eq!(vec_1.len(), 3);
    assert_eq!(vec_2.len(), 3);
}

fn test_option() {
    let vec = [Some("1".to_string()), None, Some("3".to_string())];

    let result = {
        let mut result = Vec::new();
        for x in vec.iter().flatten() {
            result.push(x.clone());
        }
        result
    };
    // ↓ 等价于 ↓
    let result = vector![
        x.clone()
        for x in vec.iter().flatten()
    ];

    assert_eq!(result, vec!["1".to_string(), "3".to_string()]);
}

fn some_real_example_1() {
    // 创建3x3矩阵
    let matrix = vector![
        vector![i * 3 + j + 1 for j in 0..3]
        for i in 0..3
    ];

    // 矩阵转置
    let transposed = vector![
    vector![row[i]
            for row in matrix.iter()]
    for i in 0..3
    ];
    // matrix is alive
    assert_eq!(matrix, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    assert_eq!(
        transposed,
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
}

fn some_real_example_2() {
    #[derive(Debug, PartialEq, Eq)]
    struct Score {
        subject: &'static str,
        score: u8,
    }
    #[derive(Debug, PartialEq, Eq)]
    struct Student {
        name: String,
        age: u8,
        scores: Vec<Score>,
    }

    let students_data = [
        Student {
            name: "Alice".to_string(),
            age: 20,
            scores: vec![
                Score {
                    subject: "Math",
                    score: 95,
                },
                Score {
                    subject: "English",
                    score: 88,
                },
            ],
        },
        Student {
            name: "Bob".to_string(),
            age: 21,
            scores: vec![
                Score {
                    subject: "Math",
                    score: 78,
                },
                Score {
                    subject: "English",
                    score: 85,
                },
            ],
        },
    ];

    let math_scores: HashMap<&String, u8> = {
        let mut math_scores = HashMap::new();
        for student in &students_data {
            for score in &student.scores {
                if score.subject == "Math" {
                    math_scores.insert(&student.name, score.score);
                }
            }
        }
        math_scores
    };
    // ↓ 等价于 ↓
    let math_scores: HashMap<&String, u8> = hash_map![
        &student.name => score.score
        for student in &students_data
        for score in &student.scores if score.subject == "Math"
    ];

    assert_eq!(
        math_scores,
        HashMap::from([(&"Alice".to_string(), 95), (&"Bob".to_string(), 78)])
    );

    let high_scores = {
        let mut high_scores = BTreeMap::new();
        for student in &students_data {
            let mut subjects = Vec::new();
            for score in &student.scores {
                if score.score >= 85 {
                    subjects.push(score.subject);
                }
            }
            high_scores.insert(&student.name, subjects);
        }
        high_scores
    };
    // ↓ 等价于 ↓
    let high_scores = b_tree_map![
        &student.name =>
            vector![score.subject for score in &student.scores if score.score >= 85]
        for student in &students_data
    ];

    assert_eq!(
        high_scores,
        BTreeMap::from([
            (&"Alice".to_string(), vec!["Math", "English"]),
            (&"Bob".to_string(), vec!["English"])
        ])
    );
}

#[test]
fn some() {
use better_comprehension::iterator_ref;
let vec_1 = ["123".to_string(),
             "456".to_string(),
             "789".to_string()];
let vec_2 = ["ABC".to_string(),
             "DEF".to_string(),
             "GHI".to_string()];

let mut result3 = iterator_ref![
    (x.clone(), y.clone()) if x.contains("1") else (y.clone(), x.clone())
    for x in vec_1 if x.contains("1") || x.contains("7")
    for i in 1..=2
    for y in vec_2 if y.contains("D") || x.contains("3")];

// still alive
println!("{:?}", vec_1);
println!("{:?}", vec_2);

for _ in 0..=9 {
    println!("{:?}", result3.next());
}
}
