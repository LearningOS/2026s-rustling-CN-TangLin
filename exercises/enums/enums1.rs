// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // 定义枚举的所有变体，匹配 main 中使用的类型
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
