//! 亚马逊棋
//!
//! 走棋后放箭封路的抽象策略棋，最后能走者胜

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AmazonsGameRules,
    name: "亚马逊棋",
    desc: "走棋后放箭封路的抽象策略棋，最后能走者胜",
    origin: "美国",
    tags: ["棋类", "抽象", "策略"]
}

impl AmazonsGameRules {
    /// 棋盘布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "十乘十方格棋盘",
            "每方四枚亚马逊棋子",
            "棋子分别置于四角",
            "开局中央区域空置",
        ]
    }

    /// 回合行动
    pub fn turn(&self) -> Vec<&'static str> {
        vec![
            "先移动己方一枚棋子",
            "棋子如皇后般直行或斜行",
            "再以同棋子放箭",
            "箭沿直线射至空格",
        ]
    }

    /// 封路规则
    pub fn blocking(&self) -> Vec<&'static str> {
        vec![
            "放箭处成为障碍",
            "障碍不可通过或落子",
            "移动不可跨越障碍",
            "障碍永久存在",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "轮到自己无棋可走则败",
            "每步必走棋并放箭",
            "封锁对手使其无路",
            "最后行动者获胜",
        ]
    }

    /// 策略要点
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "分割棋盘限制对手",
            "保留己方活动空间",
            "箭既伤敌亦阻己",
            "空间控制是核心",
        ]
    }
}

impl Rule for AmazonsGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("amazons_game")
    }

    fn explain(&self) -> String {
        format!(
            "【亚马逊棋】\n{}",
            [
                format!(
                    "棋盘布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合行动：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "封路规则：\\n{}",
                    self.blocking()
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
                    "策略要点：\\n{}",
                    self.strategy()
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
    fn test_amazonsgamerules_basic() {
        let rules = AmazonsGameRules::new();
        assert_eq!(rules.metadata().name, "亚马逊棋");
        assert!(!rules.setup().is_empty());
        assert!(!rules.turn().is_empty());
        assert!(!rules.blocking().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_amazonsgamerules_validation() {
        let rules = AmazonsGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("amazons_game"));
    }

    #[test]
    fn test_amazonsgamerules_explain() {
        let rules = AmazonsGameRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘布置"));
        assert!(e.contains("回合行动"));
        assert!(e.contains("封路规则"));
    }
}
