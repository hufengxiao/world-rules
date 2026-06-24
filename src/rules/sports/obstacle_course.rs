//! 障碍赛跑规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ObstacleCourseRules, name: "障碍赛跑规则", desc: "障碍赛跑竞赛规则", origin: "国际", tags: ["体育", "综合"] }
impl ObstacleCourseRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["斯巴达勇士赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["攀爬泥潭"]
    }
}
impl Rule for ObstacleCourseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("obstacle_course")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "障碍赛跑规则",
            &[("类型", &self.section_0()), ("障碍", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ObstacleCourseRules::new();
        assert!(!r.explain().is_empty());
    }
}
