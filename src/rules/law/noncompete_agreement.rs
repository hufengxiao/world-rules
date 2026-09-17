//! 竞业限制条款
//!
//! 劳动合同竞业限制的义务、期限与补偿要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NoncompeteAgreementRules,
    name: "竞业限制条款",
    desc: "劳动合同竞业限制的义务、期限与补偿要点",
    origin: "中国",
    tags: ["法律", "竞业限制", "保密"]
}

impl NoncompeteAgreementRules {
    /// 约定范围
    pub fn scope(&self) -> Vec<&'static str> {
        vec![
            "竞业限制书面约定",
            "范围限于同类竞争",
            "期限约定在法定上限内",
            "针对确实涉及机密岗位",
        ]
    }

    /// 竞业义务
    pub fn duty(&self) -> Vec<&'static str> {
        vec![
            "离职后遵守竞业约定",
            "不从事竞争业务",
            "不挖角泄露客户",
            "遵守期间诚信",
        ]
    }

    /// 经济补偿
    pub fn compensation(&self) -> Vec<&'static str> {
        vec![
            "竞业限制应有补偿",
            "补偿按月支付",
            "未依约付偿可主张",
            "不了解补偿先协商",
        ]
    }

    /// 争议处理
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "有争议先看是否适用",
            "依约依规处理",
            "泄密致损或担责",
            "保留协议与记录",
        ]
    }
}

impl Rule for NoncompeteAgreementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("noncompete")
    }

    fn explain(&self) -> String {
        format!(
            "【竞业限制条款】\n{}",
            [
                format!(
                    "约定范围：\\n{}",
                    self.scope()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "竞业义务：\\n{}",
                    self.duty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "经济补偿：\\n{}",
                    self.compensation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "争议处理：\\n{}",
                    self.remedy()
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
    fn test_noncompeteagreementrules_basic() {
        let rules = NoncompeteAgreementRules::new();
        assert_eq!(rules.metadata().name, "竞业限制条款");
        assert!(!rules.scope().is_empty());
        assert!(!rules.duty().is_empty());
        assert!(!rules.compensation().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_noncompeteagreementrules_validation() {
        let rules = NoncompeteAgreementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("noncompete"));
    }

    #[test]
    fn test_noncompeteagreementrules_explain() {
        let rules = NoncompeteAgreementRules::new();
        let e = rules.explain();
        assert!(e.contains("约定范围"));
        assert!(e.contains("竞业义务"));
        assert!(e.contains("经济补偿"));
    }
}
