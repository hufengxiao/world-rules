//! 灯谜猜谜
//!
//! 元宵灯谜的谜面、猜法与传统趣味

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RiddleLanternFestivalRules,
    name: "灯谜猜谜",
    desc: "元宵灯谜的谜面、猜法与传统趣味",
    origin: "中国",
    tags: ["游戏", "灯谜", "正月十五", "传统"]
}

impl RiddleLanternFestivalRules {
    /// 灯谜特点
    pub fn style(&self) -> Vec<&'static str> {
        vec![
            "谜面短小精妙",
            "谜底讲究对仗",
            "常用拆字会意",
            "传统文字魅力",
        ]
    }

    /// 猜谜方法
    pub fn guess(&self) -> Vec<&'static str> {
        vec!["细读谜面含义", "会意联想拆字", "谐音提示", "多试多推理"]
    }

    /// 猜中欢喜
    pub fn reward(&self) -> Vec<&'static str> {
        vec!["猜中自豪有礼", "灯下围猜热闹", "不抢撕灯谜", "大家共享乐趣"]
    }

    /// 节庆氛围
    pub fn festival(&self) -> Vec<&'static str> {
        vec!["赏灯猜谜应景", "阖家共乐", "传承文化雅趣", "猜谜不止一时"]
    }
}

impl Rule for RiddleLanternFestivalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("lantern_riddle")
    }

    fn explain(&self) -> String {
        format!(
            "【灯谜猜谜】\n{}",
            [
                format!(
                    "灯谜特点：\\n{}",
                    self.style()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "猜谜方法：\\n{}",
                    self.guess()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "猜中欢喜：\\n{}",
                    self.reward()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "节庆氛围：\\n{}",
                    self.festival()
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
    fn test_riddlelanternfestivalrules_basic() {
        let rules = RiddleLanternFestivalRules::new();
        assert_eq!(rules.metadata().name, "灯谜猜谜");
        assert!(!rules.style().is_empty());
        assert!(!rules.guess().is_empty());
        assert!(!rules.reward().is_empty());
        assert!(!rules.festival().is_empty());
    }

    #[test]
    fn test_riddlelanternfestivalrules_validation() {
        let rules = RiddleLanternFestivalRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("lantern_riddle"));
    }

    #[test]
    fn test_riddlelanternfestivalrules_explain() {
        let rules = RiddleLanternFestivalRules::new();
        let e = rules.explain();
        assert!(e.contains("灯谜特点"));
        assert!(e.contains("猜谜方法"));
        assert!(e.contains("猜中欢喜"));
    }
}
