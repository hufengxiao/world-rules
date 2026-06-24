//! 儒家礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ConfucianEtiquetteRules, name: "儒家礼仪", desc: "儒家传统礼仪", origin: "中国", tags: ["社交", "文化"] }
impl ConfucianEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["仁义礼智信"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["君臣父子"]
    }
}
impl Rule for ConfucianEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("confucian_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "儒家礼仪",
            &[("五常", &self.section_0()), ("五伦", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ConfucianEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
