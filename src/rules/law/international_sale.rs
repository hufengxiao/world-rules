//! 国际货物销售法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalSaleRules, name: "国际货物销售法", desc: "CISG国际销售合同", origin: "国际", tags: ["法律", "商法"] }
impl InternationalSaleRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["适用范围"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["买方卖方义务"]
    }
}
impl Rule for InternationalSaleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_sale")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际货物销售法",
            &[("适用", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalSaleRules::new();
        assert!(!r.explain().is_empty());
    }
}
