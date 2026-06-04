//! 教育法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: EducationDetailedRules, name: "教育法详解", desc: "教育法详解", origin: "中国", tags: ["法律", "教育"] }
impl EducationDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["办学条件", "教师资格"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["义务教育", "高等教育"]
    }
}
impl Rule for EducationDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("education_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "教育法详解",
            &[("学校", &self.section_0()), ("制度", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EducationDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
