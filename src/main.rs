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
        let vec_1 = vec![("a", 1), ("b", 2), ("c", 3)];
        let vec_2 = vec![("a", 1), ("b", 2), ("c", 3)];
        let result = rusthension![
            y if x > y else y
            for x in vec_1
            for y in &vec_2
        ];

        println!("{:?}", vec_1);
        println!("{:?}", vec_2);
        println!("{:?}", result);
    }
}
