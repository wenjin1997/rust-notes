## 39｜借用检查：完全理解 Scope 和 NLL

* rust 编译过程：

  text -> tokens -> ast -> hir -> mir -> llvmir -> llvm -> 1101.. 二进制

* rust 借用检查不仅在词法作用域检查，同时在 mir 中会进行更加细致地检查。

* 在函数的内部，rust 也会对局部变量的引用进行检查。

## 40｜借用检查：深入理解生命周期和生命周期参数

```rust
'static: 'a
'long: 'short
// 长的生命周期是短的生命周期的子类
```

生命周期参数出现的意义是为了防止悬垂指针的出现。生命周期参数定义的时候看作一个泛型，在使用的时候才带进去进行实例化，具有 late bound。

## 41｜借用检查：深入理解生命周期参数 Early bound

**Early bound**: 类似于泛型，会先指定泛型类型。会进行静态分发，进行单态化。

```rust
#![allow(unused)]

fn g<'a: 'a>() {}
fn h<T>() { }

fn main() {
    let pg = g::<'static> as fn(); // early bound
    let f = h::<i32>(); // early bound
}
```

这里 `pg`和`f`都是 early bound。



**late bound**：在实际调用的地方会实例化一个生命周期参数，不能显式指定参数，否则会报错。

```rust
#![allow(unused)]
fn f<'a>() {}
fn main() {
    let pf = f::<'static> as fn(); // late bound，不能显式指定
  	// error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
}
```

输入的生命周期要 >= 输出的生命周期。

```rust
#![allow(unused)]
struct Buffer {
    buf: Vec<u8>,
    pos: usize,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            buf: vec![1,2,3, 4, 5,6],
            pos: 0,
        }
    }

    fn read_bytes<'a>(&'a mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos-3..self.pos]
    }
}

fn print(b1 :&[u8], b2: &[u8]) {
    println!("{:#?} {:#?}", b1, b2)
}

fn main() {
    let mut buf = Buffer::new();
    let b1 = buf.read_bytes(); // don't work 报错
    // let b1 = &(buf.read_bytes().to_owned());
    let b2 = buf.read_bytes();
    print(b1,b2)
}
```

修改上述代码：改为 early bound，用 `impl<'a>`。

```rust
#![allow(unused)]
fn main() {
    let v = vec![1,2,3, 4, 5,6];
    let mut buf = Buffer::new(&v);
    let b1 = buf.read_bytes();
    // let b1 = &buf.read_bytes().to_owned();
    let b2 = buf.read_bytes();
    print(b1,b2)
}

fn print(b1 :&[u8], b2: &[u8]) {
    println!("{:#?} {:#?}", b1, b2)
}

struct Buffer<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'b, 'a: 'b> Buffer<'a> { // 限定 'a 的生命周期比 'b 的长
    fn new(b: &'a [u8]) -> Buffer {
        Buffer {
            buf: b,
            pos: 0,
        }
    }

    fn read_bytes(&'b mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos-3..self.pos]
    }
}
```

## 42｜借用检查：深入理解生命周期参数 T vs &T



## 43｜借用检查：深入理解生命周期参数： trait 对象的生命周期参数



## 44｜借用检查：深入理解生命周期参数：高阶生命周期（上）



## 45｜借用检查：深入理解生命周期参数：高阶生命周期（中）

闭包和高阶生命周期：

```rust
fn main() {
    let f = |x: &i32| x; // error
    // 假如支持下面的语法就方便多了，目前还未支持
    // let f: for<'a> Fn(&'a i32) -> &'a i32 = |x| x;
    let i = &3;
    let j = f(i);
}
```

修正：

```rust
// fn annotate<'a, T: 'a ,F>(f: F) -> F where F: Fn(&'a T) -> &'a T { f }

fn annotate<T,F>(f: F) -> F where for<'a> F: Fn(&'a T) -> &'a T { f }

fn main() {
    let f = annotate(|x| x);
    let i = &3;
    let j = f(i);
    assert_eq!(*j, 3);
}
```



## 46｜借用检查：深入理解生命周期参数：高阶生命周期（下）



