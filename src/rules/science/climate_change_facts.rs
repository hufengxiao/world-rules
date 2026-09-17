//! 气候变化常识
//!
//! 气候变化成因、影响与节能减排行动

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ClimateChangeFactsRules,
    name: "气候变化常识",
    desc: "气候变化成因、影响与节能减排行动",
    origin: "国际",
    tags: ["科学", "气候", "环保", "温室"]
}

impl ClimateChangeFactsRules {
    /// 成因认识
    pub fn cause(&self) -> Vec<&'static str> {
        vec!["温室气体增多", "人类活动排放", "气温上升", "称全球变暖"]
    }

    /// 影响
    pub fn impact(&self) -> Vec<&'static str> {
        vec!["极端天气增多", "海平面上升", "冰川融化", "生态受影响"]
    }

    /// 节能减排
    pub fn action(&self) -> Vec<&'static str> {
        vec!["节约用电用水", "减少开车步行", "少用一次性", "多绿色出行"]
    }

    /// 珍爱地球
    pub fn care(&self) -> Vec<&'static str> {
        vec!["植树护绿", "支持可再生", "从小行动起", "低碳生活"]
    }
}

impl Rule for ClimateChangeFactsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("climate")
    }

    fn explain(&self) -> String {
        format!(
            "【气候变化常识】\n{}",
            [
                format!(
                    "成因认识：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "影响：\\n{}",
                    self.impact()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "节能减排：\\n{}",
                    self.action()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "珍爱地球：\\n{}",
                    self.care()
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
    fn test_climatechangefactsrules_basic() {
        let rules = ClimateChangeFactsRules::new();
        assert_eq!(rules.metadata().name, "气候变化常识");
        assert!(!rules.cause().is_empty());
        assert!(!rules.impact().is_empty());
        assert!(!rules.action().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_climatechangefactsrules_validation() {
        let rules = ClimateChangeFactsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("climate"));
    }

    #[test]
    fn test_climatechangefactsrules_explain() {
        let rules = ClimateChangeFactsRules::new();
        let e = rules.explain();
        assert!(e.contains("成因认识"));
        assert!(e.contains("影响"));
        assert!(e.contains("节能减排"));
    }
}
