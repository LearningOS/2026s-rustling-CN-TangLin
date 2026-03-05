// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 关键：用 Mutex 包裹可变数据，Arc 实现多线程共享
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 加锁获取可变引用，修改共享数据
            // lock() 返回 Result，unwrap() 处理锁中毒（如其他线程恐慌）
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 先等待所有线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }

    // 最后打印最终结果（而非循环中打印）
    // 循环中打印会看到中间值，最终打印才能得到正确的 10
    let final_status = status.lock().unwrap();
    println!("jobs completed {}", final_status.jobs_completed);
}