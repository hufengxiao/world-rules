//! 音乐理论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MusicTheoryRules,
    name: "音乐理论定律",
    desc: "音乐理论定律",
    origin: "国际",
    tags: ["科学", "艺术"],
    category: RuleCategory::science("music_theory"),
    sections: [("和声", section_0), ("节奏", section_1)]
}

impl MusicTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["和弦进行", "调性体系"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["节拍体系", "节奏型"]
    }
}
