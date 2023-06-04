# Chapter 7. Error Handling

## Panic

panic 是程序本身出现一个bug。比如：

* 数组越界
* 整数除零
* 对于一个 `Result`调用`.expect()`。
* 断言失败

### Unwinding

panic发生：

* 终端打印错误信息
* 运行 `RUST_BACKTRACE=1`会将栈展开。资源会关闭，会调用drop方法。
* 最后，线程退出。

panic像Java中的运行时异常，它是可定义的，只是不应该发生。panic是安全的，panic是单个线程的。

### Aborting

* 如果`.drop()`方法触发了第二个panic当Rust在第一个panic后尝试 clean up，这会被当作是致命的。Rust会停止展开并且直接终止整个流程。
* 编译加上`-C panic=abort`，第一个 panic 就立马终止程序。

## Result

如果不使用Result值，会得到一个警告。

### Catching Errors

用 match 来处理 error

```rust
match get_weather(hometown) {
  Ok(report) => {
    display_weather(hometown, &report);
  }
  Err(err) => {
    println!("error querying the weather: {}", err);
    schedule_weather_retry();
  }
}
```

但是上面这种方式太冗长了。这里有一些方法：

```rust
// 1. 返回一个bool值
result.is_ok()
result.is_err()
// 2. 返回成功的值，返回 Option<T>, 成功就返回值，失败就返回 None
result.ok()
// 3. 返回失败的值
result.err()
// 4. 返回成功的值，否则就返回 fallback，抛弃错误的值
result.unwrap_or(fallback)
// 5. 返回成功的值，否则返回 fallback_fn，这里可以是一个函数或者闭包
result.unwrap_or_else(fallback_fn)
// 6. 返回成功的值，否则就 panic
result.unwrap()
// 7. 将 Result<T, E> 转换为 Result<&T, &E>，这对于还要用原来的 result 的值很有用
result.as_ref()
// 8. 将 Result<T, E> 转换为 Result<&mut T, &mut E>
result.as_mut()
```

### Result Type Aliases

在标准库 std::io 模块中有：

```rust
pub type Result<T> = result::Result<T, Error>;
```

这里用了 type aliases。

### Printing Errors

* `println!()`

  打印错误，用`{}`可以打印简短的错误信息，用`{:?}`的Debug模式打印出错误。

* `err.to_string()`

  返回错误信息字符串。

* `err.source()`

  返回一个Option，包含潜在的错误。

### Propagating Errors

如果错误发生，我们希望调用者来处理。

```rust
let weather = get_weather(hometown)?;
let weather = try!(get_weather(hometown));
```

`?`可以类似地用在`Option`类型的值上面。

### Working with Multiple Error Types

* `?`不能将`std::num::ParseIntError`转换为`std::io::Error`类型。
* 所有标准库中的错误都可以转成`Box<dyn std::error::Error + Send + Sync + 'static'>`类型。

```rust
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult = Result<T, GenericError>;
```

`?`会调用`GenericError::from()`方法来进行自动转换。

* 如果想要其中一种特别的错误类型进行处理，其它的错误进行传播的话，可以用泛型方法`error.downcast_ref::<ErrorType>()`

### Dealing with Errors That "Can't Happen"

* 当我们不想处理不会发生的错误时，用`.unwrap()`方法。
* 也可以用`.expect(message)`方法。

### Ingoring Errors

```rust
let _ = writeln!(stderr(), "error: {}", err);
```

用 `let _ = ...`来忽略警告。

### Handling Errors in main()

* 一般情况下在 main 函数中不能用 `?` ，因为 `main()` 函数的返回类型不是 `Result`。
* 最简单的方法是用 `.expect()`方法。发生错误时会 panic 并且返回一个非零的退出代码。
* 或者可以返回一个 `Result` 类型

```rust
fn main() -> Result<(), TideCalcError> { }
```

* 或者自己处理相应的错误。

### Declaring a Custom Error Type

* Errors 应该实现 `std::error::Error` trait。
* 或者直接用`thiserror` 中的。

```rust
use thiserror::Error;
#[derive(Error, Debug)]
#[error("{message:} ({line:}, {column})")]
pub struct JsonError {
  message: String,
  line: usize,
  column: usize,
}
```

### Why Results?

* 在代码中记录错误，对错误进行分类。
* 允许错误传播。
* 每一个函数都有返回类型，这样更清楚函数可不可能失败。
* Rust 会检查 Result 类型的值是否被使用。
* 因为 Result 是一个类型，因此更容易处理一系列成功或者失败的值，也更容易存储。