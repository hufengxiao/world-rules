//! 地球自转与昼夜
//!
//! 地球自转、昼夜交替与时区的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EarthRotationFactsRules,
    name: "地球自转与昼夜",
    desc: "地球自转、昼夜交替与时区的科学常识",
    origin: "国际",
    tags: ["科学", "地球", "自转", "时区"]
}

impl EarthRotationFactsRules {
    /// 自转原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "地球绕自身轴自转",
            "自西向东旋转",
            "周期约一天",
            "自转产生昼夜交替",
        ]
    }

    /// 昼夜分界
    pub fn day_night(&self) -> Vec<&'static str> {
        vec![
            "阳光照亮半球为白昼",
            "另一半为黑夜",
            "晨昏线移动",
            "自转使昼夜循环",
        ]
    }

    /// 时区概念
    pub fn timezone(&self) -> Vec<&'static str> {
        vec!["经度划分时区", "东西有时差", "标准时间协调", "跨时区需调整"]
    }

    /// 科学生活
    pub fn life(&self) -> Vec<&'static str> {
        vec![
            "理解日出日落规律",
            "提醒时差调整",
            "不因目视误解自转",
            "尊重时间科学",
        ]
    }
}

impl Rule for EarthRotationFactsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("earth_rotation")
    }

    fn explain(&self) -> String {
        format!(
            "【地球自转与昼夜】\n{}",
            [
                format!(
                    "自转原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "昼夜分界：\\n{}",
                    self.day_night()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "时区概念：\\n{}",
                    self.timezone()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "科学生活：\\n{}",
                    self.life()
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
    fn test_earthrotationfactsrules_basic() {
        let rules = EarthRotationFactsRules::new();
        assert_eq!(rules.metadata().name, "地球自转与昼夜");
        assert!(!rules.principle().is_empty());
        assert!(!rules.day_night().is_empty());
        assert!(!rules.timezone().is_empty());
        assert!(!rules.life().is_empty());
    }

    #[test]
    fn test_earthrotationfactsrules_validation() {
        let rules = EarthRotationFactsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("earth_rotation"));
    }

    #[test]
    fn test_earthrotationfactsrules_explain() {
        let rules = EarthRotationFactsRules::new();
        let e = rules.explain();
        assert!(e.contains("自转原理"));
        assert!(e.contains("昼夜分界"));
        assert!(e.contains("时区概念"));
    }
}
