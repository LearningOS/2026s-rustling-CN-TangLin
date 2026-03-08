use std::sync::{Arc, Mutex}; // 新增导入 Mutex
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 关键修改：用 Arc<Mutex<JobStatus>> 包裹共享数据
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO 1：锁定 Mutex 后修改共享数据
            // lock() 获取互斥锁（阻塞直到拿到锁），unwrap 处理锁中毒（poisoned）错误
            let mut status_lock = status_shared.lock().unwrap();
            status_lock.jobs_completed += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
        // TODO 2：锁定 Mutex 后读取并打印共享数据
        let status_lock = status.lock().unwrap();
        println!("jobs completed {}", status_lock.jobs_completed);
    }
}