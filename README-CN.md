# better_comprehension

在rust中的集合推导式和迭代器推导式, 提供更好的Rust使用体验

本库旨在成为所有推导式库的良好替代品, 对于你在crate.io中搜索"comprehension"时遇到的库, 我们已经做到了 :

完全覆盖的库 :
    * [comprehension](https://crates.io/crates/comprehension)
    * [kt-list-comprehensions](https://crates.io/crates/kt-list-comprehensions)
    * [iter-comprehensions](https://crates.io/crates/iter-comprehensions)
    * [cute](https://crates.io/crates/cute)

部分覆盖的库 :
* [list_comprehension_macro](https://crates.io/crates/list_comprehension_macro)
  * 暂未提供一个统一的宏, 通过mapping表达式进行区分(就像真正的python推导式那样)
    (不计划支持, 因为本库已经提供了所有rust标准库中的集合类型)

  * 暂未支持while loop

* [list_comprehension](https://crates.io/crates/list_comprehension)
  * 暂未支持let else变量绑定
    (不计划支持, 过分提高了 for in 部分的复杂度, 这些事情完全可以在映射返回块中解决)


# 说明

语法源自 [python推导式](https://docs.python.org/3/tutorial/datastructures.html#list-comprehensions)

本库为Rust标准库中的所有集合类型提供推导式宏
( Vec 和 std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque})

以及基于引用的迭代器推导式宏



# 集合推导式

你可以完全将集合推导式宏视为 `for` 循环的语法糖
(事实上, 这些宏就是使用 `for` 循环实现的)
所以你可以看到很多很熟悉的语法
不过, 他们将更加符合人体工程学, 更加便于阅读和使用

# 简单示例
```rust
use better_comprehension::vector;
let vec_1 = vec!["AB".to_string(), "CD".to_string()];
let vec_2 = vec!["12".to_string(), "34".to_string()];


// 消耗所有权的迭代 (仅传递单一标识符即可)
// x 的类型为 &String
let vec: Vec<String> = vector![x.clone() for x in vec_1];
// println!("{:?}", vec_1); // borrow of moved value
assert_eq!(vec, vec!["AB".to_string(), "CD".to_string()]);

// 保留所有权的迭代 (传递 &collection 或者 collection.iter().other_method())
// x 的类型为 &String
let vec: Vec<String> = vector![x.clone() for x in vec_2.iter()];
// let vec: Vec<String> = vector![x.clone() for x in &vec_2]; // 等价的写法
println!("{:?}", vec_2); // vec_2还活着
assert_eq!(vec, vec!["12".to_string(), "34".to_string()]);
```

# 集合推导中的 if

`for` pattern `in` collection `if` ... 将被完全翻译为
```rust
for pattern in collection {
    if ... {

    }
}
```

## if conditions 作为过滤条件
其中 conditions 为任意值为bool的表达式
只有当表达式返回true时, 才会将对其进行推导映射
```rust
use better_comprehension::linked_list;
use std::collections::LinkedList;
// i 的类型为 i32
let linked_list = linked_list![ i*2 for i in 1..=3 if i != 2 ];
assert_eq!(linked_list, LinkedList::from([2, 6]));
```

```rust
use better_comprehension::linked_list;
use std::collections::LinkedList;
let judge_function = |i: i32| i != 2;
// i 的类型为 i32
let linked_list = linked_list![ i*2 for i in 1..=3 if judge_function(i) ];
assert_eq!(linked_list, LinkedList::from([2, 6]));
```

## if let 表达式
```rust
use better_comprehension::vector;
let vec_1 = vec![Some("123".to_string()), None, Some("456".to_string())];
let vec = vector![
    __x__.clone()
    for x in vec_1 if let Some(__x__) = x
];
assert_eq!(vec, vec!["123".to_string(), "456".to_string()]);
```

# 根据条件返回不同的值
```rust
use better_comprehension::b_tree_set;
use std::collections::BTreeSet;
let b_tree_set = b_tree_set!{
    i if i-1 == 0 else i+10
    for i in 1..=3 if i != 2
    };
assert_eq!(b_tree_set, BTreeSet::from([1, 13]));
```

# let 表达式
let 表达式所属的范围是它上方最近的 for in 表达式, 且受到if表达式筛选后的结果(如果有的话), 可以有多个let表达式, 他们的阅读顺序是从上到下的.
完全等价于
```ignore
for pattern in collection {
    if ... { // 如果有的话
    let ...;
    let ...;

    }
}
```

## 使用let表达式绑定变量
```rust
use better_comprehension::vector;
let vec = vector![
    b
    for x in 1..=3 if x != 2
    let __x__ = x*2
    for y in 4..=6 if y+__x__ != 7
    let z = __x__ + y
    let a = z*2
    let b = match z {
        5..=6 => 1,
        7..=8 => 2,
        _ => 3
    }
];
assert_eq!(vec, vec![1, 2, 3, 3, 3]);
```

## 使用 let _ = 或 let () = 执行任意代码
这是一个极其强大的功能, 请谨慎使用
```rust
use better_comprehension::vector;
let vec = vector![
    x
    for x in 1..=3
    let _ = println!("{}", x)
    let () = {
        for i in 1..=3 {
            println!("{}", i);
        }
    }
];
```

# 使用模式匹配
```rust
use better_comprehension::vec_deque;
use std::collections::VecDeque;
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

let people = [Person { name: "Joe".to_string(), age: 20 },
              Person { name: "Bob".to_string(), age: 25 }];

// name 的类型为 &String
let vec_deque: VecDeque<String> = vec_deque![
    name.clone()
    for person @ Person { name, ..} in &people if person.age > 20
];

println!("{:?}", people); // people还活着
assert_eq!(vec_deque, VecDeque::from(["Bob".to_string()]));
```

# 嵌套推导式
和python的推导式一样, 本库的for循环是从上到下读取的.

```rust
use better_comprehension::binary_heap;
use std::collections::BinaryHeap;
let binary_heap = binary_heap![
    i if (i-1 == 0 || j -2 == 0) else i+10
    for i in 1..=3 if i != 2
    for j in 1..=3 if j+i != 4];
assert_eq!(binary_heap.into_sorted_vec(), vec![1, 1, 3, 13]);
```

```rust
use better_comprehension::vector;
let vec = vector![
    (top,bottom)
    for top in 1..=3 if top != 2
    for bottom in 4..=6 if bottom+top != 4]; // 可以在下层使用上层变量
assert_eq!(vec, vec![(1, 4), (1, 5), (1, 6),
                     (3, 4), (3, 5), (3, 6)]);
```

# 使用块在返回前执行代码
这是一个极其强大的功能, 你可以在返回前执行任意代码但会降低可读性, 请谨慎使用
如果可以, 更推荐使用在最后一个 `for in` 后 `let _ =` 或 `let () =` 执行代码来代替
```rust
use better_comprehension::vector;
let vec_1 = vec!["123".to_string(), "456".to_string()];
let vec_2 = vec!["abc".to_string(), "def".to_string()];
let vec = vector![
    {
        let some = x.clone() + y;
        println!("{}", some);

        (x.clone(), y.clone())
    } if y.contains("d") else {
        println!("{}", y);
        (y.clone(), x.clone())
    }
    for x in vec_1 if x.contains("1")
    for y in vec_2.iter()
];

// println!("{:?}", vec_1); // borrow of moved value
println!("{:?}", vec_2); // 存活
assert_eq!(
    vec,
    vec![
        ("abc".to_string(), "123".to_string()),
        ("123".to_string(), "def".to_string())
    ]
);
```

# 简写说明
请你注意,由于在rust中, for loop 是消耗所有权的.
所以通常来说, 对于多层循环, 如果你希望原容器被消耗, 你应该写成如下这样:
```rust
use better_comprehension::vector;
let vec_1 = vec!["ABC".to_string(), "DEF".to_string()];
let vec_2 = vec!["abc".to_string(), "def".to_string()];
let vec_3 = vec![123, 456];
let vec = {
    // 移动想消耗的集合容器进入块中
    let vec_1 = vec_1;
    let vec_3 = vec_3;

    let mut vec = vec![];
    // 在外层循环里, 你可以选择使用iter()保留所有权
    // 为了保持设计的一致性, 这里选择使用iter()
    for i in vec_1.iter() {
        if i == "ABC" {
            // 在内层循环里, 你必须使用iter(), 否则所有权会在第一次被转移
            for j in vec_2.iter() {
                if j == "abc" {
                    for k in vec_3.iter() {
                        if k == &123 {
                            // 仅在必要时使用clone, 以避免不必要的资源浪费
                            vec.push((i.clone(), j.clone(), *k));
                        }
                    }
                }
            }
        }
    }
    vec
};
// println!("{:?}", vec_1); // borrow of moved value
println!("{:?}", vec_2); // work well
// println!("{:?}", vec_3); // borrow of moved value
```

但在本库中, 你不需要这么做, 提供的宏会自动帮你处理这些问题.
你需要做的只有两件事:
1. 对于想保留所有权的集合后面`加上.iter()`或`使用 &`
2. 在你想消耗的集合直接传入它的变量名
其余会在宏内自动处理.
```rust
use better_comprehension::vector;
let vec_1 = vec!["ABC".to_string(), "DEF".to_string()];
let vec_2 = vec!["abc".to_string(), "def".to_string()];
let vec_3 = vec![123, 456];

let vec = vector![
    (i.clone(),j.clone(),*k)
    for i in vec_1 if i == "ABC"
    for j in vec_2.iter() if j == "abc"
    for k in vec_3 if k == &123
];
// println!("{:?}", vec_1); // borrow of moved value
println!("{:?}", vec_2); // work well
// println!("{:?}", vec_3); // borrow of moved value
```

# 键值对容器类型
同时, 该库还支持键值对容器类型, HashMap, BTreeMap
并且支持三种键值对分隔符 "=>" ":" ","

```rust
use better_comprehension::hash_map;
use std::collections::HashMap;
let vec_key = vec![
    "key_1".to_string(),
    "key_2".to_string(),
    "key_3".to_string(),
];
let vec_value = vec![1, 2, 3];

let hash_map = hash_map! {
    key.clone() : *value
    // key.clone() => *value
    // key.clone() , *value
    for key in &vec_key
    for value in vec_value
};

println!("{:?}", vec_key); // 存活
// println!("{:?}", vec_value); // borrow of moved value
assert_eq!(
    hash_map,
    HashMap::from([
        ("key_1".to_string(), 3),
        ("key_2".to_string(), 3),
        ("key_3".to_string(), 3)
    ])
);
```

# 一些细节
vector! :       push() 添加元素

binary_heap! :  push() 添加元素

vec_deque! :    push_back() 添加元素

linked_list! :  push_back() 添加元素

hash_set! :     insert() 添加元素

hash_map! :     insert() 添加键值对

b_tree_map! :   insert() 添加键值对

b_tree_set! :   insert() 添加元素

# 迭代器推导式
该库也支持迭代器推导式, 但作为作者我并不推荐使用, 原因如下:
1. 在集合推导式中, 我们也是通过引用进行推导的, 只要我们不消耗原集合, 那么就能做到相同的事情
2. 得到引用的副本的代价并不大
3. 由于rust中没有`yield`关键字, 所以迭代器推导式的实现是复杂的, 这导致存在迭代器推导式不能使用很多集合推导式的特性

该迭代器推导式是基于引用的, 所以总是不会消耗所有权
不过, 为了确保迭代器推导式的正确性, 只允许你传入两种可迭代对象:
* 单一标识符(不跟随任何方法调用)
* 范围表达式(如: 1..=3 或者 1..x )

```rust
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
/*
Some(("123", "ABC"))
Some(("123", "DEF"))
Some(("123", "GHI"))
Some(("123", "ABC"))
Some(("123", "DEF"))
Some(("123", "GHI"))
Some(("789", "DEF"))
Some(("789", "DEF"))
None
None
*/
```

以上写法与下面的写法是完全的等价形式
```rust
let vec_1 = ["123".to_string(),
             "456".to_string(),
             "789".to_string()];
let vec_2 = ["ABC".to_string(),
             "DEF".to_string(),
             "GHI".to_string()];

let mut result3 = {
    let vec_2 = vec_2.iter().collect::<Vec<_>>();
    let vec_1 = vec_1.iter().collect::<Vec<_>>();
    (vec_1).into_iter().filter_map(move |x| {
        (x.contains("1") || x.contains("7")).then(|| {
            let vec_2 = vec_2.clone();
            (1..=2).into_iter().filter_map(move |_| {
                (true).then(|| {
                    let vec_2 = vec_2.clone();
                    (vec_2).into_iter().filter_map(move |y| {
                        (y.contains("D") || x.contains("3")).then(|| {
                            if x.contains("1") {
                                (x.clone(), y.clone())
                            } else {
                                (y.clone(), x.clone())
                            }
                        })
                    })
                })
            })
        })
    })
    .flatten()
    .flatten()
};
```
这样的实现方式, 使得以下在集合推导式的功能在迭代器推导式中无法使用:

* if let 表达式

# 差异说明
* 是否消耗所有权:
  * 集合推导式:
    * 使用 & 或者 .iter() 不会消耗所有权
    * 直接传递变量名会消耗所有权
  * 迭代器推导式:
    * 总是不会消耗所有权, 但只允许传入不跟随任何方法调用的单一标识符和范围表达式

* 差异特性:
  * if let 表达式
    * 集合推导式: 支持
    * 迭代器推导式: 不支持

# 一些实际的例子

```rust
use better_comprehension::vector;
use std::collections::{HashMap, BTreeMap};
// 创建3x3矩阵

// python-like
let matrix = vector![
    vector![i * 3 + j + 1 for j in 0..3]
    for i in 0..3
];

// 更推荐的写法
let matrix = vector![
    row
    for i in 0..3
    let row = vector![
        i * 3 + j + 1
        for j in 0..3
    ]
];

// 矩阵转置
// python-like
let transposed = vector![
vector![row[i]
        for row in matrix.iter()]
for i in 0..3
];

// 更推荐的写法
let transposed = vector![
    row
    for i in 0..3
    let row = vector![
        row[i]
        for row in matrix.iter()
    ]
];
// matrix is alive
assert_eq!(matrix, vec![vec![1, 2, 3],
                        vec![4, 5, 6],
                        vec![7, 8, 9]]);
assert_eq!(
    transposed,
    vec![vec![1, 4, 7],
         vec![2, 5, 8],
         vec![3, 6, 9]]
);
```


```rust
use better_comprehension::{hash_map, b_tree_map, vector};
use std::collections::{HashMap, BTreeMap};
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

// use for loop
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
// use comprehension!
let math_scores: HashMap<&String, u8> = hash_map![
    &student.name => score.score
    for student in &students_data
    for score in &student.scores if score.subject == "Math"
];

assert_eq!(
    math_scores,
    HashMap::from([(&"Alice".to_string(), 95),
                   (&"Bob".to_string(), 78)]));

// use for loop
let name_high_scores_map = {
    let mut name_high_scores_map = BTreeMap::new();
    for student in &students_data {
        let mut subjects = Vec::new();
        for score in &student.scores {
            if score.score >= 85 {
                subjects.push(score.subject);
            }
        }
        name_high_scores_map.insert(&student.name, subjects);
    }
    name_high_scores_map
};
// ↓ 等价于 ↓
// use comprehension! (python-like)
let name_high_scores_map = b_tree_map![
    &student.name =>
        vector![score.subject for score in &student.scores if score.score >= 85]
    for student in &students_data
];
// ↓ 等价于 ↓
// 更推荐的写法
let name_high_scores_map = b_tree_map![
    &student.name => subjects
    for student in &students_data
    let subjects = vector![
        score.subject
        for score in &student.scores if score.score >= 85
    ]
];

assert_eq!(
    name_high_scores_map,
    BTreeMap::from([
        (&"Alice".to_string(), vec!["Math", "English"]),
        (&"Bob".to_string(), vec!["English"])
    ])
);
```
