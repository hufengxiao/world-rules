//! 水权法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RightToWaterRules, name: "水权法", desc: "水权法律规则", origin: "国际", tags: ["法律", "资源"] }
impl RightToWaterRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["安全饮用水"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["水资源管理"]
    }
}
impl Rule for RightToWaterRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("right_to_water")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "水权法",
            &[("权利", &self.section_0()), ("管理", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RightToWaterRules::new();
        assert!(!r.explain().is_empty());
    }
}