## 47｜线程与并发：理解线程与并发

```rust
#![allow(unused)]
use std::thread;

fn main() {
    // Duration 实现了 Copy、Send、Sync
    let duration = std::time::Duration::from_millis(500);

    println!("Main thread");

    let handle  = thread::spawn(move || {
        println!("Sub thread 1");

        // 注意：它的父线程是主线程，而不是线程1
        let handle2 = thread::spawn( move || {
            println!("Sub thread 2");
            thread::sleep(duration);
        });

        handle2.join().unwrap();
        thread::sleep(duration);
    });

    handle.join().unwrap();
    thread::sleep(duration);
}
```

* 注意这里要将 `duration` 变量 `move`到线程中，也就是要给线程所有权，因为程序无法保证子线程的生命周期短于主线程的生命周期，不会产生悬垂指针。

## 48｜线程与并发：线程间安全共享数据

```rust
fn main() {
    let mut v = vec![1,2,3];
    thread::spawn(move || {
        v.push(4);
    });
    // Can no longer access `v` here.
}
```

* 在这里 `v` 的所有权已经移动到子线程中，无法在所有权移动后使用。
* 这里的 `move` 不能去掉，为了能在线程中使用，因为无法保证子线程存活时间比主线程短，否则会发生使用已经释放的内存。

```rust
// invalid
use std::thread;

fn main() {
    let mut v = vec![1,2,3];
    for i in 0..10 {
        thread::spawn(move || {
            v.push(i);
        });
    }
}
```

* 这个程序会报错，原因在于创建第一个线程的时候，`v`的所有权已经移动了，后续的线程无法再使用`v`。

```rust
// invalid
fn inner_func(vref: &mut Vec<u32>) {
    std::thread::spawn(move || {
    	vref.push(3);
    });
}

fn main() {
    let mut v = vec![1,2,3];
    inner_func(&mut v);
}
```

* 上面这个程序会报错，原因在于函数中使用了可变引用，但是不知道它的生命周期，也不知道里面会创建多少个线程，编译器提示要加上`'static`，但是这里是没办法加的。

```rust
use std::fmt;
use std::time::Duration;
use std::thread;

struct Foo {
    string: String,
    v: Vec<f64>,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.string, self.v)
    }
}

fn test<T: Send + Sync + fmt::Display + 'static >(val: T) {
    thread::spawn(move || println!("{}", val));
}

fn main() {
    test("hello");                // &'static str
    test(String::from("hello"));  // String ，是 'static 的
    test(5);                      // i32

    // Arbitrary struct containing String and Vec<f64>
    test(Foo {string: String::from("hi"), v: vec![1.2, 2.3]});
    thread::sleep(Duration::new(1, 0));
}
```

* 为了让数据在线程中共享，要求`T: Send + Sync + fmt::Display + 'static`。

```rust
use crossbeam;
use std::{thread, time::Duration};

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    crossbeam::scope(|scope| {
        for e in &vec {
            scope.spawn(move |_| {
                println!("{:?}", e);
            });
        }
    })
    .expect("A child thread panicked");

    println!("{:?}", vec);
}
```

* 这里使用第三方库 crossbeam，使用 `crossbeam::scope`，其实原来标准库中也有[曾经的 thread::scoped 会泄漏 JoinGuard 所以被废弃](https://github.com/rust-lang/rust/issues/24292)，但是返回的 JoinGuard 会造成内存泄漏。
* 这里的 `crossbeam::scope` 保证了子线程的存活时间比主线程短，因此是线程安全的。

```rust
use crossbeam; // 0.6.0
use std::{thread, time::Duration};

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    crossbeam::scope(|scope| {
        for e in &mut vec {
            scope.spawn(move |_| {
                thread::sleep(Duration::from_secs(1));
                *e += 1;
            });
        }
    })
    .expect("A child thread panicked");

    println!("{:?}", vec);
}
```

* 这里用 `crossbeam::scope`来在线程中修改数据。

```rust
#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let v = Arc::new(Mutex::new(vec![1,2,3]));

    for i in 0..3 {
        let cloned_v = v.clone();
        thread::spawn(move || {
            cloned_v.lock().unwrap().push(i);
        });
    }
}
```

* 还可以使用标准库中的`Arc`和`Mutex`来在线程间共享数据。

## 56｜trait与泛型：trait静态分发

#### trait 四种作用

- 接口 (interface)
- 类型标记（Mark)
- 泛型限定（trait bound）
- 抽象类型（trait object）

