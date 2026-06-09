//! 生物地理学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BiogeographyRules,
    name: "生物地理学定律",
    desc: "生物地理学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("biogeography"),
    sections: [("分布", section_0), ("规律", section_1)]
}

impl BiogeographyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["隔离分化", "扩散", "板块构造"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["物种面积关系", "距离衰减"]
    }
}
