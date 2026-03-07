// 改为泛型结构体，T 约束为 Display（支持格式化输出）
pub struct ReportCard<T: std::fmt::Display> {
    pub grade: T, // 泛型类型 T，支持 f32 / String / &str 等
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
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
            grade: 2.1, // 数值型成绩（f32）
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
            grade: "A+", // 字母型成绩（&str），也可用 String::from("A+")
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
