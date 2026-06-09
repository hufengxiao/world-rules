//! 代数学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AlgebraMathRules,
    name: "代数学定律",
    desc: "代数学定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("algebra_math"),
    sections: [("群论", section_0), ("线性代数", section_1)]
}

impl AlgebraMathRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["群的定义与性质", "拉格朗日定理", "同态基本定理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["特征值与特征向量", "矩阵分解", "线性变换"]
    }
}
