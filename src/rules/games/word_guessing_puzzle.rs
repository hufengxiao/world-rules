//! 猜字词游戏
//!
//! 猜字、猜词的规则与提示分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WordGuessingPuzzleRules,
    name: "猜字词游戏",
    desc: "猜字、猜词的规则与提示分寸",
    origin: "中国",
    tags: ["游戏", "猜字", "字谜", "益智"]
}

impl WordGuessingPuzzleRules {
    /// 选题有度
    pub fn choose(&self) -> Vec<&'static str> {
        vec!["出题难易适当", "主题大家懂", "不偏冷门", "兼顾参与度"]
    }

    /// 给提示
    pub fn hint(&self) -> Vec<&'static str> {
        vec!["逐步给线索", "不一口气道破", "由范围到细节", "鼓励思考"]
    }

    /// 抢答规则
    pub fn answer(&self) -> Vec<&'static str> {
        vec!["轮答或抢答事先约定", "答对得分", "答错换下家", "公平记录"]
    }

    /// 氛围
    pub fn atmosphere(&self) -> Vec<&'static str> {
        vec!["不嘲笑答错", "留时间思考", "揭晓讲丹妙", "娱乐优先"]
    }
}

impl Rule for WordGuessingPuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("word_guess")
    }

    fn explain(&self) -> String {
        format!(
            "【猜字词游戏】\n{}",
            [
                format!(
                    "选题有度：\\n{}",
                    self.choose()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "给提示：\\n{}",
                    self.hint()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抢答规则：\\n{}",
                    self.answer()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "氛围：\\n{}",
                    self.atmosphere()
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
    fn test_wordguessingpuzzlerules_basic() {
        let rules = WordGuessingPuzzleRules::new();
        assert_eq!(rules.metadata().name, "猜字词游戏");
        assert!(!rules.choose().is_empty());
        assert!(!rules.hint().is_empty());
        assert!(!rules.answer().is_empty());
        assert!(!rules.atmosphere().is_empty());
    }

    #[test]
    fn test_wordguessingpuzzlerules_validation() {
        let rules = WordGuessingPuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("word_guess"));
    }

    #[test]
    fn test_wordguessingpuzzlerules_explain() {
        let rules = WordGuessingPuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("选题有度"));
        assert!(e.contains("给提示"));
        assert!(e.contains("抢答规则"));
    }
}
