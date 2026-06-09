//! 心理健康维护规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MentalWellnessRules,
    name: "心理健康维护规则",
    desc: "心理健康维护规则",
    origin: "国际",
    tags: ["健康", "心理"],
    category: RuleCategory::health("mental_wellness"),
    sections: [("自我调节", section_0), ("求助", section_1)]
}

impl MentalWellnessRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["正念冥想", "深呼吸", "规律作息"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["识别心理问题", "寻求专业帮助", "不讳疾忌医"]
    }
}
