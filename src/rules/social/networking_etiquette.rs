//! 社交网络礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NetworkingEtiquetteRules, name: "社交网络礼仪", desc: "商务社交礼仪", origin: "国际", tags: ["社交", "职场"] }
impl NetworkingEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["交换名片"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["事后跟进"]
    }
}
impl Rule for NetworkingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("networking_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "社交网络礼仪",
            &[("名片", &self.section_0()), ("跟进", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NetworkingEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
