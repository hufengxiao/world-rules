//! 排球详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: VolleyballDetailedRules, name: "排球详细规则", desc: "排球详细比赛规则", origin: "FIVB", tags: ["体育", "球类"] }
impl VolleyballDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["6人轮转", "自由人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["触网", "过中线"]
    }
}
impl Rule for VolleyballDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("volleyball_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "排球详细规则",
            &[("轮转", &self.section_0()), ("犯规", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VolleyballDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
