//! 皮划艇激流回旋
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KayakingSlalomRules, name: "皮划艇激流回旋", desc: "皮划艇激流回旋规则", origin: "国际", tags: ["体育", "水上"] }
impl KayakingSlalomRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["顺水门逆水门"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["漏门碰杆"]
    }
}
impl Rule for KayakingSlalomRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kayaking_slalom")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "皮划艇激流回旋",
            &[("门杆", &self.section_0()), ("罚分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KayakingSlalomRules::new();
        assert!(!r.explain().is_empty());
    }
}
