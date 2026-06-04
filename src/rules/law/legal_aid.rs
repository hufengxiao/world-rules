//! 法律援助法
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: LegalAidRules, name: "法律援助法", desc: "法律援助法律规则", origin: "中国", tags: ["法律", "援助"] }
impl LegalAidRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["刑事辩护", "民事代理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["经济困难", "值班律师"]
    }
}
impl Rule for LegalAidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("legal_aid")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "法律援助法",
            &[("范围", &self.section_0()), ("条件", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LegalAidRules::new();
        assert!(!r.explain().is_empty());
    }
}
