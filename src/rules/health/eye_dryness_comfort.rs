//! 干眼症
//!
//! 长时间用眼里干红涩等干眼不适的缓解方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EyeDrynessComfortRules,
    name: "干眼症",
    desc: "长时间用眼里干红涩等干眼不适的缓解方法",
    origin: "中国",
    tags: ["健康", "眼睛", "干眼", "护理"]
}

impl EyeDrynessComfortRules {
    /// 干眼表现
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec!["眼干酸涩", "异物感畏光", "视物模糊易累", "眼红流泪"]
    }

    /// 即时缓解
    pub fn relief(&self) -> Vec<&'static str> {
        vec!["适度眨眼", "热敷眼睑", "人工泪液滴眼", "远眺放松"]
    }

    /// 屏幕护眼
    pub fn screen(&self) -> Vec<&'static str> {
        vec!["屏幕在平视线", "定时远眺", "调整亮度", "避免长时间盯屏"]
    }

    /// 就医提示
    pub fn seek_help(&self) -> Vec<&'static str> {
        vec![
            "持续干涩加重",
            "视力明显下降",
            "眼痛红肿不消",
            "及时眼科就诊",
        ]
    }
}

impl Rule for EyeDrynessComfortRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("eye_dryness")
    }

    fn explain(&self) -> String {
        format!(
            "【干眼症】\n{}",
            [
                format!(
                    "干眼表现：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "即时缓解：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "屏幕护眼：\\n{}",
                    self.screen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek_help()
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
    fn test_eyedrynesscomfortrules_basic() {
        let rules = EyeDrynessComfortRules::new();
        assert_eq!(rules.metadata().name, "干眼症");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.screen().is_empty());
        assert!(!rules.seek_help().is_empty());
    }

    #[test]
    fn test_eyedrynesscomfortrules_validation() {
        let rules = EyeDrynessComfortRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("eye_dryness"));
    }

    #[test]
    fn test_eyedrynesscomfortrules_explain() {
        let rules = EyeDrynessComfortRules::new();
        let e = rules.explain();
        assert!(e.contains("干眼表现"));
        assert!(e.contains("即时缓解"));
        assert!(e.contains("屏幕护眼"));
    }
}
