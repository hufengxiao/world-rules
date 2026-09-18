//! 告辞离席
//!
//! 聚会中途或结束告辞的礼貌与时机把握

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FarewellLeavingPartyRules,
    name: "告辞离席",
    desc: "聚会中途或结束告辞的礼貌与时机把握",
    origin: "中国",
    tags: ["社交", "告辞", "离席", "聚会"]
}

impl FarewellLeavingPartyRules {
    /// 告辞时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "尽兴时告辞最好",
            "不在高潮中途走",
            "必要先走先致意",
            "别拖泥带水",
        ]
    }

    /// 礼貌告别
    pub fn wording(&self) -> Vec<&'static str> {
        vec!["向主人当面辞行", "说明离开原因", "感谢款待", "不出人意而别"]
    }

    /// 先辞开幅
    pub fn host(&self) -> Vec<&'static str> {
        vec![
            "先跟主持人辞",
            "慢慢与其他宾客",
            "不惊动全场",
            "挥手致意即可",
        ]
    }

    /// 离后续礼
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "到家报个平安",
            "发消息再谢一次",
            "途中注意安全",
            "改日再聚作为回谢",
        ]
    }
}

impl Rule for FarewellLeavingPartyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("farewell_leaving")
    }

    fn explain(&self) -> String {
        format!(
            "【告辞离席】\n{}",
            [
                format!(
                    "告辞时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼貌告别：\\n{}",
                    self.wording()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "先辞开幅：\\n{}",
                    self.host()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "离后续礼：\\n{}",
                    self.after()
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
    fn test_farewellleavingpartyrules_basic() {
        let rules = FarewellLeavingPartyRules::new();
        assert_eq!(rules.metadata().name, "告辞离席");
        assert!(!rules.timing().is_empty());
        assert!(!rules.wording().is_empty());
        assert!(!rules.host().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_farewellleavingpartyrules_validation() {
        let rules = FarewellLeavingPartyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("farewell_leaving"));
    }

    #[test]
    fn test_farewellleavingpartyrules_explain() {
        let rules = FarewellLeavingPartyRules::new();
        let e = rules.explain();
        assert!(e.contains("告辞时机"));
        assert!(e.contains("礼貌告别"));
        assert!(e.contains("先辞开幅"));
    }
}
