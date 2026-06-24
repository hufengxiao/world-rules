//! MotoGP FIM详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MotogpFimDetailedRules, name: "MotoGP FIM详细", desc: "FIM摩托GP规则", origin: "国际", tags: ["体育", "赛车"] }
impl MotogpFimDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["MotoGP Moto2 Moto3"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["25-16-13"]
    }
}
impl Rule for MotogpFimDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("motogp_fim_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "MotoGP FIM详细",
            &[("组别", &self.section_0()), ("积分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MotogpFimDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
