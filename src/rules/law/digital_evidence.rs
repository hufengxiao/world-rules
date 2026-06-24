//! 数字证据法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DigitalEvidenceRules, name: "数字证据法", desc: "数字证据法律规则", origin: "国际", tags: ["法律", "证据"] }
impl DigitalEvidenceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["电子数据"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["取证保全"]
    }
}
impl Rule for DigitalEvidenceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("digital_evidence")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数字证据法",
            &[("类型", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DigitalEvidenceRules::new();
        assert!(!r.explain().is_empty());
    }
}
