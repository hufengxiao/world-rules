//! 季节性过敏应对
//!
//! 花粉过敏的打喷嚏、抗敏与防护规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SeasonalAllergyReliefRules,
    name: "季节性过敏应对",
    desc: "花粉过敏的打喷嚏、抗敏与防护规则",
    origin: "医学",
    tags: ["健康", "过敏", "花粉", "鼻敏感"]
}

impl SeasonalAllergyReliefRules {
    /// 识别过敏
    pub fn identify(&self) -> Vec<&'static str> {
        vec![
            "季节发作打喷嚏流涕",
            "眼痒红敏",
            "鼻塞咽喉痒",
            "过敏季更明显",
        ]
    }

    /// 防护措施
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "过敏季少外出",
            "外出戴口罩",
            "回家洗尘换衣",
            "关闭门窗滤空气",
        ]
    }

    /// 缓解用药
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "遵医嘱使用抗过敏药",
            "眼药滴服按说明",
            "避免胡乱试药",
            "症状重就诊",
        ]
    }

    /// 就医判断
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "呼吸不畅立即就医",
            "诱发哮喘及时处理",
            "症状持续多日就诊",
            "确定过敏原作管理",
        ]
    }
}

impl Rule for SeasonalAllergyReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("allergy")
    }

    fn explain(&self) -> String {
        format!(
            "【季节性过敏应对】\n{}",
            [
                format!(
                    "识别过敏：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "防护措施：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "缓解用药：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医判断：\\n{}",
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
    fn test_seasonalallergyreliefrules_basic() {
        let rules = SeasonalAllergyReliefRules::new();
        assert_eq!(rules.metadata().name, "季节性过敏应对");
        assert!(!rules.identify().is_empty());
        assert!(!rules.protect().is_empty());
        assert!(!rules.medication().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_seasonalallergyreliefrules_validation() {
        let rules = SeasonalAllergyReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("allergy"));
    }

    #[test]
    fn test_seasonalallergyreliefrules_explain() {
        let rules = SeasonalAllergyReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("识别过敏"));
        assert!(e.contains("防护措施"));
        assert!(e.contains("缓解用药"));
    }
}