#### 静态分发（单态化 - Monomorphized）

```rust
use std::string::ToString;

fn print<T: ToString>(v: T) {
    println!("{}", v.to_string());
}
```

或 `impl Trait`语法

```rust
use std::string::ToString;

#[inline(never)]
fn print(v: &impl ToString) {
    println!("{}", v.to_string());
}
```

* `#[inline(never)]`表示永远不内联函数。

使用 `impl Trait` 解决问题：

```rust
// error codes：

use std::fmt::Display;

fn main() {
    println!("{}", make_value(0));
    println!("{}", make_value(1));
}

fn make_value<T: Display>(index: usize) -> T {
    match index {
        0 => "Hello, World",
        1 => "Hello, World (1)",
        _ => panic!(),
    }
}
```

修正：

```rust
use std::fmt::Display;

fn make_value(index: usize) -> impl Display {
    match index {
        0 => "Hello, World",
        1 => "Hello, World (1)",
        _ => panic!(),
    }
}
```

* `impl Trait` 也是一种静态分发。

`impl Trait` 生命周期相关：

```rust
// error
fn make_debug<T>(_: T) -> impl std::fmt::Debug + 'static{
    42u8
}

// fn make_debug<'a, T: 'static>(_: &'a T) -> impl std::fmt::Debug + 'static{
//     42u8
// }

fn test() -> impl std::fmt::Debug {
    let value = "value".to_string();
    make_debug(&value)

```

* 这里注意传入参数 `T` 和 `&T` 的区别，early bound 和 late bound。

实际案例 - 模版模式：https://github.com/actix/actix-extras/tree/master/actix-web-httpauth

#### trait 一致性

trait 和 类型 必须有一个在本地。

## 57｜trait 与泛型：认识 trait 对象

## 动态分发

```rust
#![allow(unused)]

use core::any::{Any,TypeId};
use std::sync::Arc;

/// Class definition
struct Class {
    /// The name of the class
    name: String,
    /// The corresponding Rust type
    type_id: TypeId,
}

impl Class {
    /// Create a new class definition for the type `T`
    fn new<T: 'static>() -> Self {
        Self {
            name: std::any::type_name::<T>().to_string(),
            type_id: TypeId::of::<T>(),
        }
    }
}

/// An instance of a class
struct Instance {
    inner: Arc<dyn Any>, // `Arc` because we don't need/want mutability
}

impl Instance {
    /// Construct a new `Instance` from a type that
    /// implements `Any` (i.e. any sized type).
    fn new(obj: impl Any) -> Self {
        Self {
            inner: Arc::new(obj)
        }
    }
}

impl Instance {
    /// Check whether this is an instance of the provided class
    fn instance_of(&self, class: &Class) -> bool {
        // self.inner.type_id() == class.type_id
        self.inner.as_ref().type_id() == class.type_id
    }
}

struct Foo {}
struct Bar {}

fn main(){


    let foo_class: Class = Class::new::<Foo>();
    let bar_class: Class = Class::new::<Bar>();
    let foo_instance: Instance = Instance::new(Foo {});

    assert!(foo_instance.instance_of(&foo_class));
    assert!(!foo_instance.instance_of(&bar_class));
}
```

* `Any` 可以进行反射
* trait object 可以看作面向对象里面对象的概念，但是实现有所不同。

## 58｜trait 与泛型：泛型和 trait 实现模板方法

实际案例 - 模版模式：https://github.com/actix/actix-extras/tree/master/actix-web-httpauth

## 59｜trait 与泛型：trait 对象本质

* trait object 本质是一个虚表，和 C++ 中的虚表还是有一些区别。

