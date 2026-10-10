//! 斗兽棋
//!
//! 以动物大小相克为规则的儿童策略棋戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DoushouqiRules,
    name: "斗兽棋",
    desc: "以动物大小相克为规则的儿童策略棋戏",
    origin: "中国",
    tags: ["棋类", "儿童", "动物"]
}

impl DoushouqiRules {
    /// 棋子等级
    pub fn hierarchy(&self) -> Vec<&'static str> {
        vec![
            "象狮虎豹狼狗猫鼠八级",
            "大兽吃小兽",
            "鼠可吃象为特例",
            "同级相吃同归于尽",
        ]
    }

    /// 走子规则
    pub fn movement(&self) -> Vec<&'static str> {
        vec!["每步走一格", "可上下左右移动", "不可走对角线", "狮虎可跳河"]
    }

    /// 河流规则
    pub fn river(&self) -> Vec<&'static str> {
        vec![
            "鼠可入河游动",
            "河中鼠不可被岸上兽吃",
            "狮虎可跳过河",
            "河中不可停留",
        ]
    }

    /// 胜负条件
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "进入对方兽穴即胜",
            "吃光对方棋子即胜",
            "对方无子可动亦胜",
            "不得进入己方兽穴",
        ]
    }
}

impl Rule for DoushouqiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("doushouqi")
    }

    fn explain(&self) -> String {
        format!(
            "【斗兽棋】\n{}",
            [
                format!(
                    "棋子等级：\\n{}",
                    self.hierarchy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "走子规则：\\n{}",
                    self.movement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "河流规则：\\n{}",
                    self.river()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负条件：\\n{}",
                    self.winning()
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
    fn test_doushouqirules_basic() {
        let rules = DoushouqiRules::new();
        assert_eq!(rules.metadata().name, "斗兽棋");
        assert!(!rules.hierarchy().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.river().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_doushouqirules_validation() {
        let rules = DoushouqiRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("doushouqi"));
    }

    #[test]
    fn test_doushouqirules_explain() {
        let rules = DoushouqiRules::new();
        let e = rules.explain();
        assert!(e.contains("棋子等级"));
        assert!(e.contains("走子规则"));
        assert!(e.contains("河流规则"));
    }
}
