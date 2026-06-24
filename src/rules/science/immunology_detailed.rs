//! 免疫学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ImmunologyDetailedRules, name: "免疫学详细定律", desc: "免疫学定律", origin: "国际", tags: ["科学", "生物"] }
impl ImmunologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["屏障吞噬"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["T细胞B细胞"]
    }
}
impl Rule for ImmunologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("immunology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "免疫学详细定律",
            &[("先天", &self.section_0()), ("适应", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ImmunologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
