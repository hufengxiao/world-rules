//! 公益诉讼法
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: PublicInterestLitigationRules, name: "公益诉讼法", desc: "公益诉讼法律规则", origin: "中国", tags: ["法律", "诉讼"] }
impl PublicInterestLitigationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["环境公益", "消费者公益"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["诉前程序", "举证责任"]
    }
}
impl Rule for PublicInterestLitigationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("public_interest_litigation")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "公益诉讼法",
            &[("类型", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PublicInterestLitigationRules::new();
        assert!(!r.explain().is_empty());
    }
}
