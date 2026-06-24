//! 海牙公约规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HagueConventionRules, name: "海牙公约规则", desc: "海牙国际私法规则", origin: "国际", tags: ["法律", "国际"] }
impl HagueConventionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["域外送达"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["域外取证"]
    }
}
impl Rule for HagueConventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("hague_convention")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "海牙公约规则",
            &[("送达", &self.section_0()), ("取证", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HagueConventionRules::new();
        assert!(!r.explain().is_empty());
    }
}
