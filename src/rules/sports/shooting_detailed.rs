//! 射击详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ShootingDetailedRules,
    name: "射击详细规则",
    desc: "射击详细比赛规则",
    origin: "ISSF",
    tags: ["体育", "精准"],
    category: RuleCategory::sports("shooting_detailed"),
    sections: [("步枪", section_0), ("手枪", section_1)]
}

impl ShootingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["10米气步枪", "50米步枪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["10米气手枪", "25米速射"]
    }
}
