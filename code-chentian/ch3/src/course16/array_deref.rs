use std::fmt;
use std::iter::Iterator;

#[test]
fn main() {
    let v = vec![1, 2, 3, 4];

    // Vec 实现了 Deref，&Vec<T> 会被自动解引用为 &[T]，符合接口定义
    print_slice(&v);
    // 直接是 &[T]，符合接口定义
    print_slice(&v[..]);

    // &Vec<T> 支持 AsRef<T>
    print_slice1(&v);
    // &[T] 支持 AsRef<T>
    print_slice1(&v[..]);
    // Vec<T> 也支持 AsRef<T>
    print_slice1(v);

    let arr = [1, 2, 3, 4];
    // 数组虽然没有实现 Deref，但它的解引用就是 &[T]
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);
}
fn print_slice<T: fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}
fn print_slice1<T, U>(s: T)
    where
        T: AsRef<[U]>,
        U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

#[test]
fn test_iter() {
    // 这里 Vec<T> 在调用 iter() 时被解引用成 &[T]，所以可以访问 iter()
    let result = vec![1, 2, 3, 4]
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 16)
        .take(1)
        .collect::<Vec<_>>();
    println!("{:?}", result);
}
