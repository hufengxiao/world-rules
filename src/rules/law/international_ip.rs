//! 国际知识产权法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalIpRules, name: "国际知识产权法", desc: "国际知识产权规则", origin: "国际", tags: ["法律", "知识产权"] }
impl InternationalIpRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["TRIPS协定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["WIPO公约"]
    }
}
impl Rule for InternationalIpRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_ip")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际知识产权法",
            &[("TRIPS", &self.section_0()), ("WIPO", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalIpRules::new();
        assert!(!r.explain().is_empty());
    }
}
