//! 跳棋基本规则
//!
//! 跳棋的走子、跳跃与获胜规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JumpChessRules,
    name: "跳棋基本规则",
    desc: "跳棋的走子、跳跃与获胜规则",
    origin: "国际",
    tags: ["游戏", "跳棋", "棋盘"]
}

impl JumpChessRules {
    /// 布局走子
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "对角摆放己方棋子",
            "轮流按步走子",
            "可移动相邻格",
            "目标占领对方起始区",
        ]
    }

    /// 跳跃规则
    pub fn jump(&self) -> Vec<&'static str> {
        vec![
            "跳过相邻对方子",
            "跳越同样可连续",
            "需空位可落",
            "跳跃不可斜跳",
        ]
    }

    /// 胜负判定
    pub fn win(&self) -> Vec<&'static str> {
        vec![
            "先铺满对方区胜",
            "可单人完成",
            "团体按到位计",
            "阻塞灵活应变",
        ]
    }

    /// 游戏礼仪
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["轮到时再动子", "不悔棋赖棋", "出子干脆", "友好对弈"]
    }
}

impl Rule for JumpChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("jump_chess")
    }

    fn explain(&self) -> String {
        format!(
            "【跳棋基本规则】\n{}",
            [
                format!(
                    "布局走子：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "跳跃规则：\\n{}",
                    self.jump()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负判定：\\n{}",
                    self.win()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "游戏礼仪：\\n{}",
                    self.manner()
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
    fn test_jumpchessrules_basic() {
        let rules = JumpChessRules::new();
        assert_eq!(rules.metadata().name, "跳棋基本规则");
        assert!(!rules.setup().is_empty());
        assert!(!rules.jump().is_empty());
        assert!(!rules.win().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_jumpchessrules_validation() {
        let rules = JumpChessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("jump_chess"));
    }

    #[test]
    fn test_jumpchessrules_explain() {
        let rules = JumpChessRules::new();
        let e = rules.explain();
        assert!(e.contains("布局走子"));
        assert!(e.contains("跳跃规则"));
        assert!(e.contains("胜负判定"));
    }
}
