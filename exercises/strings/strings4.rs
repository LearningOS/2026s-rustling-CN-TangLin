// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // &str → 字符串字面量
    string("red".to_string()); // String → to_string() 生成String
    string(String::from("hi")); // String → String::from直接创建String
    string("rust is fun!".to_owned()); // String → to_owned()为&str创建所有权String
    string("nice weather".into()); // String → into()推导为String
    string(format!("Interpolation {}", "Station")); // String → format!宏返回String
    string_slice(&String::from("abc")[0..1]); // &str → 切片操作返回&str
    string_slice("  hello there ".trim()); // &str → trim()返回&str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // String → replace返回String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // String → to_lowercase返回String
}