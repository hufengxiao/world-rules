//! 食品安全详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: FoodSafetyDetailedRules,
    name: "食品安全详解",
    desc: "食品安全法详解",
    origin: "中国",
    tags: ["法律", "食品"],
    category: RuleCategory::law("food_safety_detailed"),
    sections: [("标准", section_0), ("监管", section_1)]
}

impl FoodSafetyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["安全标准", "添加剂", "标签"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生产许可", "抽检制度"]
    }
}
