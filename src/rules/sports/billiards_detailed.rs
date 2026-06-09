//! 台球详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BilliardsDetailedRules,
    name: "台球详细规则",
    desc: "台球详细比赛规则",
    origin: "WPBSA",
    tags: ["体育", "桌球"],
    category: RuleCategory::sports("billiards_detailed"),
    sections: [("斯诺克", section_0), ("九球", section_1)]
}

impl BilliardsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["15红球", "清台"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["开球", "赢球局"]
    }
}
