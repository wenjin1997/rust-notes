use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::List::{Cons, Nil};

fn main() {
    fn test1() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }

    /// 示例15-27 创建没有子节点的 leaf 节点和以 leaf 作为子节点的 branch 节点
    fn ref_node_test() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),    // 1
            Rc::weak_count(&leaf)       // 0
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),  // 1
                Rc::weak_count(&branch)     // 1
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),    // 2
                Rc::weak_count(&leaf)       // 0
            );

        }
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),    // 1
            Rc::weak_count(&leaf)       // 0
        );
    }

    ref_node_test();

}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
