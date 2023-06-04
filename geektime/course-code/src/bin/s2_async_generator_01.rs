/// 生成器哦
///
/// Future 底层实现依赖于生成器。`async/await` 对应底层生成器 `resume/yield`。
#[feature(generators, generator_trait)]
use std::ops::Generator;
use std::pin::Pin;
use std::future;
fn main() {
    let mut gen = ||{
        yield 1;
        yield 2;
        yield 3;
        return 4;
    };

    for _ in 0..4 {
        // 为了给嵌入式支持异步，多传入了一个空的 unit 给 resume 方法
        let c = Pin::new(&mut gen).resume(());
        println!("{:?}", c);
    }
}