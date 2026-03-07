// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    // 定义带不同数据类型的枚举变体，匹配 main 中的使用方式
    Quit,                    // 无数据变体
    Echo(String),            // 携带 String 类型的变体
    Move { x: i32, y: i32 }, // 携带匿名结构体的变体
    ChangeColor(u8, u8, u8), // 携带三个 u8 类型的元组变体
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
