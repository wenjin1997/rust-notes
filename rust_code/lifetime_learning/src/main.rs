// 10.3 生命周期与引用有效性
#![allow(unused)]
use std::fmt::Display;
fn main() {
    // 示例10-20: main 函数调用 largest 函数来寻找两个字符串 slice 中较长的一个
    fn test1() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);

        
    }

    // 示例10-23:通过拥有不同的具体什么周期的String值调用longest函数
    fn test2() {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }

    // 示例10-25:一个存放引用的结构体，所以其定义需要生命周期标注
    fn test3() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmal. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }

    // 结合泛型类型参数、trait bounds和生命周期
    fn test4() {
        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
            where T: Display
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
    test4();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
