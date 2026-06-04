//! 体操详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: GymnasticsDetailedRules, name: "体操详细规则", desc: "体操详细比赛规则", origin: "FIG", tags: ["体育", "体操"] }
impl GymnasticsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["难度分", "完成分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["跳马", "高低杠"]
    }
}
impl Rule for GymnasticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("gymnastics_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "体操详细规则",
            &[("评分", &self.section_0()), ("项目", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GymnasticsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
