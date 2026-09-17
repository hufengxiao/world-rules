//! 儿童疫苗接种
//!
//! 儿童免费疫苗、接种时间与接种前后注意

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChildVaccinationScheduleRules,
    name: "儿童疫苗接种",
    desc: "儿童免费疫苗、接种时间与接种前后注意",
    origin: "医学",
    tags: ["健康", "疫苗", "接种", "儿童"]
}

impl ChildVaccinationScheduleRules {
    /// 按表接种
    pub fn schedule(&self) -> Vec<&'static str> {
        vec![
            "按免疫规划接种",
            "出生至各月龄安排",
            "按时不延误",
            "保存接种证",
        ]
    }

    /// 接种前
    pub fn before(&self) -> Vec<&'static str> {
        vec![
            "宝宝健康打疫苗",
            "发热感冒暂缓",
            "告知既往反应",
            "空腹不适宜",
        ]
    }

    /// 接种后
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "留观约半小时",
            "局部红肿多为正常",
            "发热可适当应对",
            "反应重及时就医",
        ]
    }

    /// 补充自费
    pub fn optional(&self) -> Vec<&'static str> {
        vec!["自费苗按需选", "问清适应症", "自愿选择", "遵循医生建议"]
    }
}

impl Rule for ChildVaccinationScheduleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("vaccination")
    }

    fn explain(&self) -> String {
        format!(
            "【儿童疫苗接种】\n{}",
            [
                format!(
                    "按表接种：\\n{}",
                    self.schedule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "接种前：\\n{}",
                    self.before()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "接种后：\\n{}",
                    self.after()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "补充自费：\\n{}",
                    self.optional()
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
    fn test_childvaccinationschedulerules_basic() {
        let rules = ChildVaccinationScheduleRules::new();
        assert_eq!(rules.metadata().name, "儿童疫苗接种");
        assert!(!rules.schedule().is_empty());
        assert!(!rules.before().is_empty());
        assert!(!rules.after().is_empty());
        assert!(!rules.optional().is_empty());
    }

    #[test]
    fn test_childvaccinationschedulerules_validation() {
        let rules = ChildVaccinationScheduleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("vaccination"));
    }

    #[test]
    fn test_childvaccinationschedulerules_explain() {
        let rules = ChildVaccinationScheduleRules::new();
        let e = rules.explain();
        assert!(e.contains("按表接种"));
        assert!(e.contains("接种前"));
        assert!(e.contains("接种后"));
    }
}
