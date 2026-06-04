//! 国际数据保护法
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: DataProtectionIntlRules, name: "国际数据保护法", desc: "国际数据保护法", origin: "国际", tags: ["法律", "数据"] }
impl DataProtectionIntlRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合法性基础", "数据主体权利"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["罚款", "影响评估"]
    }
}
impl Rule for DataProtectionIntlRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("data_protection_intl")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "国际数据保护法",
            &[("GDPR", &self.section_0()), ("执法", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DataProtectionIntlRules::new();
        assert!(!r.explain().is_empty());
    }
}
