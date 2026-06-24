//! 废弃物管理定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WasteManagementRules, name: "废弃物管理定律", desc: "废弃物管理定律", origin: "国际", tags: ["科学", "环境"] }
impl WasteManagementRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["焚烧填埋"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["回收利用"]
    }
}
impl Rule for WasteManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("waste_management")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "废弃物管理定律",
            &[("处理", &self.section_0()), ("减量", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WasteManagementRules::new();
        assert!(!r.explain().is_empty());
    }
}
