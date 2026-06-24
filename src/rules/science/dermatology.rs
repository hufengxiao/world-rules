//! 皮肤病学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DermatologyRules, name: "皮肤病学定律", desc: "皮肤病学定律", origin: "国际", tags: ["科学", "医学"] }
impl DermatologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["湿疹银屑病"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["外用系统"]
    }
}
impl Rule for DermatologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("dermatology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "皮肤病学定律",
            &[("常见", &self.section_0()), ("治疗", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DermatologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
