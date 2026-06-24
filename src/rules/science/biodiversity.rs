//! 生物多样性定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BiodiversityRules, name: "生物多样性定律", desc: "生物多样性定律", origin: "国际", tags: ["科学", "环境"] }
impl BiodiversityRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["基因物种生态"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["栖息地丧失"]
    }
}
impl Rule for BiodiversityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("biodiversity")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "生物多样性定律",
            &[("层次", &self.section_0()), ("威胁", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BiodiversityRules::new();
        assert!(!r.explain().is_empty());
    }
}
