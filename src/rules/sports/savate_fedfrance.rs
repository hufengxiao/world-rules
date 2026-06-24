//! 法式拳击联盟规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SavateFedfranceRules, name: "法式拳击联盟规则", desc: "法式拳击联盟规则", origin: "法国", tags: ["体育", "格斗"] }
impl SavateFedfranceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["手套级别"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拳腿技术"]
    }
}
impl Rule for SavateFedfranceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("savate_fedfrance")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "法式拳击联盟规则",
            &[("级别", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SavateFedfranceRules::new();
        assert!(!r.explain().is_empty());
    }
}
