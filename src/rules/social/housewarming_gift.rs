//! 乔迁送礼
//!
//! 乔迁之喜的贺礼、祝福与到访礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HousewarmingGiftRules,
    name: "乔迁送礼",
    desc: "乔迁之喜的贺礼、祝福与到访礼仪",
    origin: "中国",
    tags: ["社交", "乔迁", "送礼", "到访"]
}

impl HousewarmingGiftRules {
    /// 贺礼相宜
    pub fn gift(&self) -> Vec<&'static str> {
        vec!["送实用新家居", "寓意兴旺礼品", "按亲近程度选", "不空手登门"]
    }

    /// 进门礼节
    pub fn arrive(&self) -> Vec<&'static str> {
        vec!["先敲门礼貌问候", "换鞋进室", "先称赞新居", "不多评点"]
    }

    /// 参访有度
    pub fn visit(&self) -> Vec<&'static str> {
        vec!["欣赏布置有分寸", "不随意开柜", "帮忙不添乱", "适当时告辞"]
    }

    /// 祝福暖意
    pub fn bless(&self) -> Vec<&'static str> {
        vec!["道福迁新居", "祝新宅和睦", "问是否需要帮忙", "情谊往来"]
    }
}

impl Rule for HousewarmingGiftRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("housewarming")
    }

    fn explain(&self) -> String {
        format!(
            "【乔迁送礼】\n{}",
            [
                format!(
                    "贺礼相宜：\\n{}",
                    self.gift()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "进门礼节：\\n{}",
                    self.arrive()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "参访有度：\\n{}",
                    self.visit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "祝福暖意：\\n{}",
                    self.bless()
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
    fn test_housewarminggiftrules_basic() {
        let rules = HousewarmingGiftRules::new();
        assert_eq!(rules.metadata().name, "乔迁送礼");
        assert!(!rules.gift().is_empty());
        assert!(!rules.arrive().is_empty());
        assert!(!rules.visit().is_empty());
        assert!(!rules.bless().is_empty());
    }

    #[test]
    fn test_housewarminggiftrules_validation() {
        let rules = HousewarmingGiftRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("housewarming"));
    }

    #[test]
    fn test_housewarminggiftrules_explain() {
        let rules = HousewarmingGiftRules::new();
        let e = rules.explain();
        assert!(e.contains("贺礼相宜"));
        assert!(e.contains("进门礼节"));
        assert!(e.contains("参访有度"));
    }
}
