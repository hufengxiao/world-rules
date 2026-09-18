//! 晕车晕船预防
//!
//! 预防晕动症、乘车坐位与缓解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MotionSicknessPreventRules,
    name: "晕车晕船预防",
    desc: "预防晕动症、乘车坐位与缓解",
    origin: "医学",
    tags: ["健康", "晕车", "晕船", "预防"]
}

impl MotionSicknessPreventRules {
    /// 出行选择
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "坐车前部前排",
            "顺行驶方向坐",
            "靠窗视线放宽",
            "少低头玩手机",
        ]
    }

    /// 饮食注意
    pub fn diet(&self) -> Vec<&'static str> {
        vec!["出行前清淡", "勿空腹勿过饱", "少油腻刺激", "备生姜薄荷"]
    }

    /// 缓解措施
    pub fn relieve(&self) -> Vec<&'static str> {
        vec!["远眺固定物", "闭目休息", "晕车药提前服", "按压内关"]
    }

    /// 及时处理
    pub fn handle(&self) -> Vec<&'static str> {
        vec!["不适开窗通气", "恶心想吐停车歇", "严重及时就医", "减轻症状"]
    }
}

impl Rule for MotionSicknessPreventRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("motion_sickness")
    }

    fn explain(&self) -> String {
        format!(
            "【晕车晕船预防】\n{}",
            [
                format!(
                    "出行选择：\\n{}",
                    self.seating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食注意：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "缓解措施：\\n{}",
                    self.relieve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "及时处理：\\n{}",
                    self.handle()
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
    fn test_motionsicknesspreventrules_basic() {
        let rules = MotionSicknessPreventRules::new();
        assert_eq!(rules.metadata().name, "晕车晕船预防");
        assert!(!rules.seating().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.relieve().is_empty());
        assert!(!rules.handle().is_empty());
    }

    #[test]
    fn test_motionsicknesspreventrules_validation() {
        let rules = MotionSicknessPreventRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("motion_sickness"));
    }

    #[test]
    fn test_motionsicknesspreventrules_explain() {
        let rules = MotionSicknessPreventRules::new();
        let e = rules.explain();
        assert!(e.contains("出行选择"));
        assert!(e.contains("饮食注意"));
        assert!(e.contains("缓解措施"));
    }
}
