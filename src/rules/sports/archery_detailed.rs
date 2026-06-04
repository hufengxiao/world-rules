//! 射箭详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ArcheryDetailedRules, name: "射箭详细规则", desc: "射箭详细比赛规则", origin: "WA", tags: ["体育", "精准"] }
impl ArcheryDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["70米靶", "10环制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["18米靶", "淘汰赛"]
    }
}
impl Rule for ArcheryDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("archery_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "射箭详细规则",
            &[("室外", &self.section_0()), ("室内", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ArcheryDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
