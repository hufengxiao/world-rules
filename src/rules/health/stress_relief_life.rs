//! 压力纾解技巧
//!
//! 应对生活与工作压力、放松身心的方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: StressReliefRules,
    name: "压力纾解技巧",
    desc: "应对生活与工作压力、放松身心的方法",
    origin: "心理学",
    tags: ["健康", "压力", "纾解", "放松"]
}

impl StressReliefRules {
    /// 深呼吸放松
    pub fn breathing(&self) -> Vec<&'static str> {
        vec![
            "压力来袭时先深呼吸",
            "用腹式呼吸放慢节奏",
            "吸气默数呼气更长",
            "重复多次稳定情绪",
        ]
    }

    /// 压力管理
    pub fn manage(&self) -> Vec<&'static str> {
        vec![
            "把大任务拆成小步",
            "合理设定优先次序",
            "安排好时间不积压",
            "学会适度说不",
        ]
    }

    /// 身心调节
    pub fn physical_activity(&self) -> Vec<&'static str> {
        vec![
            "坚持适度运动宣泄",
            "保证充足睡眠",
            "培养放松爱好的间隔",
            "适度亲近自然",
        ]
    }

    /// 社会支持
    pub fn social(&self) -> Vec<&'static str> {
        vec![
            "向信任的人倾诉",
            "主动寻求家人朋友陪伴",
            "必要时寻求专业帮助",
            "不独自硬扛过久",
        ]
    }
}

impl Rule for StressReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("stress_relief")
    }

    fn explain(&self) -> String {
        format!(
            "【压力纾解技巧】\n{}",
            [
                format!(
                    "深呼吸放松：\\n{}",
                    self.breathing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "压力管理：\\n{}",
                    self.manage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "身心调节：\\n{}",
                    self.physical_activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "社会支持：\\n{}",
                    self.social()
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
    fn test_stressreliefrules_basic() {
        let rules = StressReliefRules::new();
        assert_eq!(rules.metadata().name, "压力纾解技巧");
        assert!(!rules.breathing().is_empty());
        assert!(!rules.manage().is_empty());
        assert!(!rules.physical_activity().is_empty());
        assert!(!rules.social().is_empty());
    }

    #[test]
    fn test_stressreliefrules_validation() {
        let rules = StressReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("stress_relief"));
    }

    #[test]
    fn test_stressreliefrules_explain() {
        let rules = StressReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("深呼吸放松"));
        assert!(e.contains("压力管理"));
        assert!(e.contains("身心调节"));
    }
}
