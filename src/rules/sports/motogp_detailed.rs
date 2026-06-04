//! MotoGP详细
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MotogpDetailedRules, name: "MotoGP详细", desc: "MotoGP详细规则", origin: "FIM", tags: ["体育", "赛车"] }
impl MotogpDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["MotoGP", "Moto2"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["积分", "排位赛"]
    }
}
impl Rule for MotogpDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("motogp_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "MotoGP详细",
            &[("组别", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MotogpDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
