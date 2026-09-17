//! 山脉形成
//!
//! 地壳运动、板块挤压与山脉形成

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MountainFormationRules,
    name: "山脉形成",
    desc: "地壳运动、板块挤压与山脉形成",
    origin: "国际",
    tags: ["科学", "山", "地质", "板块"]
}

impl MountainFormationRules {
    /// 板块运动
    pub fn plate(&self) -> Vec<&'static str> {
        vec!["地球表层分板块", "板块缓慢移动", "碰撞与挤压", "断裂带形成"]
    }

    /// 造山机制
    pub fn fold(&self) -> Vec<&'static str> {
        vec![
            "板块碰撞地层褶皱",
            "抬升形成山脉",
            "喜马拉雅为实例",
            "缓慢持续隆升",
        ]
    }

    /// 火山成山
    pub fn volcano(&self) -> Vec<&'static str> {
        vec!["岩浆上涌喷发成山", "火山线带分布", "熔岩堆积", "成锥形山"]
    }

    /// 侵蚀演替
    pub fn erosion(&self) -> Vec<&'static str> {
        vec!["风雨水流侵蚀", "山形渐改变", "河谷形成", "地貌演化"]
    }
}

impl Rule for MountainFormationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("mountain")
    }

    fn explain(&self) -> String {
        format!(
            "【山脉形成】\n{}",
            [
                format!(
                    "板块运动：\\n{}",
                    self.plate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "造山机制：\\n{}",
                    self.fold()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "火山成山：\\n{}",
                    self.volcano()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "侵蚀演替：\\n{}",
                    self.erosion()
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
    fn test_mountainformationrules_basic() {
        let rules = MountainFormationRules::new();
        assert_eq!(rules.metadata().name, "山脉形成");
        assert!(!rules.plate().is_empty());
        assert!(!rules.fold().is_empty());
        assert!(!rules.volcano().is_empty());
        assert!(!rules.erosion().is_empty());
    }

    #[test]
    fn test_mountainformationrules_validation() {
        let rules = MountainFormationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("mountain"));
    }

    #[test]
    fn test_mountainformationrules_explain() {
        let rules = MountainFormationRules::new();
        let e = rules.explain();
        assert!(e.contains("板块运动"));
        assert!(e.contains("造山机制"));
        assert!(e.contains("火山成山"));
    }
}
