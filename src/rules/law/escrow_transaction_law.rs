//! 资金监管（第三方托管）
//!
//! 二手房买卖等大额交易中资金监管的法律规则，包括监管协议、资金划转条件、风险防范与争议处理，保障交易双方资金安全。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EscrowTransactionLawRules,
    name: "资金监管（第三方托管）",
    desc: "二手房买卖等大额交易中资金监管的法律规则，包括监管协议、资金划转条件、风险防范与争议处理，保障交易双方资金安全。",
    origin: "房地产交易与金融监管相关法律规定",
    tags: ["法律", "交易", "资金监管", "房地产"]
}

impl EscrowTransactionLawRules {
    /// 监管协议
    pub fn agreement(&self) -> Vec<&'static str> {
        vec![
            "交易双方与监管方签订书面协议",
            "明确监管金额与用途",
            "约定划转条件与时限",
            "约定违约责任",
        ]
    }

    /// 资金划转
    pub fn transfer(&self) -> Vec<&'static str> {
        vec![
            "过户完成后方可划转房款",
            "按约定节点分批划付",
            "买方违约时资金退还规则",
            "监管方须按指令操作",
        ]
    }

    /// 风险防范
    pub fn risk(&self) -> Vec<&'static str> {
        vec![
            "选择正规银行或监管机构",
            "核实监管账户真实有效",
            "警惕私下划款要求",
            "留存全部交易凭证",
        ]
    }

    /// 争议处理
    pub fn dispute(&self) -> Vec<&'static str> {
        vec![
            "划转争议按协议协商",
            "协商不成可诉诸法院",
            "监管方失职可追责",
            "保全证据便于维权",
        ]
    }
}

impl Rule for EscrowTransactionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("escrow")
    }

    fn explain(&self) -> String {
        format!(
            "【资金监管（第三方托管）】\n{}",
            [
                format!(
                    "监管协议：\\n{}",
                    self.agreement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "资金划转：\\n{}",
                    self.transfer()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "风险防范：\\n{}",
                    self.risk()
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
    fn test_escrowtransactionlawrules_basic() {
        let rules = EscrowTransactionLawRules::new();
        assert_eq!(rules.metadata().name, "资金监管（第三方托管）");
        assert!(!rules.agreement().is_empty());
        assert!(!rules.transfer().is_empty());
        assert!(!rules.risk().is_empty());
        assert!(!rules.dispute().is_empty());
    }

    #[test]
    fn test_escrowtransactionlawrules_validation() {
        let rules = EscrowTransactionLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("escrow"));
    }

    #[test]
    fn test_escrowtransactionlawrules_explain() {
        let rules = EscrowTransactionLawRules::new();
        let e = rules.explain();
        assert!(e.contains("监管协议"));
        assert!(e.contains("资金划转"));
        assert!(e.contains("风险防范"));
    }
}
