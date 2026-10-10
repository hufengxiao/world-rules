//! Farkle 骰子游戏
//!
//! Farkle 是考验取舍的骰子计分游戏，玩家轮流掷六枚骰子，挑出得分骰子累积分数，掷出无分则失去本轮所有得分。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FarkleDiceRules,
    name: "Farkle 骰子游戏",
    desc: "Farkle 是考验取舍的骰子计分游戏，玩家轮流掷六枚骰子，挑出得分骰子累积分数，掷出无分则失去本轮所有得分。",
    origin: "流行于北美的传统骰子游戏，又称 10000 分",
    tags: ["骰子", "计分", "运气", "桌面游戏"]
}

impl FarkleDiceRules {
    /// 基本流程
    pub fn turn(&self) -> Vec<&'static str> {
        vec![
            "每轮掷六枚骰子",
            "挑出至少一枚得分骰子",
            "剩余骰子可继续掷",
            "无分则本轮归零",
        ]
    }

    /// 计分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "单个1得100分单个5得50分",
            "三个相同点数按面值计分",
            "三个1得1000分",
            "连续顺子或特定组合有额外分",
        ]
    }

    /// 继续或停止
    pub fn choice(&self) -> Vec<&'static str> {
        vec![
            "可将本轮分数入账结束回合",
            "也可继续掷追求更高分",
            "六枚全部得分可重掷",
            "权衡风险再决定",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "先达到约定分数者胜",
            "常见目标为一万分",
            "达标后其他玩家可追一轮",
            "最终分高者获胜",
        ]
    }
}

impl Rule for FarkleDiceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("farkle")
    }

    fn explain(&self) -> String {
        format!(
            "【Farkle 骰子游戏】\n{}",
            [
                format!(
                    "基本流程：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分规则：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "继续或停止：\\n{}",
                    self.choice()
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
    fn test_farkledicerules_basic() {
        let rules = FarkleDiceRules::new();
        assert_eq!(rules.metadata().name, "Farkle 骰子游戏");
        assert!(!rules.turn().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.choice().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_farkledicerules_validation() {
        let rules = FarkleDiceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("farkle"));
    }

    #[test]
    fn test_farkledicerules_explain() {
        let rules = FarkleDiceRules::new();
        let e = rules.explain();
        assert!(e.contains("基本流程"));
        assert!(e.contains("计分规则"));
        assert!(e.contains("继续或停止"));
    }
}
