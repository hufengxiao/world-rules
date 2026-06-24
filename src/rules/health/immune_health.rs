//! 免疫健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ImmuneHealthRules, name: "免疫健康规则", desc: "免疫健康护理规则", origin: "国际", tags: ["健康", "免疫"] }
impl ImmuneHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["维C锌"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["充足睡眠"]
    }
}
impl Rule for ImmuneHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("immune_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "免疫健康规则",
            &[("营养", &self.section_0()), ("生活", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ImmuneHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
