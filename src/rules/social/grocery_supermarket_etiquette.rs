//! 超市购物礼仪
//!
//! 超市选品、购物车、结账时的公共礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GrocerySupermarketEtiquetteRules,
    name: "超市购物礼仪",
    desc: "超市选品、购物车、结账时的公共礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "超市", "购物", "结账"]
}

impl GrocerySupermarketEtiquetteRules {
    /// 选品文明
    pub fn selecting(&self) -> Vec<&'static str> {
        vec![
            "挑选后归放原位的食物",
            "不挤压弄碎货品",
            "不随意拆封品尝",
            "放回不买注意卫生",
        ]
    }

    /// 购物车空间
    pub fn cart(&self) -> Vec<&'static str> {
        vec![
            "靠边行走不挡通道",
            "不推车去挤窄道",
            "儿童坐车注意安全",
            "放回推车于指定处",
        ]
    }

    /// 排队结账
    pub fn checkout(&self) -> Vec<&'static str> {
        vec![
            "有序排队结账",
            "轮到再上前",
            "准备付款顺畅",
            "不大声争执插队",
        ]
    }

    /// 与员工相处
    pub fn staff(&self) -> Vec<&'static str> {
        vec![
            "询问客服礼貌客气",
            "对补货员工让步",
            "尊重收银与工作人员",
            "有异议友好解决",
        ]
    }
}

impl Rule for GrocerySupermarketEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("grocery")
    }

    fn explain(&self) -> String {
        format!(
            "【超市购物礼仪】\n{}",
            [
                format!(
                    "选品文明：\\n{}",
                    self.selecting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "购物车空间：\\n{}",
                    self.cart()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "排队结账：\\n{}",
                    self.checkout()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "与员工相处：\\n{}",
                    self.staff()
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
    fn test_grocerysupermarketetiquetterules_basic() {
        let rules = GrocerySupermarketEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "超市购物礼仪");
        assert!(!rules.selecting().is_empty());
        assert!(!rules.cart().is_empty());
        assert!(!rules.checkout().is_empty());
        assert!(!rules.staff().is_empty());
    }

    #[test]
    fn test_grocerysupermarketetiquetterules_validation() {
        let rules = GrocerySupermarketEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("grocery"));
    }

    #[test]
    fn test_grocerysupermarketetiquetterules_explain() {
        let rules = GrocerySupermarketEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("选品文明"));
        assert!(e.contains("购物车空间"));
        assert!(e.contains("排队结账"));
    }
}
