//! 古生物学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PaleontologyDetailedRules, name: "古生物学详细定律", desc: "古生物学定律", origin: "国际", tags: ["科学", "生物"] }
impl PaleontologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["形成保存"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["物种演化"]
    }
}
impl Rule for PaleontologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("paleontology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "古生物学详细定律",
            &[("化石", &self.section_0()), ("进化", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PaleontologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
