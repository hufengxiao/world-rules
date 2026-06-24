//! 冲浪皮艇规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SurfKayakRules, name: "冲浪皮艇规则", desc: "冲浪皮艇运动规则", origin: "国际", tags: ["体育", "水上"] }
impl SurfKayakRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["浪上技巧"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["皮艇桨"]
    }
}
impl Rule for SurfKayakRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("surf_kayak")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "冲浪皮艇规则",
            &[("比赛", &self.section_0()), ("装备", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SurfKayakRules::new();
        assert!(!r.explain().is_empty());
    }
}
