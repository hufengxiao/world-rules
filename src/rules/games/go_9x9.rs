//! 围棋9路盘规则
//!
//! 9x9小棋盘入门变体，适合初学者和快节奏对局。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: Go9x9Rules,
    name: "围棋9路盘规则",
    desc: "围棋9路盘（9x9）入门变体规则",
    origin: "日本",
    tags: ["游戏", "棋类", "围棋", "入门"]
}

impl Go9x9Rules {
    /// 获取基本规则章节
    pub fn section_basic(&self) -> Vec<&'static str> {
        vec![
            "棋盘大小: 9×9=81个交叉点",
            "黑先白后，轮流落子于交叉点",
            "落子后不可移动，除非被提走",
            "气: 棋子相邻的空交叉点（上下左右）",
            "无气的棋子被提走",
            "禁止自杀: 不能下无气的点，除非能提对方子",
        ]
    }

    /// 获取计分规则章节
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "中国规则（数子法）: 地盘+棋子数",
            "日本规则（数目法）: 围空-提子数",
            "贴目: 白方获得5.5目补偿",
            "终局: 双方连续pass后计算",
            "9路盘适合初学者练习基本技巧",
        ]
    }

    /// 获取策略章节
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "9路盘节奏快，平均15-20分钟",
            "先手优势明显，需要贴目补偿",
            "边角战术重要，中腹较少",
            "适合练习死活和手筋",
            "常见开局: 星位、小目",
        ]
    }

    /// 获取9路盘贴目值
    pub fn komi(&self) -> f32 {
        5.5 // 9路盘标准贴目
    }
}

impl Rule for Go9x9Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_9x9")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "围棋9路盘规则",
            &[
                ("基本规则", &self.section_basic()),
                ("计分规则", &self.section_scoring()),
                ("策略特点", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_9x9_rules() {
        let rules = Go9x9Rules::new();
        assert_eq!(rules.metadata().name, "围棋9路盘规则");
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("9×9"));
        assert!(rules.explain().contains("5.5目"));
    }

    #[test]
    fn test_komi() {
        let rules = Go9x9Rules::new();
        assert_eq!(rules.komi(), 5.5);
    }

    #[test]
    fn test_category() {
        let rules = Go9x9Rules::new();
        assert!(matches!(rules.category(), RuleCategory::Games(_)));
    }
}
