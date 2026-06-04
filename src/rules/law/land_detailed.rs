//! 土地管理详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: LandDetailedRules, name: "土地管理详解", desc: "土地管理法详解", origin: "中国", tags: ["法律", "土地"] }
impl LandDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["国有土地", "集体土地"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["用途管制", "耕地保护"]
    }
}
impl Rule for LandDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("land_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "土地管理详解",
            &[("权属", &self.section_0()), ("利用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LandDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
