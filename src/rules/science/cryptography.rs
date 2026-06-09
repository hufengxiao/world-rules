//! 密码学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CryptographyRules,
    name: "密码学定律",
    desc: "密码学定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("cryptography"),
    sections: [("对称加密", section_0), ("非对称加密", section_1)]
}

impl CryptographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["AES算法", "DES算法", "分组密码工作模式"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["RSA算法", "椭圆曲线密码", "数字签名"]
    }
}
