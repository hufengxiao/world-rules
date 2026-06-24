//! 肝脏健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LiverHealthRules, name: "肝脏健康规则", desc: "肝脏健康规则", origin: "国际", tags: ["健康", "器官"] }
impl LiverHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["限酒"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["低脂饮食"]
    }
}
impl Rule for LiverHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("liver_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "肝脏健康规则",
            &[("保护", &self.section_0()), ("饮食", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LiverHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
