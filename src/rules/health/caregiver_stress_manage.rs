//! 照护者减压
//!
//! 长期照护者的自我关爱、求助与减压

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CaregiverStressManageRules,
    name: "照护者减压",
    desc: "长期照护者的自我关爱、求助与减压",
    origin: "心理",
    tags: ["健康", "照护", "压力", "减压"]
}

impl CaregiverStressManageRules {
    /// 自我关爱
    pub fn self_care(&self) -> Vec<&'static str> {
        vec![
            "也要照顾好自己的身体",
            "留点时间休息",
            "睡眠饮食规律",
            "不硬扛到底",
        ]
    }

    /// 寻求支持
    pub fn support(&self) -> Vec<&'static str> {
        vec!["请家人分担", "找照护服务", "社区互助", "不独自苦撑"]
    }

    /// 情绪释放
    pub fn emotion(&self) -> Vec<&'static str> {
        vec!["倾诉发泄情绪", "写日记放松", "适度运动缓解", "允许自己休息"]
    }

    /// 专业求助
    pub fn help(&self) -> Vec<&'static str> {
        vec![
            "压力巨大请咨询",
            "情绪低落就医",
            "合理分配精力",
            "善待照护者",
        ]
    }
}

impl Rule for CaregiverStressManageRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("caregiver")
    }

    fn explain(&self) -> String {
        format!(
            "【照护者减压】\n{}",
            [
                format!(
                    "自我关爱：\\n{}",
                    self.self_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "寻求支持：\\n{}",
                    self.support()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "情绪释放：\\n{}",
                    self.emotion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "专业求助：\\n{}",
                    self.help()
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
    fn test_caregiverstressmanagerules_basic() {
        let rules = CaregiverStressManageRules::new();
        assert_eq!(rules.metadata().name, "照护者减压");
        assert!(!rules.self_care().is_empty());
        assert!(!rules.support().is_empty());
        assert!(!rules.emotion().is_empty());
        assert!(!rules.help().is_empty());
    }

    #[test]
    fn test_caregiverstressmanagerules_validation() {
        let rules = CaregiverStressManageRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("caregiver"));
    }

    #[test]
    fn test_caregiverstressmanagerules_explain() {
        let rules = CaregiverStressManageRules::new();
        let e = rules.explain();
        assert!(e.contains("自我关爱"));
        assert!(e.contains("寻求支持"));
        assert!(e.contains("情绪释放"));
    }
}
