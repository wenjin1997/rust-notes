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

### 2.4.1 字符串与切片

2.🌟🌟 如果要使用 `str` 类型，只能配合 `Box`。 `&` 可以用来将 `Box<str>` 转换为 `&str` 类型

```rust
// 使用至少两种方法来修复错误
fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}
```

方法一：

```rust
fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(&s) // 这里用字符串切片引用
}

fn greetings(s: &str) {
    println!("{}",s)
}
```

方法二：🚩还需要多理解

```rust
fn main() {
    let s: Box<&str> =  "hello, world".into(); // 这里取到了字符串切片的引用
    greetings(*s) // 解引用
}

fn greetings(s: &str) {
    println!("{}",s)
}
```

4.

```rust
// 修复所有错误，并且不要新增代码行
fn main() {
    let  s =  String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s)
}
```

解答：

* 追加字符串用`s.push_str("somestr")`
* 使用 `+` 或者 `+=` 连接字符串，要求右边的参数必须为字符串的切片引用（Slice)类型。而`s += "!".to_string();`这里将`"!"`转换成了字符串类型，其实不用转换，`"!"`本身就是`&str`切片引用类型。

```rust
fn main() {
    let mut s =  String::from("hello");
    s.push(',');
    s.push_str(" world"); // 修改1
    s += "!"; 						// 修改2

    println!("{}", s)
}
```

10.🌟🌟🌟 有时候需要转义的字符很多，我们会希望使用更方便的方式来书写字符串: raw string.

```rust
/* 填空并修复所有错误 */
fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    let long_delimiter = __;
    assert_eq!(long_delimiter, "Hello, \"##\"")
}
```

解答：`long_delimiter`想要输出的是`Hello, \"##\"`，其中的`\`是转移字符标志，不是要输出`\`。

```rust
/* 填空并修复所有错误 */
fn main() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    let long_delimiter = r###"Hello, "##""###; // 注意！！！
    assert_eq!(long_delimiter, "Hello, \"##\"")
}
```

12.

```rust
fn main() {
    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".__ {
        println!("{}", c)
    }
}
```

解答：如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 `chars` 方法。

```rust

fn main() {
    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
```

切片相关题目

切片跟数组相似，但是切片的长度无法在编译期得知，因此你无法直接使用切片类型。

1. 🌟🌟 这里, `[i32]` 和 `str` 都是切片类型，但是直接使用它们会造成编译错误，如下代码所示。为了解决，你需要使用切片的引用： `&[i32]`, `&str`.

```rust
// 修复代码中的错误，不要新增代码行!
fn main() {
    let arr = [1, 2, 3];
    let s1: [i32] = arr[0..2];

    let s2: str = "hello, world" as str;
}
```

解答：

```rust
// 修复代码中的错误，不要新增代码行!
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;
}
```

2. 一个切片引用占用了2个字大小的内存空间( 从现在开始，为了简洁性考虑，如无特殊原因，**我们统一使用切片来特指切片引用** )。 该切片的第一个字是指向数据的指针，第二个字是切片的长度。字的大小取决于处理器架构，例如在 `x86-64` 上，字的大小是 64 位也就是 8 个字节，那么一个切片引用就是 16 个字节大小。

   切片( 引用 )可以用来借用数组的某个连续的部分，对应的签名是 `&[T]`，大家可以与数组的签名对比下 `[T; Length]`。

```rust
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
  
    // 修改数字 `6` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： 因为'中'和'国'是 UTF-8 字符，它们每个占用 3 个字节，2 个字符就是 6 个字节
    assert!(std::mem::size_of_val(&slice) == 16); // 注意：这里应该是16，表示一个切片引用
}
```

​		3.🌟🌟

```rust
fn main() {
   let arr: [i32; 5] = [1, 2, 3, 4, 5];
  // 填空让代码工作起来
  let slice: __ = __;
  assert_eq!(slice, &[2, 3, 4]);
}
```

解答：

```rust
fn main() {
   let arr: [i32; 5] = [1, 2, 3, 4, 5];
  // 填空让代码工作起来
  let slice: &[i32] = &arr[1..4];
  assert_eq!(slice, &[2, 3, 4]);
}
```

6. 🌟🌟 `&String` 可以被隐式地转换成 `&str` 类型.

   ```rust
   // 修复所有错误
   fn main() {
       let mut s = String::from("hello world");
   
       // 这里, &s 是 `&String` 类型，但是 `first_word` 函数需要的是 `&str` 类型。
       // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
       let word = first_word(&s);
   
       s.clear(); // error!
   
       println!("the first word is: {}", word);
   }
   fn first_word(s: &str) -> &str {
       &s[..1]
   }
   ```

   错误原因：当我们已经有了可变借用时，就无法再拥有不可变的借用。因为 `clear` 需要清空改变 `String`，因此它需要一个可变借用（利用 VSCode 可以看到该方法的声明是 `pub fn clear(&mut self)` ，参数是对自身的可变借用 ）；而之后的 `println!` 又使用了不可变借用，也就是在 `s.clear()` 处可变借用与不可变借用试图同时生效，因此编译无法通过。

   复习借用规则：

   总的来说，借用规则如下：

   - 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
   - 引用必须总是有效的

   解答：

   ```rust
   // 修复所有错误
   fn main() {
       let mut s = String::from("hello world");
   
       // 这里, &s 是 `&String` 类型，但是 `first_word` 函数需要的是 `&str` 类型。
       // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
       let word = first_word(&s);
       println!("the first word is: {}", word);
     	s.clear(); 
   }
   fn first_word(s: &str) -> &str {
       &s[..1]
   }
   ```

### 2.4.2 元组

3. 🌟 过长的元组无法被打印输出

```rust
// 修复代码错误
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
}
```

解答：参考官方答案，元组长度为13时无法打印，改为12时可以打印，也就是说最长可打印输出长度为12的元组。

```rust
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}
```

### 2.4.3 结构体

1. 初始化实例时，**每个字段**都需要进行初始化
2. 初始化时的字段顺序**不需要**和结构体定义时的顺序一致

