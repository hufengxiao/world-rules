//! 孕期健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PrenatalHealthRules, name: "孕期健康规则", desc: "孕期健康规则", origin: "国际", tags: ["健康", "孕期"] }
impl PrenatalHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["叶酸铁"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["产检"]
    }
}
impl Rule for PrenatalHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("prenatal_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "孕期健康规则",
            &[("营养", &self.section_0()), ("检查", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PrenatalHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
