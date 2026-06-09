//! 体操详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: GymnasticsDetailedRules,
    name: "体操详细规则",
    desc: "体操详细比赛规则",
    origin: "FIG",
    tags: ["体育", "体操"],
    category: RuleCategory::sports("gymnastics_detailed"),
    sections: [("评分", section_0), ("项目", section_1)]
}

impl GymnasticsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["难度分", "完成分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["跳马", "高低杠"]
    }
}
