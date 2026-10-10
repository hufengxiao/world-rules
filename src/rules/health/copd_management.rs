//! 慢阻肺管理
//!
//! 慢性阻塞性肺疾病的日常管理与康复

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CopdManagementRules,
    name: "慢阻肺管理",
    desc: "慢性阻塞性肺疾病的日常管理与康复",
    origin: "中国",
    tags: ["慢性病", "呼吸", "管理"]
}

impl CopdManagementRules {
    /// 戒烟防护
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "戒烟是首要措施",
            "避免二手烟与粉尘",
            "远离刺激性气体",
            "注意保暖防感冒",
        ]
    }

    /// 呼吸训练
    pub fn breathing(&self) -> Vec<&'static str> {
        vec![
            "缩唇呼吸练习",
            "腹式呼吸锻炼",
            "缓慢深长呼气",
            "每日坚持训练",
        ]
    }

    /// 运动康复
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "适度有氧运动",
            "量力而行不勉强",
            "呼吸操配合锻炼",
            "增强呼吸肌力量",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "遵医嘱规律用药",
            "勿自行停用吸入剂",
            "急性加重及时就医",
            "定期复查肺功能",
        ]
    }
}

impl Rule for CopdManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("copd_management")
    }

    fn explain(&self) -> String {
        format!(
            "【慢阻肺管理】\n{}",
            [
                format!(
                    "戒烟防护：\\n{}",
                    self.prevention()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "呼吸训练：\\n{}",
                    self.breathing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动康复：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_copdmanagementrules_basic() {
        let rules = CopdManagementRules::new();
        assert_eq!(rules.metadata().name, "慢阻肺管理");
        assert!(!rules.prevention().is_empty());
        assert!(!rules.breathing().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_copdmanagementrules_validation() {
        let rules = CopdManagementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("copd_management"));
    }

    #[test]
    fn test_copdmanagementrules_explain() {
        let rules = CopdManagementRules::new();
        let e = rules.explain();
        assert!(e.contains("戒烟防护"));
        assert!(e.contains("呼吸训练"));
        assert!(e.contains("运动康复"));
    }
}
