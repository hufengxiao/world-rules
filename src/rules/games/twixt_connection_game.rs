//! Twixt 连线棋
//!
//! Twixt 是两人对弈的连接棋，双方在点阵格上放置棋子并以骑士步连接，目标是把自己的两侧边用一条连续链路连接起来。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TwixtConnectionGameRules,
    name: "Twixt 连线棋",
    desc: "Twixt 是两人对弈的连接棋，双方在点阵格上放置棋子并以骑士步连接，目标是把自己的两侧边用一条连续链路连接起来。",
    origin: "由 Alex Randolph 于1962年发明",
    tags: ["棋类", "对战", "连接", "桌面游戏"]
}

impl TwixtConnectionGameRules {
    /// 棋盘与棋子
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为点阵网格",
            "双方各执一种颜色棋子",
            "每人目标连接自己的两侧边",
            "棋子落在格点交叉处",
        ]
    }

    /// 落子规则
    pub fn placing(&self) -> Vec<&'static str> {
        vec![
            "轮流在空格点放置一枚棋子",
            "落子后不可移动或移除",
            "双方棋子不可占据同一点",
            "落子须在棋盘范围内",
        ]
    }

    /// 连接规则
    pub fn linking(&self) -> Vec<&'static str> {
        vec![
            "两子以骑士步即1x2对角相连",
            "连线不得跨越对方棋子",
            "己方棋子间可形成链式连接",
            "被对方棋子阻断处不可连",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "先连通自己两侧边者胜",
            "对方无法阻断即告负",
            "连接须为连续无断裂链",
            "和棋少见一般需重赛",
        ]
    }
}

impl Rule for TwixtConnectionGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("twixt")
    }

    fn explain(&self) -> String {
        format!(
            "【Twixt 连线棋】\n{}",
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
                    "落子规则：\\n{}",
                    self.placing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "连接规则：\\n{}",
                    self.linking()
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
    fn test_twixtconnectiongamerules_basic() {
        let rules = TwixtConnectionGameRules::new();
        assert_eq!(rules.metadata().name, "Twixt 连线棋");
        assert!(!rules.board().is_empty());
        assert!(!rules.placing().is_empty());
        assert!(!rules.linking().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_twixtconnectiongamerules_validation() {
        let rules = TwixtConnectionGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("twixt"));
    }

    #[test]
    fn test_twixtconnectiongamerules_explain() {
        let rules = TwixtConnectionGameRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘与棋子"));
        assert!(e.contains("落子规则"));
        assert!(e.contains("连接规则"));
    }
}
