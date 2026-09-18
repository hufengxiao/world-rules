//! 痤疮肌肤护理
//!
//! 青春痘痤疮的正确清洁与护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AcneSkinCareRules,
    name: "痤疮肌肤护理",
    desc: "青春痘痤疮的正确清洁与护理方法",
    origin: "中国",
    tags: ["健康", "护肤", "痤疮", "青春痘"]
}

impl AcneSkinCareRules {
    /// 温和清洁
    pub fn clean(&self) -> Vec<&'static str> {
        vec!["每日温和洁面", "温水勿过热", "避免过度搓洗", "选用温和产品"]
    }

    /// 勿挤压
    pub fn avoid_pick(&self) -> Vec<&'static str> {
        vec!["不要用手挤痘", "勿乱抠抓", "挤痘易留疤", "防感染扩散"]
    }

    /// 日常护理
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "注意保湿",
            "选用清爽护肤品",
            "避免油腻厚重",
            "防晒减色素沉着",
        ]
    }

    /// 生活习惯
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "规律作息少熬夜",
            "饮食控油少甜食",
            "勤换枕巾毛巾",
            "严重及时就医",
        ]
    }
}

impl Rule for AcneSkinCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("acne_care")
    }

    fn explain(&self) -> String {
        format!(
            "【痤疮肌肤护理】\n{}",
            [
                format!(
                    "温和清洁：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "勿挤压：\\n{}",
                    self.avoid_pick()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常护理：\\n{}",
                    self.care()
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
    fn test_acneskincarerules_basic() {
        let rules = AcneSkinCareRules::new();
        assert_eq!(rules.metadata().name, "痤疮肌肤护理");
        assert!(!rules.clean().is_empty());
        assert!(!rules.avoid_pick().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.lifestyle().is_empty());
    }

    #[test]
    fn test_acneskincarerules_validation() {
        let rules = AcneSkinCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("acne_care"));
    }

    #[test]
    fn test_acneskincarerules_explain() {
        let rules = AcneSkinCareRules::new();
        let e = rules.explain();
        assert!(e.contains("温和清洁"));
        assert!(e.contains("勿挤压"));
        assert!(e.contains("日常护理"));
    }
}
