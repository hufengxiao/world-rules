//! 信息论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: InformationTheoryRules,
    name: "信息论定律",
    desc: "香农信息论定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("information_theory"),
    sections: [("基础", section_0), ("编码", section_1)]
}

impl InformationTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["信息熵", "信道容量", "数据压缩"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["哈夫曼编码", "纠错码"]
    }
}
