//! 屏幕护眼与用眼休息
//!
//! 看屏久视导致眼疲劳、干涩的护眼方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ScreenEyeStrainCareRules,
    name: "屏幕护眼与用眼休息",
    desc: "看屏久视导致眼疲劳、干涩的护眼方法",
    origin: "医学",
    tags: ["健康", "护眼", "屏幕", "眼睛", "用眼休息"]
}

impl ScreenEyeStrainCareRules {
    /// 合理用屏
    pub fn screen(&self) -> Vec<&'static str> {
        vec![
            "保持屏幕一臂距离",
            "视线略低于屏幕",
            "调整亮度不刺眼",
            "适时休息减少连续看屏",
        ]
    }

    /// 眨眼休息
    pub fn blink(&self) -> Vec<&'static str> {
        vec![
            "有意识地多眨眼",
            "遵循用眼20-20-20",
            "远望放松调节肌",
            "避免久盯不眨眼",
        ]
    }

    /// 环境光线
    pub fn lighting(&self) -> Vec<&'static str> {
        vec![
            "环境光不过强过暗",
            "避开直射反光",
            "可轻微调低高亮",
            "夜间减少蓝光刺激",
        ]
    }

    /// 就医提示
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "眼红痛持续就医",
            "视力模糊加重检查",
            "干涩明显用人工泪液遵医嘱",
            "定期检查排除眼疾",
        ]
    }
}

impl Rule for ScreenEyeStrainCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("blue_light_eye")
    }

    fn explain(&self) -> String {
        format!(
            "【屏幕护眼与用眼休息】\n{}",
            [
                format!(
                    "合理用屏：\\n{}",
                    self.screen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "眨眼休息：\\n{}",
                    self.blink()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境光线：\\n{}",
                    self.lighting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
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
    fn test_screeneyestraincarerules_basic() {
        let rules = ScreenEyeStrainCareRules::new();
        assert_eq!(rules.metadata().name, "屏幕护眼与用眼休息");
        assert!(!rules.screen().is_empty());
        assert!(!rules.blink().is_empty());
        assert!(!rules.lighting().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_screeneyestraincarerules_validation() {
        let rules = ScreenEyeStrainCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("blue_light_eye"));
    }

    #[test]
    fn test_screeneyestraincarerules_explain() {
        let rules = ScreenEyeStrainCareRules::new();
        let e = rules.explain();
        assert!(e.contains("合理用屏"));
        assert!(e.contains("眨眼休息"));
        assert!(e.contains("环境光线"));
    }
}
