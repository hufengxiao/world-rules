//! 志愿者礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: VolunteerEtiquetteRules, name: "志愿者礼仪", desc: "志愿服务礼仪", origin: "国际", tags: ["社交", "公益"] }
impl VolunteerEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["尊重受益者"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["遵守规则"]
    }
}
impl Rule for VolunteerEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("volunteer_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "志愿者礼仪",
            &[("态度", &self.section_0()), ("行为", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VolunteerEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
