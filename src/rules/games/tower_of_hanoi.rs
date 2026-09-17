//! 汉诺塔益智
//!
//! 汉诺塔滑块搬运、唯一解的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TowerOfHanoiRules,
    name: "汉诺塔益智",
    desc: "汉诺塔滑块搬运、唯一解的规则",
    origin: "国际",
    tags: ["游戏", "汉诺塔", "益智"]
}

impl TowerOfHanoiRules {
    /// 盘与柱
    pub fn tower(&self) -> Vec<&'static str> {
        vec![
            "三根柱子竖立",
            "大小不同的盘",
            "小在上大在下",
            "初始集中一根柱",
        ]
    }

    /// 搬运规则
    pub fn move_disc(&self) -> Vec<&'static str> {
        vec!["每次移一个盘", "大不压小", "借助中柱", "移到目标柱"]
    }

    /// 最少步数
    pub fn steps(&self) -> Vec<&'static str> {
        vec!["最少步数按规律", "三盘需七步", "讲究递推法", "巧用中间柱"]
    }

    /// 耐心思考
    pub fn approach(&self) -> Vec<&'static str> {
        vec!["由表及里递推", "计划后再动手", "不急求快", "乐在逻辑"]
    }
}

impl Rule for TowerOfHanoiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("hanoi")
    }

    fn explain(&self) -> String {
        format!(
            "【汉诺塔益智】\n{}",
            [
                format!(
                    "盘与柱：\\n{}",
                    self.tower()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "搬运规则：\\n{}",
                    self.move_disc()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "最少步数：\\n{}",
                    self.steps()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "耐心思考：\\n{}",
                    self.approach()
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
    fn test_towerofhanoirules_basic() {
        let rules = TowerOfHanoiRules::new();
        assert_eq!(rules.metadata().name, "汉诺塔益智");
        assert!(!rules.tower().is_empty());
        assert!(!rules.move_disc().is_empty());
        assert!(!rules.steps().is_empty());
        assert!(!rules.approach().is_empty());
    }

    #[test]
    fn test_towerofhanoirules_validation() {
        let rules = TowerOfHanoiRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("hanoi"));
    }

    #[test]
    fn test_towerofhanoirules_explain() {
        let rules = TowerOfHanoiRules::new();
        let e = rules.explain();
        assert!(e.contains("盘与柱"));
        assert!(e.contains("搬运规则"));
        assert!(e.contains("最少步数"));
    }
}
