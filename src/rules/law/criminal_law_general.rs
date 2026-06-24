//! 刑法总则详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CriminalLawGeneralRules, name: "刑法总则详解", desc: "刑法总则详解", origin: "中国", tags: ["法律", "刑法"] }
impl CriminalLawGeneralRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["犯罪构成"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["主刑附加刑"]
    }
}
impl Rule for CriminalLawGeneralRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_law_general")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑法总则详解",
            &[("犯罪", &self.section_0()), ("刑罚", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalLawGeneralRules::new();
        assert!(!r.explain().is_empty());
    }
}
