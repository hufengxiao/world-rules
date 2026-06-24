//! 噪声污染科学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NoisePollutionScienceRules, name: "噪声污染科学定律", desc: "噪声污染定律", origin: "国际", tags: ["科学", "环境"] }
impl NoisePollutionScienceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["分贝"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["隔声吸声"]
    }
}
impl Rule for NoisePollutionScienceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("noise_pollution_science")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "噪声污染科学定律",
            &[("测量", &self.section_0()), ("控制", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NoisePollutionScienceRules::new();
        assert!(!r.explain().is_empty());
    }
}
