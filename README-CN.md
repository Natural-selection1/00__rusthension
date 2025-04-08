# better_comprehension

在rust中的集合推导式和迭代器推导式。提供更好的Rust使用体验

# 用法
语法源自[python推导式](https://docs.python.org/3/tutorial/datastructures.html#list-comprehensions)

本库为Rust标准库中的所有集合类型提供宏，以及基于引用的迭代器

---
简单示例

```rust
let vec_1 = vec!["AB".to_string(), "CD".to_string()];

let vec: Vec<String> = vector![x.clone() for x in vec_1];
assert_eq!(vec, vec!["AB".to_string(), "CD".to_string()]);
```

---
你也可以在推导式中使用模式
```rust
struct Person {
    name: String,
    age: i32,
}
let people = [Person { name: "Joe".to_string(), age: 20 },
              Person { name: "Bob".to_string(), age: 25 }];

let vec_deque = vec_deque![name.clone() for Person { name, .. } in people];
assert_eq!(vec_deque, VecDeque::from(["Joe".to_string(), "Bob".to_string()]));
```
---

过滤集合中的值
```rust
let linked_list = linked_list![ i*2 for i in 1..=3 if i != 2 ];
assert_eq!(linked_list, LinkedList::from([2, 6]));
```
---

根据条件返回不同的值
```rust
let b_tree_set = b_tree_set!{
    i if i-1 == 0 else i+10
    for i in 1..=3 if i != 2
    };
assert_eq!(b_tree_set, BTreeSet::from([1, 13]));
```

---
嵌套推导式
```rust
let binary_heap = binary_heap![
    i if (i-1 == 0 || j -2 == 0) else i+10
    for i in 1..=3 if i != 2
    for j in 1..=3 if j+i != 4];
assert_eq!(binary_heap.into_sorted_vec(), vec![1, 1, 3, 13]);
```
---

和python的推导式一样, 本库的for循环是从上到下读取的.
```rust
let vec = vector![
    (top,bottom)
    for top in 1..=3 if top != 2
    for bottom in 4..=6 if bottom+top != 4];
assert_eq!(vec, vec![(1, 4), (1, 5), (1, 6), (3, 4), (3, 5), (3, 6)]);
```

需要注意的是, 由于在rust中, for loop 是消耗所有权的.

所以通常来说, 对于多层循环, 如果你希望原容器被消耗, 你应该写成如下这样:
```rust
let vec_1 = vec!["ABC".to_string(), "DEF".to_string()];
let vec_2 = vec!["abc".to_string(), "def".to_string()];
let vec_3 = vec![123, 456];
let vec = {
    // 遮蔽想消耗的变量
    let vec_1 = vec_1;
    let vec_3 = vec_3;

    let mut vec = vec![];
    // 在外层循环里, 你可以选择使用iter()保留所有权
    for i in vec_1.iter() {
        if i == "ABC" {
            // 在内层循环里, 你必须使用iter() , 否则所有权会在第一次被转移
            for j in vec_2.iter() {
                if j == "abc" {
                    // 如果不使用iter(), 那么vec_3的所有权会在第一次被转移
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

你唯一需要做的就是在你想要保留所有权的变量后面加上.iter() 或 使用 & , 其余会在宏内自动处理.
```rust
let vec_1 = vec!["ABC".to_string(), "DEF".to_string()];
let vec_2 = vec!["abc".to_string(), "def".to_string()];
let vec_3 = vec![123, 456];

let vec = vector![
    (i.clone(),j.clone(),*k)
    for i in vec_1 if i == "ABC"
    for j in vec_2.iter() if j == "abc"
    // for j in &vec_2 if j == "abc" 这种写法也是可以的
    for k in vec_3 if k == &123
];
// println!("{:?}", vec_1); // borrow of moved value
println!("{:?}", vec_2); // work well
// println!("{:?}", vec_3); // borrow of moved value
```

同时, 该库还支持键值对容器类型, HashMap, BTreeMap

```rust
let vec_key = vec!["key_1".to_string(), "key_2".to_string(), "key_3".to_string()];
    let vec_value = [1, 2, 3];
    let hash_map = hash_map!{
        key.clone() : *value // 三种键值对分隔符都支持
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

该库也支持迭代器推导式, 但不同于上面的集合推导式

该迭代器推导式是基于引用的, 所以不会消耗所有权

除此之外的写法与集合推导式完全相同
```rust
let vec_1 = ["123".to_string(), "456".to_string(), "789".to_string()];
let vec_2 = ["ABC".to_string(), "DEF".to_string(), "GHI".to_string()];

let mut result3 = iterator_ref![
(x.clone(), y.clone()) if x.contains("1") else (y.clone(), x.clone())
for x in vec_1 if x.contains("1") || x.contains("7")
for i in 1..=2
for y in vec_2 if y.contains("D") || x.contains("3")];


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
let vec_1 = ["123".to_string(), "456".to_string(), "789".to_string()];
let vec_2 = ["ABC".to_string(), "DEF".to_string(), "GHI".to_string()];
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


# 一些细节

vector! :       使用push()添加元素

vec_deque! :    使用push_back()添加元素

linked_list! :  使用push_back()添加元素

hash_set! :     使用insert()添加元素

hash_map! :     使用insert()添加键值对

b_tree_map! :   使用insert()添加键值对

b_tree_set! :   使用insert()添加元素

binary_heap! :  使用push()添加元素
