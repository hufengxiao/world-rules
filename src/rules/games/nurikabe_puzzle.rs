//! 数墙
//!
//! 依数字提示涂黑方块形成连续墙壁的逻辑谜题

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NurikabePuzzleRules,
    name: "数墙",
    desc: "依数字提示涂黑方块形成连续墙壁的逻辑谜题",
    origin: "日本",
    tags: ["解谜", "逻辑", "网格"]
}

impl NurikabePuzzleRules {
    /// 谜题布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "网格内分布数字提示",
            "数字代表岛屿大小",
            "其余格子需涂黑",
            "形成黑色墙壁",
        ]
    }

    /// 涂黑规则
    pub fn rules(&self) -> Vec<&'static str> {
        vec![
            "数字格属白色岛屿",
            "岛屿大小等于其数字",
            "黑格不可形成二乘二块",
            "所有黑格须连成一片",
        ]
    }

    /// 岛屿规则
    pub fn islands(&self) -> Vec<&'static str> {
        vec![
            "白格构成独立岛屿",
            "各岛互不相连",
            "岛屿大小匹配数字",
            "每格必属白岛或黑墙",
        ]
    }

    /// 解答验证
    pub fn verification(&self) -> Vec<&'static str> {
        vec![
            "黑格全连通",
            "无二乘二黑块",
            "岛大小与数字一致",
            "网格填满无空缺",
        ]
    }
}

impl Rule for NurikabePuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("nurikabe_puzzle")
    }

    fn explain(&self) -> String {
        format!(
            "【数墙】\n{}",
            [
                format!(
                    "谜题布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "涂黑规则：\\n{}",
                    self.rules()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "岛屿规则：\\n{}",
                    self.islands()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解答验证：\\n{}",
                    self.verification()
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
    fn test_nurikabepuzzlerules_basic() {
        let rules = NurikabePuzzleRules::new();
        assert_eq!(rules.metadata().name, "数墙");
        assert!(!rules.setup().is_empty());
        assert!(!rules.rules().is_empty());
        assert!(!rules.islands().is_empty());
        assert!(!rules.verification().is_empty());
    }

    #[test]
    fn test_nurikabepuzzlerules_validation() {
        let rules = NurikabePuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("nurikabe_puzzle"));
    }

    #[test]
    fn test_nurikabepuzzlerules_explain() {
        let rules = NurikabePuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("谜题布置"));
        assert!(e.contains("涂黑规则"));
        assert!(e.contains("岛屿规则"));
    }
}
