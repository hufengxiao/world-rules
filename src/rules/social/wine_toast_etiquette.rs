//! 敬酒与酒桌礼仪
//!
//! 宴饮敬酒、挡酒与酒桌分寸的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WineToastEtiquetteRules,
    name: "敬酒与酒桌礼仪",
    desc: "宴饮敬酒、挡酒与酒桌分寸的礼仪",
    origin: "中国",
    tags: ["社交", "敬酒", "酒桌", "礼仪"]
}

impl WineToastEtiquetteRules {
    /// 举杯示意
    pub fn gesture(&self) -> Vec<&'static str> {
        vec![
            "酒杯低于长辈的杯",
            "双手端杯敬人",
            "眼神真诚交流",
            "碰杯轻不碰撞",
        ]
    }

    /// 敬酒顺序
    pub fn order(&self) -> Vec<&'static str> {
        vec!["先敬长辈主人", "尊长者先后依次", "宾客有序敬", "祝词得体"]
    }

    /// 挡酒自量
    pub fn decline(&self) -> Vec<&'static str> {
        vec![
            "不胜酒力如实说",
            "以茶代酒致意",
            "不劝酒强灌",
            "尊重他人酒量",
        ]
    }

    /// 节制不贪
    pub fn moderate(&self) -> Vec<&'static str> {
        vec!["适量健康饮酒", "不醉驾", "醉失态不雅", "酒桌欢乐有度"]
    }
}

impl Rule for WineToastEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("wine_toast")
    }

    fn explain(&self) -> String {
        format!(
            "【敬酒与酒桌礼仪】\n{}",
            [
                format!(
                    "举杯示意：\\n{}",
                    self.gesture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "敬酒顺序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "挡酒自量：\\n{}",
                    self.decline()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "节制不贪：\\n{}",
                    self.moderate()
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
    fn test_winetoastetiquetterules_basic() {
        let rules = WineToastEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "敬酒与酒桌礼仪");
        assert!(!rules.gesture().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.decline().is_empty());
        assert!(!rules.moderate().is_empty());
    }

    #[test]
    fn test_winetoastetiquetterules_validation() {
        let rules = WineToastEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("wine_toast"));
    }

    #[test]
    fn test_winetoastetiquetterules_explain() {
        let rules = WineToastEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("举杯示意"));
        assert!(e.contains("敬酒顺序"));
        assert!(e.contains("挡酒自量"));
    }
}
