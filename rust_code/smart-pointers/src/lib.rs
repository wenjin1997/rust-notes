//! 15.智能指针
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;
    use crate::{CustomSmartPointer, hello, MyBox};
    use crate::List::{Cons, Nil};
    use super::*;

    /// 15-1 使用box在堆上存储一个i32值
    #[test]
    fn use_box() {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    /// 15-6 使用解引用运算符来解出i32的值
    #[test]
    fn dereference() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    /// 15-7 在Box<i32>上使用解引用运算符
    #[test]
    fn box_deference() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn my_box_deference() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }


    #[test]
    fn my_box_string_hello() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }

    #[test]
    fn drop_custom_smart_pointer() {
        let c = CustomSmartPointer {data: String::from("my stuff") };
        println!("CustomSmartPointers created.");
        drop(c);
        let d = CustomSmartPointer {data: String::from("other stuff") };
        println!("CustomSmartPointers dropped before the end of main.")
    }

    // /// 示例15-18 使用Rc<T>定义的List
    // #[test]
    // fn rc_list() {
    //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //     let b = Cons(3, Rc::clone(&a));
    //     let c = Cons(4, Rc::clone(&a));
    // }

    // /// 示例15-19 打印出引用计数
    // #[test]
    // fn rc_list_reference_counting() {
    //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //     println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    //     let b = Cons(3, Rc::clone(&a));
    //     println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    //     {
    //         let c = Cons(4, Rc::clone(&a));
    //         println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    //     }
    //     println!("count after creating c goes out of scope = {}", Rc::strong_count(&a)); // 2
    // }


    /// 示例15-21 尝试实现MockMessenger，借用检查器不允许这么做
    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{ sent_messages: RefCell::new(vec![])}
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }


    #[test]
    fn rc_refcell_list() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }


}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// 示例15-11 hello函数有着&str类型的参数name
fn hello(name: &str) {
    println!("Hello, {}", name);
}

/// 示例15-14 结构体CustomSmartPointer，其实现了放置清理代码的Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }


/// 示例15-20 一个记录某个值与最大值差距的库，并根据此值的特定级别发出警告
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75{
            self.messenger.send("Warning: You've used up over 75% of your quota!")
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}