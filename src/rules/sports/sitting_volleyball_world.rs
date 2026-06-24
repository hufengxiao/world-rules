//! 坐式排球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SittingVolleyballWorldRules, name: "坐式排球规则", desc: "坐式排球世界规则", origin: "国际", tags: ["体育", "残疾人"] }
impl SittingVolleyballWorldRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["坐姿比赛网高"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["5局3胜"]
    }
}
impl Rule for SittingVolleyballWorldRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sitting_volleyball_world")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "坐式排球规则",
            &[("规则", &self.section_0()), ("比赛", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SittingVolleyballWorldRules::new();
        assert!(!r.explain().is_empty());
    }
}
