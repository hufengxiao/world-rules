//! 外卖取送相处礼仪
//!
//! 与外卖骑手互动时的礼节规则，包括地址清晰、及时接听、态度尊重与合理评价，营造互相体谅的配送体验。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FoodDeliveryCourierMannerRules,
    name: "外卖取送相处礼仪",
    desc: "与外卖骑手互动时的礼节规则，包括地址清晰、及时接听、态度尊重与合理评价，营造互相体谅的配送体验。",
    origin: "城市生活中外卖配送高频互动场景",
    tags: ["社交", "生活", "服务", "礼仪"]
}

impl FoodDeliveryCourierMannerRules {
    /// 下单配合
    pub fn ordering(&self) -> Vec<&'static str> {
        vec![
            "地址填写准确到门牌",
            "备注必要的门禁信息",
            "合理选择配送时间",
            "不频繁更改已派单订单",
        ]
    }

    /// 接单沟通
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "保持电话畅通及时接听",
            "语气平和说明需求",
            "骑手迟到先问原因",
            "不无端催促与指责",
        ]
    }

    /// 取餐礼节
    pub fn pickup(&self) -> Vec<&'static str> {
        vec![
            "提前到约定地点等候",
            "当面道谢接取餐品",
            "雨雪天多一分体谅",
            "小费或好评表达感谢",
        ]
    }

    /// 纠纷处理
    pub fn dispute(&self) -> Vec<&'static str> {
        vec![
            "餐品问题先联系平台",
            "客观评价不人身攻击",
            "无理要求不强行提出",
            "遇纠纷走正规投诉渠道",
        ]
    }
}

impl Rule for FoodDeliveryCourierMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("delivery")
    }

    fn explain(&self) -> String {
        format!(
            "【外卖取送相处礼仪】\n{}",
            [
                format!(
                    "下单配合：\\n{}",
                    self.ordering()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "接单沟通：\\n{}",
                    self.communication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "取餐礼节：\\n{}",
                    self.pickup()
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
    fn test_fooddeliverycouriermannerrules_basic() {
        let rules = FoodDeliveryCourierMannerRules::new();
        assert_eq!(rules.metadata().name, "外卖取送相处礼仪");
        assert!(!rules.ordering().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.pickup().is_empty());
        assert!(!rules.dispute().is_empty());
    }

    #[test]
    fn test_fooddeliverycouriermannerrules_validation() {
        let rules = FoodDeliveryCourierMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("delivery"));
    }

    #[test]
    fn test_fooddeliverycouriermannerrules_explain() {
        let rules = FoodDeliveryCourierMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("下单配合"));
        assert!(e.contains("接单沟通"));
        assert!(e.contains("取餐礼节"));
    }
}