![image-20220701131109727](/Users/jinjin/code/rust-notes/geektime/notes-geektime/img/image-20220701131109727.png)

* C++的虚表直接在 Cat Layout 中。

## 60｜trait 与泛型：对象安全本质

#### 对象安全

```
一个 trait 如果能实现自己，就认为它是对象安全的
```

为什么必须是对象安全呢？

trait对象，在运行时已经擦除了类型信息，要通过虚表调用相应的方法。不像静态分发那样，trait对象不是为每个类型都实现trait的方法，而是只实现一个副本（自动为其实现自身），结合虚函数去调用。

现在想一个问题： 假如那个类型没有实现这个方法怎么办？ 实际上，会有很多种情况下，会出现这个问题。运行时确定的类型和方法应该合法的，保证trait对象在运行时可以安全地调用相关的方法。

比如trait里有泛型函数。这就搞的很复杂了，可能运行时无法确定该调用哪个函数。反正是各种情况吧。所以，为了避免出现这种问题，官方引入了对象安全的概念。 实际上就是引入了一系列的规则，也就是上面列出的那些。编译器根据这些规则，在编译期判断一个你写的trait对象，是不是合法的。

比如：trait对象其实在内部维护两个表：safe_vtable和nonself_vtable，标记有where Self: Sized的会被归类到nonself_vtable，也就是说，不会被trait对象调用。 这样的话，方法标记有where Self: Sized的trait对象自然是安全的，因为这表示 这个方法 只能为 Self: Sized 都类型实现，是有条件的，所以在运行时有可能存在无效（万一有不是Sized的类型调用，就没有该方法）调用。

如果是合法的，则代表了，这个trait对象在运行时调用方法应该是没问题的。 不会出现没有实现，或者不知道该调用哪个的情况。 这就是对象安全的概念。它和内存安全并无直接关系。 所以，对象安全的本质就是为了让trait对象可以安全地调用相应的方法。

如果没有Sized的限定，那么就会很容易写出无用的类型。比如 Box，它用做trait对象即便会编译，但是不能用它做任何事情（后面有演示代码）。 对于更复杂的trait，往往就没有这么明显了，只有在做了大量繁重的工作之后可能会突然发现某个trait对象无法正常调用方法。 所以，为trait增加Sized限定，然后编译器自动为该trait实现自身，就可以在编译期准确排除无效的trait对象。 这就是对象安全。需要注意的是，对象安全和内存安全并无直接的关联，它只是保证trait对象在运行时可以安全准确地调用相关的方法。

## 61｜trait 与泛型：利用 Enum 代替 trait 对象

## 62｜trait 与泛型：trait 覆盖实现的一个解决方案

## 63｜trait 与泛型：trait 对象与 Sized

## 64｜trait 与泛型：trait 对象与 Box Self

## 65｜编程范式：Rust 语言编程范式讨论（上）

## 66｜编程范式：Rust 语言编程范式讨论（下）

```text
+----------------------------------------------------+
|                crate                               |
|                                                    |
|      +-----------------------------------+         |
|      |           std                     |         |
|      |                                   |         |
|      |       +---------------------+     |         |
|      |       |                     |     |         |
|      |       |     core            |     |         |
|      |       |    +----------+     |     |         |
|      |       |    | compiler |     |     |         |
|      |       |    +----------+     |     |         |
|      |       |                     |     |         |
|      |       +---------------------+     |         |
|      |                                   |         |
|      |                                   |         |
|      +-----------------------------------+         |
|                                                    |
|                                                    |
+----------------------------------------------------+
```

洋葱模型：

1. 最小内核所谓所有权和借用规则，就是编译器特性
2. 基于最小内核开始构造了 core
3. 基于core 构造了 std
4. 基于 std 构造生态 crate
5. 命令式编程为主（类 C），OOP 和 FP Style 辅助

## 67｜Rust 错误处理概要

## 68｜Rust错误处理：Option

## 69｜Rust错误处理：Result

* 错误处理第三方库：thiserror，配合anyhow

## 72｜Rust错误处理：Panic

