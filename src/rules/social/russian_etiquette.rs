//! 俄罗斯礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RussianEtiquetteRules, name: "俄罗斯礼仪", desc: "俄罗斯社交礼仪", origin: "俄罗斯", tags: ["社交", "文化"] }
impl RussianEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["握手礼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["奇数花束"]
    }
}
impl Rule for RussianEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("russian_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "俄罗斯礼仪",
            &[("握手", &self.section_0()), ("送花", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RussianEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
