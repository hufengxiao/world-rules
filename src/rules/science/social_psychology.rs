//! 社会心理学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SocialPsychologyRules,
    name: "社会心理学定律",
    desc: "社会心理学定律",
    origin: "国际",
    tags: ["科学", "心理学"],
    category: RuleCategory::science("social_psychology"),
    sections: [("认知", section_0), ("影响", section_1)]
}

impl SocialPsychologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["归因理论", "认知失调", "刻板印象"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["从众效应", "服从权威", "社会促进"]
    }
}
