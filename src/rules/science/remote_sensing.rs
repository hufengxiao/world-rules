//! 遥感定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RemoteSensingRules, name: "遥感定律", desc: "遥感科学定律", origin: "国际", tags: ["科学", "地球"] }
impl RemoteSensingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["电磁波"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["卫星遥感"]
    }
}
impl Rule for RemoteSensingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("remote_sensing")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "遥感定律",
            &[("原理", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RemoteSensingRules::new();
        assert!(!r.explain().is_empty());
    }
}
