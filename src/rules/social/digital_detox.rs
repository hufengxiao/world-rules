//! 数字排毒规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DigitalDetoxRules, name: "数字排毒规则", desc: "数字排毒健康规则", origin: "国际", tags: ["社交", "健康"] }
impl DigitalDetoxRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["屏幕时间"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["户外活动"]
    }
}
impl Rule for DigitalDetoxRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("digital_detox")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数字排毒规则",
            &[("限制", &self.section_0()), ("替代", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DigitalDetoxRules::new();
        assert!(!r.explain().is_empty());
    }
}
