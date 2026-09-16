//! 高血压生活方式管理
//!
//! 高血压患者控制血压的饮食、运动与作息规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HypertensionLifestyleRules,
    name: "高血压生活方式管理",
    desc: "高血压患者控制血压的饮食、运动与作息规则",
    origin: "医学",
    tags: ["健康", "血压", "慢病", "生活方式"]
}

impl HypertensionLifestyleRules {
    /// 饮食控盐
    pub fn salt(&self) -> Vec<&'static str> {
        vec![
            "每日摄盐量控制不超过5克",
            "少吃腌制腊味与加工食品",
            "烹调多用天然调味替代盐",
            "警惕酱料隐形盐分",
        ]
    }

    /// 均衡饮食
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "多摄入蔬果与全谷物",
            "控制饱和脂肪与胆固醇",
            "适量优质蛋白",
            "限制甜食与含糖饮料",
        ]
    }

    /// 运动与体重
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "每周中等强度运动等时机积累",
            "减轻体重控制腰围",
            "戒烟并控制饮酒",
            "规律作息避免熬夜",
        ]
    }

    /// 用药与监测
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "遵医嘱规律服用降压药",
            "不自行停药或改量",
            "家庭自测固定时段记录",
            "定期复查与医生沟通调整",
        ]
    }
}

impl Rule for HypertensionLifestyleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hypertension")
    }

    fn explain(&self) -> String {
        let parts = vec![
            format!(
                "饮食控盐：\\n{}",
                self.salt()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "均衡饮食：\\n{}",
                self.diet()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "运动与体重：\\n{}",
                self.exercise()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "用药与监测：\\n{}",
                self.medication()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
        ];
        format!("【高血压生活方式管理】\n{}", parts.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_hypertensionlifestylerules_basic() {
        let rules = HypertensionLifestyleRules::new();
        assert_eq!(rules.metadata().name, "高血压生活方式管理");
        assert!(!rules.salt().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.medication().is_empty());
    }

    #[test]
    fn test_hypertensionlifestylerules_validation() {
        let rules = HypertensionLifestyleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hypertension"));
    }

    #[test]
    fn test_hypertensionlifestylerules_explain() {
        let rules = HypertensionLifestyleRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食控盐"));
        assert!(e.contains("均衡饮食"));
        assert!(e.contains("运动与体重"));
    }
}
