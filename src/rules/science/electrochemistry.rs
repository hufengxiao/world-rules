//! 电化学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ElectrochemistryRules,
    name: "电化学定律",
    desc: "电化学定律",
    origin: "国际",
    tags: ["科学", "化学"],
    category: RuleCategory::science("electrochemistry"),
    sections: [("电极", section_0), ("应用", section_1)]
}

impl ElectrochemistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["能斯特方程", "电极电位", "超电势"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["原电池", "电解池", "燃料电池", "电镀"]
    }
}
