//! 社保法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SocialInsuranceLawDetailedRules, name: "社保法详解", desc: "社会保险法详解", origin: "中国", tags: ["法律", "社保"] }
impl SocialInsuranceLawDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["养老医疗工伤失业生育"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["缴费基数"]
    }
}
impl Rule for SocialInsuranceLawDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("social_insurance_law_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "社保法详解",
            &[("五险", &self.section_0()), ("缴费", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SocialInsuranceLawDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
