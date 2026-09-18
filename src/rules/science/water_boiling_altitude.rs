//! 水的沸点与海拔
//!
//! 海拔越高水沸点越低的气压原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WaterBoilingAltitudeRules,
    name: "水的沸点与海拔",
    desc: "海拔越高水沸点越低的气压原理",
    origin: "物理",
    tags: ["科学", "水", "沸点", "气压"]
}

impl WaterBoilingAltitudeRules {
    /// 沸点真相
    pub fn fact(&self) -> Vec<&'static str> {
        vec![
            "标准气压前提",
            "海平面百摄氏度沸",
            "气压高沸点高",
            "气压低沸点低",
        ]
    }

    /// 海拔影响
    pub fn altitude(&self) -> Vec<&'static str> {
        vec!["高山气压低", "水不到百度即沸", "高原煮饭欠熟", "用高压锅补"]
    }

    /// 高压锅原理
    pub fn pressure_cooker(&self) -> Vec<&'static str> {
        vec!["密闭增气压", "沸点升高", "更快煮熟", "高原适用"]
    }

    /// 生活常识
    pub fn life(&self) -> Vec<&'static str> {
        vec!["山上烧水会早沸", "烹饪时间调整", "理解原理", "科学应对"]
    }
}

impl Rule for WaterBoilingAltitudeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("water_boiling")
    }

    fn explain(&self) -> String {
        format!(
            "【水的沸点与海拔】\n{}",
            [
                format!(
                    "沸点真相：\\n{}",
                    self.fact()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "海拔影响：\\n{}",
                    self.altitude()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "高压锅原理：\\n{}",
                    self.pressure_cooker()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活常识：\\n{}",
                    self.life()
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
    fn test_waterboilingaltituderules_basic() {
        let rules = WaterBoilingAltitudeRules::new();
        assert_eq!(rules.metadata().name, "水的沸点与海拔");
        assert!(!rules.fact().is_empty());
        assert!(!rules.altitude().is_empty());
        assert!(!rules.pressure_cooker().is_empty());
        assert!(!rules.life().is_empty());
    }

    #[test]
    fn test_waterboilingaltituderules_validation() {
        let rules = WaterBoilingAltitudeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("water_boiling"));
    }

    #[test]
    fn test_waterboilingaltituderules_explain() {
        let rules = WaterBoilingAltitudeRules::new();
        let e = rules.explain();
        assert!(e.contains("沸点真相"));
        assert!(e.contains("海拔影响"));
        assert!(e.contains("高压锅原理"));
    }
}
