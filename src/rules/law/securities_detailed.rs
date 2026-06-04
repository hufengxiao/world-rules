//! 证券法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: SecuritiesDetailedRules, name: "证券法详解", desc: "证券法详解", origin: "中国", tags: ["法律", "金融"] }
impl SecuritiesDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["IPO注册制", "信息披露"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["内幕交易", "操纵市场"]
    }
}
impl Rule for SecuritiesDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("securities_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "证券法详解",
            &[("发行", &self.section_0()), ("交易", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SecuritiesDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
