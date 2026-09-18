//! 干咳缓解护理
//!
//! 干咳无痰的缓解与就医护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DryCoughSoothesRules,
    name: "干咳缓解护理",
    desc: "干咳无痰的缓解与就医护理",
    origin: "医学",
    tags: ["健康", "咳嗽", "干咳", "护理"]
}

impl DryCoughSoothesRules {
    /// 滋润气道
    pub fn moisturize(&self) -> Vec<&'static str> {
        vec!["温水润喉", "蜜柠檬茶", "加湿空气", "减少干痒"]
    }

    /// 减少刺激
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["避烟尘异味", "少冷刺激", "咳嗽勿硬忍", "缓深呼吸"]
    }

    /// 注意补眠
    pub fn rest(&self) -> Vec<&'static str> {
        vec!["多休息少熬夜", "枕头抬高", "睡前润喉", "缓解夜咳"]
    }

    /// 就医判断
    pub fn visit(&self) -> Vec<&'static str> {
        vec!["久咳不愈就医", "带痰血注意", "胸痛发热就医", "慎用止咳药"]
    }
}

impl Rule for DryCoughSoothesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("dry_cough")
    }

    fn explain(&self) -> String {
        format!(
            "【干咳缓解护理】\n{}",
            [
                format!(
                    "滋润气道：\\n{}",
                    self.moisturize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "减少刺激：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "注意补眠：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医判断：\\n{}",
                    self.visit()
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
    fn test_drycoughsoothesrules_basic() {
        let rules = DryCoughSoothesRules::new();
        assert_eq!(rules.metadata().name, "干咳缓解护理");
        assert!(!rules.moisturize().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.rest().is_empty());
        assert!(!rules.visit().is_empty());
    }

    #[test]
    fn test_drycoughsoothesrules_validation() {
        let rules = DryCoughSoothesRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("dry_cough"));
    }

    #[test]
    fn test_drycoughsoothesrules_explain() {
        let rules = DryCoughSoothesRules::new();
        let e = rules.explain();
        assert!(e.contains("滋润气道"));
        assert!(e.contains("减少刺激"));
        assert!(e.contains("注意补眠"));
    }
}
