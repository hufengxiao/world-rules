//! 鼻塞鼻堵缓解
//!
//! 感冒过敏等所致鼻塞的缓解与护理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NasalCongestionReliefRules,
    name: "鼻塞鼻堵缓解",
    desc: "感冒过敏等所致鼻塞的缓解与护理规则",
    origin: "医学",
    tags: ["健康", "鼻塞", "鼻子", "护理"]
}

impl NasalCongestionReliefRules {
    /// 日常缓解
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "用生理盐水洗鼻",
            "热蒸汽湿润鼻腔",
            "保持室内空气湿润",
            "按需睡时稍稍抬高头部",
        ]
    }

    /// 通气技巧
    pub fn breathe(&self) -> Vec<&'static str> {
        vec![
            "交替按压鼻翼通流",
            "避免反复猛烈擤鼻",
            "用纸巾轻擦",
            "保证通风换气",
        ]
    }

    /// 用药注意
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "不长期滥用滴鼻药",
            "感冒鼻塞遵医嘱",
            "过敏鼻塞找过敏原",
            "连续用药别超建议天",
        ]
    }

    /// 就医提示
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "鼻塞伴发热头痛就医",
            "单侧脓涕或血涕检查",
            "鼻塞影响睡眠多日就诊",
            "儿童鼻塞奶注意",
        ]
    }
}

impl Rule for NasalCongestionReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nasal_stuffy")
    }

    fn explain(&self) -> String {
        format!(
            "【鼻塞鼻堵缓解】\n{}",
            [
                format!(
                    "日常缓解：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "通气技巧：\\n{}",
                    self.breathe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用药注意：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek()
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
    fn test_nasalcongestionreliefrules_basic() {
        let rules = NasalCongestionReliefRules::new();
        assert_eq!(rules.metadata().name, "鼻塞鼻堵缓解");
        assert!(!rules.relief().is_empty());
        assert!(!rules.breathe().is_empty());
        assert!(!rules.medication().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_nasalcongestionreliefrules_validation() {
        let rules = NasalCongestionReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("nasal_stuffy"));
    }

    #[test]
    fn test_nasalcongestionreliefrules_explain() {
        let rules = NasalCongestionReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("日常缓解"));
        assert!(e.contains("通气技巧"));
        assert!(e.contains("用药注意"));
    }
}
