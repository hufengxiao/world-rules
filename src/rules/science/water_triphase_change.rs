//! 水的三态变化
//!
//! 水固体液体气体三态与相变知识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WaterTriphaseChangeRules,
    name: "水的三态变化",
    desc: "水固体液体气体三态与相变知识",
    origin: "常识",
    tags: ["科学", "水", "三态", "相变"]
}

impl WaterTriphaseChangeRules {
    /// 三态现象
    pub fn states(&self) -> Vec<&'static str> {
        vec!["冰为固态", "水为液态", "水蒸气为气态", "同一物质三形"]
    }

    /// 相变条件
    pub fn transition(&self) -> Vec<&'static str> {
        vec!["加热融化蒸发", "冷却凝结凝固", "零度结冰", "百度过热汽"]
    }

    /// 溶解相反
    pub fn condense(&self) -> Vec<&'static str> {
        vec!["水汽遇冷凝结", "水凝成水滴", "蒸发散走", "循环往复"]
    }

    /// 实际应用
    pub fn apply(&self) -> Vec<&'static str> {
        vec!["结冰体积膨大", "烧水见水汽", "冷藏更保鲜", "生活常见"]
    }
}

impl Rule for WaterTriphaseChangeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("water_triphase")
    }

    fn explain(&self) -> String {
        format!(
            "【水的三态变化】\n{}",
            [
                format!(
                    "三态现象：\\n{}",
                    self.states()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相变条件：\\n{}",
                    self.transition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "溶解相反：\\n{}",
                    self.condense()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "实际应用：\\n{}",
                    self.apply()
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
    fn test_watertriphasechangerules_basic() {
        let rules = WaterTriphaseChangeRules::new();
        assert_eq!(rules.metadata().name, "水的三态变化");
        assert!(!rules.states().is_empty());
        assert!(!rules.transition().is_empty());
        assert!(!rules.condense().is_empty());
        assert!(!rules.apply().is_empty());
    }

    #[test]
    fn test_watertriphasechangerules_validation() {
        let rules = WaterTriphaseChangeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("water_triphase"));
    }

    #[test]
    fn test_watertriphasechangerules_explain() {
        let rules = WaterTriphaseChangeRules::new();
        let e = rules.explain();
        assert!(e.contains("三态现象"));
        assert!(e.contains("相变条件"));
        assert!(e.contains("溶解相反"));
    }
}
