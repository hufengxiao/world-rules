//! 密码学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CryptographyDetailedRules, name: "密码学详细定律", desc: "密码学详细定律", origin: "国际", tags: ["科学", "计算机"] }
impl CryptographyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["AES DES"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["RSA ECC"]
    }
}
impl Rule for CryptographyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("cryptography_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "密码学详细定律",
            &[("对称", &self.section_0()), ("非对称", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CryptographyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
