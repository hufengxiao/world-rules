//! 居家办公礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WorkFromHomeRules, name: "居家办公礼仪", desc: "居家办公社交礼仪", origin: "国际", tags: ["社交", "职场"] }
impl WorkFromHomeRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["背景着装"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["及时回复"]
    }
}
impl Rule for WorkFromHomeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("work_from_home")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "居家办公礼仪",
            &[("视频", &self.section_0()), ("沟通", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WorkFromHomeRules::new();
        assert!(!r.explain().is_empty());
    }
}
