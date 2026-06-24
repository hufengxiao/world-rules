//! 赛艇奥运规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RowingOlympicRules, name: "赛艇奥运规则", desc: "赛艇奥运会规则", origin: "国际", tags: ["体育", "水上"] }
impl RowingOlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["单人双桨八人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["6航道"]
    }
}
impl Rule for RowingOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rowing_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "赛艇奥运规则",
            &[("项目", &self.section_0()), ("航道", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RowingOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
