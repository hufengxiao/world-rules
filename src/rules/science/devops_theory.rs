//! DevOps理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DevopsTheoryRules,
    name: "DevOps理论",
    desc: "DevOps工程理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("devops_theory"),
    sections: [("原则", section_0), ("实践", section_1)]
}

impl DevopsTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["持续集成持续部署", "基础设施即代码", "监控与可观测性"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["容器化与编排", "微服务架构", "自动化测试"]
    }
}
