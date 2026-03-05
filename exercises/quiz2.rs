pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 修复：移除引用，参数直接接收 Vec<(String, Command)>
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // 补全输出类型：Vec<String>
        let mut output: Vec<String> = vec![];
        for (string, command) in input { // 直接迭代，无需 iter()
            // 匹配命令并执行对应的字符串转换
            let result = match command {
                Command::Uppercase => string.to_uppercase(), // 转大写
                Command::Trim => string.trim().to_string(),   // 去除首尾空格
                Command::Append(n) => {
                    // 拼接 n 次 "bar"
                    let mut s = string; // 直接使用，无需克隆（已获取所有权）
                    s.push_str(&"bar".repeat(n));
                    s
                }
            };
            output.push(result);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 导入 my_module 中的 transformer 函数
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}