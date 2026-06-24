//! 赛车通用规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MotorsportGenericRules, name: "赛车通用规则", desc: "赛车运动通用规则", origin: "国际", tags: ["体育", "赛车"] }
impl MotorsportGenericRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["头盔HANS"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["排位赛正赛"]
    }
}
impl Rule for MotorsportGenericRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("motorsport_generic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "赛车通用规则",
            &[("安全", &self.section_0()), ("赛制", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MotorsportGenericRules::new();
        assert!(!r.explain().is_empty());
    }
}
