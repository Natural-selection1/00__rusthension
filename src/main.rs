use rusthension::rusthension;

fn main() {
    {
        println!("测试返回元组_1迭代器_有条件");
        let vec = vec![("a", 1), ("b", 2), ("c", 3)];
        let result = rusthension![(x, y) for (x, y) in vec if y >= 2];
        assert_eq!(result, [("b", 2), ("c", 3)]);
    }

    {
        println!("测试返回String_1迭代器_有条件");
        let vec = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("c".to_string(), 3),
        ];
        let result = rusthension![(x, y) for (x, y) in vec if y >= 2];

        assert_eq!(result, [("b".to_string(), 2), ("c".to_string(), 3)]);
    }
    {
        let result = rusthension![
            [x, y] if x > y else [y, x]
            for x in 0..10 if x % 2 == 0
            for y in 0..x if y % 3 == 0
        ];
    }
}
