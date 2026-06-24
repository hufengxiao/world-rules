//! 微生物学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MicrobiologyDetailedRules, name: "微生物学详细定律", desc: "微生物学定律", origin: "国际", tags: ["科学", "生物"] }
impl MicrobiologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["生长代谢"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["复制周期"]
    }
}
impl Rule for MicrobiologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("microbiology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "微生物学详细定律",
            &[("细菌", &self.section_0()), ("病毒", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MicrobiologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
