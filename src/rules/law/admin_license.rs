//! 行政许可法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AdminLicenseRules, name: "行政许可法详解", desc: "行政许可法详解", origin: "中国", tags: ["法律", "行政"] }
impl AdminLicenseRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["许可设定"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["申请审批"]
    }
}
impl Rule for AdminLicenseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("admin_license")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "行政许可法详解",
            &[("设定", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AdminLicenseRules::new();
        assert!(!r.explain().is_empty());
    }
}
