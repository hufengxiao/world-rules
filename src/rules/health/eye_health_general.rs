//! 眼部疲劳与眼保健
//!
//! 缓解视疲劳、科学用眼与眼保健的习惯规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EyeHealthGeneralRules,
    name: "眼部疲劳与眼保健",
    desc: "缓解视疲劳、科学用眼与眼保健的习惯规则",
    origin: "国际",
    tags: ["健康", "眼睛", "视疲劳", "干眼"]
}

impl EyeHealthGeneralRules {
    /// 视疲劳缓解
    pub fn fatigue(&self) -> Vec<&'static str> {
        vec![
            "长时间用眼后远眺放松",
            "遵循20-20-20法则休息",
            "短暂闭眼或热敷舒缓",
            "不持续强光下用眼",
        ]
    }

    /// 环境护眼
    pub fn environment(&self) -> Vec<&'static str> {
        vec![
            "保持用眼光线充足舒适",
            "屏幕亮度与环境协调",
            "减少长时间盯着同一处",
            "避免光线直射眼睛",
        ]
    }

    /// 干眼应对
    pub fn dry_eye(&self) -> Vec<&'static str> {
        vec![
            "多眨眼保持泪液湿润",
            "干涩明显可适度人工泪液",
            "室内注意加湿",
            "长期干涩就医评估",
        ]
    }

    /// 就医提示
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "视物变形或黑影增多就医",
            "眼突然疼痛或视力下降急诊",
            "近视者定期检查度数",
            "儿童护眼重视早期筛查",
        ]
    }
}

impl Rule for EyeHealthGeneralRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("eye_health")
    }

    fn explain(&self) -> String {
        format!(
            "【眼部疲劳与眼保健】\n{}",
            [
                format!(
                    "视疲劳缓解：\\n{}",
                    self.fatigue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境护眼：\\n{}",
                    self.environment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "干眼应对：\\n{}",
                    self.dry_eye()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek_care()
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
    fn test_eyehealthgeneralrules_basic() {
        let rules = EyeHealthGeneralRules::new();
        assert_eq!(rules.metadata().name, "眼部疲劳与眼保健");
        assert!(!rules.fatigue().is_empty());
        assert!(!rules.environment().is_empty());
        assert!(!rules.dry_eye().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_eyehealthgeneralrules_validation() {
        let rules = EyeHealthGeneralRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("eye_health"));
    }

    #[test]
    fn test_eyehealthgeneralrules_explain() {
        let rules = EyeHealthGeneralRules::new();
        let e = rules.explain();
        assert!(e.contains("视疲劳缓解"));
        assert!(e.contains("环境护眼"));
        assert!(e.contains("干眼应对"));
    }
}
