//! 力量举详细
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: PowerliftingDetailedRules, name: "力量举详细", desc: "力量举详细规则", origin: "IPF", tags: ["体育", "力量"] }
impl PowerliftingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["深蹲", "卧推", "硬拉"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["3次试举", "重量递增"]
    }
}
impl Rule for PowerliftingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("powerlifting_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "力量举详细",
            &[("项目", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PowerliftingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
