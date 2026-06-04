//! 射击详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ShootingDetailedRules, name: "射击详细规则", desc: "射击详细比赛规则", origin: "ISSF", tags: ["体育", "精准"] }
impl ShootingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["10米气步枪", "50米步枪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["10米气手枪", "25米速射"]
    }
}
impl Rule for ShootingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("shooting_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "射击详细规则",
            &[("步枪", &self.section_0()), ("手枪", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ShootingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
