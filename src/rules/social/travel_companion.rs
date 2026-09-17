//! 结伴出行相处
//!
//! 结伴旅行的事前商量、互相体谅与行程配合

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TravelCompanionRules,
    name: "结伴出行相处",
    desc: "结伴旅行的事前商量、互相体谅与行程配合",
    origin: "中国",
    tags: ["社交", "旅行", "结伴", "相处"]
}

impl TravelCompanionRules {
    /// 行前商量
    pub fn plan(&self) -> Vec<&'static str> {
        vec![
            "行程共同商量",
            "预算事先讲清",
            "目的地求同存异",
            "分工分担任务",
        ]
    }

    /// 途中配合
    pub fn cooperate(&self) -> Vec<&'static str> {
        vec!["照顾彼此节奏", "遇分歧好商量", "不各顾各难", "互相体谅"]
    }

    /// 费用坦诚
    pub fn expense(&self) -> Vec<&'static str> {
        vec!["费用透明分担", "不斤斤计较", "提前定分摊", "少占人便宜"]
    }

    /// 愉快相陪
    pub fn joy(&self) -> Vec<&'static str> {
        vec!["分享见闻乐趣", "旅途互相帮助", "不抱怨不扫兴", "结伴欢乐"]
    }
}

impl Rule for TravelCompanionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("travel")
    }

    fn explain(&self) -> String {
        format!(
            "【结伴出行相处】\n{}",
            [
                format!(
                    "行前商量：\\n{}",
                    self.plan()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "途中配合：\\n{}",
                    self.cooperate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "费用坦诚：\\n{}",
                    self.expense()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "愉快相陪：\\n{}",
                    self.joy()
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
    fn test_travelcompanionrules_basic() {
        let rules = TravelCompanionRules::new();
        assert_eq!(rules.metadata().name, "结伴出行相处");
        assert!(!rules.plan().is_empty());
        assert!(!rules.cooperate().is_empty());
        assert!(!rules.expense().is_empty());
        assert!(!rules.joy().is_empty());
    }

    #[test]
    fn test_travelcompanionrules_validation() {
        let rules = TravelCompanionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("travel"));
    }

    #[test]
    fn test_travelcompanionrules_explain() {
        let rules = TravelCompanionRules::new();
        let e = rules.explain();
        assert!(e.contains("行前商量"));
        assert!(e.contains("途中配合"));
        assert!(e.contains("费用坦诚"));
    }
}
