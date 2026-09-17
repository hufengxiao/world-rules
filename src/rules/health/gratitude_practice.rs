//! 感恩练习
//!
//! 培养感恩心态、发现美好与提升幸福感

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GratitudePracticeRules,
    name: "感恩练习",
    desc: "培养感恩心态、发现美好与提升幸福感",
    origin: "心理学",
    tags: ["健康", "感恩", "心态", "幸福感"]
}

impl GratitudePracticeRules {
    /// 记录感恩
    pub fn journal(&self) -> Vec<&'static str> {
        vec![
            "每日记下值得感恩的事",
            "感谢具体的人与事",
            "不流于表面罗列",
            "细品当下的小确幸",
        ]
    }

    /// 表达感激
    pub fn express(&self) -> Vec<&'static str> {
        vec![
            "及时向他人道谢",
            "真诚表达欣赏",
            "不吝惜夸奖",
            "感谢身边支持",
        ]
    }

    /// 视角转换
    pub fn reframe(&self) -> Vec<&'static str> {
        vec![
            "多看拥有的部分",
            "从困境找成长",
            "不与他人一味攀比",
            "珍惜眼前与当下",
        ]
    }

    /// 养成习惯
    pub fn habit(&self) -> Vec<&'static str> {
        vec![
            "固定时间练习",
            "融入生活点滴",
            "成效不需立见",
            "坚持自会沉淀",
        ]
    }
}

impl Rule for GratitudePracticeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("gratitude")
    }

    fn explain(&self) -> String {
        format!(
            "【感恩练习】\n{}",
            [
                format!(
                    "记录感恩：\\n{}",
                    self.journal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表达感激：\\n{}",
                    self.express()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "视角转换：\\n{}",
                    self.reframe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "养成习惯：\\n{}",
                    self.habit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_gratitudepracticerules_basic() {
        let rules = GratitudePracticeRules::new();
        assert_eq!(rules.metadata().name, "感恩练习");
        assert!(!rules.journal().is_empty());
        assert!(!rules.express().is_empty());
        assert!(!rules.reframe().is_empty());
        assert!(!rules.habit().is_empty());
    }

    #[test]
    fn test_gratitudepracticerules_validation() {
        let rules = GratitudePracticeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("gratitude"));
    }

    #[test]
    fn test_gratitudepracticerules_explain() {
        let rules = GratitudePracticeRules::new();
        let e = rules.explain();
        assert!(e.contains("记录感恩"));
        assert!(e.contains("表达感激"));
        assert!(e.contains("视角转换"));
    }
}
