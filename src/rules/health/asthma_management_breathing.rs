//! 哮喘管理与呼吸
//!
//! 哮喘用药、诱发因素与呼吸训练

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AsthmaManagementBreathingRules,
    name: "哮喘管理与呼吸",
    desc: "哮喘用药、诱发因素与呼吸训练",
    origin: "医学",
    tags: ["健康", "哮喘", "呼吸", "管理"]
}

impl AsthmaManagementBreathingRules {
    /// 规范用药
    pub fn medication(&self) -> Vec<&'static str> {
        vec!["按时使用吸入剂", "遵医嘱调整", "备用急救药", "不擅自停药"]
    }

    /// 避开诱因
    pub fn trigger(&self) -> Vec<&'static str> {
        vec!["远离花粉粉尘", "防过敏原", "冷空气保暖", "少烟尘刺激"]
    }

    /// 早期应对
    pub fn response(&self) -> Vec<&'static str> {
        vec![
            "胸闷气急早用药",
            "学会腹式呼吸",
            "发作仍不缓就医",
            "记录发作",
        ]
    }

    /// 平稳练习
    pub fn breathing(&self) -> Vec<&'static str> {
        vec!["缓慢匀呼勿促", "用鼻吸气", "放松保持心态", "规律运动"]
    }
}

impl Rule for AsthmaManagementBreathingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("asthma")
    }

    fn explain(&self) -> String {
        format!(
            "【哮喘管理与呼吸】\n{}",
            [
                format!(
                    "规范用药：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避开诱因：\\n{}",
                    self.trigger()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "早期应对：\\n{}",
                    self.response()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "平稳练习：\\n{}",
                    self.breathing()
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
    fn test_asthmamanagementbreathingrules_basic() {
        let rules = AsthmaManagementBreathingRules::new();
        assert_eq!(rules.metadata().name, "哮喘管理与呼吸");
        assert!(!rules.medication().is_empty());
        assert!(!rules.trigger().is_empty());
        assert!(!rules.response().is_empty());
        assert!(!rules.breathing().is_empty());
    }

    #[test]
    fn test_asthmamanagementbreathingrules_validation() {
        let rules = AsthmaManagementBreathingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("asthma"));
    }

    #[test]
    fn test_asthmamanagementbreathingrules_explain() {
        let rules = AsthmaManagementBreathingRules::new();
        let e = rules.explain();
        assert!(e.contains("规范用药"));
        assert!(e.contains("避开诱因"));
        assert!(e.contains("早期应对"));
    }
}
