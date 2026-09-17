//! 晕车晕船缓解
//!
//! 乘车乘船晕动症的预防、座位与缓解方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MotionSicknessReliefRules,
    name: "晕车晕船缓解",
    desc: "乘车乘船晕动症的预防、座位与缓解方法",
    origin: "医学",
    tags: ["健康", "晕车", "晕船", "晕动症"]
}

impl MotionSicknessReliefRules {
    /// 乘前预防
    pub fn before(&self) -> Vec<&'static str> {
        vec![
            "出行前避免空腹或过饱",
            "选择靠前平稳座位",
            "坐车前少食油腻",
            "必要时遵医嘱用晕车药",
        ]
    }

    /// 乘车姿势
    pub fn during(&self) -> Vec<&'static str> {
        vec![
            "目视远前方固定物",
            "减少低头看手机",
            "开窗透风适度",
            "保持头部稳定",
        ]
    }

    /// 不适应对
    pub fn cope(&self) -> Vec<&'static str> {
        vec![
            "出现恶心欲吐及时停车",
            "闭目休息放松呼吸",
            "含生姜或薄荷缓解",
            "按压合谷内关穴",
        ]
    }

    /// 特殊人群
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "老人儿童更易晕动需照护",
            "孕妇用药需遵医嘱",
            "反复严重晕动就医评估",
            "加强前庭适应锻炼",
        ]
    }
}

impl Rule for MotionSicknessReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("motion_sickness")
    }

    fn explain(&self) -> String {
        format!(
            "【晕车晕船缓解】\n{}",
            [
                format!(
                    "乘前预防：\\n{}",
                    self.before()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "乘车姿势：\\n{}",
                    self.during()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "不适应对：\\n{}",
                    self.cope()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊人群：\\n{}",
                    self.special()
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
    fn test_motionsicknessreliefrules_basic() {
        let rules = MotionSicknessReliefRules::new();
        assert_eq!(rules.metadata().name, "晕车晕船缓解");
        assert!(!rules.before().is_empty());
        assert!(!rules.during().is_empty());
        assert!(!rules.cope().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_motionsicknessreliefrules_validation() {
        let rules = MotionSicknessReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("motion_sickness"));
    }

    #[test]
    fn test_motionsicknessreliefrules_explain() {
        let rules = MotionSicknessReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("乘前预防"));
        assert!(e.contains("乘车姿势"));
        assert!(e.contains("不适应对"));
    }
}
