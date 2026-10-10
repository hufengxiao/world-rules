//! 类风湿关节炎护理
//!
//! 类风湿关节炎的关节保护与生活管理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RheumatoidArthritisCareRules,
    name: "类风湿关节炎护理",
    desc: "类风湿关节炎的关节保护与生活管理",
    origin: "中国",
    tags: ["慢性病", "关节", "护理"]
}

impl RheumatoidArthritisCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "对称性小关节肿痛",
            "晨僵超过一小时",
            "关节活动受限",
            "可伴全身乏力",
        ]
    }

    /// 关节保护
    pub fn protection(&self) -> Vec<&'static str> {
        vec![
            "避免关节过度负重",
            "用大关节代替小关节",
            "注意关节保暖",
            "急性期适当休息",
        ]
    }

    /// 功能锻炼
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "缓解期适度活动",
            "关节活动度训练",
            "肌力维持锻炼",
            "循序渐进不勉强",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "早期诊断早治疗",
            "遵医嘱规律用药",
            "勿自行停用抗风湿药",
            "定期复查监测",
        ]
    }
}

impl Rule for RheumatoidArthritisCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("rheumatoid_arthritis_care")
    }

    fn explain(&self) -> String {
        format!(
            "【类风湿关节炎护理】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "关节保护：\\n{}",
                    self.protection()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "功能锻炼：\\n{}",
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
    fn test_rheumatoidarthritiscarerules_basic() {
        let rules = RheumatoidArthritisCareRules::new();
        assert_eq!(rules.metadata().name, "类风湿关节炎护理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.protection().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_rheumatoidarthritiscarerules_validation() {
        let rules = RheumatoidArthritisCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("rheumatoid_arthritis_care")
        );
    }

    #[test]
    fn test_rheumatoidarthritiscarerules_explain() {
        let rules = RheumatoidArthritisCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("关节保护"));
        assert!(e.contains("功能锻炼"));
    }
}
