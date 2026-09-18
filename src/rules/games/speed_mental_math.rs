//! 心算速算
//!
//! 加减乘除心算技巧与简化方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SpeedMentalMathRules,
    name: "心算速算",
    desc: "加减乘除心算技巧与简化方法",
    origin: "数学",
    tags: ["游戏", "心算", "速算", "数学"]
}

impl SpeedMentalMathRules {
    /// 凑整简便
    pub fn rounding(&self) -> Vec<&'static str> {
        vec!["先凑整再算", "加减拆数成整", "借位补数", "化简运算"]
    }

    /// 平方技巧
    pub fn square(&self) -> Vec<&'static str> {
        vec!["末位五数平方", "两位平方可拆分", "依位相加", "简算口诀"]
    }

    /// 快速乘除
    pub fn quick(&self) -> Vec<&'static str> {
        vec!["乘五除二处理", "末位皆零简化", "化整为零", "善用倍数"]
    }

    /// 勤练生巧
    pub fn practice(&self) -> Vec<&'static str> {
        vec!["常做题锻练脑力", "由简单到复杂", "不急躁", "日积月累"]
    }
}

impl Rule for SpeedMentalMathRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mental_math")
    }

    fn explain(&self) -> String {
        format!(
            "【心算速算】\n{}",
            [
                format!(
                    "凑整简便：\\n{}",
                    self.rounding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "平方技巧：\\n{}",
                    self.square()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "快速乘除：\\n{}",
                    self.quick()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "勤练生巧：\\n{}",
                    self.practice()
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
    fn test_speedmentalmathrules_basic() {
        let rules = SpeedMentalMathRules::new();
        assert_eq!(rules.metadata().name, "心算速算");
        assert!(!rules.rounding().is_empty());
        assert!(!rules.square().is_empty());
        assert!(!rules.quick().is_empty());
        assert!(!rules.practice().is_empty());
    }

    #[test]
    fn test_speedmentalmathrules_validation() {
        let rules = SpeedMentalMathRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("mental_math"));
    }

    #[test]
    fn test_speedmentalmathrules_explain() {
        let rules = SpeedMentalMathRules::new();
        let e = rules.explain();
        assert!(e.contains("凑整简便"));
        assert!(e.contains("平方技巧"));
        assert!(e.contains("快速乘除"));
    }
}
