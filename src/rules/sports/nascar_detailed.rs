//! NASCAR详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NascarDetailedRules, name: "NASCAR详细规则", desc: "NASCAR详细比赛规则", origin: "美国", tags: ["体育", "赛车"] }
impl NascarDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["季后赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["黄旗绿旗"]
    }
}
impl Rule for NascarDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("nascar_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NASCAR详细规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NascarDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
