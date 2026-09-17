//! 房贷注意事项
//!
//! 申请房贷、审阅条款与还款履约的要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MortgageLoanBasicsRules,
    name: "房贷注意事项",
    desc: "申请房贷、审阅条款与还款履约的要点",
    origin: "中国",
    tags: ["法律", "房贷", "贷款", "合同"]
}

impl MortgageLoanBasicsRules {
    /// 贷前准备
    pub fn prepare(&self) -> Vec<&'static str> {
        vec![
            "评估自身还款能力",
            "比较多家贷款方案",
            "了解利率与期限",
            "预留备用资金",
        ]
    }

    /// 条款审阅
    pub fn terms(&self) -> Vec<&'static str> {
        vec![
            "细读贷款与担保条款",
            "确认利率计息方式",
            "留意提前还款规定",
            "看清违约金与费用",
        ]
    }

    /// 按时履约
    pub fn repay(&self) -> Vec<&'static str> {
        vec![
            "按时足额偿还月供",
            "收入变化及时沟通",
            "保留还款凭证",
            "不消费超负荷",
        ]
    }

    /// 风险应对
    pub fn risk(&self) -> Vec<&'static str> {
        vec![
            "遇还款困难主动协商",
            "不逃避逾期债务",
            "警惕中介乱收费",
            "依法依规处理抵押物",
        ]
    }
}

impl Rule for MortgageLoanBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("mortgage_loan")
    }

    fn explain(&self) -> String {
        format!(
            "【房贷注意事项】\n{}",
            [
                format!(
                    "贷前准备：\\n{}",
                    self.prepare()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "条款审阅：\\n{}",
                    self.terms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "按时履约：\\n{}",
                    self.repay()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "风险应对：\\n{}",
                    self.risk()
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
    fn test_mortgageloanbasicsrules_basic() {
        let rules = MortgageLoanBasicsRules::new();
        assert_eq!(rules.metadata().name, "房贷注意事项");
        assert!(!rules.prepare().is_empty());
        assert!(!rules.terms().is_empty());
        assert!(!rules.repay().is_empty());
        assert!(!rules.risk().is_empty());
    }

    #[test]
    fn test_mortgageloanbasicsrules_validation() {
        let rules = MortgageLoanBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("mortgage_loan"));
    }

    #[test]
    fn test_mortgageloanbasicsrules_explain() {
        let rules = MortgageLoanBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("贷前准备"));
        assert!(e.contains("条款审阅"));
        assert!(e.contains("按时履约"));
    }
}
