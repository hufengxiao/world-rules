//! 宴席座次礼仪
//!
//! 宴请座次安排与按座入席的中西礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BanquetSeatingRules,
    name: "宴席座次礼仪",
    desc: "宴请座次安排与按座入席的中西礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "座次", "宴席"]
}

impl BanquetSeatingRules {
    /// 主宾位
    pub fn mainseat(&self) -> Vec<&'static str> {
        vec![
            "面门之座为主宾位",
            "主人为主位相对门",
            "重要宾客请上座",
            "尊长者安排显要位置",
        ]
    }

    /// 按位入座
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "按名牌或引导入座",
            "不抢占他人座位",
            "先请宾长入座",
            "落座轻拉椅子",
        ]
    }

    /// 男女搭配
    pub fn arrangement(&self) -> Vec<&'static str> {
        vec![
            "宴会常男女交叉就座",
            "正式场合依礼安排",
            "亲近者靠近便于交谈",
            "避免背对主宾",
        ]
    }

    /// 起身礼节
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "敬酒起身致意",
            "离席向主人示意",
            "入座离座轻缓",
            "尊重当地座次习惯",
        ]
    }
}

impl Rule for BanquetSeatingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("banquet_seating")
    }

    fn explain(&self) -> String {
        format!(
            "【宴席座次礼仪】\n{}",
            [
                format!(
                    "主宾位：\\n{}",
                    self.mainseat()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "按位入座：\\n{}",
                    self.seating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "男女搭配：\\n{}",
                    self.arrangement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "起身礼节：\\n{}",
                    self.manner()
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
    fn test_banquetseatingrules_basic() {
        let rules = BanquetSeatingRules::new();
        assert_eq!(rules.metadata().name, "宴席座次礼仪");
        assert!(!rules.mainseat().is_empty());
        assert!(!rules.seating().is_empty());
        assert!(!rules.arrangement().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_banquetseatingrules_validation() {
        let rules = BanquetSeatingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("banquet_seating"));
    }

    #[test]
    fn test_banquetseatingrules_explain() {
        let rules = BanquetSeatingRules::new();
        let e = rules.explain();
        assert!(e.contains("主宾位"));
        assert!(e.contains("按位入座"));
        assert!(e.contains("男女搭配"));
    }
}
