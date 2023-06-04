//! 示例二：引入线程池，工作的顺序将无法确定
#![allow(unused)]
use crossbeam_channel::unbounded;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::{Condvar, Mutex};
use std::thread;

// 此消息用于发送到与 「主组件」 并行运行的其他组件。
enum WorkMsg {
    Work(u8),
    Exit,
}

// 此消息用于从并行运行的其他组件 发送回「主组件」。
enum ResultMsg {
    Result(u8),
    Exited,
}

#[test]
fn thread_work() {
    let (work_sender, work_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();

    // 引入线程池，开两个工作线程
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    // 生成子线程用于执行另一个并行组件
    let _ = thread::spawn(move || loop {
        // 接收并处理消息，直到收到 exit 消息
        match work_receiver.recv() {
            Ok(WorkMsg::Work(num)) => {
                let result_sender = result_sender.clone();
                // 使用线程池中的线程
                pool.spawn(move || {
                    // 执行一些工作，并且发送消息给 Result 队列
                    let _ = result_sender.send(ResultMsg::Result(num));
                });
            }
            Ok(WorkMsg::Exit) => {
                // 发送 exit 确认消息
                let _ = result_sender.send(ResultMsg::Exited);
                break;
            }
            _ => panic!("Error receiving a WorkMsg."),
        }
    });

    let _ = work_sender.send(WorkMsg::Work(0));
    let _ = work_sender.send(WorkMsg::Work(1));
    let _ = work_sender.send(WorkMsg::Exit);

    loop {
        match result_receiver.recv() {
            Ok(ResultMsg::Result(num)) => {
                // 不能再断言顺序了
            }
            Ok(ResultMsg::Exited) => {
                // 也不能再断言在退出消息之前已经收到了结果
                break;
            }
            _ => panic!("Error receiving a ResultMsg."),
        };
    }
}