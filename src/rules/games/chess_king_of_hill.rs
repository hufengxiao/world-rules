//! 山丘之王象棋
//!
//! 占据棋盘中央山丘即获胜的国际象棋变体

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChessKingOfHillRules,
    name: "山丘之王象棋",
    desc: "占据棋盘中央山丘即获胜的国际象棋变体",
    origin: "欧洲",
    tags: ["棋类", "变体", "国际象棋"]
}

impl ChessKingOfHillRules {
    /// 胜利条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "王者抵达中央四格即胜",
            "中央四格为山丘",
            "将杀对方仍可获胜",
            "两种方式皆可取胜",
        ]
    }

    /// 中央区域
    pub fn hill(&self) -> Vec<&'static str> {
        vec![
            "棋盘中央四格为山丘",
            "王者进入即宣告胜利",
            "无需将死对方",
            "须安全抵达中央",
        ]
    }

    /// 策略变化
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "王主动出击更有价值",
            "中心争夺异常激烈",
            "防守反击同样重要",
            "兑子换取通路",
        ]
    }

    /// 走子规则
    pub fn movement(&self) -> Vec<&'static str> {
        vec![
            "其余规则同国际象棋",
            "将军须应对",
            "吃过路兵与升变照旧",
            "以王安全为前提",
        ]
    }
}

impl Rule for ChessKingOfHillRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess_king_of_hill")
    }

    fn explain(&self) -> String {
        format!(
            "【山丘之王象棋】\n{}",
            [
                format!(
                    "胜利条件：\\n{}",
                    self.winning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "中央区域：\\n{}",
                    self.hill()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "策略变化：\\n{}",
                    self.strategy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "走子规则：\\n{}",
                    self.movement()
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
    fn test_chesskingofhillrules_basic() {
        let rules = ChessKingOfHillRules::new();
        assert_eq!(rules.metadata().name, "山丘之王象棋");
        assert!(!rules.winning().is_empty());
        assert!(!rules.hill().is_empty());
        assert!(!rules.strategy().is_empty());
        assert!(!rules.movement().is_empty());
    }

    #[test]
    fn test_chesskingofhillrules_validation() {
        let rules = ChessKingOfHillRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("chess_king_of_hill"));
    }

    #[test]
    fn test_chesskingofhillrules_explain() {
        let rules = ChessKingOfHillRules::new();
        let e = rules.explain();
        assert!(e.contains("胜利条件"));
        assert!(e.contains("中央区域"));
        assert!(e.contains("策略变化"));
    }
}
