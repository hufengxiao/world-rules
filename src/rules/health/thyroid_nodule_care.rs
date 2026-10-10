//! 甲状腺结节管理
//!
//! 甲状腺结节的观察随访与生活调理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ThyroidNoduleCareRules,
    name: "甲状腺结节管理",
    desc: "甲状腺结节的观察随访与生活调理",
    origin: "中国",
    tags: ["慢性病", "内分泌", "管理"]
}

impl ThyroidNoduleCareRules {
    /// 认识结节
    pub fn understanding(&self) -> Vec<&'static str> {
        vec![
            "多为良性无需恐慌",
            "女性更常见",
            "多数无症状",
            "定期检查为主",
        ]
    }

    /// 随访观察
    pub fn followup(&self) -> Vec<&'static str> {
        vec![
            "按医嘱定期做超声",
            "关注结节大小变化",
            "必要时做细针穿刺",
            "甲功定期检测",
        ]
    }

    /// 生活调理
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec!["情绪稳定减少压力", "规律作息", "均衡碘摄入", "避免辐射暴露"]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "结节迅速增大就医",
            "声音嘶哑吞咽困难就诊",
            "超声提示可疑须评估",
            "遵医嘱决定处理方式",
        ]
    }
}

impl Rule for ThyroidNoduleCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("thyroid_nodule_care")
    }

    fn explain(&self) -> String {
        format!(
            "【甲状腺结节管理】\n{}",
            [
                format!(
                    "认识结节：\\n{}",
                    self.understanding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "随访观察：\\n{}",
                    self.followup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活调理：\\n{}",
                    self.lifestyle()
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
    fn test_thyroidnodulecarerules_basic() {
        let rules = ThyroidNoduleCareRules::new();
        assert_eq!(rules.metadata().name, "甲状腺结节管理");
        assert!(!rules.understanding().is_empty());
        assert!(!rules.followup().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_thyroidnodulecarerules_validation() {
        let rules = ThyroidNoduleCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("thyroid_nodule_care")
        );
    }

    #[test]
    fn test_thyroidnodulecarerules_explain() {
        let rules = ThyroidNoduleCareRules::new();
        let e = rules.explain();
        assert!(e.contains("认识结节"));
        assert!(e.contains("随访观察"));
        assert!(e.contains("生活调理"));
    }
}