- Unwinding（栈展开）。
- Aborting（中止）。

Unwinding 可以使应用程序线程以相对干净的方式关闭。 回收所有分配的系统资源，正确删除所有应用程序对象，依此类推。 此外，恐慌停止在有问题的线程的边界，而不是杀死整个应用程序过程。 所有这一切意味着，如果所有对象都具有明智的析构函数，则尽管有困难，但仍可以从紧急情况中恢复应用程序。

如果你应用程序是为此目的而设计的，则可以检测到线程紧急情况并重新启动有问题的线程，希望该操作能够正确恢复。 在无法关闭应用程序的情况下，例如在关键系统中，类似于Erlang的容错方法可能是有意义的。

对于Aborting，不存在应用程序恢复的可能性。一旦某些代码中止，应用程序进程将立即终止，这意味着要实现容错功能，就需要进行更加复杂的多进程设计。 另外，由于未运行资源析构函数，因此整个系统可能处于不一致状态，这意味着重新启动应用程序可能非常不容易。

总而言之，仅应在确实不关心应用程序立即崩溃并可能破坏在崩溃过程中操作的任何硬件/操作系统状态的情况下启用Aborting恐慌。

需要了解一个事实，Rust 目前对 OOM(out of memory)对处理是直接 Aborting ，无论你如何设置Panic类型。

- catch_unwind

```rust
use std::panic;
fn sum(a: i32, b: i32) -> i32{
    a + b
}
fn main() {
    let result = panic::catch_unwind(|| { println!("hello!"); });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| { panic!("oh no!"); });
    assert!(result.is_err());
   println!("{}", sum(1, 2));
}
```

Function [std](https://rustwiki.org/zh-CN/std/index.html)::[panic](https://rustwiki.org/zh-CN/std/panic/index.html)::[catch_unwind](https://rustwiki.org/zh-CN/std/panic/fn.catch_unwind.html#)

```rust
pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R>
```

调用一个闭包，如果发生，则捕获展开 panic 的原因。

如果闭包不是 panic，则此函数将返回 `Ok`，并返回闭包的结果; 如果闭包 panics，则此函数将返回 `Err(cause)`。 返回的 `cause` 是最初调用 panic 的对象。

unwind 当前从 Rust 代码转换为外来代码是未定义的行为，因此当从另一种语言 (通常为 C) 调用 Rust 时，此函数特别有用。 这可以运行任意的 Rust 代码，捕获 panic 并允许对错误进行适当的处理。

不建议将此函数用于一般的 try/catch 机制。[`Result`](https://rustwiki.org/zh-CN/std/thread/type.Result.html) 类型更适合用于经常失败的函数。 此外，不能保证此函数可以捕获所有 panics，请参见下面的 “Notes” 部分。

提供的闭包必须遵守 [`UnwindSafe`](https://rustwiki.org/zh-CN/std/panic/trait.UnwindSafe.html) trait，以确保所有捕获的变量都可以安全越过此边界。 此绑定的目的是在类型系统中对 [异常安全](https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md) 的概念进行编码。 此函数的大多数用法都不必担心此绑定，因为没有 `unsafe` 代码的程序自然是 unwind 安全的。 如果出现问题，可以使用 [`AssertUnwindSafe`](https://rustwiki.org/zh-CN/std/panic/struct.AssertUnwindSafe.html) 包装器结构体快速断言此处的使用确实是 unwind 安全的。

* 使用 set_hook

```rust
use std::panic;
fn sum(a: i32, b: i32) -> i32{
    a + b
}
fn main() {
    let result = panic::catch_unwind(|| { println!("hello!"); });
    assert!(result.is_ok());
    panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            println!("panic occurred '{}' at {}",
                location.file(), location.line()
            );
       } else {
            println!("can't get location information...");
       }
   }));
   let result = panic::catch_unwind(|| { panic!("oh no!"); });
   assert!(result.is_err());
   println!("{}", sum(1, 2));
}
```

输出的错误更友好。

运行结果为：

```bash
hello!
panic occurred 'src/main.rs' at 18
3
```

## 73｜Rust元编程之反射

