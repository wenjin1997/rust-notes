//! 16. 无畏并发
#![allow(unused)]

use std::rc::Rc;
use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::time::Duration;
fn main() {
    mutex_ten_threads();
}

/// 示例16-2： 从`thread::spawn`保存一个`JoinHandle`以确保该线程能够运行至结束
fn thread_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

/// 示例16-1：创建一个打印某些内容的新线程，但是主线程打印其它内容
pub fn thread_spawn() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/// 示例16-3：尝试在另一个线程使用主线程创建的`vector`
pub fn thread_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

/// 示例16-7： 将 tx 移动到一个新建的线程中并发送"hi"
pub fn thread_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/// 示例16-9：在我们已经发送到通道中后，尝试使用`val`引用
fn thread_mpsc() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // 会发生错误，因为这里 val 所有权已经移动
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/// 示例16-10：发送多个消息，并在每次发送后暂停一段时间
fn thread_mpsc_send_many_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       let vals = vec![
           String::from("hi"),
           String::from("from"),
           String::from("the"),
           String::from("thread"),
       ];

        for val in  vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}

/// 示例16-11: 从多个生产者发送多个消息
fn thread_mpsc_multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
       let vals = vec![
           String::from("hi"),
           String::from("from"),
           String::from("the"),
           String::from("thread"),
       ];

        for val in  vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in  vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

/// 示例16-12：在一个单线程上下文中探索Mutex<T>的API
fn mutex_use() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    print!("m = {:?}", m);
}

/// 示例10-13：程序启动了10个线程，每个线程都通过`Mutex<T>`来增加计数器的值
fn mutex_ten_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in  0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    print!("Result: {}", *counter.lock().unwrap());
}