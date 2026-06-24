//! 饮酒礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AlcoholEtiquetteRules, name: "饮酒礼仪", desc: "社交饮酒礼仪", origin: "国际", tags: ["社交", "饮酒"] }
impl AlcoholEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["敬酒礼节"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["适度饮酒"]
    }
}
impl Rule for AlcoholEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("alcohol_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "饮酒礼仪",
            &[("敬酒", &self.section_0()), ("适度", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AlcoholEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
