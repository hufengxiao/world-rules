//! 消费者权益保护
//!
//! 消费者购物、退换货、维权时应当知晓的权益基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ConsumerRightsBasicsRules,
    name: "消费者权益保护",
    desc: "消费者购物、退换货、维权时应当知晓的权益基础规则",
    origin: "中国",
    tags: ["法律", "消费者", "权益", "维权", "合同"]
}

impl ConsumerRightsBasicsRules {
    /// 知悉权与自主权
    pub fn knowing(&self) -> Vec<&'static str> {
        vec![
            "有权了解商品真实信息",
            "自愿公平购买不被迫消费",
            "索要并保存购物凭证",
            "不盲目签不平等的格式条款",
        ]
    }

    /// 质量保障
    pub fn quality(&self) -> Vec<&'static str> {
        vec![
            "商品应符合质量承诺",
            "明确三包与保修范围",
            "发现问题及时在期限内反馈",
            "保存宣传页与订单记录",
        ]
    }

    /// 退换与售后
    pub fn return_policy(&self) -> Vec<&'static str> {
        vec![
            "依规定行使退换货权利",
            "新业态(网购)依消法七日退",
            "保留开箱、售后过程证据",
            "超保留期限说明沟通",
        ]
    }

    /// 维权途径
    pub fn resolution(&self) -> Vec<&'static str> {
        vec![
            "先与商家协商解决",
            "协商不成可向消协或投诉",
            "依合同或法律主张",
            "情形严重保留法律追诉权",
        ]
    }
}

impl Rule for ConsumerRightsBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("consumer_rights")
    }

    fn explain(&self) -> String {
        format!(
            "【消费者权益保护】\n{}",
            [
                format!(
                    "知悉权与自主权：\\n{}",
                    self.knowing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "质量保障：\\n{}",
                    self.quality()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退换与售后：\\n{}",
                    self.return_policy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权途径：\\n{}",
                    self.resolution()
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
    fn test_consumerrightsbasicsrules_basic() {
        let rules = ConsumerRightsBasicsRules::new();
        assert_eq!(rules.metadata().name, "消费者权益保护");
        assert!(!rules.knowing().is_empty());
        assert!(!rules.quality().is_empty());
        assert!(!rules.return_policy().is_empty());
        assert!(!rules.resolution().is_empty());
    }

    #[test]
    fn test_consumerrightsbasicsrules_validation() {
        let rules = ConsumerRightsBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("consumer_rights"));
    }

    #[test]
    fn test_consumerrightsbasicsrules_explain() {
        let rules = ConsumerRightsBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("知悉权与自主权"));
        assert!(e.contains("质量保障"));
        assert!(e.contains("退换与售后"));
    }
}
