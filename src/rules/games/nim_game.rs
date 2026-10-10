//! 尼姆取物游戏
//!
//! 从若干堆物品中轮流取物，取走最后一件者胜或负的经典博弈

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NimGameRules,
    name: "尼姆取物游戏",
    desc: "从若干堆物品中轮流取物，取走最后一件者胜或负的经典博弈",
    origin: "古代中国",
    tags: ["博弈", "数学", "策略"]
}

impl NimGameRules {
    /// 开局摆法
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "摆出若干堆火柴或棋子",
            "各堆数量可相同可不同",
            "约定取走最后者胜负",
            "常见三堆三五七枚",
        ]
    }

    /// 取子规则
    pub fn taking(&self) -> Vec<&'static str> {
        vec![
            "每次只从一堆中取子",
            "可取一枚或多枚",
            "可一次取光整堆",
            "不得跨堆同时取",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "取走最后一件者胜",
            "或约定取最后者输",
            "胜负约定须开局定明",
            "中途不可更改",
        ]
    }

    /// 必胜策略
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "各堆数量按二进制异或",
            "异或为零为必败局面",
            "每次取子令异或归零",
            "对手失误即可反制",
        ]
    }

    /// 公平博弈
    pub fn fairness(&self) -> Vec<&'static str> {
        vec![
            "完全信息无随机因素",
            "先手在多数局面占优",
            "双方均按最优应对",
            "属组合博弈论典范",
        ]
    }
}

impl Rule for NimGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("nim_game")
    }

    fn explain(&self) -> String {
        format!(
            "【尼姆取物游戏】\n{}",
            [
                format!(
                    "开局摆法：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "取子规则：\\n{}",
                    self.taking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负判定：\\n{}",
                    self.winning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "必胜策略：\\n{}",
                    self.strategy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公平博弈：\\n{}",
                    self.fairness()
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
    fn test_nimgamerules_basic() {
        let rules = NimGameRules::new();
        assert_eq!(rules.metadata().name, "尼姆取物游戏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.taking().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.strategy().is_empty());
        assert!(!rules.fairness().is_empty());
    }

    #[test]
    fn test_nimgamerules_validation() {
        let rules = NimGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("nim_game"));
    }

    #[test]
    fn test_nimgamerules_explain() {
        let rules = NimGameRules::new();
        let e = rules.explain();
        assert!(e.contains("开局摆法"));
        assert!(e.contains("取子规则"));
        assert!(e.contains("胜负判定"));
    }
}
