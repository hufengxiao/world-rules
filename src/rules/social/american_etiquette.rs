//! 美国礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AmericanEtiquetteRules, name: "美国礼仪", desc: "美国社交礼仪", origin: "美国", tags: ["社交", "文化"] }
impl AmericanEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["小费文化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["保持距离"]
    }
}
impl Rule for AmericanEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("american_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "美国礼仪",
            &[("小费", &self.section_0()), ("个人空间", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AmericanEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
