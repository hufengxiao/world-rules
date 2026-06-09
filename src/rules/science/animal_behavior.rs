//! 动物行为学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AnimalBehaviorRules,
    name: "动物行为学定律",
    desc: "动物行为学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("animal_behavior"),
    sections: [("本能", section_0), ("学习", section_1)]
}

impl AnimalBehaviorRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["固定动作模式", "释放机制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["条件反射", "印记", "观察学习"]
    }
}
