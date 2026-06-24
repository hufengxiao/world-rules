//! 病理学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PathologyRules, name: "病理学定律", desc: "病理学定律", origin: "国际", tags: ["科学", "医学"] }
impl PathologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["细胞损伤炎症"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["器官病理"]
    }
}
impl Rule for PathologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("pathology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "病理学定律",
            &[("总论", &self.section_0()), ("各论", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PathologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
