//! 反垄断法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AntimonopolyDetailedRules,
    name: "反垄断法详解",
    desc: "反垄断法详解",
    origin: "中国",
    tags: ["法律", "竞争"],
    category: RuleCategory::law("antimonopoly_detailed"),
    sections: [("垄断协议", section_0), ("经营者集中", section_1)]
}

impl AntimonopolyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["横向垄断", "纵向垄断"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["申报标准", "审查程序"]
    }
}
