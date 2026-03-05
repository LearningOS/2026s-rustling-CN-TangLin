// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.


mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    /// 制作香肠。
///
/// 此函数通过获取秘密配方并打印输出结果来制作香肠。
///
/// # Examples
///
/// ```
/// make_sausage();
/// ```
///
/// # Panics
///
/// 此函数不会 panic。
pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}