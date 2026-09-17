//! 清新口气
//!
//! 保持口气清新、处理口臭的基本方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FreshBreathHabitRules,
    name: "清新口气",
    desc: "保持口气清新、处理口臭的基本方法",
    origin: "牙科",
    tags: ["健康", "口气", "口臭", "清洁"]
}

impl FreshBreathHabitRules {
    /// 清洁根源
    pub fn clean(&self) -> Vec<&'static str> {
        vec!["认真刷牙清舌", "牙线去残留", "减少异味菌", "饭后漱口"]
    }

    /// 注意饮食
    pub fn diet(&self) -> Vec<&'static str> {
        vec!["少食重味刺激", "戒烟限酒", "多喝水", "吃清新果蔬"]
    }

    /// 异常检查
    pub fn check(&self) -> Vec<&'static str> {
        vec!["长期口臭就医", "查牙龈牙周", "查胃肠原因", "对症处理"]
    }

    /// 保持风度
    pub fn gracious(&self) -> Vec<&'static str> {
        vec!["重要场合备口香糖", "说话距离适当", "不捂嘴失礼", "自然自信"]
    }
}

impl Rule for FreshBreathHabitRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("fresh_breath")
    }

    fn explain(&self) -> String {
        format!(
            "【清新口气】\n{}",
            [
                format!(
                    "清洁根源：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "注意饮食：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "异常检查：\\n{}",
                    self.check()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保持风度：\\n{}",
                    self.gracious()
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
    fn test_freshbreathhabitrules_basic() {
        let rules = FreshBreathHabitRules::new();
        assert_eq!(rules.metadata().name, "清新口气");
        assert!(!rules.clean().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.check().is_empty());
        assert!(!rules.gracious().is_empty());
    }

    #[test]
    fn test_freshbreathhabitrules_validation() {
        let rules = FreshBreathHabitRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("fresh_breath"));
    }

    #[test]
    fn test_freshbreathhabitrules_explain() {
        let rules = FreshBreathHabitRules::new();
        let e = rules.explain();
        assert!(e.contains("清洁根源"));
        assert!(e.contains("注意饮食"));
        assert!(e.contains("异常检查"));
    }
}
