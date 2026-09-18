//! 婚宴坐席举止
//!
//! 婚宴入座、用餐与离席的礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WeddingBanquetMannerRules,
    name: "婚宴坐席举止",
    desc: "婚宴入座、用餐与离席的礼节",
    origin: "中国",
    tags: ["社交", "婚宴", "坐席", "举止"]
}

impl WeddingBanquetMannerRules {
    /// 入座有序
    pub fn seating(&self) -> Vec<&'static str> {
        vec!["按引导就座", "不抢座占位", "先把主宾入", "礼让同桌"]
    }

    /// 用餐文明
    pub fn dining(&self) -> Vec<&'static str> {
        vec!["随主持人节奏", "动筷适时", "夹取适量", "不狼吞虎咽"]
    }

    /// 席间互动
    pub fn interact(&self) -> Vec<&'static str> {
        vec!["敬酒助兴", "分享喜事互动", "照顾邻座", "气氛融洽"]
    }

    /// 散席告辞
    pub fn leaving(&self) -> Vec<&'static str> {
        vec!["饭后道贺离开", "向新人致谢", "不拖沓", "欢喜收场"]
    }
}

impl Rule for WeddingBanquetMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("wedding_banquet")
    }

    fn explain(&self) -> String {
        format!(
            "【婚宴坐席举止】\n{}",
            [
                format!(
                    "入座有序：\\n{}",
                    self.seating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用餐文明：\\n{}",
                    self.dining()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "席间互动：\\n{}",
                    self.interact()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "散席告辞：\\n{}",
                    self.leaving()
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
    fn test_weddingbanquetmannerrules_basic() {
        let rules = WeddingBanquetMannerRules::new();
        assert_eq!(rules.metadata().name, "婚宴坐席举止");
        assert!(!rules.seating().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.interact().is_empty());
        assert!(!rules.leaving().is_empty());
    }

    #[test]
    fn test_weddingbanquetmannerrules_validation() {
        let rules = WeddingBanquetMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("wedding_banquet"));
    }

    #[test]
    fn test_weddingbanquetmannerrules_explain() {
        let rules = WeddingBanquetMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("入座有序"));
        assert!(e.contains("用餐文明"));
        assert!(e.contains("席间互动"));
    }
}
