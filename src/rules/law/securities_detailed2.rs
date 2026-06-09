//! 证券法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SecuritiesDetailed2Rules,
    name: "证券法详解2",
    desc: "证券法详解2",
    origin: "中国",
    tags: ["法律", "金融"],
    category: RuleCategory::law("securities_detailed2"),
    sections: [("发行", section_0), ("监管", section_1)]
}

impl SecuritiesDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["注册制", "信息披露"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["内幕交易", "操纵市场"]
    }
}
