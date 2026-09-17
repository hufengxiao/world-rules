//! 流鼻血处理
//!
//! 流鼻血的正确止血、护理与就医判断

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NosebleedManagementRules,
    name: "流鼻血处理",
    desc: "流鼻血的正确止血、护理与就医判断",
    origin: "医学",
    tags: ["健康", "流鼻血", "止血"]
}

impl NosebleedManagementRules {
    /// 正确止血
    pub fn stop(&self) -> Vec<&'static str> {
        vec![
            "身体前倾以防倒流",
            "按压鼻翼下方",
            "用口呼吸",
            "持续按压几分钟",
        ]
    }

    /// 解除错误
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "不仰头以免血倒流",
            "勿塞棉花堵紧",
            "不频繁擤鼻",
            "勿低头后猛抬头",
        ]
    }

    /// 后续护理
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "保持安静少活动",
            "室内湿润防干",
            "避免抠鼻挖鼻",
            "观察是否止血",
        ]
    }

    /// 就医判断
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "出血不止半小时就医",
            "出血量大伴随不适",
            "反复频繁鼻出血",
            "儿童或老人重视",
        ]
    }
}

impl Rule for NosebleedManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nosebleed")
    }

    fn explain(&self) -> String {
        format!(
            "【流鼻血处理】\n{}",
            [
                format!(
                    "正确止血：\\n{}",
                    self.stop()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解除错误：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "后续护理：\\n{}",
                    self.after()
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
    fn test_nosebleedmanagementrules_basic() {
        let rules = NosebleedManagementRules::new();
        assert_eq!(rules.metadata().name, "流鼻血处理");
        assert!(!rules.stop().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.after().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_nosebleedmanagementrules_validation() {
        let rules = NosebleedManagementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("nosebleed"));
    }

    #[test]
    fn test_nosebleedmanagementrules_explain() {
        let rules = NosebleedManagementRules::new();
        let e = rules.explain();
        assert!(e.contains("正确止血"));
        assert!(e.contains("解除错误"));
        assert!(e.contains("后续护理"));
    }
}
