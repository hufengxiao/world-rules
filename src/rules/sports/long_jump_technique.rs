//! 跳远技术与规则
//!
//! 急行跳远的助跑、起跳、腾空与落地规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LongJumpTechniqueRules,
    name: "跳远技术与规则",
    desc: "急行跳远的助跑、起跳、腾空与落地规则",
    origin: "国际",
    tags: ["体育", "跳远", "田径"]
}

impl LongJumpTechniqueRules {
    /// 助跑起跳
    pub fn run_takeoff(&self) -> Vec<&'static str> {
        vec![
            "助跑节奏稳定",
            "踏板准确不犯规",
            "起跳迅速有力",
            "重心略降再跃起",
        ]
    }

    /// 腾空着陆
    pub fn flight(&self) -> Vec<&'static str> {
        vec![
            "腾空保持合理姿势",
            "收腿前伸远润",
            "落地缓冲防伤",
            "落地痕迹从近点算",
        ]
    }

    /// 犯规判定
    pub fn foul(&self) -> Vec<&'static str> {
        vec![
            "踏板越线犯规",
            "触及沙坑外判无效",
            "落地后后倒量最近点",
            "连续犯规成绩受限",
        ]
    }

    /// 训练安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "充分热身防拉伤",
            "沙坑平整松软",
            "起跳板完好",
            "量力进阶不蛮练",
        ]
    }
}

impl Rule for LongJumpTechniqueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("long_jump")
    }

    fn explain(&self) -> String {
        format!(
            "【跳远技术与规则】\n{}",
            [
                format!(
                    "助跑起跳：\\n{}",
                    self.run_takeoff()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "腾空着陆：\\n{}",
                    self.flight()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "犯规判定：\\n{}",
                    self.foul()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "训练安全：\\n{}",
                    self.safety()
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
    fn test_longjumptechniquerules_basic() {
        let rules = LongJumpTechniqueRules::new();
        assert_eq!(rules.metadata().name, "跳远技术与规则");
        assert!(!rules.run_takeoff().is_empty());
        assert!(!rules.flight().is_empty());
        assert!(!rules.foul().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_longjumptechniquerules_validation() {
        let rules = LongJumpTechniqueRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("long_jump"));
    }

    #[test]
    fn test_longjumptechniquerules_explain() {
        let rules = LongJumpTechniqueRules::new();
        let e = rules.explain();
        assert!(e.contains("助跑起跳"));
        assert!(e.contains("腾空着陆"));
        assert!(e.contains("犯规判定"));
    }
}
