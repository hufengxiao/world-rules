//! 隔离棋（Quoridor）
//!
//! Quoridor 是两人竞速棋，玩家每回合选择移动己方棋子或放置挡板阻挠对手，先让自己的棋子抵达对方底线者获胜。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: QuoridorMazeRaceRules,
    name: "隔离棋（Quoridor）",
    desc: "Quoridor 是两人竞速棋，玩家每回合选择移动己方棋子或放置挡板阻挠对手，先让自己的棋子抵达对方底线者获胜。",
    origin: "由 Mirko Marchesi 于1997年设计",
    tags: ["棋类", "对战", "迷宫", "桌面游戏"]
}

impl QuoridorMazeRaceRules {
    /// 棋盘与棋子
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为九乘九格点阵",
            "双方各一枚棋子从底线出发",
            "每人持有十块挡板",
            "目标为抵达对方底线",
        ]
    }

    /// 回合行动
    pub fn action(&self) -> Vec<&'static str> {
        vec![
            "每回合二选一行动",
            "移动己方棋子一格",
            "或放置一块挡板",
            "行动后轮到对手",
        ]
    }

    /// 移动与挡板
    pub fn moving(&self) -> Vec<&'static str> {
        vec![
            "棋子可前后左右移动一格",
            "不可穿越挡板",
            "正面对峙时可斜跳越过",
            "挡板横竖放置阻隔两格间通行",
        ]
    }

    /// 限制与胜负
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "挡板不得完全封死对手通路",
            "挡板用尽后只能移动",
            "先抵达对方底线者胜",
            "无法移动则判负",
        ]
    }
}

impl Rule for QuoridorMazeRaceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("quoridor")
    }

    fn explain(&self) -> String {
        format!(
            "【隔离棋（Quoridor）】\n{}",
            [
                format!(
                    "棋盘与棋子：\\n{}",
                    self.board()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合行动：\\n{}",
                    self.action()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "移动与挡板：\\n{}",
                    self.moving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "限制与胜负：\\n{}",
                    self.winning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_quoridormazeracerules_basic() {
        let rules = QuoridorMazeRaceRules::new();
        assert_eq!(rules.metadata().name, "隔离棋（Quoridor）");
        assert!(!rules.board().is_empty());
        assert!(!rules.action().is_empty());
        assert!(!rules.moving().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_quoridormazeracerules_validation() {
        let rules = QuoridorMazeRaceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("quoridor"));
    }

    #[test]
    fn test_quoridormazeracerules_explain() {
        let rules = QuoridorMazeRaceRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘与棋子"));
        assert!(e.contains("回合行动"));
        assert!(e.contains("移动与挡板"));
    }
}
