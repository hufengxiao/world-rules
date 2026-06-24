//! 剑道FIK详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KendoFikDetailedRules, name: "剑道FIK详细规则", desc: "剑道国际联盟规则", origin: "日本", tags: ["体育", "格斗"] }
impl KendoFikDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["面小手胴"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["气剑体一致"]
    }
}
impl Rule for KendoFikDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kendo_fik_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "剑道FIK详细规则",
            &[("打击", &self.section_0()), ("审查", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KendoFikDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
