//! 旅伴相处礼仪
//!
//! 结伴出行时互相体谅、分工与相处的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TravelBuddyMannersRules,
    name: "旅伴相处礼仪",
    desc: "结伴出行时互相体谅、分工与相处的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "旅行", "旅伴"]
}

impl TravelBuddyMannersRules {
    /// 计划协商
    pub fn plan(&self) -> Vec<&'static str> {
        vec![
            "共同商议行程安排",
            "尊重彼此偏好",
            "分工明确众人参与",
            "行程留有余地",
        ]
    }

    /// 互相体谅
    pub fn consider(&self) -> Vec<&'static str> {
        vec![
            "作息彼此迁就",
            "消费观互相理解",
            "拍照等互相帮忙",
            "有矛盾平和沟通",
        ]
    }

    /// 责任分担
    pub fn share(&self) -> Vec<&'static str> {
        vec![
            "费用账目及时结清",
            "重担轮流承担",
            "守时不拖累团队",
            "彼此照顾安全",
        ]
    }

    /// 愉快共处
    pub fn enjoy(&self) -> Vec<&'static str> {
        vec!["分享见闻其乐", "尊重独立时间", "不搞小团体", "留下美好回忆"]
    }
}

impl Rule for TravelBuddyMannersRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("travel_buddy")
    }

    fn explain(&self) -> String {
        format!(
            "【旅伴相处礼仪】\n{}",
            [
                format!(
                    "计划协商：\\n{}",
                    self.plan()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "互相体谅：\\n{}",
                    self.consider()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "责任分担：\\n{}",
                    self.share()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "愉快共处：\\n{}",
                    self.enjoy()
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
    fn test_travelbuddymannersrules_basic() {
        let rules = TravelBuddyMannersRules::new();
        assert_eq!(rules.metadata().name, "旅伴相处礼仪");
        assert!(!rules.plan().is_empty());
        assert!(!rules.consider().is_empty());
        assert!(!rules.share().is_empty());
        assert!(!rules.enjoy().is_empty());
    }

    #[test]
    fn test_travelbuddymannersrules_validation() {
        let rules = TravelBuddyMannersRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("travel_buddy"));
    }

    #[test]
    fn test_travelbuddymannersrules_explain() {
        let rules = TravelBuddyMannersRules::new();
        let e = rules.explain();
        assert!(e.contains("计划协商"));
        assert!(e.contains("互相体谅"));
        assert!(e.contains("责任分担"));
    }
}
