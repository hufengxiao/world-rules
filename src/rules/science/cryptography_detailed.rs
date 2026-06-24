//! 密码学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CryptographyDetailedRules, name: "密码学详细定律", desc: "密码学详细定律", origin: "国际", tags: ["科学", "计算机"] }
impl CryptographyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "AES:高级加密标准分组长度128位密钥128/192/256位",
            "DES:数据加密标准已不安全密钥56位",
            "分组模式:ECB/CBC/CTR/GCM",
            "流密码:RC4(已不安全)/ChaCha20",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "RSA:基于大数分解困难性密钥2048位以上",
            "ECC:椭圆曲线密码密钥更短安全性更高",
            "Diffie-Hellman:密钥交换协议",
            "数字签名:RSA签名/ECDSA签名",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "SHA-256:输出256位抗碰撞",
            "SHA-3:Keccak算法",
            "MD5:已不安全可被碰撞",
            "HMAC:基于哈希的消息认证码",
            "密码哈希:bcrypt/scrypt/Argon2",
        ]
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
            &[
                ("对称加密", &self.section_0()),
                ("非对称加密", &self.section_1()),
                ("哈希函数", &self.section_2()),
            ],
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
