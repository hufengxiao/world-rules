//! 攀岩馆练习规范
//!
//! 室内攀岩的安全、路线与共练礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RockClimbingGymRules,
    name: "攀岩馆练习规范",
    desc: "室内攀岩的安全、路线与共练礼仪",
    origin: "国际",
    tags: ["体育", "攀岩", "室内", "安全"]
}

impl RockClimbingGymRules {
    /// 安全准备
    pub fn gear(&self) -> Vec<&'static str> {
        vec![
            "用合格的装备",
            "检查绳索安全带",
            "热身充分再攀",
            "确保保险正确",
        ]
    }

    /// 攀登路线
    pub fn route(&self) -> Vec<&'static str> {
        vec![
            "选定难度合适路线",
            "使用岩点借力",
            "下落稳妥控身",
            "量力逐级挑战",
        ]
    }

    /// 搭档保障
    pub fn belay(&self) -> Vec<&'static str> {
        vec![
            "攀爬时有人保护",
            "沟通喊话清楚",
            "不随便解扣",
            "认真负责保护",
        ]
    }

    /// 场地礼仪
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["不占线路长久", "落点确认安全", "不劝扰他人", "遵守场馆规章"]
    }
}

impl Rule for RockClimbingGymRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rock_climbing")
    }

    fn explain(&self) -> String {
        format!(
            "【攀岩馆练习规范】\n{}",
            [
                format!(
                    "安全准备：\\n{}",
                    self.gear()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "攀登路线：\\n{}",
                    self.route()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "搭档保障：\\n{}",
                    self.belay()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场地礼仪：\\n{}",
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
    fn test_rockclimbinggymrules_basic() {
        let rules = RockClimbingGymRules::new();
        assert_eq!(rules.metadata().name, "攀岩馆练习规范");
        assert!(!rules.gear().is_empty());
        assert!(!rules.route().is_empty());
        assert!(!rules.belay().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_rockclimbinggymrules_validation() {
        let rules = RockClimbingGymRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("rock_climbing"));
    }

    #[test]
    fn test_rockclimbinggymrules_explain() {
        let rules = RockClimbingGymRules::new();
        let e = rules.explain();
        assert!(e.contains("安全准备"));
        assert!(e.contains("攀登路线"));
        assert!(e.contains("搭档保障"));
    }
}
