//! 燃烧三要素
//!
//! 燃烧需要可燃物、助燃物与温度的组合

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CombustionConditionsTriangleRules,
    name: "燃烧三要素",
    desc: "燃烧需要可燃物、助燃物与温度的组合",
    origin: "化学",
    tags: ["科学", "燃烧", "化学", "安全"]
}

impl CombustionConditionsTriangleRules {
    /// 三要素
    pub fn conditions(&self) -> Vec<&'static str> {
        vec!["可燃物质", "助燃氧气充足", "达到着火点", "三者齐备才燃"]
    }

    /// 灭火原理
    pub fn extinguish(&self) -> Vec<&'static str> {
        vec!["移走可燃物", "隔绝氧气", "冷却降温", "破坏燃烧条件"]
    }

    /// 安全注意
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["远离易燃物", "用气用火谨慎", "灭火器常备", "防患未然"]
    }

    /// 生活现象
    pub fn example(&self) -> Vec<&'static str> {
        vec!["覆盖盖灭火焰", "浇水降温", "隔氧离火源", "明火慎用"]
    }
}

impl Rule for CombustionConditionsTriangleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("combustion")
    }

    fn explain(&self) -> String {
        format!(
            "【燃烧三要素】\n{}",
            [
                format!(
                    "三要素：\\n{}",
                    self.conditions()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "灭火原理：\\n{}",
                    self.extinguish()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全注意：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活现象：\\n{}",
                    self.example()
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
    fn test_combustionconditionstrianglerules_basic() {
        let rules = CombustionConditionsTriangleRules::new();
        assert_eq!(rules.metadata().name, "燃烧三要素");
        assert!(!rules.conditions().is_empty());
        assert!(!rules.extinguish().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.example().is_empty());
    }

    #[test]
    fn test_combustionconditionstrianglerules_validation() {
        let rules = CombustionConditionsTriangleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("combustion"));
    }

    #[test]
    fn test_combustionconditionstrianglerules_explain() {
        let rules = CombustionConditionsTriangleRules::new();
        let e = rules.explain();
        assert!(e.contains("三要素"));
        assert!(e.contains("灭火原理"));
        assert!(e.contains("安全注意"));
    }
}
