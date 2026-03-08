// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    // 保持私有：仅模块内可访问（符合注释要求）
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 公开函数：允许模块外（如main）调用
    pub fn make_sausage() {
        get_secret_recipe(); // 模块内可访问私有函数
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage(); // 现在可以访问公开的函数
}