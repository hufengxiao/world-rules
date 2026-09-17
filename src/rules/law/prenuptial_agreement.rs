//! 婚前财产约定
//!
//! 婚前财产协议的内容、必要性与效力

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PrenuptialAgreementRules,
    name: "婚前财产约定",
    desc: "婚前财产协议的内容、必要性与效力",
    origin: "中国",
    tags: ["法律", "婚前财产", "协议"]
}

impl PrenuptialAgreementRules {
    /// 协议范围
    pub fn scope(&self) -> Vec<&'static str> {
        vec![
            "约定婚前财产归属",
            "约定婚后所得归属",
            "可约定债权债务",
            "内容合法不违背公序",
        ]
    }

    /// 形式要求
    pub fn form(&self) -> Vec<&'static str> {
        vec!["采用书面形式", "双方真实自愿", "可办理公证", "内容明确具体"]
    }

    /// 开诚协商
    pub fn discuss(&self) -> Vec<&'static str> {
        vec![
            "婚前坦诚沟通",
            "不损害他人利益",
            "兼顾情感与保护",
            "获得共同认可",
        ]
    }

    /// 执行注意
    pub fn perform(&self) -> Vec<&'static str> {
        vec![
            "按约定履行",
            "涉及房产依法变更",
            "离婚时依约分割",
            "争议依法认定",
        ]
    }
}

impl Rule for PrenuptialAgreementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("prenuptial")
    }

    fn explain(&self) -> String {
        format!(
            "【婚前财产约定】\n{}",
            [
                format!(
                    "协议范围：\\n{}",
                    self.scope()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "形式要求：\\n{}",
                    self.form()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "开诚协商：\\n{}",
                    self.discuss()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "执行注意：\\n{}",
                    self.perform()
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
    fn test_prenuptialagreementrules_basic() {
        let rules = PrenuptialAgreementRules::new();
        assert_eq!(rules.metadata().name, "婚前财产约定");
        assert!(!rules.scope().is_empty());
        assert!(!rules.form().is_empty());
        assert!(!rules.discuss().is_empty());
        assert!(!rules.perform().is_empty());
    }

    #[test]
    fn test_prenuptialagreementrules_validation() {
        let rules = PrenuptialAgreementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("prenuptial"));
    }

    #[test]
    fn test_prenuptialagreementrules_explain() {
        let rules = PrenuptialAgreementRules::new();
        let e = rules.explain();
        assert!(e.contains("协议范围"));
        assert!(e.contains("形式要求"));
        assert!(e.contains("开诚协商"));
    }
}
