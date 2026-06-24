//! 统计物理定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: StatisticalPhysicsRules, name: "统计物理定律", desc: "统计物理定律", origin: "国际", tags: ["科学", "物理"] }
impl StatisticalPhysicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["玻尔兹曼", "费米-狄拉克", "玻色-爱因斯坦"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["相变", "临界现象"]
    }
}
impl Rule for StatisticalPhysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("statistical_physics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "统计物理定律",
            &[("分布", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = StatisticalPhysicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
