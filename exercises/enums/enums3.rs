// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

// 定义Message枚举变体，匹配测试用例中使用的类型
enum Message {
    Quit,                          // 无关联数据：退出
    Echo(String),                  // 携带字符串：回声
    Move(Point),                   // 携带Point结构体：移动位置
    ChangeColor(u8, u8, u8),       // 携带三个u8：RGB颜色值
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) { self.message = s; }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // 用match表达式匹配不同的Message变体，调用对应方法
        match message {
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)), // 解包RGB值，转成元组传入
            Message::Echo(s) => self.echo(s),                             // 解包字符串，传入echo方法
            Message::Move(p) => self.move_position(p),                     // 解包Point，传入move_position方法
            Message::Quit => self.quit(),                                 // 无关联数据，直接调用quit方法
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}