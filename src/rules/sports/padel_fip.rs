//! 板式网球FIP
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PadelFipRules, name: "板式网球FIP", desc: "FIP板式网球规则", origin: "阿根廷", tags: ["体育", "休闲"] }
impl PadelFipRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["有墙场地"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["类似网球"]
    }
}
impl Rule for PadelFipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("padel_fip")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "板式网球FIP",
            &[("场地", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PadelFipRules::new();
        assert!(!r.explain().is_empty());
    }
}
