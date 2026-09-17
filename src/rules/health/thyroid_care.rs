//! 甲状腺健康管理
//!
//! 甲状腺疾病的早期识别、饮食与随诊规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ThyroidCareRules,
    name: "甲状腺健康管理",
    desc: "甲状腺疾病的早期识别、饮食与随诊规则",
    origin: "国际",
    tags: ["健康", "甲状腺", "内分泌", "慢病", "随诊"]
}

impl ThyroidCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "持续疲劳、体重变化需留意",
            "心悸、怕冷或怕热异常留意",
            "颈部发现肿块或不适需就医",
            "情绪波动与代谢异常并存时评估",
        ]
    }

    /// 饮食注意
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "甲状腺功能异常者规律饮食",
            "碘摄入遵从医嘱适量",
            "均衡营养素支持代谢",
            "避免过度摄入刺激性食物",
        ]
    }

    /// 随诊用药
    pub fn followup(&self) -> Vec<&'static str> {
        vec![
            "遵医嘱规律服用甲状腺药物",
            "不随意自行增减剂量",
            "定期复查甲功",
            "副作用或不适及时沟通医生",
        ]
    }

    /// 生活方式
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "保持规律作息减少压力",
            "适度运动控制体重",
            "禁烟限酒利于内分泌",
            "怀孕或备孕者主动评估甲状腺",
        ]
    }
}

impl Rule for ThyroidCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("thyroid_care")
    }

    fn explain(&self) -> String {
        format!(
            "【甲状腺健康管理】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
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
                    "随诊用药：\\n{}",
                    self.followup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活方式：\\n{}",
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
    fn test_thyroidcarerules_basic() {
        let rules = ThyroidCareRules::new();
        assert_eq!(rules.metadata().name, "甲状腺健康管理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.followup().is_empty());
        assert!(!rules.lifestyle().is_empty());
    }

    #[test]
    fn test_thyroidcarerules_validation() {
        let rules = ThyroidCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("thyroid_care"));
    }

    #[test]
    fn test_thyroidcarerules_explain() {
        let rules = ThyroidCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("饮食注意"));
        assert!(e.contains("随诊用药"));
    }
}
