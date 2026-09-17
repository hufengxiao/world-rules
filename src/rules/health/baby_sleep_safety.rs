//! 婴儿睡眠安全
//!
//! 婴儿仰睡、婴儿床与防窒息的安全要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BabySleepSafetyRules,
    name: "婴儿睡眠安全",
    desc: "婴儿仰睡、婴儿床与防窒息的安全要点",
    origin: "医学",
    tags: ["健康", "婴儿", "睡眠", "安全"]
}

impl BabySleepSafetyRules {
    /// 仰卧姿势
    pub fn back(&self) -> Vec<&'static str> {
        vec!["婴儿仰卧睡觉", "不俯卧软枕", "防捂盖口鼻", "头部不固定枕"]
    }

    /// 床面安全
    pub fn crib(&self) -> Vec<&'static str> {
        vec![
            "用坚实婴儿床",
            "床面无松软物",
            "护栏间距合适",
            "不放大枕毛绒",
        ]
    }

    /// 睡眠环境
    pub fn room(&self) -> Vec<&'static str> {
        vec!["温度舒适不过热", "不吸烟环境", "房间通风", "面朝上睡"]
    }

    /// 照护观察
    pub fn monitor(&self) -> Vec<&'static str> {
        vec!["睡时留意呼吸", "同室不同床", "异常及时检查", "遵循安全惯例"]
    }
}

impl Rule for BabySleepSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("baby_sleep")
    }

    fn explain(&self) -> String {
        format!(
            "【婴儿睡眠安全】\n{}",
            [
                format!(
                    "仰卧姿势：\\n{}",
                    self.back()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "床面安全：\\n{}",
                    self.crib()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "睡眠环境：\\n{}",
                    self.room()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "照护观察：\\n{}",
                    self.monitor()
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
    fn test_babysleepsafetyrules_basic() {
        let rules = BabySleepSafetyRules::new();
        assert_eq!(rules.metadata().name, "婴儿睡眠安全");
        assert!(!rules.back().is_empty());
        assert!(!rules.crib().is_empty());
        assert!(!rules.room().is_empty());
        assert!(!rules.monitor().is_empty());
    }

    #[test]
    fn test_babysleepsafetyrules_validation() {
        let rules = BabySleepSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("baby_sleep"));
    }

    #[test]
    fn test_babysleepsafetyrules_explain() {
        let rules = BabySleepSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("仰卧姿势"));
        assert!(e.contains("床面安全"));
        assert!(e.contains("睡眠环境"));
    }
}
