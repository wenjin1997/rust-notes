//! 代码清单 12-9: 非静态生命周期类型没有实现 Any

use std::any::Any;
struct Unstatic<'a> { x: &'a i32 }
#[test]
fn main() {
    let a = 42;
    let v = Unstatic { x: &a };
    let mut any: &dyn Any;
    // any = &v; // 编译错误
}