//! 13. 类型系统
//! 字符串解析器，可以把字符串的某部分解析称某个类型
use regex::Regex; // 使用正则表达式
pub trait Parse {
    fn parse(s: &str) -> Self; // 第一个参数和 self 无关，是 trait 的静态方法
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            // 取第一个 match，将其捕获的 digits 换成 u8
            captures
                .get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("1234abcd"), 0); // u8 的范围是 0-255，所以超过这个范围应该是 0
    assert_eq!(u8::parse("abcd"), 0);
}

#[test]
fn test() {
    println!("result: {}", u8::parse("255 hello world"));
}


