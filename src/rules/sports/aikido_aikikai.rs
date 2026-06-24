//! 合气道开祖规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AikidoAikikaiRules, name: "合气道开祖规则", desc: "合气道开祖流规则", origin: "日本", tags: ["体育", "格斗"] }
impl AikidoAikikaiRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["入身投"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["初段到十段"]
    }
}
impl Rule for AikidoAikikaiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("aikido_aikikai")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "合气道开祖规则",
            &[("技术", &self.section_0()), ("段位", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AikidoAikikaiRules::new();
        assert!(!r.explain().is_empty());
    }
}
