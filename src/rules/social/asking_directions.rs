//! 问路礼貌
//!
//! 向陌生人问路、指路与致谢的礼貌表达

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AskingDirectionsRules,
    name: "问路礼貌",
    desc: "向陌生人问路、指路与致谢的礼貌表达",
    origin: "中国",
    tags: ["社交", "问路", "指路", "礼貌"]
}

impl AskingDirectionsRules {
    /// 礼貌开口
    pub fn ask(&self) -> Vec<&'static str> {
        vec!["选合适的人问", "礼貌打招呼", "说明想去哪", "态度诚恳"]
    }

    /// 指路清晰
    pub fn show(&self) -> Vec<&'static str> {
        vec!["耐心说明方向", "可说标志地标", "不确定不硬说", "必要时画图"]
    }

    /// 以防误解
    pub fn confirm(&self) -> Vec<&'static str> {
        vec!["复疑问清", "关键岔路再确认", "不轻信模糊", "不懂再问"]
    }

    /// 真诚致谢
    pub fn thanks(&self) -> Vec<&'static str> {
        vec!["答谢对方时间", "礼貌道别", "被问也热心答", "将心比心"]
    }
}

impl Rule for AskingDirectionsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("directions")
    }

    fn explain(&self) -> String {
        format!(
            "【问路礼貌】\n{}",
            [
                format!(
                    "礼貌开口：\\n{}",
                    self.ask()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "指路清晰：\\n{}",
                    self.show()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "以防误解：\\n{}",
                    self.confirm()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "真诚致谢：\\n{}",
                    self.thanks()
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
    fn test_askingdirectionsrules_basic() {
        let rules = AskingDirectionsRules::new();
        assert_eq!(rules.metadata().name, "问路礼貌");
        assert!(!rules.ask().is_empty());
        assert!(!rules.show().is_empty());
        assert!(!rules.confirm().is_empty());
        assert!(!rules.thanks().is_empty());
    }

    #[test]
    fn test_askingdirectionsrules_validation() {
        let rules = AskingDirectionsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("directions"));
    }

    #[test]
    fn test_askingdirectionsrules_explain() {
        let rules = AskingDirectionsRules::new();
        let e = rules.explain();
        assert!(e.contains("礼貌开口"));
        assert!(e.contains("指路清晰"));
        assert!(e.contains("以防误解"));
    }
}
