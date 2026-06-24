//! 人体工程学规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ErgonomicsRules, name: "人体工程学规则", desc: "人体工程学规则", origin: "国际", tags: ["健康", "工作"] }
impl ErgonomicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["桌椅屏幕"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["定时休息"]
    }
}
impl Rule for ErgonomicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("ergonomics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "人体工程学规则",
            &[("工作站", &self.section_0()), ("休息", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ErgonomicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
