//! 刑法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CriminalDetailedRules,
    name: "刑法详解",
    desc: "刑法罪名详解",
    origin: "中国",
    tags: ["法律", "刑法"],
    category: RuleCategory::law("criminal_detailed"),
    sections: [("侵犯人身", section_0), ("侵犯财产", section_1)]
}

impl CriminalDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["故意杀人", "故意伤害"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["盗窃", "诈骗", "抢劫"]
    }
}
