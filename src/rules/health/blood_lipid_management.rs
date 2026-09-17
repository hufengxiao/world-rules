//! 血脂管理
//!
//! 血脂异常的饮食、运动与监测管理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BloodLipidManagementRules,
    name: "血脂管理",
    desc: "血脂异常的饮食、运动与监测管理规则",
    origin: "国际",
    tags: ["健康", "血脂", "胆固醇", "心血管", "慢病"]
}

impl BloodLipidManagementRules {
    /// 饮食调理
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "减少饱和脂肪与反式脂肪",
            "多摄入可溶性膳食纤维",
            "控制红肉与油炸食品",
            "选择优质植物油脂",
        ]
    }

    /// 运动体重
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "坚持规律有氧运动",
            "控制体重与腰围",
            "每周多天适度活动",
            "减重有助于改善血脂",
        ]
    }

    /// 监测用药
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "定期复查血脂四项",
            "遵医嘱规范服用降脂药",
            "不自行停药或改量",
            "关注药物相关不适",
        ]
    }

    /// 生活习惯
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "戒烟并限制饮酒",
            "规律作息控制压力",
            "坚持均衡饮食长期管理",
            "注意与医生沟通个体目标",
        ]
    }
}

impl Rule for BloodLipidManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("blood_lipid")
    }

    fn explain(&self) -> String {
        format!(
            "【血脂管理】\n{}",
            [
                format!(
                    "饮食调理：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动体重：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "监测用药：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活习惯：\\n{}",
                    self.lifestyle()
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
    fn test_bloodlipidmanagementrules_basic() {
        let rules = BloodLipidManagementRules::new();
        assert_eq!(rules.metadata().name, "血脂管理");
        assert!(!rules.diet().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.medication().is_empty());
        assert!(!rules.lifestyle().is_empty());
    }

    #[test]
    fn test_bloodlipidmanagementrules_validation() {
        let rules = BloodLipidManagementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("blood_lipid"));
    }

    #[test]
    fn test_bloodlipidmanagementrules_explain() {
        let rules = BloodLipidManagementRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食调理"));
        assert!(e.contains("运动体重"));
        assert!(e.contains("监测用药"));
    }
}
