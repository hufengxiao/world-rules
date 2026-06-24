//! 环境诉讼法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EnvironmentalLitigationRules, name: "环境诉讼法", desc: "环境诉讼法律规则", origin: "国际", tags: ["法律", "环境"] }
impl EnvironmentalLitigationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["环境公益诉讼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生态赔偿"]
    }
}
impl Rule for EnvironmentalLitigationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("environmental_litigation")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "环境诉讼法",
            &[("公益", &self.section_0()), ("赔偿", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EnvironmentalLitigationRules::new();
        assert!(!r.explain().is_empty());
    }
}
