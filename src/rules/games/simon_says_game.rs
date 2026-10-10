//! 老师说
//!
//! 按口令做动作的儿童反应游戏，误做者出局

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SimonSaysGameRules,
    name: "老师说",
    desc: "按口令做动作的儿童反应游戏，误做者出局",
    origin: "欧美",
    tags: ["儿童", "反应", "口令"]
}

impl SimonSaysGameRules {
    /// 游戏准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "一人当指令者",
            "其余人站立听令",
            "约定口令前缀",
            "场地宽敞无障碍",
        ]
    }

    /// 口令规则
    pub fn commands(&self) -> Vec<&'static str> {
        vec![
            "以固定前缀开头才有效",
            "有效口令须照做",
            "无前缀口令不可做",
            "做错即被淘汰",
        ]
    }

    /// 判定要点
    pub fn judging(&self) -> Vec<&'static str> {
        vec![
            "动作须迅速准确",
            "犹豫迟缓亦可判出局",
            "指令者语速可变化",
            "公平判定不偏私",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "最后留下者获胜",
            "可轮换当指令者",
            "适合多人同乐",
            "锻炼专注与反应",
        ]
    }
}

impl Rule for SimonSaysGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("simon_says_game")
    }

    fn explain(&self) -> String {
        format!(
            "【老师说】\n{}",
            [
                format!(
                    "游戏准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "口令规则：\\n{}",
                    self.commands()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "判定要点：\\n{}",
                    self.judging()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_simonsaysgamerules_basic() {
        let rules = SimonSaysGameRules::new();
        assert_eq!(rules.metadata().name, "老师说");
        assert!(!rules.setup().is_empty());
        assert!(!rules.commands().is_empty());
        assert!(!rules.judging().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_simonsaysgamerules_validation() {
        let rules = SimonSaysGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("simon_says_game"));
    }

    #[test]
    fn test_simonsaysgamerules_explain() {
        let rules = SimonSaysGameRules::new();
        let e = rules.explain();
        assert!(e.contains("游戏准备"));
        assert!(e.contains("口令规则"));
        assert!(e.contains("判定要点"));
    }
}
