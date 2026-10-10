//! 塞尼特棋（Senet）
//!
//! 塞尼特是古埃及的两人掷骰竞速棋，玩家让棋子沿三十格棋盘前进，通过掷签移动并可攻击或保护棋子，最先全部走出者胜。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SenetEgyptRules,
    name: "塞尼特棋（Senet）",
    desc: "塞尼特是古埃及的两人掷骰竞速棋，玩家让棋子沿三十格棋盘前进，通过掷签移动并可攻击或保护棋子，最先全部走出者胜。",
    origin: "古埃及法老时代的棋盘游戏，距今约五千年",
    tags: ["棋类", "古代", "竞速", "桌面游戏"]
}

impl SenetEgyptRules {
    /// 棋盘与棋子
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为三行十列共三十格",
            "双方各执五枚棋子",
            "棋子沿蛇形路线前进",
            "末几格有特殊标记",
        ]
    }

    /// 行棋规则
    pub fn moving(&self) -> Vec<&'static str> {
        vec![
            "掷签或骰决定前进步数",
            "按点数沿路线移动一子",
            "不能落在己方占据的格",
            "不能越过对方的阻挡",
        ]
    }

    /// 攻防规则
    pub fn capture(&self) -> Vec<&'static str> {
        vec![
            "落在对方孤立棋子上将其送回起点",
            "相邻两枚己方棋子互相保护",
            "受保护的棋子不可被攻击",
            "特殊格有额外规则限制",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "棋子须恰好点数走出末格",
            "全部棋子走出者获胜",
            "点数超出则无法移动",
            "先出完为胜者",
        ]
    }
}

impl Rule for SenetEgyptRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("senet")
    }

    fn explain(&self) -> String {
        format!(
            "【塞尼特棋（Senet）】\n{}",
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
                    "行棋规则：\\n{}",
                    self.moving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "攻防规则：\\n{}",
                    self.capture()
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
    fn test_senetegyptrules_basic() {
        let rules = SenetEgyptRules::new();
        assert_eq!(rules.metadata().name, "塞尼特棋（Senet）");
        assert!(!rules.board().is_empty());
        assert!(!rules.moving().is_empty());
        assert!(!rules.capture().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_senetegyptrules_validation() {
        let rules = SenetEgyptRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("senet"));
    }

    #[test]
    fn test_senetegyptrules_explain() {
        let rules = SenetEgyptRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘与棋子"));
        assert!(e.contains("行棋规则"));
        assert!(e.contains("攻防规则"));
    }
}
