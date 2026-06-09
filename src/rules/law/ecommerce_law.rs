//! 电子商务法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EcommerceLawRules,
    name: "电子商务法",
    desc: "电子商务法律规则",
    origin: "中国",
    tags: ["法律", "电商"],
    category: RuleCategory::law("ecommerce_law"),
    sections: [("经营者义务", section_0), ("平台责任", section_1)]
}

impl EcommerceLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["市场主体登记", "纳税义务", "信息公示"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["平台治理责任", "知识产权保护", "消费者权益"]
    }
}
