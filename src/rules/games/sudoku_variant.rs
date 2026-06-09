//! 数独变体规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SudokuVariantRules,
    name: "数独变体规则",
    desc: "数独变体游戏规则",
    origin: "国际",
    tags: ["游戏", "益智"],
    category: RuleCategory::games("sudoku_variant"),
    sections: [("杀手数独", section_0), ("对角线数独", section_1)]
}

impl SudokuVariantRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["区域内数字和指定", "不重复规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["对角线也需1-9不重复"]
    }
}
