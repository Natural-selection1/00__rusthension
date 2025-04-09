# better_comprehension
[中文README](https://github.com/Natural-selection1/better-comprehension-in-rust/blob/master/README-CN.md)

Collection comprehension and Iterator comprehension in Rust.
And it provides a better experience in Rust.

# Usage

The syntax is derived from [Python's comprehension](https://docs.python.org/3/tutorial/datastructures.html#list-comprehensions).

This library provides macros for all collection types
in the Rust standard library and an Iterator based on references.

---
simple example
```rust
use better_comprehension::vector;
let vec_1 = vec!["AB".to_string(), "CD".to_string()];
let vec: Vec<String> = vector![x.clone() for x in vec_1];
assert_eq!(vec, vec!["AB".to_string(), "CD".to_string()]);
```
---
You can also use patterns in it
```rust
use better_comprehension::vec_deque;
use std::collections::VecDeque;
struct Person {
    name: String,
    age: i32,
}
let people = [Person { name: "Joe".to_string(), age: 20 },
              Person { name: "Bob".to_string(), age: 25 }];
let vec_deque = vec_deque![ name.clone()
                            for Person { name, ..} in people];
assert_eq!(vec_deque,
           VecDeque::from(["Joe".to_string(),
                           "Bob".to_string()]));
```
---
filtering values before comprehension
```rust
use better_comprehension::linked_list;
use std::collections::LinkedList;
let linked_list = linked_list![ i*2 for i in 1..=3 if i != 2 ];
assert_eq!(linked_list, LinkedList::from([2, 6]));
```
---
return different values based on conditions
```rust
use better_comprehension::b_tree_set;
use std::collections::BTreeSet;
let b_tree_set = b_tree_set!{
    i if i-1 == 0 else i+10
    for i in 1..=3 if i != 2
    };
assert_eq!(b_tree_set, BTreeSet::from([1, 13]));
```
---
nested comprehension
```rust
use better_comprehension::binary_heap;
use std::collections::BinaryHeap;
let binary_heap = binary_heap![
    i if (i-1 == 0 || j -2 == 0) else i+10
    for i in 1..=3 if i != 2
    for j in 1..=3 if j+i != 4];
assert_eq!(binary_heap.into_sorted_vec(), vec![1, 1, 3, 13]);
```
---
the reading order of the for loop in this library is from top to bottom,
just like Python's comprehension.
```rust
use better_comprehension::vector;
let vec = vector![
    (top,bottom)
    for top in 1..=3 if top != 2
    for bottom in 4..=6 if bottom+top != 4];
assert_eq!(vec, vec![(1, 4), (1, 5), (1, 6),
                     (3, 4), (3, 5), (3, 6)]);
```
---

Note that in Rust, for loops consume ownership.
So typically, for nested loops,
if you want the original container to be consumed,
you should write it like this:

```rust
use better_comprehension::vector;
let vec_1 = vec!["ABC".to_string(), "DEF".to_string()];
let vec_2 = vec!["abc".to_string(), "def".to_string()];
let vec_3 = vec![123, 456];
let vec = {
    // shadow the variable you want to consume
    let vec_1 = vec_1;
    let vec_3 = vec_3;

    let mut vec = vec![];
    for i in vec_1.iter() {
        if i == "ABC" {
            // In the inner loop, you must use iter(),
            // otherwise ownership will be transferred for the first time
            for j in vec_2.iter() {
                if j == "abc" {
                    // If you do not use iter(),
                    // then the ownership of vec_3
                    // will be transferred for the first time
                    for k in vec_3.iter() {
                        if k == &123 {
                            // Only use clone when necessary
                            // to avoid unnecessary resource waste
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
---
But in this library, you don't need to do this,

the provided macros will automatically handle these problems for you.

You only need to add `.iter()` or
use `&` before the variable you want to keep ownership,

the rest will be automatically handled in the macro.
```rust
use better_comprehension::vector;
let vec_1 = vec!["ABC".to_string(), "DEF".to_string()];
let vec_2 = vec!["abc".to_string(), "def".to_string()];
let vec_3 = vec![123, 456];
let vec = vector![
    (i.clone(),j.clone(),*k)
    for i in vec_1 if i == "ABC"
    for j in vec_2.iter() if j == "abc"
    // for j in &vec_2 if j == "abc"  // this is also reasonable
    for k in vec_3 if k == &123
];
// println!("{:?}", vec_1); // borrow of moved value
println!("{:?}", vec_2); // work well
// println!("{:?}", vec_3); // borrow of moved value
```
---
This library also supports key-value collection types, HashMap, BTreeMap
```rust
use better_comprehension::hash_map;
use std::collections::HashMap;
let vec_key = vec!["key_1".to_string(),
                   "key_2".to_string(),
                   "key_3".to_string()];
let vec_value = [1, 2, 3];
let hash_map = hash_map!{
    // the following three key-value pair separators are supported
    key.clone() : *value
    // key.clone() => *value
    // key.clone() , *value
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
```
---
Iterator comprehension is also supported,
but unlike the collection comprehension above,

this iterator comprehension is based on references,
so it will not consume ownership.
By the way, to ensure the correctness of the iterator comprehension,
only two types of iterable objects are allowed to be passed in:
* single identifier (not followed by any method calls)
* range expression (e.g., 1..=3 or 1..x where x is an number)
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

The above writing is equivalent to the following writing
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
# some details

vector! : push() to add elements

vec_deque! : push_back() to add elements

linked_list! : push_back() to add elements

binary_heap! : push() to add elements

hash_set! : insert() to add elements

b_tree_set! : insert() to add elements

hash_map! : insert() to add key-value pairs

b_tree_map! : insert() to add key-value pairs

# some real examples

```rust
use better_comprehension::vector;
use std::collections::{HashMap, BTreeMap};
// create a 3x3 matrix
let matrix = vector![
    vector![i * 3 + j + 1 for j in 0..3]
    for i in 0..3
];

// transpose the matrix
let transposed = vector![
vector![row[i]
        for row in matrix.iter()]
for i in 0..3
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
// ↓ is equivalent to ↓
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
// ↓ is equivalent to ↓
// use comprehension!
let high_scores = b_tree_map![
    &student.name =>
        vector![score.subject
                for score in &student.scores if score.score >= 85]
    for student in &students_data
];

assert_eq!(
    high_scores,
    BTreeMap::from([
        (&"Alice".to_string(), vec!["Math", "English"]),
        (&"Bob".to_string(), vec!["English"])
    ])
);
```