//! 国际人道法详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternationalHumanitarianRules, name: "国际人道法详细", desc: "国际人道法律规则", origin: "国际", tags: ["法律", "人道"] }
impl InternationalHumanitarianRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["日内瓦公约"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["战俘平民"]
    }
}
impl Rule for InternationalHumanitarianRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("international_humanitarian")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国际人道法详细",
            &[("日内瓦", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternationalHumanitarianRules::new();
        assert!(!r.explain().is_empty());
    }
}
