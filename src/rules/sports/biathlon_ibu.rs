//! 冬季两项IBU规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BiathlonIbuRules, name: "冬季两项IBU规则", desc: "IBU冬季两项规则", origin: "国际", tags: ["体育", "冬季"] }
impl BiathlonIbuRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["卧射立射罚圈"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["经典自由"]
    }
}
impl Rule for BiathlonIbuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("biathlon_ibu")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "冬季两项IBU规则",
            &[("射击", &self.section_0()), ("越野", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BiathlonIbuRules::new();
        assert!(!r.explain().is_empty());
    }
}
