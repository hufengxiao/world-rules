//! 牙痛缓解与护牙
//!
//! 牙痛诱因识别、临时缓解与就医护理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ToothAcheCareRules,
    name: "牙痛缓解与护牙",
    desc: "牙痛诱因识别、临时缓解与就医护理规则",
    origin: "医学",
    tags: ["健康", "牙痛", "口腔", "护牙"]
}

impl ToothAcheCareRules {
    /// 常见诱因
    pub fn causes(&self) -> Vec<&'static str> {
        vec![
            "龋齿受刺激酸痛",
            "牙龈发炎肿胀",
            "智齿问题疼痛",
            "冷热刺激敏感",
        ]
    }

    /// 临时缓解
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "用温水轻轻漱口",
            "避免过冷过热食物",
            "疼侧少咀嚼",
            "必要时用止痛药遵说明",
        ]
    }

    /// 护牙习惯
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "早晚正确刷牙",
            "使用牙线清洁",
            "少吃甜黏食物",
            "定期口腔检查",
        ]
    }

    /// 就医判断
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "持续剧痛就医",
            "肿扩散发热就医",
            "松动或受伤检查",
            "儿童牙痛及时看",
        ]
    }
}

impl Rule for ToothAcheCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("tooth_ache")
    }

    fn explain(&self) -> String {
        format!(
            "【牙痛缓解与护牙】\n{}",
            [
                format!(
                    "常见诱因：\\n{}",
                    self.causes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "临时缓解：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "护牙习惯：\\n{}",
                    self.care()
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
    fn test_toothachecarerules_basic() {
        let rules = ToothAcheCareRules::new();
        assert_eq!(rules.metadata().name, "牙痛缓解与护牙");
        assert!(!rules.causes().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_toothachecarerules_validation() {
        let rules = ToothAcheCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("tooth_ache"));
    }

    #[test]
    fn test_toothachecarerules_explain() {
        let rules = ToothAcheCareRules::new();
        let e = rules.explain();
        assert!(e.contains("常见诱因"));
        assert!(e.contains("临时缓解"));
        assert!(e.contains("护牙习惯"));
    }
}
