//! 保险理赔基础
//!
//! 购买保险与主张理赔时应知晓的诚信与程序要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InsuranceClaimsBasicsRules,
    name: "保险理赔基础",
    desc: "购买保险与主张理赔时应知晓的诚信与程序要点",
    origin: "国际",
    tags: ["法律", "保险", "理赔", "合同", "权益"]
}

impl InsuranceClaimsBasicsRules {
    /// 投保如实
    pub fn honesty(&self) -> Vec<&'static str> {
        vec![
            "投保时如实告知健康与财产状况",
            "不隐瞒重要事项",
            "仔细阅读保险条款",
            "确认保障范围与免责",
        ]
    }

    /// 风险管理
    pub fn risk(&self) -> Vec<&'static str> {
        vec![
            "按约定履行抵御风险义务",
            "发生风险及时止损",
            "配合保险公司调查",
            "不可故意制造事故骗保",
        ]
    }

    /// 理赔程序
    pub fn claim(&self) -> Vec<&'static str> {
        vec![
            "事故后按要求报备",
            "提交完整材料与凭证",
            "确认理赔时效",
            "对拒赔有异议可复核",
        ]
    }

    /// 依法维权
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "理赔争议先协商",
            "可向监管反映或仲裁诉讼",
            "警惕以逐利相诱的违规行为",
            "合理主张合法权益",
        ]
    }
}

impl Rule for InsuranceClaimsBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("insurance_claims")
    }

    fn explain(&self) -> String {
        format!(
            "【保险理赔基础】\n{}",
            [
                format!(
                    "投保如实：\\n{}",
                    self.honesty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "风险管理：\\n{}",
                    self.risk()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "理赔程序：\\n{}",
                    self.claim()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "依法维权：\\n{}",
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
    fn test_insuranceclaimsbasicsrules_basic() {
        let rules = InsuranceClaimsBasicsRules::new();
        assert_eq!(rules.metadata().name, "保险理赔基础");
        assert!(!rules.honesty().is_empty());
        assert!(!rules.risk().is_empty());
        assert!(!rules.claim().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_insuranceclaimsbasicsrules_validation() {
        let rules = InsuranceClaimsBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("insurance_claims"));
    }

    #[test]
    fn test_insuranceclaimsbasicsrules_explain() {
        let rules = InsuranceClaimsBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("投保如实"));
        assert!(e.contains("风险管理"));
        assert!(e.contains("理赔程序"));
    }
}
