//! 高压锅原理
//!
//! 提高气压使水沸点升高从而加快烹饪的物理原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PressureCookerPhysicsRules,
    name: "高压锅原理",
    desc: "提高气压使水沸点升高从而加快烹饪的物理原理",
    origin: "科学常识",
    tags: ["物理", "热学", "生活"]
}

impl PressureCookerPhysicsRules {
    /// 核心原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "密封使锅内气压升高",
            "气压升高水沸点提高",
            "沸点可达一百二十度",
            "高温加快食物熟化",
        ]
    }

    /// 沸点关系
    pub fn boiling_point(&self) -> Vec<&'static str> {
        vec![
            "气压越高沸点越高",
            "高原气压低水沸点低",
            "高压锅反之提速",
            "同一原理两个方向",
        ]
    }

    /// 安全装置
    pub fn safety_valve(&self) -> Vec<&'static str> {
        vec![
            "限压阀控制锅内气压",
            "安全阀超压自动泄气",
            "密封圈防止漏气",
            "泄压前不可强开盖",
        ]
    }

    /// 使用注意
    pub fn usage(&self) -> Vec<&'static str> {
        vec![
            "食物加水不宜过满",
            "排气孔保持通畅",
            "自然冷却或水淋泄压",
            "老旧胶圈及时更换",
        ]
    }
}

impl Rule for PressureCookerPhysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("pressure_cooker_physics")
    }

    fn explain(&self) -> String {
        format!(
            "【高压锅原理】\n{}",
            [
                format!(
                    "核心原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沸点关系：\\n{}",
                    self.boiling_point()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全装置：\\n{}",
                    self.safety_valve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "使用注意：\\n{}",
                    self.usage()
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
    fn test_pressurecookerphysicsrules_basic() {
        let rules = PressureCookerPhysicsRules::new();
        assert_eq!(rules.metadata().name, "高压锅原理");
        assert!(!rules.principle().is_empty());
        assert!(!rules.boiling_point().is_empty());
        assert!(!rules.safety_valve().is_empty());
        assert!(!rules.usage().is_empty());
    }

    #[test]
    fn test_pressurecookerphysicsrules_validation() {
        let rules = PressureCookerPhysicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("pressure_cooker_physics")
        );
    }

    #[test]
    fn test_pressurecookerphysicsrules_explain() {
        let rules = PressureCookerPhysicsRules::new();
        let e = rules.explain();
        assert!(e.contains("核心原理"));
        assert!(e.contains("沸点关系"));
        assert!(e.contains("安全装置"));
    }
}
