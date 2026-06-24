//! 古生物学地球定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PaleontologyEarthRules, name: "古生物学地球定律", desc: "古生物学地球定律", origin: "国际", tags: ["科学", "地球"] }
impl PaleontologyEarthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["标准化石"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["大灭绝事件"]
    }
}
impl Rule for PaleontologyEarthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("paleontology_earth")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "古生物学地球定律",
            &[("化石", &self.section_0()), ("灭绝", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PaleontologyEarthRules::new();
        assert!(!r.explain().is_empty());
    }
}
