//! 可持续发展科学
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SustainabilityScienceRules, name: "可持续发展科学", desc: "可持续发展定律", origin: "国际", tags: ["科学", "环境"] }
impl SustainabilityScienceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["联合国目标"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["ESG"]
    }
}
impl Rule for SustainabilityScienceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("sustainability_science")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "可持续发展科学",
            &[("SDGs", &self.section_0()), ("指标", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SustainabilityScienceRules::new();
        assert!(!r.explain().is_empty());
    }
}
