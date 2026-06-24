//! 反垄断法详解3
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AntiMonopolyDetailedLawRules, name: "反垄断法详解3", desc: "反垄断法详解3", origin: "中国", tags: ["法律", "竞争"] }
impl AntiMonopolyDetailedLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["协议支配地位集中"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["罚款拆分"]
    }
}
impl Rule for AntiMonopolyDetailedLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("anti_monopoly_detailed_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "反垄断法详解3",
            &[("垄断", &self.section_0()), ("执法", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AntiMonopolyDetailedLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
