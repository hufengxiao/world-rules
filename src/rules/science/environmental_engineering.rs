//! 环境工程定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EnvironmentalEngineeringRules, name: "环境工程定律", desc: "环境工程定律", origin: "国际", tags: ["科学", "工程"] }
impl EnvironmentalEngineeringRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["净水技术"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["脱硫脱硝"]
    }
}
impl Rule for EnvironmentalEngineeringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("environmental_engineering")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "环境工程定律",
            &[("水处理", &self.section_0()), ("废气", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EnvironmentalEngineeringRules::new();
        assert!(!r.explain().is_empty());
    }
}
