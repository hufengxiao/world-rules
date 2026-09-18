//! 2048数字合并
//!
//! 滑动合并相同数字方块的2048益智游戏规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: Game2048TileRules,
    name: "2048数字合并",
    desc: "滑动合并相同数字方块的2048益智游戏规则",
    origin: "中国",
    tags: ["游戏", "2048", "益智", "数字"]
}

impl Game2048TileRules {
    /// 基本规则
    pub fn basic(&self) -> Vec<&'static str> {
        vec!["四乘四方格", "滑动合并相同数字", "合并翻倍", "凑出2048获胜"]
    }

    /// 操作方式
    pub fn moving(&self) -> Vec<&'static str> {
        vec![
            "上下左右滑动",
            "方块划向一侧",
            "相同数值相撞合并",
            "产生新方块",
        ]
    }

    /// 合并规则
    pub fn merge(&self) -> Vec<&'static str> {
        vec![
            "相同数字合并",
            "两合一次翻倍",
            "一盘一列合并",
            "翻倍生成大数",
        ]
    }

    /// 策略技巧
    pub fn strategy(&self) -> Vec<&'static str> {
        vec!["集中大数一角", "沿固定方向滑", "避免满盘堵死", "留空给合并"]
    }
}

impl Rule for Game2048TileRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("game_2048")
    }

    fn explain(&self) -> String {
        format!(
            "【2048数字合并】\n{}",
            [
                format!(
                    "基本规则：\\n{}",
                    self.basic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "操作方式：\\n{}",
                    self.moving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合并规则：\\n{}",
                    self.merge()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "策略技巧：\\n{}",
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
    fn test_game2048tilerules_basic() {
        let rules = Game2048TileRules::new();
        assert_eq!(rules.metadata().name, "2048数字合并");
        assert!(!rules.basic().is_empty());
        assert!(!rules.moving().is_empty());
        assert!(!rules.merge().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_game2048tilerules_validation() {
        let rules = Game2048TileRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("game_2048"));
    }

    #[test]
    fn test_game2048tilerules_explain() {
        let rules = Game2048TileRules::new();
        let e = rules.explain();
        assert!(e.contains("基本规则"));
        assert!(e.contains("操作方式"));
        assert!(e.contains("合并规则"));
    }
}
