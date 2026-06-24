//! 乒乓球奥运规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TableTennisOlympicRules, name: "乒乓球奥运规则", desc: "乒乓球奥运会规则", origin: "国际", tags: ["体育", "球类"] }
impl TableTennisOlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["单打双打混双"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["11分制"]
    }
}
impl Rule for TableTennisOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("table_tennis_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "乒乓球奥运规则",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TableTennisOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
