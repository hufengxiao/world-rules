//! 电竞详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EsportsDetailedRules,
    name: "电竞详细规则",
    desc: "电子竞技详细比赛规则",
    origin: "IESF",
    tags: ["体育", "电子"],
    category: RuleCategory::sports("esports_detailed"),
    sections: [("MOBA", section_0), ("FPS", section_1)]
}

impl EsportsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["英雄选择", "BO3/BO5"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["回合制", "经济系统"]
    }
}
