//! 颈肩舒缓锻炼
//!
//! 久坐族颈肩放松、拉伸与姿态

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NeckShoulderReliefRules,
    name: "颈肩舒缓锻炼",
    desc: "久坐族颈肩放松、拉伸与姿态",
    origin: "医学",
    tags: ["健康", "颈肩", "拉伸", "坐姿"]
}

impl NeckShoulderReliefRules {
    /// 久坐放松
    pub fn pause(&self) -> Vec<&'static str> {
        vec!["每坐一小时起身", "活动颈肩", "抬手伸懒腰", "缓解紧绷"]
    }

    /// 拉伸动作
    pub fn stretch(&self) -> Vec<&'static str> {
        vec!["缓慢转头", "耸肩放肩", "手臂向后扩", "轻缓不猛"]
    }

    /// 正确坐姿
    pub fn posture(&self) -> Vec<&'static str> {
        vec!["背靠椅背", "屏幕与眼平", "肩颈放松", "双脚平放"]
    }

    /// 异常就医
    pub fn visit(&self) -> Vec<&'static str> {
        vec!["持续麻木就医", "不盲目大力转动", "结合理疗", "遵医嘱"]
    }
}

impl Rule for NeckShoulderReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("neck_shoulder")
    }

    fn explain(&self) -> String {
        format!(
            "【颈肩舒缓锻炼】\n{}",
            [
                format!(
                    "久坐放松：\\n{}",
                    self.pause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "拉伸动作：\\n{}",
                    self.stretch()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确坐姿：\\n{}",
                    self.posture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "异常就医：\\n{}",
                    self.visit()
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
    fn test_neckshoulderreliefrules_basic() {
        let rules = NeckShoulderReliefRules::new();
        assert_eq!(rules.metadata().name, "颈肩舒缓锻炼");
        assert!(!rules.pause().is_empty());
        assert!(!rules.stretch().is_empty());
        assert!(!rules.posture().is_empty());
        assert!(!rules.visit().is_empty());
    }

    #[test]
    fn test_neckshoulderreliefrules_validation() {
        let rules = NeckShoulderReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("neck_shoulder"));
    }

    #[test]
    fn test_neckshoulderreliefrules_explain() {
        let rules = NeckShoulderReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("久坐放松"));
        assert!(e.contains("拉伸动作"));
        assert!(e.contains("正确坐姿"));
    }
}
