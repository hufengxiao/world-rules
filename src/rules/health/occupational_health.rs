//! 职业健康规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: OccupationalHealthRules,
    name: "职业健康规则",
    desc: "职业健康与安全规则",
    origin: "国际",
    tags: ["健康", "职业"],
    category: RuleCategory::health("occupational_health"),
    sections: [("防护", section_0), ("心理", section_1)]
}

impl OccupationalHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["个人防护装备", "工作环境通风", "噪音防护"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工作压力管理", "职业倦怠预防", "工作生活平衡"]
    }
}
