//! 宴席礼节
//!
//! 赴宴时间、入座、用餐与主客互动的礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BanquetCeremonyRules,
    name: "宴席礼节",
    desc: "赴宴时间、入座、用餐与主客互动的礼节",
    origin: "中国",
    tags: ["社交", "宴席", "礼节", "用餐"]
}

impl BanquetCeremonyRules {
    /// 赴宴准时
    pub fn punctual(&self) -> Vec<&'static str> {
        vec![
            "按请柬时间准时到",
            "迟到不影响席",
            "提前到场帮忙",
            "不确定先问清",
        ]
    }

    /// 入座得体
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "按主位安排就座",
            "长幼主客有序",
            "不抢先占主位",
            "听从主人引导",
        ]
    }

    /// 用餐文雅
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "夹菜适量不翻挑",
            "咀嚼不吧唧出声",
            "不在餐桌玩手机",
            "菜先请长辈动",
        ]
    }

    /// 致词谢意
    pub fn thanks(&self) -> Vec<&'static str> {
        vec!["举杯敬长辈主人", "适时致谢", "餐毕道别", "回味不宜久"]
    }
}

impl Rule for BanquetCeremonyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("banquet")
    }

    fn explain(&self) -> String {
        format!(
            "【宴席礼节】\n{}",
            [
                format!(
                    "赴宴准时：\\n{}",
                    self.punctual()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "入座得体：\\n{}",
                    self.seating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用餐文雅：\\n{}",
                    self.dining()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "致词谢意：\\n{}",
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
    fn test_banquetceremonyrules_basic() {
        let rules = BanquetCeremonyRules::new();
        assert_eq!(rules.metadata().name, "宴席礼节");
        assert!(!rules.punctual().is_empty());
        assert!(!rules.seating().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.thanks().is_empty());
    }

    #[test]
    fn test_banquetceremonyrules_validation() {
        let rules = BanquetCeremonyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("banquet"));
    }

    #[test]
    fn test_banquetceremonyrules_explain() {
        let rules = BanquetCeremonyRules::new();
        let e = rules.explain();
        assert!(e.contains("赴宴准时"));
        assert!(e.contains("入座得体"));
        assert!(e.contains("用餐文雅"));
    }
}
