##  Attributes

* `#[allow(non_camel_case_types)]` 可以不使用驼峰命名法。
* `#[cfg]` 条件编译
* `#[inline]` 内联函数。当出现一个函数或者一个方法定义在一个 crate 中，而我们在另一个 crate 中去调用，可以显式使用这个属性。
  * `#[inline(always)]` 要求函数在每一次调用的点都展开内联
  * `#[inline(never)]` 要求一个函数从不进行内联
* `#[cfg]`和`#[allow]`可以用在一整个模块或者里面的任何东西，但是 `#[inline]` 和 `#[test]`只能用在单个项上面。
* 想在整个 crate 上面绑定属性，在main.rs`或者`lib.rs`文件顶部中用`#!。`#!`也可以用在函数，结构体中，但是它只能一贯地用在文件的开头，来绑定给整个模块或者 crate 绑定一个属性。而有的属性总是用 `#!`，因为它只能作用在整个 crate 上，比如 `#![feature]`。
* `#![feature]` 用来表示 Rust 语言不稳定的 features，这些是实验性质的。当后面这个 feature 稳定之后，编译器就会警告，建议移除 `#![feature]`。

## Tests and Documention

* `#[test]` 标记函数，表明这个是测试函数，用`cargo test` 可以测试所有测试函数，如果只想测试某一个，可以用`cargo test name`来测试具体的函数。
* `assert!(expr)`：如果表达式为真就通过测试，否则 panic。
* `assert_eq!(v1, v2)`：判别 v1 和 v2 是否相等。
* 如果只想在 debug 模式下检验是否相等，可以用 `debug_assert!` 和 `debug_assert_eq!`。
* `#[should_panic]` 表示会 panic。
* 当运行 `cargo test` ，cargo 会编译两次代码。
* `#[cfg(test)]`标记整个模块。
* 一般 cargo test 会启用多线程来一次运行多个测试，用 `cargo test name` 和 `cargo test -- --test-threads 1`来限制只用一个线程测试。
* `cargo test -- --no-capture`：也输出那些通过的测试。

### Integration Tests

* 集成测试，可以用来测试一些公共的API，站在用户的角度。
* 单独建立一个 `tests`的文件夹，放在和 `src` 同一个路径下，当运行 `cargo test` 的时候，集成测试和单元测试都会运行。如果只想运行集成测试，可以用命令`cargo test --test unfurl`来运行一个具体的集成测试。

### Documentation

```rust
cargo doc --no-deps --open
```

* --no-deps ：只生成自己的文档，不生成所有它依赖的 crates。
* --open：在浏览器中打开文档。
* 生成的文档放在 target/doc 目录下。
* `///`的文档注释类似`#[doc]`属性。而`//!`和`#![doc]`一样。
* 文档注释中markdown 的链接也可以链接到相应代码的文档。

### Doc-Tests

* Rust 也会测试在文档注释中的代码，会隐身的加在 `fn main()` 函数里面。

