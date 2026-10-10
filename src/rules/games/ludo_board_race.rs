//! 飞行棋竞速（Ludo）
//!
//! Ludo 是经典的掷骰竞速棋，玩家让四枚棋子从基地出发绕行棋盘一周回到终点，途中可击退对手棋子并获得额外回合。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LudoBoardRaceRules,
    name: "飞行棋竞速（Ludo）",
    desc: "Ludo 是经典的掷骰竞速棋，玩家让四枚棋子从基地出发绕行棋盘一周回到终点，途中可击退对手棋子并获得额外回合。",
    origin: "源自印度十字戏 Pachisi，流行于英联邦国家",
    tags: ["棋类", "竞速", "骰子", "桌面游戏"]
}

impl LudoBoardRaceRules {
    /// 棋盘与棋子
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "十字形棋盘分四色阵营",
            "每方四枚棋子置于基地",
            "沿轨道绕行一圈回终点",
            "轨道分安全格与普通格",
        ]
    }

    /// 行棋规则
    pub fn moving(&self) -> Vec<&'static str> {
        vec![
            "掷出六点方可出基地",
            "按点数前进对应格数",
            "掷六可再掷一次",
            "连续三次六则本回合作废",
        ]
    }

    /// 相遇与击退
    pub fn capture(&self) -> Vec<&'static str> {
        vec![
            "落在对手棋子上将其击回基地",
            "安全格上的棋子不可被击",
            "击退对手可获额外回合",
            "同色棋子可叠放同行",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "四枚棋子全部到达终点者胜",
            "须掷出恰好点数进终点",
            "点数超出则原地不动",
            "常设积分赛累计名次",
        ]
    }
}

impl Rule for LudoBoardRaceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("ludo")
    }

    fn explain(&self) -> String {
        format!(
            "【飞行棋竞速（Ludo）】\n{}",
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
                    "相遇与击退：\\n{}",
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
    fn test_ludoboardracerules_basic() {
        let rules = LudoBoardRaceRules::new();
        assert_eq!(rules.metadata().name, "飞行棋竞速（Ludo）");
        assert!(!rules.board().is_empty());
        assert!(!rules.moving().is_empty());
        assert!(!rules.capture().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_ludoboardracerules_validation() {
        let rules = LudoBoardRaceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("ludo"));
    }

    #[test]
    fn test_ludoboardracerules_explain() {
        let rules = LudoBoardRaceRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘与棋子"));
        assert!(e.contains("行棋规则"));
        assert!(e.contains("相遇与击退"));
    }
}
