//! 证券法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: SecuritiesDetailed2Rules, name: "证券法详解2", desc: "证券法详解2", origin: "中国", tags: ["法律", "金融"] }
impl SecuritiesDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["注册制", "信息披露"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["内幕交易", "操纵市场"]
    }
}
impl Rule for SecuritiesDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("securities_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "证券法详解2",
            &[("发行", &self.section_0()), ("监管", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SecuritiesDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
