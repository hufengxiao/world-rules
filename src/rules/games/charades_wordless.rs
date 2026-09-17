//! 你比划我猜
//!
//! 比手画脚猜词、肢体猜谜的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CharadesWordlessRules,
    name: "你比划我猜",
    desc: "比手画脚猜词、肢体猜谜的规则",
    origin: "国际",
    tags: ["游戏", "比划", "猜词", "派对"]
}

impl CharadesWordlessRules {
    /// 猜词设定
    pub fn setup(&self) -> Vec<&'static str> {
        vec!["写词条分两队", "一人比划", "队友猜词", "限时猜对得分"]
    }

    /// 比划限制
    pub fn acting(&self) -> Vec<&'static str> {
        vec!["不得说话", "可用于势身体", "比划形象", "不能写字"]
    }

    /// 猜对计分
    pub fn scoring(&self) -> Vec<&'static str> {
        vec!["猜对得一分", "守时间限制", "换人轮流", "计分定胜"]
    }

    /// 热闹进行
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["比划清楚有趣", "答错不嘲笑", "提醒用暗号", "乐在其中"]
    }
}

impl Rule for CharadesWordlessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("charades")
    }

    fn explain(&self) -> String {
        format!(
            "【你比划我猜】\n{}",
            [
                format!(
                    "猜词设定：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "比划限制：\\n{}",
                    self.acting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "猜对计分：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "热闹进行：\\n{}",
                    self.fun()
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
    fn test_charadeswordlessrules_basic() {
        let rules = CharadesWordlessRules::new();
        assert_eq!(rules.metadata().name, "你比划我猜");
        assert!(!rules.setup().is_empty());
        assert!(!rules.acting().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.fun().is_empty());
    }

    #[test]
    fn test_charadeswordlessrules_validation() {
        let rules = CharadesWordlessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("charades"));
    }

    #[test]
    fn test_charadeswordlessrules_explain() {
        let rules = CharadesWordlessRules::new();
        let e = rules.explain();
        assert!(e.contains("猜词设定"));
        assert!(e.contains("比划限制"));
        assert!(e.contains("猜对计分"));
    }
}
