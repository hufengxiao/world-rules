//! 斯诺克世界锦标赛
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BilliardsSnookerWorldRules, name: "斯诺克世界锦标赛", desc: "斯诺克世界锦标赛", origin: "英国", tags: ["体育", "桌球"] }
impl BilliardsSnookerWorldRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["长局制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["147满分"]
    }
}
impl Rule for BilliardsSnookerWorldRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("billiards_snooker_world")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "斯诺克世界锦标赛",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BilliardsSnookerWorldRules::new();
        assert!(!r.explain().is_empty());
    }
}
