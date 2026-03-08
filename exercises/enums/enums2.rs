// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    // 1. 结构体风格变体：Move，包含x和y两个i32字段
    Move { x: i32, y: i32 },
    // 2. 元组风格变体：Echo，包含一个String类型参数
    Echo(String),
    // 3. 元组风格变体：ChangeColor，包含三个i32类型参数（RGB值）
    ChangeColor(i32, i32, i32),
    // 4. 单元变体：Quit，无关联数据
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}