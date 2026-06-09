//! 空手道详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: KarateDetailedRules,
    name: "空手道详细规则",
    desc: "空手道详细比赛规则",
    origin: "WKF",
    tags: ["体育", "格斗"],
    category: RuleCategory::sports("karate_detailed"),
    sections: [("组手", section_0), ("型", section_1)]
}

impl KarateDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["得分区域", "犯规"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["指定型", "评分标准"]
    }
}
