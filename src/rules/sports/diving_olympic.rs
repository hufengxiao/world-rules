//! 跳水奥运规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DivingOlympicRules, name: "跳水奥运规则", desc: "跳水奥运会规则", origin: "国际", tags: ["体育", "水上"] }
impl DivingOlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3米板10米台"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["难度系数"]
    }
}
impl Rule for DivingOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("diving_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "跳水奥运规则",
            &[("项目", &self.section_0()), ("评分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DivingOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
