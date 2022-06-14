#![allow(unused)]
fn main() {
    destruct_nested_enum();
}

/// 示例18-1
fn patterns_use() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

/// 示例18-2：使用`while let`循环只要`stack.pop()`返回`Some`就打印值
pub fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

/// 示例18-3：在`for`循环中使用模式来结构元组
pub fn use_for() {
    let v = vec!['a', 'b', 'c'];

    // enumerate方法适配一个迭代器来产生一个值和其在迭代器中的索引，位于一个元组中
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

/// 示例18-7:一个参数在结构元组的函数
pub fn function_pattern() {
    fn print_coordinate(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinate(&point);
}

/// 示例18-11 一个`match`语句其中一个分支引入了覆盖变量`y`
pub fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 5
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y); // x = Some(5), y = 10
}

/// 示例18-12：解构一个结构体的字段为单独的变量
pub fn struct_point() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7};
    let Point{ x: a, y: b} = p;
    assert_eq!(a, 0);
    assert_eq!(b, 7);
}

/// 示例18-13：使用结构体字段简写来解构结构体字段
pub fn struct_point_simple() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7};
    let Point {x, y} = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);
}

/// 示例18-14：解构和匹配模式中的字面量
pub fn struct_point_pattern() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7};
    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point{x, y} => println!("On neither axis: ({}, {})", x, y),
    }
}

/// 示例18-15：解构包含不同类型值成员的枚举
pub fn destruct_enum() {
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move {x, y} => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to read {}, green {}, and blue {}",
                r,
                g,
                b
            );
        }
    }
}

/// 示例18-16：匹配嵌套的枚举
pub fn destruct_nested_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
}

/// 示例18-29：使用@在模式中绑定值的同时测试它
pub fn bounding() {
    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello { id: 5};

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found an id in range: {}", id)
        },
    }
}
