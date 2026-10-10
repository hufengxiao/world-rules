//! 牙齿敏感护理
//!
//! 牙本质敏感导致冷热酸甜刺痛的护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SensitiveTeethCareRules,
    name: "牙齿敏感护理",
    desc: "牙本质敏感导致冷热酸甜刺痛的护理方法",
    origin: "中国",
    tags: ["口腔", "牙齿", "护理"]
}

impl SensitiveTeethCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "遇冷热酸甜刺痛",
            "刷牙时牙齿酸痛",
            "刺激去除后缓解",
            "多因牙釉质磨损",
        ]
    }

    /// 刷牙方式
    pub fn brushing(&self) -> Vec<&'static str> {
        vec![
            "用软毛牙刷",
            "采用巴氏刷牙法",
            "避免用力横刷",
            "用抗敏感牙膏",
        ]
    }

    /// 饮食注意
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "减少过冷过热食物",
            "少食酸性食物饮料",
            "避免频繁磨牙",
            "少用牙齿咬硬物",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "持续敏感应就诊",
            "排查龋齿与牙隐裂",
            "必要时做脱敏治疗",
            "定期口腔检查",
        ]
    }
}

impl Rule for SensitiveTeethCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sensitive_teeth_care")
    }

    fn explain(&self) -> String {
        format!(
            "【牙齿敏感护理】\n{}",
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
                    "刷牙方式：\\n{}",
                    self.brushing()
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
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_sensitiveteethcarerules_basic() {
        let rules = SensitiveTeethCareRules::new();
        assert_eq!(rules.metadata().name, "牙齿敏感护理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.brushing().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_sensitiveteethcarerules_validation() {
        let rules = SensitiveTeethCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("sensitive_teeth_care")
        );
    }

    #[test]
    fn test_sensitiveteethcarerules_explain() {
        let rules = SensitiveTeethCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("刷牙方式"));
        assert!(e.contains("饮食注意"));
    }
}
