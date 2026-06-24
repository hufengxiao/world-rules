//! 印地赛车规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: IndycarRules, name: "印地赛车规则", desc: "印地赛车规则", origin: "美国", tags: ["体育", "赛车"] }
impl IndycarRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["椭圆赛道"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["进站策略"]
    }
}
impl Rule for IndycarRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("indyCar")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "印地赛车规则",
            &[("赛道", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IndycarRules::new();
        assert!(!r.explain().is_empty());
    }
}
