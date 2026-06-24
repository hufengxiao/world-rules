//! 武术礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseMartialArtsEtiquetteRules, name: "武术礼仪", desc: "中国传统武术礼仪", origin: "中国", tags: ["社交", "武术"] }
impl ChineseMartialArtsEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["抱拳礼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["尊师重道"]
    }
}
impl Rule for ChineseMartialArtsEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_martial_arts_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "武术礼仪",
            &[("抱拳", &self.section_0()), ("武德", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseMartialArtsEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
