//! 肥皂泡与薄膜干涉
//!
//! 肥皂泡呈彩色的光学原理，包括薄膜干涉、表面张力、泡壁厚度变化与色彩随厚度改变的科学解释。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SoapBubbleInterferenceRules,
    name: "肥皂泡与薄膜干涉",
    desc: "肥皂泡呈彩色的光学原理，包括薄膜干涉、表面张力、泡壁厚度变化与色彩随厚度改变的科学解释。",
    origin: "光学薄膜干涉与表面张力理论",
    tags: ["科学", "光学", "物理", "自然现象"]
}

impl SoapBubbleInterferenceRules {
    /// 薄膜干涉
    pub fn interference(&self) -> Vec<&'static str> {
        vec![
            "光在泡壁内外表面分别反射",
            "两束反射光发生干涉",
            "不同波长加强或相消",
            "形成彩色条纹",
        ]
    }

    /// 色彩变化
    pub fn colors(&self) -> Vec<&'static str> {
        vec![
            "颜色随泡壁厚度变化",
            "顶部变薄色彩渐变",
            "临近破裂时呈灰黑色",
            "白光下呈现彩色环带",
        ]
    }

    /// 表面张力
    pub fn tension(&self) -> Vec<&'static str> {
        vec![
            "表面张力使泡呈球形",
            "皂液降低水的表面张力",
            "球形是表面积最小形态",
            "泡越大壁越薄越易破",
        ]
    }

    /// 稳定性
    pub fn stability(&self) -> Vec<&'static str> {
        vec![
            "水分蒸发使泡破裂",
            "灰尘扰动导致破裂",
            "甘油可延缓蒸发增稳",
            "气流与温度影响寿命",
        ]
    }
}

impl Rule for SoapBubbleInterferenceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("soap_bubble")
    }

    fn explain(&self) -> String {
        format!(
            "【肥皂泡与薄膜干涉】\n{}",
            [
                format!(
                    "薄膜干涉：\\n{}",
                    self.interference()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "色彩变化：\\n{}",
                    self.colors()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表面张力：\\n{}",
                    self.tension()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "稳定性：\\n{}",
                    self.stability()
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
    fn test_soapbubbleinterferencerules_basic() {
        let rules = SoapBubbleInterferenceRules::new();
        assert_eq!(rules.metadata().name, "肥皂泡与薄膜干涉");
        assert!(!rules.interference().is_empty());
        assert!(!rules.colors().is_empty());
        assert!(!rules.tension().is_empty());
        assert!(!rules.stability().is_empty());
    }

    #[test]
    fn test_soapbubbleinterferencerules_validation() {
        let rules = SoapBubbleInterferenceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("soap_bubble"));
    }

    #[test]
    fn test_soapbubbleinterferencerules_explain() {
        let rules = SoapBubbleInterferenceRules::new();
        let e = rules.explain();
        assert!(e.contains("薄膜干涉"));
        assert!(e.contains("色彩变化"));
        assert!(e.contains("表面张力"));
    }
}
