//! 司法鉴定法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ForensicEvidenceRules, name: "司法鉴定法", desc: "司法鉴定法律规则", origin: "中国", tags: ["法律", "证据"] }
impl ForensicEvidenceRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["DNA指纹"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["鉴定程序"]
    }
}
impl Rule for ForensicEvidenceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("forensic_evidence")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "司法鉴定法",
            &[("类型", &self.section_0()), ("程序", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ForensicEvidenceRules::new();
        assert!(!r.explain().is_empty());
    }
}
