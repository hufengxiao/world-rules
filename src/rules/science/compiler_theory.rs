//! 编译器理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: CompilerTheoryRules,
    name: "编译器理论",
    desc: "编译器设计理论",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("compiler_theory"),
    sections: [("前端", section_0), ("后端", section_1)]
}

impl CompilerTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["词法分析", "语法分析", "语义分析"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["代码优化", "目标代码生成"]
    }
}
