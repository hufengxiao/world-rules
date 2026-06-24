//! 速滑ISU规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SpeedSkatingIsuRules, name: "速滑ISU规则", desc: "ISU速度滑冰规则", origin: "国际", tags: ["体育", "冬季"] }
impl SpeedSkatingIsuRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["500m到10000m"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["内外道交换"]
    }
}
impl Rule for SpeedSkatingIsuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("speed_skating_isu")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "速滑ISU规则",
            &[("项目", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SpeedSkatingIsuRules::new();
        assert!(!r.explain().is_empty());
    }
}
