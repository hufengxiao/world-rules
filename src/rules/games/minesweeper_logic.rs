//! 扫雷推理
//!
//! 扫雷游戏中根据数字提示推断地雷位置的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MinesweeperLogicRules,
    name: "扫雷推理",
    desc: "扫雷游戏中根据数字提示推断地雷位置的规则",
    origin: "中国",
    tags: ["游戏", "扫雷", "益智", "逻辑"]
}

impl MinesweeperLogicRules {
    /// 基本规则
    pub fn basic(&self) -> Vec<&'static str> {
        vec![
            "翻开格子避雷",
            "数字表示周围地雷数",
            "插旗标记地雷",
            "排清全部空地获胜",
        ]
    }

    /// 数字推理
    pub fn deduce(&self) -> Vec<&'static str> {
        vec![
            "一周围八格计数",
            "数字即相邻雷数",
            "已标雷数相减",
            "推断未知格",
        ]
    }

    /// 常用技巧
    pub fn technique(&self) -> Vec<&'static str> {
        vec!["一格一号定雷", "两格双雷三格", "边界格优先", "组合推理破局"]
    }

    /// 误触处理
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["猜雷易翻车", "模糊处留待推", "长按插旗标记", "数格再点开"]
    }
}

impl Rule for MinesweeperLogicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("minesweeper_logic")
    }

    fn explain(&self) -> String {
        format!(
            "【扫雷推理】\n{}",
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
                    "数字推理：\\n{}",
                    self.deduce()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "常用技巧：\\n{}",
                    self.technique()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "误触处理：\\n{}",
                    self.avoid()
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
    fn test_minesweeperlogicrules_basic() {
        let rules = MinesweeperLogicRules::new();
        assert_eq!(rules.metadata().name, "扫雷推理");
        assert!(!rules.basic().is_empty());
        assert!(!rules.deduce().is_empty());
        assert!(!rules.technique().is_empty());
        assert!(!rules.avoid().is_empty());
    }

    #[test]
    fn test_minesweeperlogicrules_validation() {
        let rules = MinesweeperLogicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("minesweeper_logic"));
    }

    #[test]
    fn test_minesweeperlogicrules_explain() {
        let rules = MinesweeperLogicRules::new();
        let e = rules.explain();
        assert!(e.contains("基本规则"));
        assert!(e.contains("数字推理"));
        assert!(e.contains("常用技巧"));
    }
}
