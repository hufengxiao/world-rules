//! 英国礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BritishEtiquetteRules, name: "英国礼仪", desc: "英国社交礼仪", origin: "英国", tags: ["社交", "文化"] }
impl BritishEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["绅士排队"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["下午茶礼节"]
    }
}
impl Rule for BritishEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("british_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英国礼仪",
            &[("排队", &self.section_0()), ("下午茶", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BritishEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
