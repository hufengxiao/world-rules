//! 免疫系统基础
//!
//! 免疫防御、免疫力维持的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ImmuneSystemBasicsRules,
    name: "免疫系统基础",
    desc: "免疫防御、免疫力维持的科学常识",
    origin: "国际",
    tags: ["科学", "免疫", "防御", "生理"]
}

impl ImmuneSystemBasicsRules {
    /// 免疫防御
    pub fn defense(&self) -> Vec<&'static str> {
        vec![
            "免疫识别病原入侵",
            "先天免疫快速应对",
            "获得性免疫有记忆",
            "发热是防御反应",
        ]
    }

    /// 维持免疫
    pub fn maintain(&self) -> Vec<&'static str> {
        vec![
            "规律作息支持机能",
            "均衡营养增强防御",
            "适度运动助免疫",
            "良好心态有益",
        ]
    }

    /// 科学认知
    pub fn science(&self) -> Vec<&'static str> {
        vec![
            "疫苗训练免疫应答",
            "抗生素不治病毒",
            "不迷信免疫补品",
            "免疫过强过弱皆不利",
        ]
    }

    /// 健康促进
    pub fn health(&self) -> Vec<&'static str> {
        vec![
            "充足睡眠恢复力",
            "勤洗手防感染",
            "遵医嘱规范用药",
            "慢性病者依计划管理",
        ]
    }
}

impl Rule for ImmuneSystemBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("immune_system")
    }

    fn explain(&self) -> String {
        format!(
            "【免疫系统基础】\n{}",
            [
                format!(
                    "免疫防御：\\n{}",
                    self.defense()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维持免疫：\\n{}",
                    self.maintain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "科学认知：\\n{}",
                    self.science()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康促进：\\n{}",
                    self.health()
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
    fn test_immunesystembasicsrules_basic() {
        let rules = ImmuneSystemBasicsRules::new();
        assert_eq!(rules.metadata().name, "免疫系统基础");
        assert!(!rules.defense().is_empty());
        assert!(!rules.maintain().is_empty());
        assert!(!rules.science().is_empty());
        assert!(!rules.health().is_empty());
    }

    #[test]
    fn test_immunesystembasicsrules_validation() {
        let rules = ImmuneSystemBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("immune_system"));
    }

    #[test]
    fn test_immunesystembasicsrules_explain() {
        let rules = ImmuneSystemBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("免疫防御"));
        assert!(e.contains("维持免疫"));
        assert!(e.contains("科学认知"));
    }
}
