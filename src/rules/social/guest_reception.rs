//! 待客礼仪
//!
//! 家中接待客人时的迎客、奉茶、款待与送客礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GuestReceptionRules,
    name: "待客礼仪",
    desc: "家中接待客人时的迎客、奉茶、款待与送客礼仪",
    origin: "中国",
    tags: ["社交", "礼仪", "待客", "迎客", "茶道"]
}

impl GuestReceptionRules {
    /// 迎客
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "提前整理居所保持整洁",
            "准时或提前到门口迎候",
            "面带微笑主动问候握手",
            "引导客人入座并安置衣帽",
        ]
    }

    /// 款待
    pub fn hospitality(&self) -> Vec<&'static str> {
        vec![
            "奉上茶水或饮品点心",
            "尊重客人的饮食偏好",
            "交谈热络但不追问隐私",
            "合理安排用餐或留宿",
        ]
    }

    /// 交谈
    pub fn conversation(&self) -> Vec<&'static str> {
        vec![
            "多以客人关心的为话题",
            "不冷落或刻意回避客人",
            "避免不删当面问询私事",
            "照顾不擅长表达的客人",
        ]
    }

    /// 送客
    pub fn farewell(&self) -> Vec<&'static str> {
        vec![
            "客人起身时礼貌相送",
            "送至门口或电梯",
            "目送或挥手致意",
            "约好下次见并妥当送行",
        ]
    }
}

impl Rule for GuestReceptionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("guest_reception")
    }

    fn explain(&self) -> String {
        format!(
            "【待客礼仪】\n{}",
            [
                format!(
                    "迎客：\\n{}",
                    self.greeting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "款待：\\n{}",
                    self.hospitality()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交谈：\\n{}",
                    self.conversation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "送客：\\n{}",
                    self.farewell()
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
    fn test_guestreceptionrules_basic() {
        let rules = GuestReceptionRules::new();
        assert_eq!(rules.metadata().name, "待客礼仪");
        assert!(!rules.greeting().is_empty());
        assert!(!rules.hospitality().is_empty());
        assert!(!rules.conversation().is_empty());
        assert!(!rules.farewell().is_empty());
    }

    #[test]
    fn test_guestreceptionrules_validation() {
        let rules = GuestReceptionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("guest_reception"));
    }

    #[test]
    fn test_guestreceptionrules_explain() {
        let rules = GuestReceptionRules::new();
        let e = rules.explain();
        assert!(e.contains("迎客"));
        assert!(e.contains("款待"));
        assert!(e.contains("交谈"));
    }
}
