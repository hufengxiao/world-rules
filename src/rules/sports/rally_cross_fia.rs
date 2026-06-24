//! 跨界拉力赛规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RallyCrossFiaRules, name: "跨界拉力赛规则", desc: "FIA跨界拉力赛", origin: "国际", tags: ["体育", "赛车"] }
impl RallyCrossFiaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["短道赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["混合路面"]
    }
}
impl Rule for RallyCrossFiaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rally_cross_fia")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "跨界拉力赛规则",
            &[("赛制", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RallyCrossFiaRules::new();
        assert!(!r.explain().is_empty());
    }
}
