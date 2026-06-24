//! 残疾人游泳规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ParaSwimmingRules, name: "残疾人游泳规则", desc: "残疾人游泳规则", origin: "国际", tags: ["体育", "残疾人"] }
impl ParaSwimmingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["S1到S14"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["自由泳蛙泳"]
    }
}
impl Rule for ParaSwimmingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_swimming")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "残疾人游泳规则",
            &[("分级", &self.section_0()), ("泳姿", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ParaSwimmingRules::new();
        assert!(!r.explain().is_empty());
    }
}
