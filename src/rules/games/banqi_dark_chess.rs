//! 暗棋
//!
//! 中国象棋棋子反扣随机摆放的翻棋玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BanqiDarkChessRules,
    name: "暗棋",
    desc: "中国象棋棋子反扣随机摆放的翻棋玩法",
    origin: "中国",
    tags: ["棋类", "象棋", "翻棋"]
}

impl BanqiDarkChessRules {
    /// 开局布子
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "棋子全部反扣随机摆放",
            "摆满半张棋盘四行",
            "双方各占一侧",
            "颜色未知随机分布",
        ]
    }

    /// 翻棋规则
    pub fn revealing(&self) -> Vec<&'static str> {
        vec![
            "回合可翻开一枚暗棋",
            "翻出的棋归属其颜色方",
            "首次翻棋即确定阵营",
            "翻出后即可行动",
        ]
    }

    /// 走子吃子
    pub fn movement(&self) -> Vec<&'static str> {
        vec![
            "明棋可向上下左右走一格",
            "按象棋大小吃子",
            "将帅不可照面",
            "炮需隔子吃子",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "吃光对方棋子即胜",
            "对方无子可动亦胜",
            "运气与策略并重",
            "常见于休闲对局",
        ]
    }
}

impl Rule for BanqiDarkChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("banqi_dark_chess")
    }

    fn explain(&self) -> String {
        format!(
            "【暗棋】\n{}",
            [
                format!(
                    "开局布子：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "翻棋规则：\\n{}",
                    self.revealing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "走子吃子：\\n{}",
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
    fn test_banqidarkchessrules_basic() {
        let rules = BanqiDarkChessRules::new();
        assert_eq!(rules.metadata().name, "暗棋");
        assert!(!rules.setup().is_empty());
        assert!(!rules.revealing().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_banqidarkchessrules_validation() {
        let rules = BanqiDarkChessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("banqi_dark_chess"));
    }

    #[test]
    fn test_banqidarkchessrules_explain() {
        let rules = BanqiDarkChessRules::new();
        let e = rules.explain();
        assert!(e.contains("开局布子"));
        assert!(e.contains("翻棋规则"));
        assert!(e.contains("走子吃子"));
    }
}
