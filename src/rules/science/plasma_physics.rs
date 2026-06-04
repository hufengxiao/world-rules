//! 等离子体物理定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: PlasmaPhysicsRules,
    name: "等离子体物理定律",
    desc: "等离子体物理定律",
    origin: "国际",
    tags: ["科学", "物理"]
}

impl PlasmaPhysicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["等离子体振荡频率", "德拜长度", "磁冻结效应"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["核聚变约束", "等离子体刻蚀", "等离子体显示"]
    }
}

impl Rule for PlasmaPhysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("plasma_physics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "等离子体物理定律",
            &[("基本方程", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plasma_physics_rules() {
        let r = PlasmaPhysicsRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
