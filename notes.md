### 2.2.3 语句与表达式

```rust
// 使用两种方法让代码工作起来
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);
}
```

方法一：

```rust
// 使用两种方法让代码工作起来
fn main() {
   let v = {
       let mut x = 1;
       x += 2; // 变成一个语句
       x       // 返回
   };

   assert_eq!(v, 3);
}
```

方法二：

```rust
// 使用两种方法让代码工作起来
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, ()); // 返回的是()
}
```

### 2.2.4 函数

3.🌟🌟🌟

```rust
// 用两种方法求解
fn main() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    
}
```

解答：

方法一：

```rust
// 方法一
fn main() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
 		panic!("不返回任何值");  // 使用panic
    
}
```

方法二：使用loop

```rust
// 方法二
fn main() {
    never_return();
}

use std::thread;
use std::time;

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    loop {
        println!("不返回任何值");
        thread::sleep(time::Duration::from_secs(1));
    } 
}
```

4.🌟🌟 发散函数( Diverging function )不会返回任何值，因此它们可以用于替代需要返回任何值的地方

```rust

fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
    
}

```

方法一：

```rust
fn never_return_fn() -> ! {
    unimplemented!();
}
```

方法二：

```rust
fn never_return_fn() -> ! {
    panic!();
}
```

方法三：

```rust
fn never_return_fn() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    };
}
```

## 2.3 所有权和借用

### 2.3.1 所有权

1.🌟🌟

```rust
fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
```

方法一：

```rust
fn main() {
    // 使用尽可能多的方法来通过编译
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
```

方法二：

```rust

fn main() {
    // 使用尽可能多的方法来通过编译
    let x : &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
```

方法三：深拷贝

```rust

fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}
```

方法四：对于基本数据类型

```rust

fn main() {
    // 使用尽可能多的方法来通过编译
    let x = 10;
    let y = x;
    println!("{},{}",x,y);
}
```

3.

```rust

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.into_bytes();
    s
}
```

方法一：

```rust

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    s
}
```

方法二：**还需要弄明白`as_bytes()`**

```rust

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.as_bytes();
    s
}
```

7.🌟🌟🌟

```rust
fn main() {
    let x = Box::new(5);
    
    let ...      // 完成该行代码，不要修改其它行！
    
    *y = 4;
    
    assert_eq!(*x, 5);
}
```

解答：

```rust

fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);      // 完成该行代码，不要修改其它行！
    
    *y = 4;
    
    assert_eq!(*x, 5);
}
```

补充知识：在堆上分配一个值的最简单方式是`Box::new`。例如：

```rust
let t = (12, "eggs");
let b = Box::new(t); // 在堆上分配一个元组
```





### 2.3.2 引用和借用

6.🌟🌟🌟`ref` 与 `&` 类似，可以用来获取一个值的引用，但是它们的用法有所不同。

```rust
fn main() {
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let __ r2 = c;

    assert_eq!(*r1, *r2);
    
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1),get_addr(r2));
}

// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
```



9.🌟🌟 Ok: 从可变对象借用不可变

```rust
// 下面的代码没有任何错误
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");
}

fn borrow_object(s: &String) {}
```

## 2.4 复合类型

`unimplemented!()` 告诉编译器该函数尚未实现，`unimplemented!()` 标记通常意味着我们期望快速完成主要代码，回头再通过搜索这些标记来完成次要代码，类似的标记还有 `todo!()`，当代码执行到这种未实现的地方时，程序会直接报错。