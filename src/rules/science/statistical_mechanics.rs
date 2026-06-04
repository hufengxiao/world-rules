//! 统计力学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: StatisticalMechanicsRules, name: "统计力学定律", desc: "统计力学定律", origin: "国际", tags: ["科学", "物理"] }
impl StatisticalMechanicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["玻尔兹曼分布", "配分函数"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["相变理论", "临界现象"]
    }
}
impl Rule for StatisticalMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("statistical_mechanics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "统计力学定律",
            &[("基础", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = StatisticalMechanicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
