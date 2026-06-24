//! 计算机视觉详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CvDetailedRules, name: "计算机视觉详细", desc: "计算机视觉定律", origin: "国际", tags: ["科学", "计算机"] }
impl CvDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["目标检测"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["语义分割"]
    }
}
impl Rule for CvDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("cv_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "计算机视觉详细",
            &[("检测", &self.section_0()), ("分割", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CvDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
