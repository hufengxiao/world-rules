//! 捉迷藏
//!
//! 捉迷藏数数找藏、被找折返的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HideAndSeekRules,
    name: "捉迷藏",
    desc: "捉迷藏数数找藏、被找折返的规则",
    origin: "中国",
    tags: ["游戏", "捉迷藏", "儿童"]
}

impl HideAndSeekRules {
    /// 游戏设定
    pub fn setup(&self) -> Vec<&'static str> {
        vec!["一人蒙眼数数", "他人躲藏", "数完开始找", "选一处当基地"]
    }

    /// 寻找规则
    pub fn seeking(&self) -> Vec<&'static str> {
        vec!["找到喊名字", "被发现回基地", "先回者安全", "被抓名出局"]
    }

    /// 范围安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["约定躲藏范围", "不躲危险处", "不跑太远", "照看幼童"]
    }

    /// 有趣收场
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["找不到可喊出", "被原谅公平", "轮流当寻找者", "玩得开心"]
    }
}

impl Rule for HideAndSeekRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("hide_seek")
    }

    fn explain(&self) -> String {
        format!(
            "【捉迷藏】\n{}",
            [
                format!(
                    "游戏设定：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "寻找规则：\\n{}",
                    self.seeking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "范围安全：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "有趣收场：\\n{}",
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
    fn test_hideandseekrules_basic() {
        let rules = HideAndSeekRules::new();
        assert_eq!(rules.metadata().name, "捉迷藏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.seeking().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.fun().is_empty());
    }

    #[test]
    fn test_hideandseekrules_validation() {
        let rules = HideAndSeekRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("hide_seek"));
    }

    #[test]
    fn test_hideandseekrules_explain() {
        let rules = HideAndSeekRules::new();
        let e = rules.explain();
        assert!(e.contains("游戏设定"));
        assert!(e.contains("寻找规则"));
        assert!(e.contains("范围安全"));
    }
}
