//! 药理学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: PharmacologyRules, name: "药理学定律", desc: "药理学定律", origin: "国际", tags: ["科学", "医学"] }
impl PharmacologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["药代动力学", "药效动力学"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["受体理论", "酶抑制"]
    }
}
impl Rule for PharmacologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("pharmacology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "药理学定律",
            &[("基础", &self.section_0()), ("分类", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PharmacologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
