//! 黑白棋规则
//!
//! 翻转棋夹子、翻转与终局计子的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ReversiOthelloRules,
    name: "黑白棋规则",
    desc: "翻转棋夹子、翻转与终局计子的规则",
    origin: "国际",
    tags: ["游戏", "黑棋", "翻转棋", "棋类"]
}

impl ReversiOthelloRules {
    /// 开局
    pub fn start(&self) -> Vec<&'static str> {
        vec!["棋盘八乘八", "中心四子开局", "黑先白后", "轮流落子"]
    }

    /// 夹子翻转
    pub fn flank(&self) -> Vec<&'static str> {
        vec!["落子夹住对方子", "横竖斜均可", "把夹的子翻转", "无夹不可落"]
    }

    /// 不能下则过
    pub fn pass_turn(&self) -> Vec<&'static str> {
        vec!["无处可落则跳过", "双方均无则终局", "合理推理", "争取优势"]
    }

    /// 终局计数
    pub fn counting(&self) -> Vec<&'static str> {
        vec!["终局数子定胜负", "子多者胜", "讲究策略取舍", "先手后手两难"]
    }
}

impl Rule for ReversiOthelloRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("reversi")
    }

    fn explain(&self) -> String {
        format!(
            "【黑白棋规则】\n{}",
            [
                format!(
                    "开局：\\n{}",
                    self.start()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "夹子翻转：\\n{}",
                    self.flank()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "不能下则过：\\n{}",
                    self.pass_turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "终局计数：\\n{}",
                    self.counting()
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
    fn test_reversiothellorules_basic() {
        let rules = ReversiOthelloRules::new();
        assert_eq!(rules.metadata().name, "黑白棋规则");
        assert!(!rules.start().is_empty());
        assert!(!rules.flank().is_empty());
        assert!(!rules.pass_turn().is_empty());
        assert!(!rules.counting().is_empty());
    }

    #[test]
    fn test_reversiothellorules_validation() {
        let rules = ReversiOthelloRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("reversi"));
    }

    #[test]
    fn test_reversiothellorules_explain() {
        let rules = ReversiOthelloRules::new();
        let e = rules.explain();
        assert!(e.contains("开局"));
        assert!(e.contains("夹子翻转"));
        assert!(e.contains("不能下则过"));
    }
}
