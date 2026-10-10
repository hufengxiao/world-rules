//! 满月宴礼仪
//!
//! 婴儿满月设宴庆贺的习俗与待客礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OneMonthBanquetRules,
    name: "满月宴礼仪",
    desc: "婴儿满月设宴庆贺的习俗与待客礼节",
    origin: "中国",
    tags: ["家庭", "喜宴", "习俗"]
}

impl OneMonthBanquetRules {
    /// 设宴缘由
    pub fn occasion(&self) -> Vec<&'static str> {
        vec![
            "婴儿满月庆贺",
            "答谢亲友关爱",
            "祈愿孩子健康",
            "分享添丁之喜",
        ]
    }

    /// 主家待客
    pub fn hosting(&self) -> Vec<&'static str> {
        vec![
            "提前告知亲友",
            "妥善安排宴席",
            "备喜糖红蛋回礼",
            "热情迎送宾客",
        ]
    }

    /// 宾客礼节
    pub fn guest(&self) -> Vec<&'static str> {
        vec!["按时赴宴道贺", "备贺礼红包", "祝福孩子安康", "不喧哗扰婴孩"]
    }

    /// 探望讲究
    pub fn visiting(&self) -> Vec<&'static str> {
        vec![
            "探望前先询问方便",
            "感冒者暂勿探望",
            "洗手后再抱婴孩",
            "不宜久留打扰",
        ]
    }
}

impl Rule for OneMonthBanquetRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("one_month_banquet")
    }

    fn explain(&self) -> String {
        format!(
            "【满月宴礼仪】\n{}",
            [
                format!(
                    "设宴缘由：\\n{}",
                    self.occasion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "主家待客：\\n{}",
                    self.hosting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "宾客礼节：\\n{}",
                    self.guest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "探望讲究：\\n{}",
                    self.visiting()
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
    fn test_onemonthbanquetrules_basic() {
        let rules = OneMonthBanquetRules::new();
        assert_eq!(rules.metadata().name, "满月宴礼仪");
        assert!(!rules.occasion().is_empty());
        assert!(!rules.hosting().is_empty());
        assert!(!rules.guest().is_empty());
        assert!(!rules.visiting().is_empty());
    }

    #[test]
    fn test_onemonthbanquetrules_validation() {
        let rules = OneMonthBanquetRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("one_month_banquet"));
    }

    #[test]
    fn test_onemonthbanquetrules_explain() {
        let rules = OneMonthBanquetRules::new();
        let e = rules.explain();
        assert!(e.contains("设宴缘由"));
        assert!(e.contains("主家待客"));
        assert!(e.contains("宾客礼节"));
    }
}
