//! 打嗝缓解
//!
//! 打嗝诱因、缓解方法与持续打嗝就医提示

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HiccupReliefRules,
    name: "打嗝缓解",
    desc: "打嗝诱因、缓解方法与持续打嗝就医提示",
    origin: "医学",
    tags: ["健康", "打嗝", "膈肌", "缓解"]
}

impl HiccupReliefRules {
    /// 常见诱因
    pub fn causes(&self) -> Vec<&'static str> {
        vec![
            "过快进食或吞咽气",
            "吃得过饱或喝碳酸",
            "情绪激动诱嗝",
            "温度骤变刺激膈肌",
        ]
    }

    /// 缓解方法
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "暂停进食缓慢呼吸",
            "小口饮水缓吞",
            "屏息片刻或轻呼",
            "弯腰缓慢伸展放松",
        ]
    }

    /// 预防
    pub fn prevent(&self) -> Vec<&'static str> {
        vec![
            "细嚼慢咽不狼吞",
            "少饮用碳酸饮料",
            "避免过快大量进食",
            "餐后不立即奔跳",
        ]
    }

    /// 就医判断
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "打嗝持续超过一天就医",
            "伴随吞咽困难或反酸检查",
            "伴胸痛腹胀明显就医",
            "影响进食睡眠需诊治",
        ]
    }
}

impl Rule for HiccupReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hiccup")
    }

    fn explain(&self) -> String {
        format!(
            "【打嗝缓解】\n{}",
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
                    "缓解方法：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防：\\n{}",
                    self.prevent()
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
    fn test_hiccupreliefrules_basic() {
        let rules = HiccupReliefRules::new();
        assert_eq!(rules.metadata().name, "打嗝缓解");
        assert!(!rules.causes().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.prevent().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_hiccupreliefrules_validation() {
        let rules = HiccupReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hiccup"));
    }

    #[test]
    fn test_hiccupreliefrules_explain() {
        let rules = HiccupReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("常见诱因"));
        assert!(e.contains("缓解方法"));
        assert!(e.contains("预防"));
    }
}
