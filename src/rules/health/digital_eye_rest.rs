//! 屏幕护眼休息
//!
//! 久对屏幕的眼部休息与护眼习惯

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DigitalEyeRestRules,
    name: "屏幕护眼休息",
    desc: "久对屏幕的眼部休息与护眼习惯",
    origin: "医学",
    tags: ["健康", "护眼", "屏幕", "视疲劳"]
}

impl DigitalEyeRestRules {
    /// 屏幕操作
    pub fn screen(&self) -> Vec<&'static str> {
        vec![
            "调整屏幕亮度适当",
            "距离在臂长左右",
            "视线稍向下",
            "防屏幕眩光",
        ]
    }

    /// 定时休息
    pub fn breaks(&self) -> Vec<&'static str> {
        vec![
            "每段远眺放松",
            "眨眼湿润眼睛",
            "久看起身活动",
            "遵循休息法缓解",
        ]
    }

    /// 分泌湿润
    pub fn moist(&self) -> Vec<&'static str> {
        vec![
            "干涩可滴用人工泪液",
            "不揉搓眼睛",
            "空调房注意润",
            "眨眼充分清晰",
        ]
    }

    /// 警示就医
    pub fn care(&self) -> Vec<&'static str> {
        vec!["视力骤降就医", "眼痛明显就医", "红痛畏光检查", "不拖延"]
    }
}

impl Rule for DigitalEyeRestRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("eye_rest")
    }

    fn explain(&self) -> String {
        format!(
            "【屏幕护眼休息】\n{}",
            [
                format!(
                    "屏幕操作：\\n{}",
                    self.screen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "定时休息：\\n{}",
                    self.breaks()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分泌湿润：\\n{}",
                    self.moist()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "警示就医：\\n{}",
                    self.care()
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
    fn test_digitaleyerestrules_basic() {
        let rules = DigitalEyeRestRules::new();
        assert_eq!(rules.metadata().name, "屏幕护眼休息");
        assert!(!rules.screen().is_empty());
        assert!(!rules.breaks().is_empty());
        assert!(!rules.moist().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_digitaleyerestrules_validation() {
        let rules = DigitalEyeRestRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("eye_rest"));
    }

    #[test]
    fn test_digitaleyerestrules_explain() {
        let rules = DigitalEyeRestRules::new();
        let e = rules.explain();
        assert!(e.contains("屏幕操作"));
        assert!(e.contains("定时休息"));
        assert!(e.contains("分泌湿润"));
    }
}
