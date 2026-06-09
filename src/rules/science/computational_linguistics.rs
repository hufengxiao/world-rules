//! 计算语言学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ComputationalLinguisticsRules,
    name: "计算语言学定律",
    desc: "计算语言学定律",
    origin: "国际",
    tags: ["科学", "语言"],
    category: RuleCategory::science("computational_linguistics"),
    sections: [("方法", section_0), ("应用", section_1)]
}

impl ComputationalLinguisticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["句法分析", "语义解析"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["机器翻译", "文本挖掘"]
    }
}
