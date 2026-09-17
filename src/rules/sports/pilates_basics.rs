//! 普拉提基础
//!
//! 普拉提呼吸、核心控制与安全练习规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PilatesBasicsRules,
    name: "普拉提基础",
    desc: "普拉提呼吸、核心控制与安全练习规范",
    origin: "国际",
    tags: ["体育", "普拉提", "核心", "健身"]
}

impl PilatesBasicsRules {
    /// 呼吸配合
    pub fn breath(&self) -> Vec<&'static str> {
        vec![
            "鼻息协调动作",
            "呼气收紧核心",
            "节奏均匀不过呼",
            "配合动作起伏",
        ]
    }

    /// 核心控制
    pub fn core(&self) -> Vec<&'static str> {
        vec![
            "激活核心腹肌",
            "保持脊柱中立位",
            "缓慢流畅转动",
            "减少代偿发力",
        ]
    }

    /// 循序渐进
    pub fn progression(&self) -> Vec<&'static str> {
        vec![
            "从基础动作学起",
            "量力而行不硬撑",
            "有伤痛调整动作",
            "配合垫上器械",
        ]
    }

    /// 安全规范
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "练习前充分热身",
            "呼吸不畅停止",
            "腰背伤咨询教练",
            "场地防滑稳",
        ]
    }
}

impl Rule for PilatesBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pilates")
    }

    fn explain(&self) -> String {
        format!(
            "【普拉提基础】\n{}",
            [
                format!(
                    "呼吸配合：\\n{}",
                    self.breath()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "核心控制：\\n{}",
                    self.core()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "循序渐进：\\n{}",
                    self.progression()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全规范：\\n{}",
                    self.safety()
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
    fn test_pilatesbasicsrules_basic() {
        let rules = PilatesBasicsRules::new();
        assert_eq!(rules.metadata().name, "普拉提基础");
        assert!(!rules.breath().is_empty());
        assert!(!rules.core().is_empty());
        assert!(!rules.progression().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_pilatesbasicsrules_validation() {
        let rules = PilatesBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("pilates"));
    }

    #[test]
    fn test_pilatesbasicsrules_explain() {
        let rules = PilatesBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("呼吸配合"));
        assert!(e.contains("核心控制"));
        assert!(e.contains("循序渐进"));
    }
}
