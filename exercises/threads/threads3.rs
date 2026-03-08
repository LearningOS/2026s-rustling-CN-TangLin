use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 关键修改：克隆 Sender 给不同线程，并返回线程句柄供主线程等待
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
    // 克隆 Sender（mpsc::Sender 支持 Clone，多生产者特性）
    let tx1 = tx.clone();
    let tx2 = tx;

    // 第一个线程发送前半部分数据
    let handle1 = thread::spawn(move || {
        for val in &q.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 第二个线程发送后半部分数据
    let handle2 = thread::spawn(move || {
        for val in &q.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    (handle1, handle2)
}

fn main() {
    // 创建 mpsc 通道：tx（发送端）、rx（接收端）
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 启动发送线程，获取线程句柄
    let (handle1, handle2) = send_tx(queue, tx);

    let mut total_received: u32 = 0;
    // 接收所有消息，直到拿到预期数量（避免无限阻塞）
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        // 拿到全部 10 个数字后退出循环
        if total_received == queue_length {
            break;
        }
    }

    // 等待发送线程完成，确保所有数据都已发送
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}