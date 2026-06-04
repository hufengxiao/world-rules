//! 环境影响评价法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: EnvironmentalImpactLawRules,
    name: "环境影响评价法",
    desc: "环境影响评价法律规则",
    origin: "中国",
    tags: ["法律", "环境"]
}

impl EnvironmentalImpactLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["规划环评", "建设项目环评"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["编制报告书", "公众参与", "审批"]
    }
}

impl Rule for EnvironmentalImpactLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("environmental_impact_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "环境影响评价法",
            &[("评价范围", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_environmental_impact_law_rules() {
        let r = EnvironmentalImpactLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
