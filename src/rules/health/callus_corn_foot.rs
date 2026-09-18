//! 脚垫与鸡眼护理
//!
//! 足底硬茧、鸡眼的成因与日常处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CallusCornFootRules,
    name: "脚垫与鸡眼护理",
    desc: "足底硬茧、鸡眼的成因与日常处理",
    origin: "中国",
    tags: ["健康", "足部", "鸡眼", "护理"]
}

impl CallusCornFootRules {
    /// 成因识别
    pub fn identify(&self) -> Vec<&'static str> {
        vec![
            "足底摩擦受压",
            "鞋袜不适形成",
            "硬茧不痛鸡眼痛",
            "按压中央痛感",
        ]
    }

    /// 温水护理
    pub fn soak(&self) -> Vec<&'static str> {
        vec!["温水泡脚软化", "浮石轻磨死皮", "滋润保湿防裂", "勿硬剪过深"]
    }

    /// 鞋袜调整
    pub fn footwear(&self) -> Vec<&'static str> {
        vec!["选合脚宽松鞋", "软垫减压", "勤换袜子", "避免走路过度磨脚"]
    }

    /// 就医情形
    pub fn seek_help(&self) -> Vec<&'static str> {
        vec![
            "鸡眼反复疼痛",
            "感染红肿渗液",
            "糖尿病者足部",
            "及时就医处理",
        ]
    }
}

impl Rule for CallusCornFootRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("callus_corn")
    }

    fn explain(&self) -> String {
        format!(
            "【脚垫与鸡眼护理】\n{}",
            [
                format!(
                    "成因识别：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "温水护理：\\n{}",
                    self.soak()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "鞋袜调整：\\n{}",
                    self.footwear()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医情形：\\n{}",
                    self.seek_help()
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
    fn test_calluscornfootrules_basic() {
        let rules = CallusCornFootRules::new();
        assert_eq!(rules.metadata().name, "脚垫与鸡眼护理");
        assert!(!rules.identify().is_empty());
        assert!(!rules.soak().is_empty());
        assert!(!rules.footwear().is_empty());
        assert!(!rules.seek_help().is_empty());
    }

    #[test]
    fn test_calluscornfootrules_validation() {
        let rules = CallusCornFootRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("callus_corn"));
    }

    #[test]
    fn test_calluscornfootrules_explain() {
        let rules = CallusCornFootRules::new();
        let e = rules.explain();
        assert!(e.contains("成因识别"));
        assert!(e.contains("温水护理"));
        assert!(e.contains("鞋袜调整"));
    }
}
