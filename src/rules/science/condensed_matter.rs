//! 凝聚态物理定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CondensedMatterRules,
    name: "凝聚态物理定律",
    desc: "凝聚态物理定律",
    origin: "国际",
    tags: ["科学", "物理"],
    category: RuleCategory::science("condensed_matter"),
    sections: [("晶体", section_0), ("超导", section_1)]
}

impl CondensedMatterRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["布拉格衍射定律", "晶格振动声子", "能带理论"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["BCS理论", "迈斯纳效应", "约瑟夫森效应"]
    }
}
