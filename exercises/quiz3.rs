// 导入Display trait，用于泛型约束
use std::fmt::Display;

// 将ReportCard改为泛型结构体，T代表grade的类型
pub struct ReportCard<T> {
    pub grade: T,          // 泛型类型T，可接收f32/String等实现Display的类型
    pub student_name: String,
    pub student_age: u8,
}

// 为泛型ReportCard实现方法，约束T必须实现Display（保证能格式化输出）
impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,                // T=f32
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+".to_string(),   // T=String，改为字母成绩"A+"
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}