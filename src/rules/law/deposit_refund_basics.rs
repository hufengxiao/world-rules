//! 押金退还要点
//!
//! 租房押金归还、扣款争议与退还的要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DepositRefundBasicsRules,
    name: "押金退还要点",
    desc: "租房押金归还、扣款争议与退还的要点",
    origin: "中国",
    tags: ["法律", "押金", "租房", "退还"]
}

impl DepositRefundBasicsRules {
    /// 押金约定
    pub fn deposit(&self) -> Vec<&'static str> {
        vec![
            "签约明确押金数额",
            "注明退押条件与时限",
            "保留押金凭证",
            "金额符合约定",
        ]
    }

    /// 退租交代
    pub fn moveout(&self) -> Vec<&'static str> {
        vec![
            "按约清理房屋物品",
            "固定设施损坏如实说明",
            "结清水电物业费用",
            "做好房屋交接",
        ]
    }

    /// 退还主张
    pub fn refund(&self) -> Vec<&'static str> {
        vec![
            "符合条件要求退还",
            "无合理扣减应全额退",
            "保留沟通与凭证",
            "不无谓拖延",
        ]
    }

    /// 争议处理
    pub fn dispute(&self) -> Vec<&'static str> {
        vec![
            "先协商解决扣减争议",
            "约定不明依交易习惯",
            "有依据可投诉或依法主张",
            "贵在依法依约处理押金",
        ]
    }
}

impl Rule for DepositRefundBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("deposit_refund")
    }

    fn explain(&self) -> String {
        format!(
            "【押金退还要点】\n{}",
            [
                format!(
                    "押金约定：\\n{}",
                    self.deposit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退租交代：\\n{}",
                    self.moveout()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退还主张：\\n{}",
                    self.refund()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "争议处理：\\n{}",
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
    fn test_depositrefundbasicsrules_basic() {
        let rules = DepositRefundBasicsRules::new();
        assert_eq!(rules.metadata().name, "押金退还要点");
        assert!(!rules.deposit().is_empty());
        assert!(!rules.moveout().is_empty());
        assert!(!rules.refund().is_empty());
        assert!(!rules.dispute().is_empty());
    }

    #[test]
    fn test_depositrefundbasicsrules_validation() {
        let rules = DepositRefundBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("deposit_refund"));
    }

    #[test]
    fn test_depositrefundbasicsrules_explain() {
        let rules = DepositRefundBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("押金约定"));
        assert!(e.contains("退租交代"));
        assert!(e.contains("退还主张"));
    }
}
