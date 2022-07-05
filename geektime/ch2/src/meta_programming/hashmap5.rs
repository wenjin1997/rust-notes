macro_rules! hashmap {
    (@unit $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@unit $rest)),*]));
    ($($key:expr => $value:expr),* $(,)*) => {
        {
            let _cap = hashmap!(@count $($key),*);
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