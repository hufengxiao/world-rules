//! 五人制足球详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FutsalRules, name: "五人制足球详细", desc: "五人制足球详细规则", origin: "国际", tags: ["体育", "球类"] }
impl FutsalRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["无越位", "累计犯规"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["4秒规则"]
    }
}
impl Rule for FutsalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("futsal")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "五人制足球详细",
            &[("比赛", &self.section_0()), ("守门员", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FutsalRules::new();
        assert!(!r.explain().is_empty());
    }
}
