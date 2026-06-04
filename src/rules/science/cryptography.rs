//! 密码学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CryptographyRules,
    name: "密码学定律",
    desc: "密码学定律",
    origin: "国际",
    tags: ["科学", "计算机"]
}

impl CryptographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["AES算法", "DES算法", "分组密码工作模式"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["RSA算法", "椭圆曲线密码", "数字签名"]
    }
}

impl Rule for CryptographyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("cryptography")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "密码学定律",
            &[
                ("对称加密", &self.section_0()),
                ("非对称加密", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cryptography_rules() {
        let r = CryptographyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
