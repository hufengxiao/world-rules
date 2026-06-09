//! 证券法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SecuritiesDetailedRules,
    name: "证券法详解",
    desc: "证券法详解",
    origin: "中国",
    tags: ["法律", "金融"],
    category: RuleCategory::law("securities_detailed"),
    sections: [("发行", section_0), ("交易", section_1)]
}

impl SecuritiesDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["IPO注册制", "信息披露"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["内幕交易", "操纵市场"]
    }
}
