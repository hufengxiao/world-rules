//! 国际人道法
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: HumanitarianLawRules, name: "国际人道法", desc: "国际人道法律规则", origin: "国际", tags: ["法律", "人道"] }
impl HumanitarianLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["区分原则", "比例原则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["战俘待遇", "平民保护"]
    }
}
impl Rule for HumanitarianLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("humanitarian_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "国际人道法",
            &[("战争法", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HumanitarianLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
