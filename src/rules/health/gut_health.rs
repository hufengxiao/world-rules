//! 肠道健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GutHealthRules, name: "肠道健康规则", desc: "肠道健康规则", origin: "国际", tags: ["健康", "消化"] }
impl GutHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["益生菌纤维"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["规律饮食"]
    }
}
impl Rule for GutHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("gut_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "肠道健康规则",
            &[("饮食", &self.section_0()), ("习惯", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GutHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
