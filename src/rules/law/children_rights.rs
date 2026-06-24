//! 儿童权利法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChildrenRightsRules, name: "儿童权利法", desc: "儿童权利保障法", origin: "国际", tags: ["法律", "儿童"] }
impl ChildrenRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["儿童权利公约"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["最佳利益原则"]
    }
}
impl Rule for ChildrenRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("children_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "儿童权利法",
            &[("公约", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChildrenRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
