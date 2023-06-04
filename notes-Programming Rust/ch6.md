# Chapter 6. Expressions

Rust中绝大多数都是表达式。

## An Expression Language

* 在Rust中，if 和 match 都可以产生值。一个match表达式可以给一个函数或者宏来传参。
* Rust 中没有三元操作符。
* Rust中所有的控制流程工具是表达式。

## Precedence and Associativity

* All of the operators that can usually be chained are left-associative. 通常是左结合。
* 比较操作符，assignment iperators 以及 range operators 不能链式链接。

## Blocks and Semicolons

* block 的值是最后一个表达式的值。
* let 声明必须要有分号。
* 如果 if 表达式没有 else，那么总是返回 ()。

## Declarations

* let 声明必须要有分号。
* 在没有初始化一个变量之前是不能使用它的。
* 变量遮蔽
* 一个块可以包含 item declarations，比如 fn，struct 或者 use。
* 嵌套的 fn 不能使用 scope 中的局部变量。

## if and match

* **if 表达式的 condition 必须是 bool 类型的**，Rust 不会将数值或者指针隐式转换成 Boolean 类型。
* 一个只有 if 没有 else 的表达式相当于有一个 else 块，其为空。因此如果 if 表达式没有 else，那么总是返回 ()。
* 编译器可以用一个 jump table 来优化 match。
* Rust 禁止 match 表达式没有覆盖到所有情况。
* 一个 if 表达式的所有块必须产生相同类型的值，对于 match 表达式也是一样的。

## if let

```rust
if let pattern = expr {
  block1
} else {
  block2
}
```

等价于

```rust
match expr {
  pattern => { block1 }
  _ => { block2 }
}
```

## Loops

有四种 looping 表达式：

```rust
while condition {
  block
}

while let pattern = expr {
  block
}

loop {
  block
}

for pattern in iterable {
  block
}
```

* while 和 for  的值一直都是 ()。
* 用 loop 可以无限循环，除非遇到 break，return或者线程 panics。
* `for` 循环一些值会对其进行消耗，有时候可以使用引用。

## Control Flow in Loops

* break 只能在 loop 中，不能在 match 中。
* 可以给 break 一个表达式，这个表达式的值就是 loop 的值。
* 自然地，所有 break 表达式在一个循环中必须是相同类型的，并且这个类型就是 loop 的类型。
* 循环可以加上生命周期的标签，然后可以 break 这个标签，break 也可以同时带上值表达式。
* 标签同样可以用于 continue。

## return Expressions

* return 表达式会退出当前函数，并且给调用者返回一个值。
* 可以将函数体看成一个块表达式。
* return 可以放弃正在运行中的工作。

## Why Rust Has loop

Rust编译器分析你的程序的控制流程：

* 检查返回类型
* 检查局部变量不会使用未初始化的值
* 警告不可到达的代码

`!` 表示是一个发散函数。

## Function and Method Calls

* `.location()` 方法可能取 player 的值或者引用。
* `.` 操作符可以自动的解引用 `player`或者借一个引用。
* 类型的方法`Vec::new()` ，值的方法：`my_vec.len()`。
* 对于泛型，要用比目鱼 turbofish 操作符，比如：`return Vec::<i32>::with_capcity(1000);`

## Fields and Elements

点操作符或者方括号的左侧是一个引用或者智能指针，那么它可以被自动地解引用，这样的表达式被称为 `lvalues`，它们可以出现在一个 assignment 的左侧。

操作符：

```rust
..
a ..
.. b
a .. b
..= b
a ..= b
```

## Reference Operators

* &
* & mut
* `*`操作符可以用来解引用

## Arithmetic, Bitwise, Comparsion, and Logical Operators

* 不能将`-`和`+` 用到无符号的数值类型上。
* `%`可以用在浮点数上。
* 位操作符的优先级高于比较操作符的优先级。

## Assignment

* Rust中可以有复合操作符：`+=` `-=` `*=`
* Rust**不支持**链式 assignment，比如 `a = b = 3`
* Rust**没有**自增和自减操作符：`++`和`--`。

## Type Casts

可以用 `as` 进行类型转换。几种类型转换是允许的：

* 数值之间可以转换。
*  bool、char 或者 C-like enum 类型可能转换为数值。u8 可能转换为 char 类型。
* 有些含有 unsafe 的指针类型可能可以转换。

一些自动转换，被称为 deref coercions：

* `&String` -> `&str`
* `&Vec<i32>` -> `&[i32]`
* `&Box[Chessboard]` -> `&Chessboard`

## Closures

```rust
let is_even = |x| x % 2 == 0;
```

Rust自动推断类型。调用闭包就像调用一个函数那样：

```rust
assert_eq!(is_even(14), true);
```

## Onward

表达式只是我们思考 running code。