//! 邻里噪音克制
//!
//! 居家噪音、作息与尊重邻里的克制

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ApartmentNoiseConsiderateRules,
    name: "邻里噪音克制",
    desc: "居家噪音、作息与尊重邻里的克制",
    origin: "中国",
    tags: ["社交", "噪音", "邻里", "克制"]
}

impl ApartmentNoiseConsiderateRules {
    /// 音量控制
    pub fn volume(&self) -> Vec<&'static str> {
        vec!["深夜音乐调小", "电视不大声", "宠物少吠闹", "注意隔音"]
    }

    /// 作息尊重
    pub fn hours(&self) -> Vec<&'static str> {
        vec!["早晚不扰邻", "练琴装修择时", "避开他人休息", "体谅上下楼"]
    }

    /// 物品移动
    pub fn movement(&self) -> Vec<&'static str> {
        vec!["不拖拽重家具", "轻拿轻放", "地板加保护垫", "减少回声"]
    }

    /// 冲突化解
    pub fn reconcile(&self) -> Vec<&'static str> {
        vec!["被提醒先道歉", "友善沟通", "物业可调解", "和气解决"]
    }
}

impl Rule for ApartmentNoiseConsiderateRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("apartment_noise")
    }

    fn explain(&self) -> String {
        format!(
            "【邻里噪音克制】\n{}",
            [
                format!(
                    "音量控制：\\n{}",
                    self.volume()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "作息尊重：\\n{}",
                    self.hours()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "物品移动：\\n{}",
                    self.movement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "冲突化解：\\n{}",
                    self.reconcile()
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
    fn test_apartmentnoiseconsideraterules_basic() {
        let rules = ApartmentNoiseConsiderateRules::new();
        assert_eq!(rules.metadata().name, "邻里噪音克制");
        assert!(!rules.volume().is_empty());
        assert!(!rules.hours().is_empty());
        assert!(!rules.movement().is_empty());
        assert!(!rules.reconcile().is_empty());
    }

    #[test]
    fn test_apartmentnoiseconsideraterules_validation() {
        let rules = ApartmentNoiseConsiderateRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("apartment_noise"));
    }

    #[test]
    fn test_apartmentnoiseconsideraterules_explain() {
        let rules = ApartmentNoiseConsiderateRules::new();
        let e = rules.explain();
        assert!(e.contains("音量控制"));
        assert!(e.contains("作息尊重"));
        assert!(e.contains("物品移动"));
    }
}
