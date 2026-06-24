//! 法式滚球FIPJP
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PetanqueFipjpDetailedRules, name: "法式滚球FIPJP", desc: "FIPJP法式滚球规则", origin: "法国", tags: ["体育", "休闲"] }
impl PetanqueFipjpDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["13分获胜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["最近球得分"]
    }
}
impl Rule for PetanqueFipjpDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("petanque_fipjp_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "法式滚球FIPJP",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PetanqueFipjpDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
