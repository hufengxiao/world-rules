//! 高血脂运动计划
//!
//! 降脂运动类型、时长与饮食配合

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CholesterolExercisePlanRules,
    name: "高血脂运动计划",
    desc: "降脂运动类型、时长与饮食配合",
    origin: "医学",
    tags: ["健康", "血脂", "运动", "饮食"]
}

impl CholesterolExercisePlanRules {
    /// 有氧为主
    pub fn aerobic(&self) -> Vec<&'static str> {
        vec![
            "快走慢跑游泳",
            "每周至少五次",
            "每次三十分钟",
            "中等强度有氧",
        ]
    }

    /// 注意防护
    pub fn prevent(&self) -> Vec<&'static str> {
        vec!["循序渐进", "热身拉伸不忘", "戴护具适宜", "预防损伤"]
    }

    /// 饮食配合
    pub fn diet(&self) -> Vec<&'static str> {
        vec!["少油腻多蔬果", "控饱和脂肪", "增加膳食纤维", "规律三餐"]
    }

    /// 监测反馈
    pub fn monitor(&self) -> Vec<&'static str> {
        vec!["定期查血脂", "体重管理", "异常遵医嘱", "持之以恒"]
    }
}

impl Rule for CholesterolExercisePlanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("cholesterol")
    }

    fn explain(&self) -> String {
        format!(
            "【高血脂运动计划】\n{}",
            [
                format!(
                    "有氧为主：\\n{}",
                    self.aerobic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "注意防护：\\n{}",
                    self.prevent()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食配合：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "监测反馈：\\n{}",
                    self.monitor()
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
    fn test_cholesterolexerciseplanrules_basic() {
        let rules = CholesterolExercisePlanRules::new();
        assert_eq!(rules.metadata().name, "高血脂运动计划");
        assert!(!rules.aerobic().is_empty());
        assert!(!rules.prevent().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.monitor().is_empty());
    }

    #[test]
    fn test_cholesterolexerciseplanrules_validation() {
        let rules = CholesterolExercisePlanRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("cholesterol"));
    }

    #[test]
    fn test_cholesterolexerciseplanrules_explain() {
        let rules = CholesterolExercisePlanRules::new();
        let e = rules.explain();
        assert!(e.contains("有氧为主"));
        assert!(e.contains("注意防护"));
        assert!(e.contains("饮食配合"));
    }
}
