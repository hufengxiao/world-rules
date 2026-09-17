//! 止血与包扎
//!
//! 小伤口出血的处理、止血方法与包扎技术

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BleedingControlBandageRules,
    name: "止血与包扎",
    desc: "小伤口出血的处理、止血方法与包扎技术",
    origin: "医学",
    tags: ["健康", "止血", "包扎", "伤口", "急救"]
}

impl BleedingControlBandageRules {
    /// 初步处理
    pub fn initial(&self) -> Vec<&'static str> {
        vec![
            "先洗净双手查看伤口",
            "用清洁敷料按压止血",
            "抬高伤口高于心脏",
            "保持按压稳定数分钟",
        ]
    }

    /// 持续止血
    pub fn hemostasis(&self) -> Vec<&'static str> {
        vec![
            "严重出血持续加压",
            "敷料浸透在原基础上叠加",
            "不盲目移除已嵌敷料",
            "遵医嘱必要时用止血带",
        ]
    }

    /// 正确包扎
    pub fn bandage(&self) -> Vec<&'static str> {
        vec![
            "包扎松紧适度不过紧",
            "露出指趾末端观察血运",
            "固定端稳妥不打滑",
            "保持敷料清洁干燥",
        ]
    }

    /// 就医判断
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "动脉喷血或深大伤口就医",
            "按压无效仍不止血尽快就医",
            "有异物嵌入不硬取送医",
            "破伤风风险咨询处理",
        ]
    }
}

impl Rule for BleedingControlBandageRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("bleeding_control")
    }

    fn explain(&self) -> String {
        format!(
            "【止血与包扎】\n{}",
            [
                format!(
                    "初步处理：\\n{}",
                    self.initial()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "持续止血：\\n{}",
                    self.hemostasis()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确包扎：\\n{}",
                    self.bandage()
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
    fn test_bleedingcontrolbandagerules_basic() {
        let rules = BleedingControlBandageRules::new();
        assert_eq!(rules.metadata().name, "止血与包扎");
        assert!(!rules.initial().is_empty());
        assert!(!rules.hemostasis().is_empty());
        assert!(!rules.bandage().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_bleedingcontrolbandagerules_validation() {
        let rules = BleedingControlBandageRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("bleeding_control"));
    }

    #[test]
    fn test_bleedingcontrolbandagerules_explain() {
        let rules = BleedingControlBandageRules::new();
        let e = rules.explain();
        assert!(e.contains("初步处理"));
        assert!(e.contains("持续止血"));
        assert!(e.contains("正确包扎"));
    }
}
