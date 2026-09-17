//! 飞行棋规则
//!
//! 飞行棋掷骰、起飞与撞子回航规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FlyingChessRules,
    name: "飞行棋规则",
    desc: "飞行棋掷骰、起飞与撞子回航规则",
    origin: "国际",
    tags: ["游戏", "飞行棋", "亲子"]
}

impl FlyingChessRules {
    /// 起飞出发
    pub fn takeoff(&self) -> Vec<&'static str> {
        vec![
            "掷到六才可起飞",
            "掷六可再掷一次",
            "起飞先占起点格",
            "未起飞则待机会",
        ]
    }

    /// 行进规则
    pub fn moving(&self) -> Vec<&'static str> {
        vec![
            "依骰子数前进",
            "落格依规则效果",
            "格子有加有乘",
            "沿轨道顺时针",
        ]
    }

    /// 撞子回家
    pub fn block(&self) -> Vec<&'static str> {
        vec![
            "落到对方子同格",
            "对方回起点",
            "安全区可暂保",
            "终点须可恰好",
        ]
    }

    /// 胜利收尾
    pub fn finish(&self) -> Vec<&'static str> {
        vec!["齐到终点者胜", "落点恰好走回", "多人轮流公平", "欢快游戏"]
    }
}

impl Rule for FlyingChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("flying_chess")
    }

    fn explain(&self) -> String {
        format!(
            "【飞行棋规则】\n{}",
            [
                format!(
                    "起飞出发：\\n{}",
                    self.takeoff()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行进规则：\\n{}",
                    self.moving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "撞子回家：\\n{}",
                    self.block()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜利收尾：\\n{}",
                    self.finish()
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
    fn test_flyingchessrules_basic() {
        let rules = FlyingChessRules::new();
        assert_eq!(rules.metadata().name, "飞行棋规则");
        assert!(!rules.takeoff().is_empty());
        assert!(!rules.moving().is_empty());
        assert!(!rules.block().is_empty());
        assert!(!rules.finish().is_empty());
    }

    #[test]
    fn test_flyingchessrules_validation() {
        let rules = FlyingChessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("flying_chess"));
    }

    #[test]
    fn test_flyingchessrules_explain() {
        let rules = FlyingChessRules::new();
        let e = rules.explain();
        assert!(e.contains("起飞出发"));
        assert!(e.contains("行进规则"));
        assert!(e.contains("撞子回家"));
    }
}
