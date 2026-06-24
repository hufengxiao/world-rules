//! 国际劳工法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LaborInternationalRules, name: "国际劳工法", desc: "国际劳工法律规则", origin: "国际", tags: ["法律", "劳动"] }
impl LaborInternationalRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["结社自由废除童工"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["最低工资"]
    }
}
impl Rule for LaborInternationalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor_international")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际劳工法",
            &[("公约", &self.section_0()), ("标准", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LaborInternationalRules::new();
        assert!(!r.explain().is_empty());
    }
}
