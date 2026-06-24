//! 癌症预防规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CancerPreventionRules, name: "癌症预防规则", desc: "癌症预防规则", origin: "国际", tags: ["健康", "预防"] }
impl CancerPreventionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["定期筛查"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["戒烟限酒"]
    }
}
impl Rule for CancerPreventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("cancer_prevention")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "癌症预防规则",
            &[("筛查", &self.section_0()), ("生活", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CancerPreventionRules::new();
        assert!(!r.explain().is_empty());
    }
}
