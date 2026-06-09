//! 儿童健康规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ChildrenHealthRules,
    name: "儿童健康规则",
    desc: "儿童健康护理规则",
    origin: "国际",
    tags: ["健康", "儿童"],
    category: RuleCategory::health("children_health"),
    sections: [("营养", section_0), ("安全", section_1)]
}

impl ChildrenHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["母乳喂养", "辅食添加", "均衡膳食"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["防跌落", "防误食", "防溺水"]
    }
}
