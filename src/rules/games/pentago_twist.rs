//! 旋转五子棋（Pentago）
//!
//! Pentago 是带旋转机制的连子棋，棋盘分为四个可转动的象限，玩家落子后须旋转一个象限，先连成五子者获胜。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PentagoTwistRules,
    name: "旋转五子棋（Pentago）",
    desc: "Pentago 是带旋转机制的连子棋，棋盘分为四个可转动的象限，玩家落子后须旋转一个象限，先连成五子者获胜。",
    origin: "由瑞典 Mindtwister 公司于2005年发行",
    tags: ["棋类", "对战", "旋转", "桌面游戏"]
}

impl PentagoTwistRules {
    /// 棋盘构成
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为六乘六方格",
            "分为四个三乘三象限",
            "象限可各自旋转",
            "黑白棋子分属两方",
        ]
    }

    /// 行棋流程
    pub fn turn(&self) -> Vec<&'static str> {
        vec![
            "先在空位放置一枚己方棋子",
            "随后旋转任一象限九十度",
            "旋转可为顺时针或逆时针",
            "两步构成一个完整回合",
        ]
    }

    /// 旋转规则
    pub fn twisting(&self) -> Vec<&'static str> {
        vec![
            "旋转时该象限棋子随之移动",
            "旋转不改变棋子归属",
            "可故意用旋转打乱对方连子",
            "旋转亦可促成本方连子",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "横竖斜连成五子者获胜",
            "旋转后形成连子同样有效",
            "落子后旋转前已成五子立即胜",
            "同时成五子则后手胜",
        ]
    }
}

impl Rule for PentagoTwistRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("pentago")
    }

    fn explain(&self) -> String {
        format!(
            "【旋转五子棋（Pentago）】\n{}",
            [
                format!(
                    "棋盘构成：\\n{}",
                    self.board()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行棋流程：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "旋转规则：\\n{}",
                    self.twisting()
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
    fn test_pentagotwistrules_basic() {
        let rules = PentagoTwistRules::new();
        assert_eq!(rules.metadata().name, "旋转五子棋（Pentago）");
        assert!(!rules.board().is_empty());
        assert!(!rules.turn().is_empty());
        assert!(!rules.twisting().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_pentagotwistrules_validation() {
        let rules = PentagoTwistRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("pentago"));
    }

    #[test]
    fn test_pentagotwistrules_explain() {
        let rules = PentagoTwistRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘构成"));
        assert!(e.contains("行棋流程"));
        assert!(e.contains("旋转规则"));
    }
}
