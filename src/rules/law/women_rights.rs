//! 妇女权利法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WomenRightsRules, name: "妇女权利法", desc: "妇女权利保障法", origin: "国际", tags: ["法律", "妇女"] }
impl WomenRightsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["消除歧视"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["就业保护"]
    }
}
impl Rule for WomenRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("women_rights")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "妇女权利法",
            &[("公约", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WomenRightsRules::new();
        assert!(!r.explain().is_empty());
    }
}
