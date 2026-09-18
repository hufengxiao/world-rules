//! 中餐圆桌礼仪
//!
//! 中餐圆桌座次、转动转盘与夹菜

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChineseRoundTableRules,
    name: "中餐圆桌礼仪",
    desc: "中餐圆桌座次、转动转盘与夹菜",
    origin: "中国",
    tags: ["社交", "中餐", "圆桌", "礼仪"]
}

impl ChineseRoundTableRules {
    /// 座次主宾
    pub fn seating(&self) -> Vec<&'static str> {
        vec!["面门为主位", "主宾居主右手", "长辈上座", "论资排座"]
    }

    /// 转盘规矩
    pub fn lazy_susan(&self) -> Vec<&'static str> {
        vec![
            "等主人先动转盘",
            "转到面前才夹取",
            "不随意乱转",
            "用完转回原位",
        ]
    }

    /// 筷箸礼仪
    pub fn chopsticks(&self) -> Vec<&'static str> {
        vec!["不插饭中", "不用筷指人", "不敲碗盘", "夹菜不越过人"]
    }

    /// 敬食举止
    pub fn demeanor(&self) -> Vec<&'static str> {
        vec!["夹菜适量不抢", "让老人先动", "咀嚼放低", "斯文用餐"]
    }
}

impl Rule for ChineseRoundTableRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_table")
    }

    fn explain(&self) -> String {
        format!(
            "【中餐圆桌礼仪】\n{}",
            [
                format!(
                    "座次主宾：\\n{}",
                    self.seating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "转盘规矩：\\n{}",
                    self.lazy_susan()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "筷箸礼仪：\\n{}",
                    self.chopsticks()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "敬食举止：\\n{}",
                    self.demeanor()
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
    fn test_chineseroundtablerules_basic() {
        let rules = ChineseRoundTableRules::new();
        assert_eq!(rules.metadata().name, "中餐圆桌礼仪");
        assert!(!rules.seating().is_empty());
        assert!(!rules.lazy_susan().is_empty());
        assert!(!rules.chopsticks().is_empty());
        assert!(!rules.demeanor().is_empty());
    }

    #[test]
    fn test_chineseroundtablerules_validation() {
        let rules = ChineseRoundTableRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("chinese_table"));
    }

    #[test]
    fn test_chineseroundtablerules_explain() {
        let rules = ChineseRoundTableRules::new();
        let e = rules.explain();
        assert!(e.contains("座次主宾"));
        assert!(e.contains("转盘规矩"));
        assert!(e.contains("筷箸礼仪"));
    }
}
