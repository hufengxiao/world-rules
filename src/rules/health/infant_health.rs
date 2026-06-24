//! 婴儿健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InfantHealthRules, name: "婴儿健康规则", desc: "婴儿健康规则", origin: "国际", tags: ["健康", "婴儿"] }
impl InfantHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["母乳辅食"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["里程碑"]
    }
}
impl Rule for InfantHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("infant_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "婴儿健康规则",
            &[("喂养", &self.section_0()), ("发育", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InfantHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
