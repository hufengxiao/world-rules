//! 糖尿病患者安全运动
//!
//! 糖尿病运动时机、强度与低血糖预防

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DiabetesExerciseSafetyRules,
    name: "糖尿病患者安全运动",
    desc: "糖尿病运动时机、强度与低血糖预防",
    origin: "医学",
    tags: ["健康", "糖尿病", "运动", "安全"]
}

impl DiabetesExerciseSafetyRules {
    /// 宜选时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "宜在餐后运动",
            "饭后约一小时",
            "避免空腹运动",
            "有基础先咨询",
        ]
    }

    /// 强度适中
    pub fn intensity(&self) -> Vec<&'static str> {
        vec!["从温和运动起", "以能交谈为度", "循序渐进加量", "不过度疲劳"]
    }

    /// 防低血糖
    pub fn hypoglycemia(&self) -> Vec<&'static str> {
        vec!["运动前测血糖", "随身备糖块", "头晕心慌即停", "及时补糖"]
    }

    /// 血糖监测
    pub fn monitor(&self) -> Vec<&'static str> {
        vec!["运动前后对比", "异常及时就医", "结合用药方案", "谨遵医嘱"]
    }
}

impl Rule for DiabetesExerciseSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("diabetes_exercise")
    }

    fn explain(&self) -> String {
        format!(
            "【糖尿病患者安全运动】\n{}",
            [
                format!(
                    "宜选时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "强度适中：\\n{}",
                    self.intensity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "防低血糖：\\n{}",
                    self.hypoglycemia()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "血糖监测：\\n{}",
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
    fn test_diabetesexercisesafetyrules_basic() {
        let rules = DiabetesExerciseSafetyRules::new();
        assert_eq!(rules.metadata().name, "糖尿病患者安全运动");
        assert!(!rules.timing().is_empty());
        assert!(!rules.intensity().is_empty());
        assert!(!rules.hypoglycemia().is_empty());
        assert!(!rules.monitor().is_empty());
    }

    #[test]
    fn test_diabetesexercisesafetyrules_validation() {
        let rules = DiabetesExerciseSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("diabetes_exercise"));
    }

    #[test]
    fn test_diabetesexercisesafetyrules_explain() {
        let rules = DiabetesExerciseSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("宜选时机"));
        assert!(e.contains("强度适中"));
        assert!(e.contains("防低血糖"));
    }
}
