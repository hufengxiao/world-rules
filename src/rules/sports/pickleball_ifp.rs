//! 匹克球IFP规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PickleballIfpRules, name: "匹克球IFP规则", desc: "IFP匹克球规则", origin: "美国", tags: ["体育", "休闲"] }
impl PickleballIfpRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["小场地低网"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["发球规则"]
    }
}
impl Rule for PickleballIfpRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pickleball_ifp")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "匹克球IFP规则",
            &[("场地", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PickleballIfpRules::new();
        assert!(!r.explain().is_empty());
    }
}
