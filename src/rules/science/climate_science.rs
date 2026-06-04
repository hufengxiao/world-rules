//! 气候科学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ClimateScienceRules, name: "气候科学定律", desc: "气候科学定律", origin: "国际", tags: ["科学", "环境"] }
impl ClimateScienceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["温室效应", "碳循环"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["全球变暖", "极端天气"]
    }
}
impl Rule for ClimateScienceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("climate_science")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "气候科学定律",
            &[("机制", &self.section_0()), ("变化", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ClimateScienceRules::new();
        assert!(!r.explain().is_empty());
    }
}
