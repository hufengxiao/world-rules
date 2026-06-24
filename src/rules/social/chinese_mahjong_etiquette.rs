//! 麻将社交礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseMahjongEtiquetteRules, name: "麻将社交礼仪", desc: "中国麻将社交礼仪", origin: "中国", tags: ["社交", "麻将"] }
impl ChineseMahjongEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不偷看"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["准时赴约"]
    }
}
impl Rule for ChineseMahjongEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_mahjong_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "麻将社交礼仪",
            &[("行为", &self.section_0()), ("礼仪", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseMahjongEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
