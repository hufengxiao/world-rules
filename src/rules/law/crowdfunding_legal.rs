//! 众筹法律规则
//!
//! 网络众筹活动的法律规则，包括平台资质、信息披露、资金用途监督与投资者权益保护，规范众筹各方权利义务。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CrowdfundingLegalRules,
    name: "众筹法律规则",
    desc: "网络众筹活动的法律规则，包括平台资质、信息披露、资金用途监督与投资者权益保护，规范众筹各方权利义务。",
    origin: "互联网金融与证券监管相关法规",
    tags: ["法律", "金融", "众筹", "投资"]
}

impl CrowdfundingLegalRules {
    /// 平台资质
    pub fn platform(&self) -> Vec<&'static str> {
        vec![
            "平台须具备合法经营资质",
            "履行项目审核义务",
            "建立风险提示机制",
            "妥善保管投资者资金",
        ]
    }

    /// 信息披露
    pub fn disclosure(&self) -> Vec<&'static str> {
        vec![
            "发起人须如实披露项目信息",
            "不得虚假宣传或夸大收益",
            "明确资金用途与风险",
            "及时公布进展与结果",
        ]
    }

    /// 资金管理
    pub fn funding(&self) -> Vec<&'static str> {
        vec![
            "募集资金专款专用",
            "达不成目标须按约退款",
            "变更用途须经同意",
            "接受监督与审计",
        ]
    }

    /// 权益与救济
    pub fn rights(&self) -> Vec<&'static str> {
        vec![
            "投资者享有知情权",
            "遇欺诈可依法索赔",
            "涉嫌非法集资依法处理",
            "纠纷可投诉或诉讼",
        ]
    }
}

impl Rule for CrowdfundingLegalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("crowdfunding")
    }

    fn explain(&self) -> String {
        format!(
            "【众筹法律规则】\n{}",
            [
                format!(
                    "平台资质：\\n{}",
                    self.platform()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "信息披露：\\n{}",
                    self.disclosure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "资金管理：\\n{}",
                    self.funding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "权益与救济：\\n{}",
                    self.rights()
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
    fn test_crowdfundinglegalrules_basic() {
        let rules = CrowdfundingLegalRules::new();
        assert_eq!(rules.metadata().name, "众筹法律规则");
        assert!(!rules.platform().is_empty());
        assert!(!rules.disclosure().is_empty());
        assert!(!rules.funding().is_empty());
        assert!(!rules.rights().is_empty());
    }

    #[test]
    fn test_crowdfundinglegalrules_validation() {
        let rules = CrowdfundingLegalRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("crowdfunding"));
    }

    #[test]
    fn test_crowdfundinglegalrules_explain() {
        let rules = CrowdfundingLegalRules::new();
        let e = rules.explain();
        assert!(e.contains("平台资质"));
        assert!(e.contains("信息披露"));
        assert!(e.contains("资金管理"));
    }
}
