//! 击剑详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: FencingDetailedRules, name: "击剑详细规则", desc: "击剑详细比赛规则", origin: "FIE", tags: ["体育", "格斗"] }
impl FencingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["花剑", "重剑", "佩剑"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["优先裁判权", "电子裁判"]
    }
}
impl Rule for FencingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("fencing_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "击剑详细规则",
            &[("剑种", &self.section_0()), ("得分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FencingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
