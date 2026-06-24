//! 日本礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: JapaneseEtiquetteRules, name: "日本礼仪", desc: "日本传统礼仪", origin: "日本", tags: ["社交", "文化"] }
impl JapaneseEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["鞠躬角度"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["进屋脱鞋"]
    }
}
impl Rule for JapaneseEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("japanese_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "日本礼仪",
            &[("鞠躬", &self.section_0()), ("脱鞋", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = JapaneseEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
