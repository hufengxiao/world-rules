//! 公证办理要点
//!
//! 公证的作用、申请材料与证明使用要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NotaryPublicRules,
    name: "公证办理要点",
    desc: "公证的作用、申请材料与证明使用要点",
    origin: "中国",
    tags: ["法律", "公证", "证明"]
}

impl NotaryPublicRules {
    /// 公证作用
    pub fn benefit(&self) -> Vec<&'static str> {
        vec![
            "公证是法定证明",
            "增强文书证据力",
            "用于涉外继承等",
            "审查真实性",
        ]
    }

    /// 申请准备
    pub fn apply(&self) -> Vec<&'static str> {
        vec![
            "携带身份证明",
            "提交相关证据文件",
            "如实陈述",
            "按规定填写申请",
        ]
    }

    /// 办理流程
    pub fn process(&self) -> Vec<&'static str> {
        vec!["向公证机构申请", "配合审查核实", "领取公证书", "如需可加急"]
    }

    /// 使用注意
    pub fn usage(&self) -> Vec<&'static str> {
        vec!["公证书依法使用", "保管妥善", "不伪造冒用", "需要时申请副本"]
    }
}

impl Rule for NotaryPublicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("notary")
    }

    fn explain(&self) -> String {
        format!(
            "【公证办理要点】\n{}",
            [
                format!(
                    "公证作用：\\n{}",
                    self.benefit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "申请准备：\\n{}",
                    self.apply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "办理流程：\\n{}",
                    self.process()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "使用注意：\\n{}",
                    self.usage()
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
    fn test_notarypublicrules_basic() {
        let rules = NotaryPublicRules::new();
        assert_eq!(rules.metadata().name, "公证办理要点");
        assert!(!rules.benefit().is_empty());
        assert!(!rules.apply().is_empty());
        assert!(!rules.process().is_empty());
        assert!(!rules.usage().is_empty());
    }

    #[test]
    fn test_notarypublicrules_validation() {
        let rules = NotaryPublicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("notary"));
    }

    #[test]
    fn test_notarypublicrules_explain() {
        let rules = NotaryPublicRules::new();
        let e = rules.explain();
        assert!(e.contains("公证作用"));
        assert!(e.contains("申请准备"));
        assert!(e.contains("办理流程"));
    }
}
