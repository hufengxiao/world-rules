//! 短道ISU规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ShortTrackIsuRules, name: "短道ISU规则", desc: "ISU短道速滑规则", origin: "国际", tags: ["体育", "冬季"] }
impl ShortTrackIsuRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["500m1000m1500m接力"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["横切推人"]
    }
}
impl Rule for ShortTrackIsuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("short_track_isu")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "短道ISU规则",
            &[("项目", &self.section_0()), ("犯规", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ShortTrackIsuRules::new();
        assert!(!r.explain().is_empty());
    }
}
