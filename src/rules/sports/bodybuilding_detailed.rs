//! 健美详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BodybuildingDetailedRules, name: "健美详细规则", desc: "健美比赛详细规则", origin: "IFBB", tags: ["体育", "健身"] }
impl BodybuildingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["肌肉量", "对称性"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["规定造型", "自由造型"]
    }
}
impl Rule for BodybuildingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bodybuilding_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "健美详细规则",
            &[("评分", &self.section_0()), ("造型", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BodybuildingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
