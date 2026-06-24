//! 眼疲劳防护规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EyeStrainRulesRules, name: "眼疲劳防护规则", desc: "眼疲劳防护规则", origin: "国际", tags: ["健康", "视力"] }
impl EyeStrainRulesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["20-20-20法则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["光线调节"]
    }
}
impl Rule for EyeStrainRulesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("eye_strain_rules")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "眼疲劳防护规则",
            &[("屏幕", &self.section_0()), ("环境", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EyeStrainRulesRules::new();
        assert!(!r.explain().is_empty());
    }
}
