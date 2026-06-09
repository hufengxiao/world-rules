//! 公益诉讼法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PublicInterestLitigationRules,
    name: "公益诉讼法",
    desc: "公益诉讼法律规则",
    origin: "中国",
    tags: ["法律", "诉讼"],
    category: RuleCategory::law("public_interest_litigation"),
    sections: [("类型", section_0), ("程序", section_1)]
}

impl PublicInterestLitigationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["环境公益", "消费者公益"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["诉前程序", "举证责任"]
    }
}
