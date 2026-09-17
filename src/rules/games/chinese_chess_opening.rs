//! 中国象棋布局与规则
//!
//! 中国象棋布子、走法与开局要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChineseChessOpeningRules,
    name: "中国象棋布局与规则",
    desc: "中国象棋布子、走法与开局要点",
    origin: "中国",
    tags: ["游戏", "象棋", "棋类"]
}

impl ChineseChessOpeningRules {
    /// 布子规则
    pub fn setup(&self) -> Vec<&'static str> {
        vec!["车马炮弹卒各就位", "炮隔子吃子", "马行日字", "象走田受限"]
    }

    /// 行棋走法
    pub fn moving(&self) -> Vec<&'static str> {
        vec![
            "兵卒过河前只能前",
            "将帅不出九宫",
            "士斜行护将",
            "车直行吃子",
        ]
    }

    /// 开局要点
    pub fn opening(&self) -> Vec<&'static str> {
        vec!["先出车出动大子", "炮配合抢中", "尽早将阵", "占要道多子力"]
    }

    /// 胜负将死
    pub fn checkmate(&self) -> Vec<&'static str> {
        vec!["将死对方将帅胜", "困毙亦判负", "和棋有规则", "遵守服棋规"]
    }
}

impl Rule for ChineseChessOpeningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("xiangqi")
    }

    fn explain(&self) -> String {
        format!(
            "【中国象棋布局与规则】\n{}",
            [
                format!(
                    "布子规则：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行棋走法：\\n{}",
                    self.moving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "开局要点：\\n{}",
                    self.opening()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负将死：\\n{}",
                    self.checkmate()
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
    fn test_chinesechessopeningrules_basic() {
        let rules = ChineseChessOpeningRules::new();
        assert_eq!(rules.metadata().name, "中国象棋布局与规则");
        assert!(!rules.setup().is_empty());
        assert!(!rules.moving().is_empty());
        assert!(!rules.opening().is_empty());
        assert!(!rules.checkmate().is_empty());
    }

    #[test]
    fn test_chinesechessopeningrules_validation() {
        let rules = ChineseChessOpeningRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("xiangqi"));
    }

    #[test]
    fn test_chinesechessopeningrules_explain() {
        let rules = ChineseChessOpeningRules::new();
        let e = rules.explain();
        assert!(e.contains("布子规则"));
        assert!(e.contains("行棋走法"));
        assert!(e.contains("开局要点"));
    }
}
