//! 外伤包扎
//!
//! 小伤口清洗、止血与包扎的家庭处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WoundDressingRules,
    name: "外伤包扎",
    desc: "小伤口清洗、止血与包扎的家庭处理",
    origin: "医学",
    tags: ["健康", "外伤", "包扎", "止血"]
}

impl WoundDressingRules {
    /// 清洁伤口
    pub fn clean(&self) -> Vec<&'static str> {
        vec!["清水冲洗脏污", "棉签蘸碘伏清", "由内向外擦", "保持洁净"]
    }

    /// 止血加压
    pub fn stop_bleed(&self) -> Vec<&'static str> {
        vec!["小伤口按压止血", "抬高伤处", "出血不止就医", "不盲目拔异物"]
    }

    /// 包扎护理
    pub fn bandage(&self) -> Vec<&'static str> {
        vec!["贴创可贴或纱布", "不包太紧", "按时换药透气", "避免水浸"]
    }

    /// 感染警示
    pub fn care(&self) -> Vec<&'static str> {
        vec!["红肿热痛就医", "脓液发热就医", "伤口深大就医", "破伤风评估"]
    }
}

impl Rule for WoundDressingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("wound")
    }

    fn explain(&self) -> String {
        format!(
            "【外伤包扎】\n{}",
            [
                format!(
                    "清洁伤口：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "止血加压：\\n{}",
                    self.stop_bleed()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "包扎护理：\\n{}",
                    self.bandage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "感染警示：\\n{}",
                    self.care()
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
    fn test_wounddressingrules_basic() {
        let rules = WoundDressingRules::new();
        assert_eq!(rules.metadata().name, "外伤包扎");
        assert!(!rules.clean().is_empty());
        assert!(!rules.stop_bleed().is_empty());
        assert!(!rules.bandage().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_wounddressingrules_validation() {
        let rules = WoundDressingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("wound"));
    }

    #[test]
    fn test_wounddressingrules_explain() {
        let rules = WoundDressingRules::new();
        let e = rules.explain();
        assert!(e.contains("清洁伤口"));
        assert!(e.contains("止血加压"));
        assert!(e.contains("包扎护理"));
    }
}
