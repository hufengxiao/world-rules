//! 记忆配对牌游戏
//!
//! 翻牌配对、记忆与轮流的记忆游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MemoryMatchRules,
    name: "记忆配对牌游戏",
    desc: "翻牌配对、记忆与轮流的记忆游戏",
    origin: "国际",
    tags: ["游戏", "记忆牌", "翻牌", "益智"]
}

impl MemoryMatchRules {
    /// 排牌准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec!["卡片背面朝上", "成对排列整齐", "随机打乱", "布置围桌"]
    }

    /// 翻牌配对
    pub fn flip(&self) -> Vec<&'static str> {
        vec!["每次翻两张", "相同则配对收起", "不同盖回", "记忆位置"]
    }

    /// 轮流进行
    pub fn turns(&self) -> Vec<&'static str> {
        vec!["配对成功可续翻", "不同则换下一人", "不偷看背面", "轮流公平"]
    }

    /// 胜利判定
    pub fn win(&self) -> Vec<&'static str> {
        vec!["配完收起最多胜", "也可计时间", "累计成对多赢", "锻炼记忆"]
    }
}

impl Rule for MemoryMatchRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("memory_match")
    }

    fn explain(&self) -> String {
        format!(
            "【记忆配对牌游戏】\n{}",
            [
                format!(
                    "排牌准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "翻牌配对：\\n{}",
                    self.flip()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "轮流进行：\\n{}",
                    self.turns()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜利判定：\\n{}",
                    self.win()
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
    fn test_memorymatchrules_basic() {
        let rules = MemoryMatchRules::new();
        assert_eq!(rules.metadata().name, "记忆配对牌游戏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.flip().is_empty());
        assert!(!rules.turns().is_empty());
        assert!(!rules.win().is_empty());
    }

    #[test]
    fn test_memorymatchrules_validation() {
        let rules = MemoryMatchRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("memory_match"));
    }

    #[test]
    fn test_memorymatchrules_explain() {
        let rules = MemoryMatchRules::new();
        let e = rules.explain();
        assert!(e.contains("排牌准备"));
        assert!(e.contains("翻牌配对"));
        assert!(e.contains("轮流进行"));
    }
}
