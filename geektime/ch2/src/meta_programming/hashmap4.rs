macro_rules! unit {
    ($($x:tt)*) => (());
}
macro_rules! count {
    ($($key:expr),*) => (<[()]>::len(&[$(unit!($key)),*]));
}
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            let _cap = count!($($key),*);
            let mut _map 
                = ::std::collections::HashMap::with_capacity(_cap);
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
        "b" => 2,
        "c" => 3,
    };
    assert_eq!(map["a"], 1);
}