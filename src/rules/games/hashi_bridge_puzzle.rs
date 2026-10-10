//! 桥连谜（Hashiwokakero）
//!
//! 桥连谜是在散布的岛屿圆圈之间架设桥梁的逻辑谜题，岛屿上的数字表示应连接的桥数，桥只能水平或垂直连接且不得交叉。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HashiBridgePuzzleRules,
    name: "桥连谜（Hashiwokakero）",
    desc: "桥连谜是在散布的岛屿圆圈之间架设桥梁的逻辑谜题，岛屿上的数字表示应连接的桥数，桥只能水平或垂直连接且不得交叉。",
    origin: "由日本谜题出版社 Nikoli 创造，日语意为「架桥」",
    tags: ["解谜", "逻辑", "连线", "桌面游戏"]
}

impl HashiBridgePuzzleRules {
    /// 盘面元素
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "圆圈代表岛屿并标有数字",
            "数字表示该岛应连的桥数",
            "岛屿之间可架零到两座桥",
            "桥只能横平或竖直延伸",
        ]
    }

    /// 架桥规则
    pub fn bridging(&self) -> Vec<&'static str> {
        vec![
            "桥连接两岛须成直线",
            "同一对岛屿最多架两座桥",
            "桥不得跨越其他岛屿",
            "桥与桥之间不得交叉",
        ]
    }

    /// 连通要求
    pub fn connect(&self) -> Vec<&'static str> {
        vec![
            "最终所有岛屿须连成一片",
            "不允许存在孤立岛屿群",
            "桥数须恰好等于岛屿数字",
            "禁止多余或缺失的桥",
        ]
    }

    /// 解题思路
    pub fn solving(&self) -> Vec<&'static str> {
        vec![
            "数字为1的岛只有唯一连法",
            "角落或边缘的岛优先处理",
            "用已确定的桥反推相邻岛",
            "逐步排除导致断连的架法",
        ]
    }
}

impl Rule for HashiBridgePuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("hashi")
    }

    fn explain(&self) -> String {
        format!(
            "【桥连谜（Hashiwokakero）】\n{}",
            [
                format!(
                    "盘面元素：\\n{}",
                    self.board()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "架桥规则：\\n{}",
                    self.bridging()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "连通要求：\\n{}",
                    self.connect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解题思路：\\n{}",
                    self.solving()
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
    fn test_hashibridgepuzzlerules_basic() {
        let rules = HashiBridgePuzzleRules::new();
        assert_eq!(rules.metadata().name, "桥连谜（Hashiwokakero）");
        assert!(!rules.board().is_empty());
        assert!(!rules.bridging().is_empty());
        assert!(!rules.connect().is_empty());
        assert!(!rules.solving().is_empty());
    }

    #[test]
    fn test_hashibridgepuzzlerules_validation() {
        let rules = HashiBridgePuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("hashi"));
    }

    #[test]
    fn test_hashibridgepuzzlerules_explain() {
        let rules = HashiBridgePuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("盘面元素"));
        assert!(e.contains("架桥规则"));
        assert!(e.contains("连通要求"));
    }
}
