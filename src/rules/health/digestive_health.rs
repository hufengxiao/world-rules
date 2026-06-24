//! 消化健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DigestiveHealthRules, name: "消化健康规则", desc: "消化健康护理规则", origin: "国际", tags: ["健康", "消化"] }
impl DigestiveHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["高纤维"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["规律饮食"]
    }
}
impl Rule for DigestiveHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("digestive_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "消化健康规则",
            &[("饮食", &self.section_0()), ("习惯", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DigestiveHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
