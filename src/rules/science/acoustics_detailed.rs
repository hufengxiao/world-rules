//! 声学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AcousticsDetailedRules, name: "声学详细定律", desc: "声学详细定律", origin: "国际", tags: ["科学", "物理"] }
impl AcousticsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["声波方程", "多普勒效应"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["超声波", "噪声控制"]
    }
}
impl Rule for AcousticsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("acoustics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "声学详细定律",
            &[("波动", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AcousticsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
