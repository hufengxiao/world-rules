//! 渐进式肌肉放松
//!
//! 逐部位绷紧放松以减压助眠的练习方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ProgressiveRelaxationRules,
    name: "渐进式肌肉放松",
    desc: "逐部位绷紧放松以减压助眠的练习方法",
    origin: "心理学",
    tags: ["健康", "放松", "减压", "肌肉"]
}

impl ProgressiveRelaxationRules {
    /// 练习准备
    pub fn prepare(&self) -> Vec<&'static str> {
        vec!["找安静舒适处", "取放松姿势", "深呼深吸", "放下杂念"]
    }

    /// 逐部绷紧
    pub fn progress(&self) -> Vec<&'static str> {
        vec!["从脚到脸逐部", "绷紧肌群数秒", "感受紧张感", "再彻底放松"]
    }

    /// 呼吸配合
    pub fn breathe(&self) -> Vec<&'static str> {
        vec!["绷紧时吸气", "放松时呼气", "节奏均匀", "加深放松"]
    }

    /// 规律练习
    pub fn practice(&self) -> Vec<&'static str> {
        vec![
            "每天练几分钟",
            "睡前放松助眠",
            "压力时随时用",
            "长期坚持有效",
        ]
    }
}

impl Rule for ProgressiveRelaxationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("relaxation")
    }

    fn explain(&self) -> String {
        format!(
            "【渐进式肌肉放松】\n{}",
            [
                format!(
                    "练习准备：\\n{}",
                    self.prepare()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "逐部绷紧：\\n{}",
                    self.progress()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "呼吸配合：\\n{}",
                    self.breathe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "规律练习：\\n{}",
                    self.practice()
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
    fn test_progressiverelaxationrules_basic() {
        let rules = ProgressiveRelaxationRules::new();
        assert_eq!(rules.metadata().name, "渐进式肌肉放松");
        assert!(!rules.prepare().is_empty());
        assert!(!rules.progress().is_empty());
        assert!(!rules.breathe().is_empty());
        assert!(!rules.practice().is_empty());
    }

    #[test]
    fn test_progressiverelaxationrules_validation() {
        let rules = ProgressiveRelaxationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("relaxation"));
    }

    #[test]
    fn test_progressiverelaxationrules_explain() {
        let rules = ProgressiveRelaxationRules::new();
        let e = rules.explain();
        assert!(e.contains("练习准备"));
        assert!(e.contains("逐部绷紧"));
        assert!(e.contains("呼吸配合"));
    }
}
