//! 餐饮小费与服务礼仪
//!
//! 餐厅用餐时对小费、找零与服务的得体处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RestaurantTippingRules,
    name: "餐饮小费与服务礼仪",
    desc: "餐厅用餐时对小费、找零与服务的得体处理",
    origin: "国际",
    tags: ["社交", "礼仪", "小费", "餐饮", "服务"]
}

impl RestaurantTippingRules {
    /// 了解当地习惯
    pub fn local_custom(&self) -> Vec<&'static str> {
        vec![
            "了解当地是否采用小费制",
            "明察优秀服务可适度打赏",
            "不清楚时询问或查看账单",
            "不因不懂礼仪造成尴尬",
        ]
    }

    /// 给小费方式
    pub fn method(&self) -> Vec<&'static str> {
        vec![
            "可当面给或留在账单处",
            "按服务内容合理支付",
            "刷卡时留意附加比例",
            "以现金留小费时明确讲",
        ]
    }

    /// 对待服务生
    pub fn waiters(&self) -> Vec<&'static str> {
        vec![
            "尊重店员不呼来喝去",
            "表达感谢与善意",
            "有不满礼貌沟通而非发火",
            "体谅服务生忙碌",
        ]
    }

    /// 结账礼仪
    pub fn checkout(&self) -> Vec<&'static str> {
        vec![
            "适时买单不拖延",
            "核对账单避免多付",
            "接受找零及时答谢",
            "不懂的更规范适时询问",
        ]
    }
}

impl Rule for RestaurantTippingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("restaurant_tipping")
    }

    fn explain(&self) -> String {
        format!(
            "【餐饮小费与服务礼仪】\n{}",
            [
                format!(
                    "了解当地习惯：\\n{}",
                    self.local_custom()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "给小费方式：\\n{}",
                    self.method()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "对待服务生：\\n{}",
                    self.waiters()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结账礼仪：\\n{}",
                    self.checkout()
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
    fn test_restauranttippingrules_basic() {
        let rules = RestaurantTippingRules::new();
        assert_eq!(rules.metadata().name, "餐饮小费与服务礼仪");
        assert!(!rules.local_custom().is_empty());
        assert!(!rules.method().is_empty());
        assert!(!rules.waiters().is_empty());
        assert!(!rules.checkout().is_empty());
    }

    #[test]
    fn test_restauranttippingrules_validation() {
        let rules = RestaurantTippingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("restaurant_tipping"));
    }

    #[test]
    fn test_restauranttippingrules_explain() {
        let rules = RestaurantTippingRules::new();
        let e = rules.explain();
        assert!(e.contains("了解当地习惯"));
        assert!(e.contains("给小费方式"));
        assert!(e.contains("对待服务生"));
    }
}
