//! 奉茶与敬茶礼仪
//!
//! 家中或待客时奉茶、敬茶、续水的次序与礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TeaServingEtiquetteRules,
    name: "奉茶与敬茶礼仪",
    desc: "家中或待客时奉茶、敬茶、续水的次序与礼仪",
    origin: "中国",
    tags: ["社交", "礼仪", "奉茶", "茶道", "待客"]
}

impl TeaServingEtiquetteRules {
    /// 奉茶次序
    pub fn serving_order(&self) -> Vec<&'static str> {
        vec![
            "先敬长辈后敬平辈",
            "按就座方位顺时针递茶",
            "用双手奉上茶盏",
            "茶盏放至客人顺手位置",
        ]
    }

    /// 斟茶细节
    pub fn pouring(&self) -> Vec<&'static str> {
        vec![
            "茶水不宜过满至杯缘",
            "温度适宜不烫手",
            "续水时及时为客人斟满",
            "茶叶浓度随喜好掌握",
        ]
    }

    /// 敬茶礼节
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "奉茶时微微欠身",
            "称呼对方并示意请用",
            "不为难拒茶的客人",
            "待客期间主动续水",
        ]
    }

    /// 收尾
    pub fn conclusion(&self) -> Vec<&'static str> {
        vec![
            "客人走后清理茶具",
            "记得对没有茶的客人道歉",
            "用心奉茶表达待客诚意",
            "等候时机得体添茶",
        ]
    }
}

impl Rule for TeaServingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("serving_tea")
    }

    fn explain(&self) -> String {
        format!(
            "【奉茶与敬茶礼仪】\n{}",
            [
                format!(
                    "奉茶次序：\\n{}",
                    self.serving_order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "斟茶细节：\\n{}",
                    self.pouring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "敬茶礼节：\\n{}",
                    self.manner()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收尾：\\n{}",
                    self.conclusion()
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
    fn test_teaservingetiquetterules_basic() {
        let rules = TeaServingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "奉茶与敬茶礼仪");
        assert!(!rules.serving_order().is_empty());
        assert!(!rules.pouring().is_empty());
        assert!(!rules.manner().is_empty());
        assert!(!rules.conclusion().is_empty());
    }

    #[test]
    fn test_teaservingetiquetterules_validation() {
        let rules = TeaServingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("serving_tea"));
    }

    #[test]
    fn test_teaservingetiquetterules_explain() {
        let rules = TeaServingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("奉茶次序"));
        assert!(e.contains("斟茶细节"));
        assert!(e.contains("敬茶礼节"));
    }
}
