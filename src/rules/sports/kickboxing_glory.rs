//! Glory踢拳规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KickboxingGloryRules, name: "Glory踢拳规则", desc: "Glory踢拳锦标赛", origin: "荷兰", tags: ["体育", "格斗"] }
impl KickboxingGloryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3回合"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["K1规则"]
    }
}
impl Rule for KickboxingGloryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kickboxing_glory")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "Glory踢拳规则",
            &[("回合", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KickboxingGloryRules::new();
        assert!(!r.explain().is_empty());
    }
}
