//! 韩国礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KoreanEtiquetteRules, name: "韩国礼仪", desc: "韩国传统礼仪", origin: "韩国", tags: ["社交", "文化"] }
impl KoreanEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["敬语使用"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["尊重长辈"]
    }
}
impl Rule for KoreanEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("korean_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "韩国礼仪",
            &[("敬语", &self.section_0()), ("长辈", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KoreanEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
