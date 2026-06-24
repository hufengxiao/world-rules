//! 证券法详解2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SecuritiesLawDetailedRules, name: "证券法详解2", desc: "证券法详解2", origin: "中国", tags: ["法律", "证券"] }
impl SecuritiesLawDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["IPO注册制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["内幕操纵"]
    }
}
impl Rule for SecuritiesLawDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("securities_law_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "证券法详解2",
            &[("发行", &self.section_0()), ("交易", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SecuritiesLawDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
