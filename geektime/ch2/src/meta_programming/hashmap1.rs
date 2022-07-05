macro_rules! hashmap {
    ($($key:expr => $value:expr), *) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}
#[test]
fn main() {
    let map = hashmap!{
        "a" => 1,
        "b" => 2
        // "c" => 3, // 会报错，不支持结尾有逗号
    };
    assert_eq!(map["a"], 1);
}