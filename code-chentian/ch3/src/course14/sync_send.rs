use std:: {
    sync::{Arc, Mutex},
    thread,
};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

// Arc<Mutex<T>> 可以多线程共享且修改数据
fn arc_mutex_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }

    handle.join().unwrap();
    println!("a = {:?}", a);
}

#[test]
fn test() {
    arc_mutex_is_send_sync();
}

// #[test]
// fn rc_is_not_send_and_sync() {
//     let a = Rc::new(1);
//     let b = a.clone();
//     let c = a.clone();
//     thread::spawn(move || {    // Error `Rc<i32>` cannot be sent between threads safely
//         println!("c = {:?} ", c);
//     });
// }

#[test]
fn refcell_is_send() {
    let a = RefCell::new(1);
    thread::spawn(move || {
        println!("a = {:?}", a);
    });
}

// #[test]
// fn refcell_is_not_sync() {
//     let a = Arc::new(RefCell::new(1));
//     let b = a.clone();
//     let c = a.clone();
//     thread::spawn(move || { // error  `RefCell<i32>` cannot be shared between threads safely
//         println!("c = {:?}", c);
//     });
// }
