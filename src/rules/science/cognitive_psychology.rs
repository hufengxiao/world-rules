//! 认知心理学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CognitivePsychologyRules,
    name: "认知心理学定律",
    desc: "认知心理学定律",
    origin: "国际",
    tags: ["科学", "心理学"],
    category: RuleCategory::science("cognitive_psychology"),
    sections: [("注意", section_0), ("记忆", section_1)]
}

impl CognitivePsychologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["选择性注意", "注意资源有限", "非注意盲视"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工作记忆模型", "遗忘曲线", "编码特异性"]
    }
}
