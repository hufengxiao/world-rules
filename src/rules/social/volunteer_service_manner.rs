//! 志愿服务礼仪
//!
//! 参与志愿服务的尊重、服务与协作礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VolunteerServiceMannerRules,
    name: "志愿服务礼仪",
    desc: "参与志愿服务的尊重、服务与协作礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "志愿", "服务"]
}

impl VolunteerServiceMannerRules {
    /// 服务初心
    pub fn serve(&self) -> Vec<&'static str> {
        vec![
            "以服务他人为乐",
            "不计较得失",
            "尊重受助对象",
            "真诚投入尽责",
        ]
    }

    /// 尊重沟通
    pub fn respect(&self) -> Vec<&'static str> {
        vec!["平等与受助者沟通", "不自作主张", "倾听需要", "保护隐私"]
    }

    /// 协作守序
    pub fn cooperate(&self) -> Vec<&'static str> {
        vec!["服从组织安排", "与同伴协同", "注意自身安全", "守时守信"]
    }

    /// 持续参与
    pub fn sustain(&self) -> Vec<&'static str> {
        vec![
            "活动后反馈总结",
            "传递正能量",
            "长期坚持公益",
            "扩大善行影响",
        ]
    }
}

impl Rule for VolunteerServiceMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("volunteer")
    }

    fn explain(&self) -> String {
        format!(
            "【志愿服务礼仪】\n{}",
            [
                format!(
                    "服务初心：\\n{}",
                    self.serve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重沟通：\\n{}",
                    self.respect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "协作守序：\\n{}",
                    self.cooperate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "持续参与：\\n{}",
                    self.sustain()
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
    fn test_volunteerservicemannerrules_basic() {
        let rules = VolunteerServiceMannerRules::new();
        assert_eq!(rules.metadata().name, "志愿服务礼仪");
        assert!(!rules.serve().is_empty());
        assert!(!rules.respect().is_empty());
        assert!(!rules.cooperate().is_empty());
        assert!(!rules.sustain().is_empty());
    }

    #[test]
    fn test_volunteerservicemannerrules_validation() {
        let rules = VolunteerServiceMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("volunteer"));
    }

    #[test]
    fn test_volunteerservicemannerrules_explain() {
        let rules = VolunteerServiceMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("服务初心"));
        assert!(e.contains("尊重沟通"));
        assert!(e.contains("协作守序"));
    }
}
