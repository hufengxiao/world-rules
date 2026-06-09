//! 举重详细规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: WeightliftingDetailedRules,
    name: "举重详细规则",
    desc: "举重详细比赛规则",
    origin: "IWF",
    tags: ["体育", "力量"],
    category: RuleCategory::sports("weightlifting_detailed"),
    sections: [("抓举", section_0), ("挺举", section_1)]
}

impl WeightliftingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["单次过头", "3次试举"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["翻站挺"]
    }
}
