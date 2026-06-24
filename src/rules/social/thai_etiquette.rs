//! 泰国礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ThaiEtiquetteRules, name: "泰国礼仪", desc: "泰国传统礼仪", origin: "泰国", tags: ["社交", "文化"] }
impl ThaiEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合十礼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不可摸头"]
    }
}
impl Rule for ThaiEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("thai_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "泰国礼仪",
            &[("合十", &self.section_0()), ("头部", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ThaiEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
