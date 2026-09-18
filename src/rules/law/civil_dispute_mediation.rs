//! 民事纠纷调解
//!
//! 通过协商、人民调解解决民事纠纷的途径与要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CivilDisputeMediationRules,
    name: "民事纠纷调解",
    desc: "通过协商、人民调解解决民事纠纷的途径与要点",
    origin: "中国",
    tags: ["法律", "纠纷", "调解", "协商"]
}

impl CivilDisputeMediationRules {
    /// 调解优先
    pub fn first(&self) -> Vec<&'static str> {
        vec!["先友好协商", "讲事实摆证据", "互谅互让", "低成本化矛盾"]
    }

    /// 调解主体
    pub fn subject(&self) -> Vec<&'static str> {
        vec!["人民调解委员会", "居委会村委会", "仲裁调解", "法院诉前调解"]
    }

    /// 协议效力
    pub fn effect(&self) -> Vec<&'static str> {
        vec![
            "达成书面调解协议",
            "双方自愿签字",
            "有约束力履行",
            "可申请司法确认",
        ]
    }

    /// 不成再诉
    pub fn litigation(&self) -> Vec<&'static str> {
        vec![
            "调解不成可起诉",
            "保留证据",
            "诉讼时效注意",
            "按管辖法院起诉",
        ]
    }
}

impl Rule for CivilDisputeMediationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_mediation")
    }

    fn explain(&self) -> String {
        format!(
            "【民事纠纷调解】\n{}",
            [
                format!(
                    "调解优先：\\n{}",
                    self.first()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "调解主体：\\n{}",
                    self.subject()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "协议效力：\\n{}",
                    self.effect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "不成再诉：\\n{}",
                    self.litigation()
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
    fn test_civildisputemediationrules_basic() {
        let rules = CivilDisputeMediationRules::new();
        assert_eq!(rules.metadata().name, "民事纠纷调解");
        assert!(!rules.first().is_empty());
        assert!(!rules.subject().is_empty());
        assert!(!rules.effect().is_empty());
        assert!(!rules.litigation().is_empty());
    }

    #[test]
    fn test_civildisputemediationrules_validation() {
        let rules = CivilDisputeMediationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("civil_mediation"));
    }

    #[test]
    fn test_civildisputemediationrules_explain() {
        let rules = CivilDisputeMediationRules::new();
        let e = rules.explain();
        assert!(e.contains("调解优先"));
        assert!(e.contains("调解主体"));
        assert!(e.contains("协议效力"));
    }
}
