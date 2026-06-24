//! 网络安全定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CybersecurityRules, name: "网络安全定律", desc: "网络安全定律", origin: "国际", tags: ["科学", "计算机"] }
impl CybersecurityRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["SQL注入XSS"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["防火墙加密"]
    }
}
impl Rule for CybersecurityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("cybersecurity")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "网络安全定律",
            &[("攻击", &self.section_0()), ("防御", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CybersecurityRules::new();
        assert!(!r.explain().is_empty());
    }
}
