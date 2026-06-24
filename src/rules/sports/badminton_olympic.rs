//! 羽毛球奥运规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BadmintonOlympicRules, name: "羽毛球奥运规则", desc: "羽毛球奥运会规则", origin: "国际", tags: ["体育", "球类"] }
impl BadmintonOlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["小组赛淘汰赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["21分制"]
    }
}
impl Rule for BadmintonOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("badminton_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "羽毛球奥运规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BadmintonOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
