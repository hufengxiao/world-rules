//! 组织行为学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: OrganizationalBehaviorRules,
    name: "组织行为学定律",
    desc: "组织行为学定律",
    origin: "国际",
    tags: ["科学", "管理"],
    category: RuleCategory::science("organizational_behavior"),
    sections: [("个体", section_0), ("群体", section_1)]
}

impl OrganizationalBehaviorRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["动机理论", "人格与工作匹配", "工作满意度"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["团队动力学", "领导力理论", "冲突管理"]
    }
}
