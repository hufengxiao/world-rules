//! 预付卡风险须知
//!
//! 办预付卡的审查、额度与维权要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PrepaidCardRiskRules,
    name: "预付卡风险须知",
    desc: "办预付卡的审查、额度与维权要点",
    origin: "中国",
    tags: ["消费", "预付卡", "储值", "风险"]
}

impl PrepaidCardRiskRules {
    /// 事前审查
    pub fn check(&self) -> Vec<&'static str> {
        vec!["了解商家信誉", "看营业执照规范", "协议写清楚", "不轻信口头"]
    }

    /// 量力充值
    pub fn amount(&self) -> Vec<&'static str> {
        vec!["充值金额宜适度", "不过度囤卡", "按需消费", "留余量"]
    }

    /// 保存凭证
    pub fn receipt(&self) -> Vec<&'static str> {
        vec!["保留充值单据", "记账余额", "发票收好", "维权依据"]
    }

    /// 纠纷处理
    pub fn dispute(&self) -> Vec<&'static str> {
        vec!["停止经营可退卡", "协商退款", "投诉市场监管", "依法维权"]
    }
}

impl Rule for PrepaidCardRiskRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("prepaid_card")
    }

    fn explain(&self) -> String {
        format!(
            "【预付卡风险须知】\n{}",
            [
                format!(
                    "事前审查：\\n{}",
                    self.check()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "量力充值：\\n{}",
                    self.amount()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保存凭证：\\n{}",
                    self.receipt()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "纠纷处理：\\n{}",
                    self.dispute()
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
    fn test_prepaidcardriskrules_basic() {
        let rules = PrepaidCardRiskRules::new();
        assert_eq!(rules.metadata().name, "预付卡风险须知");
        assert!(!rules.check().is_empty());
        assert!(!rules.amount().is_empty());
        assert!(!rules.receipt().is_empty());
        assert!(!rules.dispute().is_empty());
    }

    #[test]
    fn test_prepaidcardriskrules_validation() {
        let rules = PrepaidCardRiskRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("prepaid_card"));
    }

    #[test]
    fn test_prepaidcardriskrules_explain() {
        let rules = PrepaidCardRiskRules::new();
        let e = rules.explain();
        assert!(e.contains("事前审查"));
        assert!(e.contains("量力充值"));
        assert!(e.contains("保存凭证"));
    }
}
