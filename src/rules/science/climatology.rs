//! 气候学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ClimatologyRules, name: "气候学定律", desc: "气候学定律", origin: "国际", tags: ["科学", "环境"] }
impl ClimatologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["柯本气候分类"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["气候变化"]
    }
}
impl Rule for ClimatologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("climatology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "气候学定律",
            &[("分类", &self.section_0()), ("变化", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ClimatologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
