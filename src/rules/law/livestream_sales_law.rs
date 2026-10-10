//! 直播带货法律规则
//!
//! 网络直播带货的法律规则，包括主播与商家责任、商品真实宣传、消费者权益与售后保障，规范直播营销行为。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LivestreamSalesLawRules,
    name: "直播带货法律规则",
    desc: "网络直播带货的法律规则，包括主播与商家责任、商品真实宣传、消费者权益与售后保障，规范直播营销行为。",
    origin: "网络交易与直播营销相关监管规定",
    tags: ["法律", "电商", "直播", "消费"]
}

impl LivestreamSalesLawRules {
    /// 主体责任
    pub fn liability(&self) -> Vec<&'static str> {
        vec![
            "主播须对推荐商品负责",
            "商家承担产品质量责任",
            "平台履行审核与管理义务",
            "虚假宣传须承担相应责任",
        ]
    }

    /// 宣传规范
    pub fn advertising(&self) -> Vec<&'static str> {
        vec![
            "不得夸大或虚假宣传功效",
            "如实说明商品信息",
            "价格优惠须真实可查",
            "禁止诱导性欺骗话术",
        ]
    }

    /// 消费者权益
    pub fn consumer(&self) -> Vec<&'static str> {
        vec![
            "享有七日无理由退货权",
            "商品与宣传不符可要求退换",
            "保留订单与直播录屏证据",
            "个人信息受法律保护",
        ]
    }

    /// 售后与维权
    pub fn after_sales(&self) -> Vec<&'static str> {
        vec![
            "明确售后与退换渠道",
            "平台须协助处理纠纷",
            "可向监管部门投诉举报",
            "损失可依法索赔",
        ]
    }
}

impl Rule for LivestreamSalesLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("livestream_sales")
    }

    fn explain(&self) -> String {
        format!(
            "【直播带货法律规则】\n{}",
            [
                format!(
                    "主体责任：\\n{}",
                    self.liability()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "宣传规范：\\n{}",
                    self.advertising()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "消费者权益：\\n{}",
                    self.consumer()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "售后与维权：\\n{}",
                    self.after_sales()
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
    fn test_livestreamsaleslawrules_basic() {
        let rules = LivestreamSalesLawRules::new();
        assert_eq!(rules.metadata().name, "直播带货法律规则");
        assert!(!rules.liability().is_empty());
        assert!(!rules.advertising().is_empty());
        assert!(!rules.consumer().is_empty());
        assert!(!rules.after_sales().is_empty());
    }

    #[test]
    fn test_livestreamsaleslawrules_validation() {
        let rules = LivestreamSalesLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("livestream_sales"));
    }

    #[test]
    fn test_livestreamsaleslawrules_explain() {
        let rules = LivestreamSalesLawRules::new();
        let e = rules.explain();
        assert!(e.contains("主体责任"));
        assert!(e.contains("宣传规范"));
        assert!(e.contains("消费者权益"));
    }
}
