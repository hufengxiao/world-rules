//! 人工智能法规
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AiRegulationRules,
    name: "人工智能法规",
    desc: "人工智能法律规则",
    origin: "国际",
    tags: ["法律", "科技"],
    category: RuleCategory::law("ai_regulation"),
    sections: [("基本原则", section_0), ("应用限制", section_1)]
}

impl AiRegulationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["透明可解释", "公平无歧视", "安全可控"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["人脸识别限制", "自动化决策审查", "深度伪造监管"]
    }
}
