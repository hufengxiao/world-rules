//! 皮肤护理规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SkinCareRulesRules, name: "皮肤护理规则", desc: "皮肤护理规则", origin: "国际", tags: ["健康", "护肤"] }
impl SkinCareRulesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["温和洁面"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["SPF30+"]
    }
}
impl Rule for SkinCareRulesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("skin_care_rules")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "皮肤护理规则",
            &[("清洁", &self.section_0()), ("防晒", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SkinCareRulesRules::new();
        assert!(!r.explain().is_empty());
    }
}
