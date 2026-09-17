//! 火山与地质灾害常识
//!
//! 火山活动规律、喷发与安全避险的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VolcanoSafetyRules,
    name: "火山与地质灾害常识",
    desc: "火山活动规律、喷发与安全避险的科学常识",
    origin: "国际",
    tags: ["科学", "火山", "地质", "安全"]
}

impl VolcanoSafetyRules {
    /// 火山活动
    pub fn activity(&self) -> Vec<&'static str> {
        vec![
            "板块运动致火山",
            "岩浆喷发成山",
            "火山活动有周期",
            "监测预警重要",
        ]
    }

    /// 喷发影响
    pub fn effect(&self) -> Vec<&'static str> {
        vec![
            "喷发释放灰烬高温",
            "火山灰影响空气",
            "熔岩流损毁区域",
            "火山气体有害",
        ]
    }

    /// 地震关联
    pub fn quake(&self) -> Vec<&'static str> {
        vec![
            "地震因地壳运动",
            "火山与地震相关",
            "震源浅影响大",
            "把握避险要领",
        ]
    }

    /// 防灾安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["关注火山警示", "远离危险区", "熟悉疏散路线", "配合科学避险"]
    }
}

impl Rule for VolcanoSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("volcano")
    }

    fn explain(&self) -> String {
        format!(
            "【火山与地质灾害常识】\n{}",
            [
                format!(
                    "火山活动：\\n{}",
                    self.activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "喷发影响：\\n{}",
                    self.effect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "地震关联：\\n{}",
                    self.quake()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "防灾安全：\\n{}",
                    self.safety()
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
    fn test_volcanosafetyrules_basic() {
        let rules = VolcanoSafetyRules::new();
        assert_eq!(rules.metadata().name, "火山与地质灾害常识");
        assert!(!rules.activity().is_empty());
        assert!(!rules.effect().is_empty());
        assert!(!rules.quake().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_volcanosafetyrules_validation() {
        let rules = VolcanoSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("volcano"));
    }

    #[test]
    fn test_volcanosafetyrules_explain() {
        let rules = VolcanoSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("火山活动"));
        assert!(e.contains("喷发影响"));
        assert!(e.contains("地震关联"));
    }
}
