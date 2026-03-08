// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // 定义枚举变体（单元变体，无关联数据），与main中调用的名称完全一致
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