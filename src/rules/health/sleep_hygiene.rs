//! 睡眠卫生规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SleepHygieneRules, name: "睡眠卫生规则", desc: "睡眠卫生规则", origin: "国际", tags: ["健康", "睡眠"] }
impl SleepHygieneRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["暗室安静"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["规律作息"]
    }
}
impl Rule for SleepHygieneRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("sleep_hygiene")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "睡眠卫生规则",
            &[("环境", &self.section_0()), ("习惯", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SleepHygieneRules::new();
        assert!(!r.explain().is_empty());
    }
}
