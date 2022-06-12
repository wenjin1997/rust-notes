/// # 13.2 迭代器学习
/// 见[13.2 使用迭代器处理元素序列](https://rustwiki.org/zh-CN/book/ch13-02-iterators.html)
// #[allow(unused)]
#[cfg(test)]
mod tests {
    /// 示例13-14：在一个for循环中使用迭代器
    #[test]
    fn iterator_for() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();    // 这里还未进行迭代

        for val in v1_iter {            // 这里进行迭代
            println!("Got: {}", val);
        }
    }

    /// # 示例13-15：在迭代器上（直接）调用next方法
    /// 迭代器都实现了`Iterator` trait，这个trait定义看起来像这样
    /// ```rust
    /// pub trait Iterator {
    ///     type Item;
    ///
    ///     fn next(&mut self) -> Option<Self::Item>;
    ///
    ///     // 此处省略了方法的默认实现
    /// }
    /// ```
    /// 注意每一个`next`调用都会从迭代器中消费一个项，所以`v1_iter`需要是可变的。
    /// 使用`for`循环时无需使`v1_iter`可变因为`for`循环会获取`v1_iter`的所有权
    /// 并在后台使`v1_iter`可变。
    ///
    /// - `iter`方法：生成一个不可变引用的迭代器
    /// - `into_iter`：获取所有权并返回拥有所有权的迭代器
    /// - `iter_mut`：迭代可变引用
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    /// # 示例13-16：调用sum方法获取迭代器所有项的总和
    /// 调用`next`方法的方法被称为**消费适配器**，因为调用他们会消耗迭代器。
    /// 例如：`sum`方法。
    ///
    /// 调用`sum`之后不再允许使用`v1_iter`因为调用`sum`时会获取迭代器的所有权。
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    // /// 示例13-17：调用迭代器适配器map来创建一个新迭代器
    // #[test]
    // fn iterator_map() {
    //     let v1: Vec<i32> = vec![1, 2, 3];
    //
    //     // map方法使用闭包来调用每个元素以生成新的迭代器
    //     v1.iter().map(|x| x + 1);
    // }

    /// 示例13-18：调用map方法创建一个新迭代器，接着调用collect方法消费新迭代器并创建一个vector
    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];

        // map方法使用闭包来调用每个元素以生成新的迭代器
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()       // 这里必须要获得所有权，后面才能构造Vec
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    /// 示例13-19：使用filter方法和一个捕获shoe_size的闭包
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {size: 10, style: String::from("sneaker")},
            Shoe {size: 13, style: String::from("sandal")},
            Shoe {size: 10, style: String::from("boot")},
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {size: 10, style: String::from("sneaker")},
                Shoe {size: 10, style: String::from("boot")},
            ]
        );
    }


    /// 示例13-20：定义Counter结构体和一个创建count初值为0的Counter实例的new函数
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter {count: 0}
        }
    }

    /// 示例13-21：在Counter结构体上实现Iterator trait
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    /// 示例13-22：测试next方法实现的功能
    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    /// 示例13-23：使用自定义的Counter迭代器的多种方法
    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

/// # 示例13-15：在迭代器上（直接）调用next方法
/// 迭代器都实现了`Iterator` trait，这个trait定义看起来像这样
/// ```rust
/// pub trait Iterator {
///     type Item;
///
///     fn next(&mut self) -> Option<Self::Item>;
///
///     // 此处省略了方法的默认实现
/// }
/// ```
/// 注意每一个`next`调用都会从迭代器中消费一个项，所以`v1_iter`需要是可变的。
/// 使用`for`循环时无需使`v1_iter`可变因为`for`循环会获取`v1_iter`的所有权
/// 并在后台使`v1_iter`可变。
///
/// - `iter`方法：生成一个不可变引用的迭代器
/// - `into_iter`：获取所有权并返回拥有所有权的迭代器
/// - `iter_mut`：迭代可变引用
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
