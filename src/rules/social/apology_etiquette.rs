//! 道歉礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ApologyEtiquetteRules, name: "道歉礼仪", desc: "社交道歉礼仪", origin: "国际", tags: ["社交", "沟通"] }
impl ApologyEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["真诚道歉"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["补救措施"]
    }
}
impl Rule for ApologyEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("apology_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "道歉礼仪",
            &[("真诚", &self.section_0()), ("补救", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ApologyEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
