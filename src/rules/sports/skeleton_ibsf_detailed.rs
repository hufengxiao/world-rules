//! 钢架雪车IBSF
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SkeletonIbsfDetailedRules, name: "钢架雪车IBSF", desc: "IBSF钢架雪车规则", origin: "国际", tags: ["体育", "冬季"] }
impl SkeletonIbsfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["俯卧头朝前"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["重量计时"]
    }
}
impl Rule for SkeletonIbsfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("skeleton_ibsf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "钢架雪车IBSF",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SkeletonIbsfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
