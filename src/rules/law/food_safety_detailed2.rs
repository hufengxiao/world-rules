//! 食品安全详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: FoodSafetyDetailed2Rules,
    name: "食品安全详解2",
    desc: "食品安全法详解2",
    origin: "中国",
    tags: ["法律", "食品"],
    category: RuleCategory::law("food_safety_detailed2"),
    sections: [("生产", section_0), ("检验", section_1)]
}

impl FoodSafetyDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["GMP", "HACCP"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["出厂检验", "抽检", "风险监测"]
    }
}
