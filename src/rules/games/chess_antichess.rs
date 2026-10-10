//! 反象棋
//!
//! 目标相反的国际象棋变体，吃光自己棋子者获胜

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChessAntichessRules,
    name: "反象棋",
    desc: "目标相反的国际象棋变体，吃光自己棋子者获胜",
    origin: "欧洲",
    tags: ["棋类", "变体", "国际象棋"]
}

impl ChessAntichessRules {
    /// 核心反转
    pub fn core(&self) -> Vec<&'static str> {
        vec![
            "目标为吃光己方全部棋子",
            "先无子可动者获胜",
            "与国际象棋目标相反",
            "走子规则基本不变",
        ]
    }

    /// 强制吃子
    pub fn forced_capture(&self) -> Vec<&'static str> {
        vec![
            "有吃子机会必须吃",
            "不可主动放弃吃子",
            "有多吃选择可任选",
            "无吃子时才可走普通着法",
        ]
    }

    /// 兵的特殊
    pub fn pawn(&self) -> Vec<&'static str> {
        vec![
            "兵可吃子即须吃",
            "升变仍可进行",
            "吃过路兵同样适用",
            "兵的移动照旧",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "吃光己方棋子者胜",
            "无合法着法者胜",
            "被迫吃子易生妙手",
            "策略完全反转",
        ]
    }
}

impl Rule for ChessAntichessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess_antichess")
    }

    fn explain(&self) -> String {
        format!(
            "【反象棋】\n{}",
            [
                format!(
                    "核心反转：\\n{}",
                    self.core()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "强制吃子：\\n{}",
                    self.forced_capture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "兵的特殊：\\n{}",
                    self.pawn()
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
    fn test_chessantichessrules_basic() {
        let rules = ChessAntichessRules::new();
        assert_eq!(rules.metadata().name, "反象棋");
        assert!(!rules.core().is_empty());
        assert!(!rules.forced_capture().is_empty());
        assert!(!rules.pawn().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_chessantichessrules_validation() {
        let rules = ChessAntichessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("chess_antichess"));
    }

    #[test]
    fn test_chessantichessrules_explain() {
        let rules = ChessAntichessRules::new();
        let e = rules.explain();
        assert!(e.contains("核心反转"));
        assert!(e.contains("强制吃子"));
        assert!(e.contains("兵的特殊"));
    }
}
