//! 数论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: NumberTheoryRules,
    name: "数论定律",
    desc: "数论定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("number_theory"),
    sections: [("基本定理", section_0), ("猜想", section_1)]
}

impl NumberTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["算术基本定理", "费马小定理", "欧拉定理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["黎曼假设", "孪生素数猜想", "哥德巴赫猜想"]
    }
}
