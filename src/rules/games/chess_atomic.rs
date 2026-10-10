//! 原子象棋
//!
//! 吃子引发爆炸的象棋变体，炸掉王者即胜

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChessAtomicRules,
    name: "原子象棋",
    desc: "吃子引发爆炸的象棋变体，炸掉王者即胜",
    origin: "欧洲",
    tags: ["棋类", "变体", "国际象棋"]
}

impl ChessAtomicRules {
    /// 爆炸规则
    pub fn explosion(&self) -> Vec<&'static str> {
        vec![
            "任何吃子触发爆炸",
            "被吃子与吃子一同消失",
            "周围八格棋子被炸飞",
            "兵亦可参与爆炸",
        ]
    }

    /// 走子方式
    pub fn movement(&self) -> Vec<&'static str> {
        vec![
            "走子规则同国际象棋",
            "仅吃子时触发爆炸",
            "不可吃己方棋子",
            "王同样会被炸掉",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "炸掉对方王者即胜",
            "己方王被炸即败",
            "将军概念大为弱化",
            "将杀仍是有效手段",
        ]
    }

    /// 特殊注意
    pub fn notes(&self) -> Vec<&'static str> {
        vec![
            "王不能吃有护卫的棋子",
            "爆炸可能同归于尽",
            "需重新评估兑子价值",
            "残局常现奇招",
        ]
    }
}

impl Rule for ChessAtomicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess_atomic")
    }

    fn explain(&self) -> String {
        format!(
            "【原子象棋】\n{}",
            [
                format!(
                    "爆炸规则：\\n{}",
                    self.explosion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "走子方式：\\n{}",
                    self.movement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负条件：\\n{}",
                    self.winning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊注意：\\n{}",
                    self.notes()
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
    fn test_chessatomicrules_basic() {
        let rules = ChessAtomicRules::new();
        assert_eq!(rules.metadata().name, "原子象棋");
        assert!(!rules.explosion().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.notes().is_empty());
    }

    #[test]
    fn test_chessatomicrules_validation() {
        let rules = ChessAtomicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("chess_atomic"));
    }

    #[test]
    fn test_chessatomicrules_explain() {
        let rules = ChessAtomicRules::new();
        let e = rules.explain();
        assert!(e.contains("爆炸规则"));
        assert!(e.contains("走子方式"));
        assert!(e.contains("胜负条件"));
    }
}
