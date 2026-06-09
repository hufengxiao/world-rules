//! 泛函分析定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: FunctionalAnalysisRules,
    name: "泛函分析定律",
    desc: "泛函分析定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("functional_analysis"),
    sections: [("空间", section_0), ("算子", section_1)]
}

impl FunctionalAnalysisRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["巴拿赫空间", "希尔伯特空间"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有界算子", "紧算子", "谱理论"]
    }
}
