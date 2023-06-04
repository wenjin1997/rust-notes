//! 示例3：重构
#![allow(unused)]
use crossbeam_channel::unbounded;
use crossbeam_channel::select;
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

struct WorkerState {
    ongoing: i16,
    exiting: bool,
}

impl WorkerState {
    fn init() -> Self {
        WorkerState { ongoing: 0, exiting: false }
    }

    fn set_ongoing(&mut self, count: i16) {
        self.ongoing += count;
    }

    fn set_exiting(&mut self, exit_state: bool) {
        self.exiting = exit_state;
    }

    fn is_exiting(&mut self) -> bool {
        self.exiting == true
    }

    fn is_nomore_work(&self) -> bool {
        self.ongoing == 0
    }
}

#[test]
fn refactor_thread_pool2_work() {
    let (work_sender, work_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();
    // 添加一个新的 Channel， Worker 使用它来通知"并行"组件已经完成了一个工作单元
    let (pool_result_sender, pool_result_receiver) = unbounded();
    let mut worker_state = WorkerState::init();
    // 使用线程池
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    let _ = thread::spawn(move || loop {
        // 使用 crossbeam 提供的 select! 宏选择一个就绪工作
        select! {
            recv(work_receiver) -> msg => {
                match msg {
                    Ok(WorkMsg::Work(num)) => {
                        let result_sender = result_sender.clone();
                        let pool_result_sender = pool_result_sender.clone();

                        // 注意，这里正在池上启动一个新的工作单元。
                        worker_state.set_ongoing(1);

                        pool.spawn(move || {
                            // 1. 发送结果给「主组件」
                            let _ = result_sender.send(ResultMsg::Result(num));

                            // 2. 让并行组件知道这里完成了一个工作单元
                            let _ = pool_result_sender.send(());
                        });
                    },
                    Ok(WorkMsg::Exit) => {
                        // N 注意，这里接收请求并退出
                        worker_state.set_exiting(true);

                        // 如果没有正在进行的工作则立即退出
                        if worker_state.is_nomore_work() {
                            let _ = result_sender.send(ResultMsg::Exited);
                            break;
                        }
                    },
                    _ => panic!("Error receiving a WorkMsg."),
                }
            },
            recv(pool_result_receiver) -> _ => {
                if worker_state.is_nomore_work() {
                    panic!("Received an unexpected pool result.");
                }

                // 注意，一个工作单元已经被完成了
                worker_state.set_ongoing(-1);

                // 如果没有正在进行的工作，并且接收到了退出请求，那么就退出
                if worker_state.is_nomore_work() && worker_state.is_exiting() {
                    let _ = result_sender.send(ResultMsg::Exited);
                    break;
                }
            },
        }
    });

    let _ = work_sender.send(WorkMsg::Work(0));
    let _ = work_sender.send(WorkMsg::Work(1));
    let _ = work_sender.send(WorkMsg::Exit);

    let mut counter = 0;

    loop {
        match result_receiver.recv() {
            Ok(ResultMsg::Result(_)) => {
                // 计数当前完成的工作单元
                counter += 1;
            }
            Ok(ResultMsg::Exited) => {
                // 断言检测：是在接收到两个请求以后退出的
                assert_eq!(2, counter);
                break;
            }
            _ => panic!("Error receiving a ResultMsg."),
        }
    }
}