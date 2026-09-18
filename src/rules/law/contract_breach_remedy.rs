//! 合同违约与救济
//!
//! 一方不履行合同时对方可主张的权利与救济方式

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ContractBreachRemedyRules,
    name: "合同违约与救济",
    desc: "一方不履行合同时对方可主张的权利与救济方式",
    origin: "中国",
    tags: ["法律", "合同", "违约", "救济"]
}

impl ContractBreachRemedyRules {
    /// 违约认定
    pub fn identify(&self) -> Vec<&'static str> {
        vec!["未按期履行", "履行不符合约定", "拒绝履行", "迟延履行"]
    }

    /// 主张权利
    pub fn rights(&self) -> Vec<&'static str> {
        vec!["要求继续履行", "请求赔偿损失", "退换或减价", "解除合同"]
    }

    /// 违约责任
    pub fn liability(&self) -> Vec<&'static str> {
        vec![
            "按约定担责",
            "赔偿实际损失",
            "违约金另有约定",
            "过错程度考量",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec!["协商解决", "投诉调解", "仲裁或诉讼", "保留证据举证"]
    }
}

impl Rule for ContractBreachRemedyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("contract_breach")
    }

    fn explain(&self) -> String {
        format!(
            "【合同违约与救济】\n{}",
            [
                format!(
                    "违约认定：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "主张权利：\\n{}",
                    self.rights()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "违约责任：\\n{}",
                    self.liability()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权途径：\\n{}",
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
    fn test_contractbreachremedyrules_basic() {
        let rules = ContractBreachRemedyRules::new();
        assert_eq!(rules.metadata().name, "合同违约与救济");
        assert!(!rules.identify().is_empty());
        assert!(!rules.rights().is_empty());
        assert!(!rules.liability().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_contractbreachremedyrules_validation() {
        let rules = ContractBreachRemedyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("contract_breach"));
    }

    #[test]
    fn test_contractbreachremedyrules_explain() {
        let rules = ContractBreachRemedyRules::new();
        let e = rules.explain();
        assert!(e.contains("违约认定"));
        assert!(e.contains("主张权利"));
        assert!(e.contains("违约责任"));
    }
}
