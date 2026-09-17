//! 网购消费者权利
//!
//! 网购下单、退换货、维权时的消费者权利要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OnlineShoppingRightsRules,
    name: "网购消费者权利",
    desc: "网购下单、退换货、维权时的消费者权利要点",
    origin: "中国",
    tags: ["法律", "网购", "消费者", "退换货"]
}

impl OnlineShoppingRightsRules {
    /// 下单明辨
    pub fn ordering(&self) -> Vec<&'static str> {
        vec![
            "核对商品描述与价格",
            "看清运费与促销规则",
            "留意商家资质与评价",
            "保留订单与聊天记录",
        ]
    }

    /// 七日无理由
    pub fn returns(&self) -> Vec<&'static str> {
        vec![
            "了解七日无理由退货",
            "不影响二次销售可退",
            "特定商品另有规定",
            "退货运费依约定",
        ]
    }

    /// 质量保障
    pub fn quality(&self) -> Vec<&'static str> {
        vec![
            "商品与描述不符可主张",
            "依三包享受保修",
            "虚假宣传可投诉",
            "注意开箱验收核对凭证",
        ]
    }

    /// 维权渠道
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "先与商家协商",
            "协商不成向平台投诉",
            "可向消协或监管反映",
            "严重情况依法处理",
        ]
    }
}

impl Rule for OnlineShoppingRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("online_shopping")
    }

    fn explain(&self) -> String {
        format!(
            "【网购消费者权利】\n{}",
            [
                format!(
                    "下单明辨：\\n{}",
                    self.ordering()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "七日无理由：\\n{}",
                    self.returns()
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
                    "维权渠道：\\n{}",
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
    fn test_onlineshoppingrightsrules_basic() {
        let rules = OnlineShoppingRightsRules::new();
        assert_eq!(rules.metadata().name, "网购消费者权利");
        assert!(!rules.ordering().is_empty());
        assert!(!rules.returns().is_empty());
        assert!(!rules.quality().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_onlineshoppingrightsrules_validation() {
        let rules = OnlineShoppingRightsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("online_shopping"));
    }

    #[test]
    fn test_onlineshoppingrightsrules_explain() {
        let rules = OnlineShoppingRightsRules::new();
        let e = rules.explain();
        assert!(e.contains("下单明辨"));
        assert!(e.contains("七日无理由"));
        assert!(e.contains("质量保障"));
    }
}
