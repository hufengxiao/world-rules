//! 数独变体规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: SudokuVariantRules, name: "数独变体规则", desc: "数独变体游戏规则", origin: "国际", tags: ["游戏", "益智"] }
impl SudokuVariantRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["区域内数字和指定", "不重复规则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["对角线也需1-9不重复"]
    }
}
impl Rule for SudokuVariantRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("sudoku_variant")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "数独变体规则",
            &[
                ("杀手数独", &self.section_0()),
                ("对角线数独", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SudokuVariantRules::new();
        assert!(!r.explain().is_empty());
    }
}
