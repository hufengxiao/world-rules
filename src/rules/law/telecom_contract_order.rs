//! 通讯套餐合约
//!
//! 手机套餐更名、订购与契约变动要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TelecomContractOrderRules,
    name: "通讯套餐合约",
    desc: "手机套餐更名、订购与契约变动要点",
    origin: "中国",
    tags: ["消费", "通信", "套餐", "合约"]
}

impl TelecomContractOrderRules {
    /// 办理看清
    pub fn read(&self) -> Vec<&'static str> {
        vec!["办套餐看合同", "清楚费用与期限", "不轻信推荐", "问清续期"]
    }

    /// 合约期
    pub fn term(&self) -> Vec<&'static str> {
        vec!["明悉合约期", "到期可变更", "提前解约有规定", "违约费了解"]
    }

    /// 变更退订
    pub fn change(&self) -> Vec<&'static str> {
        vec!["需变更联系客服", "保留办理凭证", "退订按流程", "客服有据"]
    }

    /// 争议维权
    pub fn remedy(&self) -> Vec<&'static str> {
        vec!["乱扣费可投诉", "向平台监管反映", "依法主张", "理性维权"]
    }
}

impl Rule for TelecomContractOrderRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("telecom_contract")
    }

    fn explain(&self) -> String {
        format!(
            "【通讯套餐合约】\n{}",
            [
                format!(
                    "办理看清：\\n{}",
                    self.read()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合约期：\\n{}",
                    self.term()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "变更退订：\\n{}",
                    self.change()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "争议维权：\\n{}",
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
    fn test_telecomcontractorderrules_basic() {
        let rules = TelecomContractOrderRules::new();
        assert_eq!(rules.metadata().name, "通讯套餐合约");
        assert!(!rules.read().is_empty());
        assert!(!rules.term().is_empty());
        assert!(!rules.change().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_telecomcontractorderrules_validation() {
        let rules = TelecomContractOrderRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("telecom_contract"));
    }

    #[test]
    fn test_telecomcontractorderrules_explain() {
        let rules = TelecomContractOrderRules::new();
        let e = rules.explain();
        assert!(e.contains("办理看清"));
        assert!(e.contains("合约期"));
        assert!(e.contains("变更退订"));
    }
}
