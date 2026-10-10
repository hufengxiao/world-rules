//! 圣托里尼棋（Santorini）
//!
//! 圣托里尼是两人对弈的建造棋，玩家每回合先移动工人再建造楼层，让自己的工人登上第三层即获胜。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SantoriniBuildRules,
    name: "圣托里尼棋（Santorini）",
    desc: "圣托里尼是两人对弈的建造棋，玩家每回合先移动工人再建造楼层，让自己的工人登上第三层即获胜。",
    origin: "由 Gordon Hamilton 于2004年设计",
    tags: ["棋类", "对战", "建造", "桌面游戏"]
}

impl SantoriniBuildRules {
    /// 棋盘与工人
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为五乘五方格",
            "每方两名工人棋子",
            "每格可堆叠至多三层",
            "四周为海不可越出",
        ]
    }

    /// 回合流程
    pub fn turn(&self) -> Vec<&'static str> {
        vec![
            "每回合先移动一名工人",
            "移动到相邻八格之一",
            "再在相邻格建造一层",
            "两步完成后轮到对手",
        ]
    }

    /// 移动限制
    pub fn moving(&self) -> Vec<&'static str> {
        vec![
            "只能移动一格到相邻空格",
            "不可移至有工人占据的格",
            "只能向上一层或同层或向下",
            "不可一次上升超过一层",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "工人登上第三层立即获胜",
            "对手无法移动则判负",
            "建造封堵对手为常见策略",
            "注意勿自封通路",
        ]
    }
}

impl Rule for SantoriniBuildRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("santorini")
    }

    fn explain(&self) -> String {
        format!(
            "【圣托里尼棋（Santorini）】\n{}",
            [
                format!(
                    "棋盘与工人：\\n{}",
                    self.board()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合流程：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "移动限制：\\n{}",
                    self.moving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负判定：\\n{}",
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
    fn test_santorinibuildrules_basic() {
        let rules = SantoriniBuildRules::new();
        assert_eq!(rules.metadata().name, "圣托里尼棋（Santorini）");
        assert!(!rules.board().is_empty());
        assert!(!rules.turn().is_empty());
        assert!(!rules.moving().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_santorinibuildrules_validation() {
        let rules = SantoriniBuildRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("santorini"));
    }

    #[test]
    fn test_santorinibuildrules_explain() {
        let rules = SantoriniBuildRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘与工人"));
        assert!(e.contains("回合流程"));
        assert!(e.contains("移动限制"));
    }
}
