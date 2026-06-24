//! 跳台滑雪FIS
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SkiJumpingFisDetailedRules, name: "跳台滑雪FIS", desc: "FIS跳台滑雪规则", origin: "国际", tags: ["体育", "冬季"] }
impl SkiJumpingFisDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["距离分姿态分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["标准台大台"]
    }
}
impl Rule for SkiJumpingFisDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ski_jumping_fis_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "跳台滑雪FIS",
            &[("评分", &self.section_0()), ("台级", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SkiJumpingFisDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
