//! 俄罗斯方块
//!
//! 俄罗斯方块落块消除的规则与策略

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TetrisFallingBlocksRules,
    name: "俄罗斯方块",
    desc: "俄罗斯方块落块消除的规则与策略",
    origin: "中国",
    tags: ["游戏", "俄罗斯方块", "益智", "消除"]
}

impl TetrisFallingBlocksRules {
    /// 方块类型
    pub fn pieces(&self) -> Vec<&'static str> {
        vec!["七种基本四方块", "长条正方形", "T形L形S形", "旋转四方格"]
    }

    /// 基本操作
    pub fn control(&self) -> Vec<&'static str> {
        vec!["左右移动", "旋转方块朝向", "加速下落", "方块落底固定"]
    }

    /// 消除规则
    pub fn clear(&self) -> Vec<&'static str> {
        vec![
            "填满一行消除",
            "整行清空得分",
            "多行消除得分更高",
            "方块层叠空隙留隐患",
        ]
    }

    /// 策略技巧
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "留一个空隙空地待长条",
            "少留弯道空隙",
            "边框贴边堆叠",
            "控速稳堆防顶到顶",
        ]
    }
}

impl Rule for TetrisFallingBlocksRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("tetris_blocks")
    }

    fn explain(&self) -> String {
        format!(
            "【俄罗斯方块】\n{}",
            [
                format!(
                    "方块类型：\\n{}",
                    self.pieces()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "基本操作：\\n{}",
                    self.control()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "消除规则：\\n{}",
                    self.clear()
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
    fn test_tetrisfallingblocksrules_basic() {
        let rules = TetrisFallingBlocksRules::new();
        assert_eq!(rules.metadata().name, "俄罗斯方块");
        assert!(!rules.pieces().is_empty());
        assert!(!rules.control().is_empty());
        assert!(!rules.clear().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_tetrisfallingblocksrules_validation() {
        let rules = TetrisFallingBlocksRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("tetris_blocks"));
    }

    #[test]
    fn test_tetrisfallingblocksrules_explain() {
        let rules = TetrisFallingBlocksRules::new();
        let e = rules.explain();
        assert!(e.contains("方块类型"));
        assert!(e.contains("基本操作"));
        assert!(e.contains("消除规则"));
    }
}
