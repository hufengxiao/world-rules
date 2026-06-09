//! 高分子化学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PolymerChemistryRules,
    name: "高分子化学定律",
    desc: "高分子化学定律",
    origin: "国际",
    tags: ["科学", "化学"],
    category: RuleCategory::science("polymer_chemistry"),
    sections: [("聚合", section_0), ("性质", section_1)]
}

impl PolymerChemistryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["加聚反应", "缩聚反应", "聚合动力学"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["玻璃化转变", "粘弹性", "降解与老化"]
    }
}
