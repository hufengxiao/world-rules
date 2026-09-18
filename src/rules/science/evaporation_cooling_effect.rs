//! 蒸发吸热致冷
//!
//! 液体蒸发吸热使温度下降的物理现象

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EvaporationCoolingEffectRules,
    name: "蒸发吸热致冷",
    desc: "液体蒸发吸热使温度下降的物理现象",
    origin: "中国",
    tags: ["科学", "蒸发", "物态", "制冷"]
}

impl EvaporationCoolingEffectRules {
    /// 原理现象
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "液体蒸发变气",
            "蒸发吸走热量",
            "周围温度下降",
            "水干了手发凉",
        ]
    }

    /// 日常体验
    pub fn experience(&self) -> Vec<&'static str> {
        vec![
            "温水酒精拭身",
            "酒精挥发凉",
            "出汗后吹风更凉",
            "湿衣风干手凉",
        ]
    }

    /// 致冷应用
    pub fn apply(&self) -> Vec<&'static str> {
        vec![
            "湿巾降温",
            "蒸发式冷风扇",
            "汗液散热降温",
            "烧水水汽带走热量",
        ]
    }

    /// 相关注意
    pub fn note(&self) -> Vec<&'static str> {
        vec!["风速加快蒸发", "通风促进降温", "冬天易干燥", "湿冷体感更冷"]
    }
}

impl Rule for EvaporationCoolingEffectRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("evaporation_cool")
    }

    fn explain(&self) -> String {
        format!(
            "【蒸发吸热致冷】\n{}",
            [
                format!(
                    "原理现象：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常体验：\\n{}",
                    self.experience()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "致冷应用：\\n{}",
                    self.apply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相关注意：\\n{}",
                    self.note()
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
    fn test_evaporationcoolingeffectrules_basic() {
        let rules = EvaporationCoolingEffectRules::new();
        assert_eq!(rules.metadata().name, "蒸发吸热致冷");
        assert!(!rules.principle().is_empty());
        assert!(!rules.experience().is_empty());
        assert!(!rules.apply().is_empty());
        assert!(!rules.note().is_empty());
    }

    #[test]
    fn test_evaporationcoolingeffectrules_validation() {
        let rules = EvaporationCoolingEffectRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("evaporation_cool"));
    }

    #[test]
    fn test_evaporationcoolingeffectrules_explain() {
        let rules = EvaporationCoolingEffectRules::new();
        let e = rules.explain();
        assert!(e.contains("原理现象"));
        assert!(e.contains("日常体验"));
        assert!(e.contains("致冷应用"));
    }
}
