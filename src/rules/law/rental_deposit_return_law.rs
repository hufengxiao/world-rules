//! 租房押金退还
//!
//! 退租时押金退还的条件与争议处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RentalDepositReturnLawRules,
    name: "租房押金退还",
    desc: "退租时押金退还的条件与争议处理",
    origin: "中国",
    tags: ["法律", "租房", "押金"]
}

impl RentalDepositReturnLawRules {
    /// 退还条件
    pub fn conditions(&self) -> Vec<&'static str> {
        vec![
            "租期届满无违约应退",
            "房屋及设施无损坏",
            "水电费用已结清",
            "按合同约定办理",
        ]
    }

    /// 合理扣除
    pub fn deduction(&self) -> Vec<&'static str> {
        vec![
            "拖欠租金可扣除",
            "人为损坏可扣除维修费",
            "欠缴费用可抵扣",
            "扣除须有依据",
        ]
    }

    /// 退租流程
    pub fn process(&self) -> Vec<&'static str> {
        vec![
            "提前通知房东退租",
            "共同查验房屋状况",
            "结清水电物业费",
            "索要押金退还凭证",
        ]
    }

    /// 争议处理
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "无理拒退可协商",
            "保留合同与付款凭证",
            "向住建部门或消协投诉",
            "可提起诉讼索还",
        ]
    }
}

impl Rule for RentalDepositReturnLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("rental_deposit_return_law")
    }

    fn explain(&self) -> String {
        format!(
            "【租房押金退还】\n{}",
            [
                format!(
                    "退还条件：\\n{}",
                    self.conditions()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合理扣除：\\n{}",
                    self.deduction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退租流程：\\n{}",
                    self.process()
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
    fn test_rentaldepositreturnlawrules_basic() {
        let rules = RentalDepositReturnLawRules::new();
        assert_eq!(rules.metadata().name, "租房押金退还");
        assert!(!rules.conditions().is_empty());
        assert!(!rules.deduction().is_empty());
        assert!(!rules.process().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_rentaldepositreturnlawrules_validation() {
        let rules = RentalDepositReturnLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::law("rental_deposit_return_law")
        );
    }

    #[test]
    fn test_rentaldepositreturnlawrules_explain() {
        let rules = RentalDepositReturnLawRules::new();
        let e = rules.explain();
        assert!(e.contains("退还条件"));
        assert!(e.contains("合理扣除"));
        assert!(e.contains("退租流程"));
    }
}
