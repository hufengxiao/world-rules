//! 疫苗接种规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: VaccinationRulesRules, name: "疫苗接种规则", desc: "疫苗接种规则", origin: "国际", tags: ["健康", "预防"] }
impl VaccinationRulesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["免疫程序"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["加强针"]
    }
}
impl Rule for VaccinationRulesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("vaccination_rules")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "疫苗接种规则",
            &[("儿童", &self.section_0()), ("成人", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VaccinationRulesRules::new();
        assert!(!r.explain().is_empty());
    }
}
