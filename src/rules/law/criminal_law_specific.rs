//! 刑法分则详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CriminalLawSpecificRules, name: "刑法分则详解", desc: "刑法分则详解", origin: "中国", tags: ["法律", "刑法"] }
impl CriminalLawSpecificRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["故意杀人伤害"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["盗窃诈骗抢劫"]
    }
}
impl Rule for CriminalLawSpecificRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_law_specific")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑法分则详解",
            &[
                ("侵犯人身", &self.section_0()),
                ("侵犯财产", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalLawSpecificRules::new();
        assert!(!r.explain().is_empty());
    }
}
