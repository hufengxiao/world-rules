//! 餐厅点菜礼仪
//!
//! 餐厅点菜、考虑他人与招呼服务生的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RestaurantOrderingEtiquetteRules,
    name: "餐厅点菜礼仪",
    desc: "餐厅点菜、考虑他人与招呼服务生的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "点菜", "餐厅"]
}

impl RestaurantOrderingEtiquetteRules {
    /// 点菜原则
    pub fn ordering(&self) -> Vec<&'static str> {
        vec![
            "考虑在场人的饮食禁忌",
            "荤素搭配营养均衡",
            "数量按人数适度",
            "价格合理不铺张",
        ]
    }

    /// 尊重偏好
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "先询问过敏与忌口",
            "尊重素食与宗教限制",
            "不替他人强制点菜",
            "让长者先点",
        ]
    }

    /// 招呼服务
    pub fn service(&self) -> Vec<&'static str> {
        vec![
            "礼貌称呼服务员",
            "说明清楚需求",
            "有疑问委婉询问",
            "不呵斥或不耐烦",
        ]
    }

    /// 结账体面
    pub fn checkout(&self) -> Vec<&'static str> {
        vec![
            "明确由谁请客或AA",
            "适时提出结账",
            "核对账单无异议",
            "以小费表达谢意(按习俗)",
        ]
    }
}

impl Rule for RestaurantOrderingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("restaurant_ordering")
    }

    fn explain(&self) -> String {
        format!(
            "【餐厅点菜礼仪】\n{}",
            [
                format!(
                    "点菜原则：\\n{}",
                    self.ordering()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重偏好：\\n{}",
                    self.respect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "招呼服务：\\n{}",
                    self.service()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结账体面：\\n{}",
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
    fn test_restaurantorderingetiquetterules_basic() {
        let rules = RestaurantOrderingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "餐厅点菜礼仪");
        assert!(!rules.ordering().is_empty());
        assert!(!rules.respect().is_empty());
        assert!(!rules.service().is_empty());
        assert!(!rules.checkout().is_empty());
    }

    #[test]
    fn test_restaurantorderingetiquetterules_validation() {
        let rules = RestaurantOrderingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("restaurant_ordering")
        );
    }

    #[test]
    fn test_restaurantorderingetiquetterules_explain() {
        let rules = RestaurantOrderingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("点菜原则"));
        assert!(e.contains("尊重偏好"));
        assert!(e.contains("招呼服务"));
    }
}
