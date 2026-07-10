//! 围棋13路盘规则
//!
//! 13x13中棋盘变体，介于入门和标准之间。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: Go13x13Rules,
    name: "围棋13路盘规则",
    desc: "围棋13路盘（13x13）中棋盘变体规则",
    origin: "日本",
    tags: ["游戏", "棋类", "围棋", "进阶"]
}

impl Go13x13Rules {
    /// 获取基本规则章节
    pub fn section_basic(&self) -> Vec<&'static str> {
        vec![
            "棋盘大小: 13×13=169个交叉点",
            "黑先白后，轮流落子于交叉点",
            "落子后不可移动，除非被提走",
            "气: 棋子相邻的空交叉点",
            "无气的棋子被提走",
            "禁止自杀: 不能下无气的点",
        ]
    }

    /// 获取计分规则章节
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "中国规则（数子法）: 地盘+棋子",
            "日本规则（数目法）: 围空-提子",
            "贴目: 白方获得5.5目补偿",
            "终局: 双方连续pass后计算",
            "13路盘是入门到标准的过渡",
        ]
    }

    /// 获取特点章节
    pub fn section_features(&self) -> Vec<&'static str> {
        vec![
            "对局时长: 约30-45分钟",
            "兼顾边角和中腹战术",
            "适合有一定基础的棋手",
            "降低复杂度，保持围棋精髓",
            "常见星位布局",
        ]
    }

    /// 获取13路盘贴目值
    pub fn komi(&self) -> f32 {
        5.5
    }
}

impl Rule for Go13x13Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_13x13")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "围棋13路盘规则",
            &[
                ("基本规则", &self.section_basic()),
                ("计分规则", &self.section_scoring()),
                ("特点", &self.section_features()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_13x13_rules() {
        let rules = Go13x13Rules::new();
        assert_eq!(rules.metadata().name, "围棋13路盘规则");
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("13×13"));
        assert!(rules.explain().contains("169"));
    }

    #[test]
    fn test_komi() {
        let rules = Go13x13Rules::new();
        assert_eq!(rules.komi(), 5.5);
    }
}
