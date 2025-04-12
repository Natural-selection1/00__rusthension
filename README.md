# better_comprehension
[中文README](https://github.com/Natural-selection1/better-comprehension-in-rust/blob/master/README-CN.md)

Collection comprehension and Iterator comprehension in Rust.
And it provides a better experience in Rust.

This library aims to be a good alternative to all comprehension libraries,
for the libraries you encounter when searching "comprehension" on crate.io,
we have already done:
Fully covered libraries:
* [comprehension](https://crates.io/crates/comprehension)
* [kt-list-comprehensions](https://crates.io/crates/kt-list-comprehensions)
* [iter-comprehensions](https://crates.io/crates/iter-comprehensions)
* [list_comprehension](https://crates.io/crates/list_comprehension)
* [cute](https://crates.io/crates/cute)

Partially covered libraries:
* [list_comprehension_macro](https://crates.io/crates/list_comprehension_macro)
  * Does not provide a unified macro that distinguishes by mapping expression (like real Python comprehensions)

    (No plans to support this, as this library already provides all collection types in the Rust standard library)

  * Does not support while loop

    (No plans to support this, [using let expression](#let-expression) is already powerful enough)

# Overview
The syntax is derived from [python comprehensions](https://docs.python.org/3/tutorial/datastructures.html#list-comprehensions), but provides more powerful features, closer to the usage of Rust

This library provides comprehension macros for all collection types in the Rust standard library

( Vec and std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque})

And provides iterator comprehension macros based on references

## Syntax
Value container
```ignore
left_mapping <if conditions else right_mapping>?
<for pattern in iterable <if conditions>?
<let expression>*>+
```

[Key-value container](#key-value-collection-types)

Supports three key-value pair representations, which are `=>` `,` `:`

The following uses `=>` as an example

```ignore
left_key=>left_value <if conditions else right_key=>right_value>?
<for pattern in iterable <if conditions>?
<let expression>*>+
```

*`?` means optional*

*`+` means at least once*

*`*` means 0 times or more times*

*It is not required to break lines, it is just for readability*

* `left/right_mapping/key/value` is an expression that produces a value, which can be a [simple expression](#simple-example), or a [block expression](#execute-code-in-block-before-returning)
* `if conditions` is an expression that produces a bool
* `for pattern in iterable` where `pattern` is a [pattern](#use-pattern-matching), and `iterable` is an iterable object
* `let expression` is a let expression, which can [bind variables](#use-let-expression-to-bind-variables) or [execute arbitrary code](#use-let-_--or-let---to-execute-code)

# Collection Comprehensions
You can completely treat collection comprehension macros as sugar for `for loop`
(In fact, these macros are implemented using `for loop`)
So you'll see many familiar syntaxes
However, they will be more ergonomic and easier to read and use

# Simple Example
```rust
use better_comprehension::vector;
let vec_1 = vec!["AB".to_string(), "CD".to_string()];
let vec_2 = vec!["12".to_string(), "34".to_string()];


// Ownership consuming iteration
// (just pass the single identifier)
// x's type is &String
let vec: Vec<String> = vector![x.clone() for x in vec_1];
// println!("{:?}", vec_1); // borrow of moved value
assert_eq!(vec, vec!["AB".to_string(), "CD".to_string()]);

// Ownership preserving iteration
// (pass &collection or collection.iter().other_method())
// x's type is &String
let vec: Vec<String> = vector![x.clone() for x in vec_2.iter()];
// equivalent writing
// let vec: Vec<String> = vector![x.clone() for x in &vec_2];
println!("{:?}", vec_2); // vec_2 is alive
assert_eq!(vec, vec!["12".to_string(), "34".to_string()]);
```

# if in comprehension

`for` pattern `in` collection `if` ... will be translated to
```ignore
for pattern in collection {
    if ... {

    }
}
```

## if conditions as filtering conditions
Where conditions is any expression that returns a bool
Only when the expression returns true, it will be mapped
```rust
use better_comprehension::linked_list;
use std::collections::LinkedList;
// i's type is i32
let linked_list = linked_list![
    i*2
    for i in 1..=3 if i != 2
];
assert_eq!(linked_list, LinkedList::from([2, 6]));
```

```rust
use better_comprehension::linked_list;
use std::collections::LinkedList;
let judge_function = |i: i32| i != 2;
// i's type is i32
let linked_list = linked_list![
    i*2
    for i in 1..=3 if judge_function(i)
];
assert_eq!(linked_list, LinkedList::from([2, 6]));
```

## if let expression
```rust
use better_comprehension::vector;
let vec_1 = vec![Some("123".to_string()),
                 None,
                 Some("456".to_string())];
let vec = vector![
    __x__.clone()
    for x in vec_1 if let Some(__x__) = x
];
assert_eq!(vec, vec!["123".to_string(),
                     "456".to_string()]
);
```

# Return different values based on conditions
```rust
use better_comprehension::b_tree_set;
use std::collections::BTreeSet;
let b_tree_set = b_tree_set!{
    i if i-1 == 0 else i+10
    for i in 1..=3 if i != 2
    };
assert_eq!(b_tree_set, BTreeSet::from([1, 13]));
```


# let expression
The scope of a let expression is the nearest for in expression above it, and it is affected by the filtering result of the if expression (if any). There can be multiple let expressions, and they are read from top to bottom.

Equivalent to
```ignore
for pattern in collection {
    // if there is an if expression
    if ... {
    let ...;
    let ...;

    }
}
```

## use let expression to bind variables
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

## use let _ = or let () = to execute code
This is a very powerful feature, please use it with caution
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

# Use pattern matching
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

// name's type is &String
let vec_deque: VecDeque<String> = vec_deque![
    name.clone()
    for person @ Person { name, ..} in &people if person.age > 20
];

println!("{:?}", people); // people is alive
assert_eq!(vec_deque, VecDeque::from(["Bob".to_string()]));
```

# Nested Comprehensions
Like Python's comprehensions, this library's for loop is read from top to bottom.

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
// You can use the upper variable in the lower loop
let vec = vector![
    (top,bottom)
    for top in 1..=3 if top != 2
    for bottom in 4..=6 if bottom+top != 4];
assert_eq!(vec, vec![(1, 4), (1, 5), (1, 6),
                     (3, 4), (3, 5), (3, 6)]);
```

# Execute code in block before returning
This is a very powerful feature, you can execute any code before returning but it will reduce readability, please use it with caution
If possible, it is recommended to use `let _ =` or `let () =` to execute code after the last `for in` instead
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
println!("{:?}", vec_2); // vec_2 is alive
assert_eq!(
    vec,
    vec![
        ("abc".to_string(), "123".to_string()),
        ("123".to_string(), "def".to_string())
    ]
);
```

# description of ergonomic
Please note, in Rust, for loop consumes ownership.
So usually, for multi-layer loops, if you want the original collection to be consumed, you should write it like this:
```rust
use better_comprehension::vector;
let vec_1 = vec!["ABC".to_string(), "DEF".to_string()];
let vec_2 = vec!["abc".to_string(), "def".to_string()];
let vec_3 = vec![123, 456];
let vec = {
    // Move the collection you want to consume into the block
    let vec_1 = vec_1;
    let vec_3 = vec_3;

    let mut vec = vec![];
    // In the outer loop, you can choose to use iter() to keep ownership
    // To keep the design consistent, here we choose to use iter()
    for i in vec_1.iter() {
        if i == "ABC" {
            // In the inner loop, you must use iter(),
            // otherwise the ownership will be transferred for the first time
            for j in vec_2.iter() {
                if j == "abc" {
                    for k in vec_3.iter() {
                        if k == &123 {
                            // Only use clone when necessary to avoid unnecessary resource waste
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

In this library, you don't need to do this, the macros will automatically handle these problems for you.
You only need to do two things:
1. For the collection you want to keep ownership, add `.iter()` or use `&`
2. Directly pass the variable name of the collection you want to consume

The rest will be automatically handled in the macro.
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

## Pay attention
The code written in
* [use let _ = or let () = to execute code](#use-let-_--or-let---to-execute-code)
* [Execute code in block before returning](#execute-code-in-block-before-returning)

will not enjoy the ergonomic rules, they are complete rust code

# Key-value collection types
Also, this library supports key-value collection types, HashMap, BTreeMap
And supports three key-value separators "=>" ":" ","

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

println!("{:?}", vec_key); // vec_key is alive
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

# Some details
vector! :       push() to add elements

binary_heap! :  push() to add elements

vec_deque! :    push_back() to add elements

linked_list! :  push_back() to add elements

hash_set! :     insert() to add elements

hash_map! :     insert() to add key-value pairs

b_tree_map! :   insert() to add key-value pairs

b_tree_set! :   insert() to add elements

# Iterator Comprehensions
This library also supports iterator comprehensions, but as the author, I do not recommend using them, the reasons are as follows:
1. In the collection comprehension, we also use references to derive, as long as we do not consume the original collection, we can achieve the same thing
2. The cost of getting a reference copy is not large
3. Because rust does not have a `yield` keyword, the implementation of iterator comprehension is complex, which leads to the inability to use many collection comprehension features

The iterator comprehension is based on references, so it always does not consume ownership
However, to ensure the correctness of the iterator comprehension, only two iterable objects are allowed to be passed in:
* Single identifier (not followed by any method calls)
* Range expression (such as: 1..=3 or 1..x )

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
This implementation makes the following features in collection comprehension unavailable in iterator comprehension:

* if let expression

# Differences
* Ownership consumption:
  * Collection comprehension:
    * Using & or .iter() does not consume ownership
    * Directly passing the variable name consumes ownership
  * Iterator comprehension:
    * Always does not consume ownership, but only allows passing in a single identifier and range expression that does not follow any method calls

* Differences in features:
  * if let expression
    * Collection comprehension: supported
    * Iterator comprehension: not supported

# Some practical examples

```rust
use better_comprehension::vector;
use std::collections::{HashMap, BTreeMap};
// Create a 3x3 matrix
// python-like
let matrix = vector![
    vector![i * 3 + j for j in 1..=3]
    for i in 0..3
];
// More recommended
let matrix = vector![
    row
    for i in 0..3
    let row = vector![
        i * 3 + j
        for j in 1..=3
    ]
];

// Transpose the matrix
// python-like
let transposed = vector![
vector![row[i]
        for row in matrix.iter()]
for i in 0..3
];
// More recommended
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
// ↓ Equivalent to ↓
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
// ↓ Equivalent to ↓
// use comprehension! (python-like)
let name_high_scores_map = b_tree_map![
    &student.name =>
        vector![score.subject for score in &student.scores if score.score >= 85]
    for student in &students_data
];
// ↓ Equivalent to ↓
// More recommended
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