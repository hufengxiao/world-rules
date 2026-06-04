//! 长曲棍球规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: LacrosseDetailedRules, name: "长曲棍球规则", desc: "长曲棍球详细规则", origin: "FIL", tags: ["体育", "球类"] }
impl LacrosseDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["4节", "越位"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["传球", "射门"]
    }
}
impl Rule for LacrosseDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("lacrosse_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "长曲棍球规则",
            &[("比赛", &self.section_0()), ("技术", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LacrosseDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
