//! 给人建议的分寸
//!
//! 想给他人提建议时如何开口不显冒昧

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OfferingAdviceTactRules,
    name: "给人建议的分寸",
    desc: "想给他人提建议时如何开口不显冒昧",
    origin: "中国",
    tags: ["社交", "建议", "分寸", "沟通"]
}

impl OfferingAdviceTactRules {
    /// 是否该说
    pub fn consider(&self) -> Vec<&'static str> {
        vec![
            "看对方是否需要",
            "不主动好为人师",
            "被请再细说",
            "对方情绪先安",
        ]
    }

    /// 开口方式
    pub fn wording(&self) -> Vec<&'static str> {
        vec!["我有个想法参考", "换种角度看", "说事不说人", "给选择留余地"]
    }

    /// 分寸拿捏
    pub fn tact(&self) -> Vec<&'static str> {
        vec!["点到为止", "不强逼接受", "尊重其决定", "不居高临下"]
    }

    /// 事后与边界
    pub fn respect(&self) -> Vec<&'static str> {
        vec!["不反复催促", "结果如何不指责", "半点人情不居", "边界把握好"]
    }
}

impl Rule for OfferingAdviceTactRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("offer_advice")
    }

    fn explain(&self) -> String {
        format!(
            "【给人建议的分寸】\n{}",
            [
                format!(
                    "是否该说：\\n{}",
                    self.consider()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "开口方式：\\n{}",
                    self.wording()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分寸拿捏：\\n{}",
                    self.tact()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "事后与边界：\\n{}",
                    self.respect()
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
    fn test_offeringadvicetactrules_basic() {
        let rules = OfferingAdviceTactRules::new();
        assert_eq!(rules.metadata().name, "给人建议的分寸");
        assert!(!rules.consider().is_empty());
        assert!(!rules.wording().is_empty());
        assert!(!rules.tact().is_empty());
        assert!(!rules.respect().is_empty());
    }

    #[test]
    fn test_offeringadvicetactrules_validation() {
        let rules = OfferingAdviceTactRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("offer_advice"));
    }

    #[test]
    fn test_offeringadvicetactrules_explain() {
        let rules = OfferingAdviceTactRules::new();
        let e = rules.explain();
        assert!(e.contains("是否该说"));
        assert!(e.contains("开口方式"));
        assert!(e.contains("分寸拿捏"));
    }
}
