//! 岩石循环
//!
//! 岩石三类型与成岩演化的基础知识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RockCycleBasicsRules,
    name: "岩石循环",
    desc: "岩石三类型与成岩演化的基础知识",
    origin: "国际",
    tags: ["科学", "岩石", "地质"]
}

impl RockCycleBasicsRules {
    /// 岩石类型
    pub fn types(&self) -> Vec<&'static str> {
        vec!["岩浆岩火山成", "沉积岩层层压", "变质岩热压变", "三类岩石"]
    }

    /// 岩浆岩
    pub fn igneous(&self) -> Vec<&'static str> {
        vec!["熔岩冷却成", "侵入喷出有别", "花岗岩即此", "常见成岩"]
    }

    /// 沉积与变质
    pub fn sediment(&self) -> Vec<&'static str> {
        vec!["风化搬运沉积成", "层理常可辨", "变质经高温压", "改造成新岩"]
    }

    /// 循环演化
    pub fn cycle(&self) -> Vec<&'static str> {
        vec!["三类可互相转化", "风化再成新", "范围变化漫长", "地壳演化"]
    }
}

impl Rule for RockCycleBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("rock_cycle")
    }

    fn explain(&self) -> String {
        format!(
            "【岩石循环】\n{}",
            [
                format!(
                    "岩石类型：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "岩浆岩：\\n{}",
                    self.igneous()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沉积与变质：\\n{}",
                    self.sediment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "循环演化：\\n{}",
                    self.cycle()
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
    fn test_rockcyclebasicsrules_basic() {
        let rules = RockCycleBasicsRules::new();
        assert_eq!(rules.metadata().name, "岩石循环");
        assert!(!rules.types().is_empty());
        assert!(!rules.igneous().is_empty());
        assert!(!rules.sediment().is_empty());
        assert!(!rules.cycle().is_empty());
    }

    #[test]
    fn test_rockcyclebasicsrules_validation() {
        let rules = RockCycleBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("rock_cycle"));
    }

    #[test]
    fn test_rockcyclebasicsrules_explain() {
        let rules = RockCycleBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("岩石类型"));
        assert!(e.contains("岩浆岩"));
        assert!(e.contains("沉积与变质"));
    }
}
