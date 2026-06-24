//! 大脑健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BrainHealthRules, name: "大脑健康规则", desc: "大脑健康规则", origin: "国际", tags: ["健康", "器官"] }
impl BrainHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["充足睡眠"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["脑力锻炼"]
    }
}
impl Rule for BrainHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("brain_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "大脑健康规则",
            &[("保护", &self.section_0()), ("锻炼", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BrainHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
