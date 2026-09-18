//! 脚后跟干裂护理
//!
//! 脚后跟角质增厚干裂的成因与护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DryCrackedHeelCareRules,
    name: "脚后跟干裂护理",
    desc: "脚后跟角质增厚干裂的成因与护理方法",
    origin: "中国",
    tags: ["健康", "脚裂", "护理", "足部"]
}

impl DryCrackedHeelCareRules {
    /// 成因认识
    pub fn cause(&self) -> Vec<&'static str> {
        vec!["足部皮肤干燥", "常穿硬底鞋摩擦", "久站承压", "角层过厚缺水"]
    }

    /// 日常保湿
    pub fn moisturize(&self) -> Vec<&'static str> {
        vec![
            "每日涂抹保湿霜",
            "温水泡脚后立刻抹",
            "重点滋润后跟",
            "敷滋润过夜",
        ]
    }

    /// 去角质
    pub fn exfoliate(&self) -> Vec<&'static str> {
        vec!["温水泡软浮石", "轻磨死皮", "勿过度剪深", "频繁去角易伤"]
    }

    /// 防护就医
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "选软底合脚鞋",
            "穿袜缓解摩擦",
            "裂口渗血感染",
            "及时就医处理",
        ]
    }
}

impl Rule for DryCrackedHeelCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("cracked_heel")
    }

    fn explain(&self) -> String {
        format!(
            "【脚后跟干裂护理】\n{}",
            [
                format!(
                    "成因认识：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常保湿：\\n{}",
                    self.moisturize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "去角质：\\n{}",
                    self.exfoliate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "防护就医：\\n{}",
                    self.protect()
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
    fn test_drycrackedheelcarerules_basic() {
        let rules = DryCrackedHeelCareRules::new();
        assert_eq!(rules.metadata().name, "脚后跟干裂护理");
        assert!(!rules.cause().is_empty());
        assert!(!rules.moisturize().is_empty());
        assert!(!rules.exfoliate().is_empty());
        assert!(!rules.protect().is_empty());
    }

    #[test]
    fn test_drycrackedheelcarerules_validation() {
        let rules = DryCrackedHeelCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("cracked_heel"));
    }

    #[test]
    fn test_drycrackedheelcarerules_explain() {
        let rules = DryCrackedHeelCareRules::new();
        let e = rules.explain();
        assert!(e.contains("成因认识"));
        assert!(e.contains("日常保湿"));
        assert!(e.contains("去角质"));
    }
}
