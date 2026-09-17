//! 气压与天气
//!
//! 大气压、高气压低压与天气变化

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AirPressureWeatherRules,
    name: "气压与天气",
    desc: "大气压、高气压低压与天气变化",
    origin: "国际",
    tags: ["科学", "气压", "天气", "气象"]
}

impl AirPressureWeatherRules {
    /// 气压概念
    pub fn concept(&self) -> Vec<&'static str> {
        vec![
            "空气有重量产生气压",
            "高空气压低",
            "地面气压高",
            "随温度变化",
        ]
    }

    /// 高低压影响
    pub fn pressure(&self) -> Vec<&'static str> {
        vec![
            "高压气沉天气晴",
            "低压气升易阴雨",
            "气压差生风",
            "气象与此相关",
        ]
    }

    /// 体验气压
    pub fn experience(&self) -> Vec<&'static str> {
        vec![
            "高原升高易缺氧",
            "机上耳腔有胀感",
            "气压变化可感知",
            "科学解释即可",
        ]
    }

    /// 看气压测天
    pub fn predict(&self) -> Vec<&'static str> {
        vec![
            "气压计看走势",
            "压降预示变天",
            "升平天气较晴",
            "搭配天气预报",
        ]
    }
}

impl Rule for AirPressureWeatherRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("air_pressure")
    }

    fn explain(&self) -> String {
        format!(
            "【气压与天气】\n{}",
            [
                format!(
                    "气压概念：\\n{}",
                    self.concept()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "高低压影响：\\n{}",
                    self.pressure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体验气压：\\n{}",
                    self.experience()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "看气压测天：\\n{}",
                    self.predict()
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
    fn test_airpressureweatherrules_basic() {
        let rules = AirPressureWeatherRules::new();
        assert_eq!(rules.metadata().name, "气压与天气");
        assert!(!rules.concept().is_empty());
        assert!(!rules.pressure().is_empty());
        assert!(!rules.experience().is_empty());
        assert!(!rules.predict().is_empty());
    }

    #[test]
    fn test_airpressureweatherrules_validation() {
        let rules = AirPressureWeatherRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("air_pressure"));
    }

    #[test]
    fn test_airpressureweatherrules_explain() {
        let rules = AirPressureWeatherRules::new();
        let e = rules.explain();
        assert!(e.contains("气压概念"));
        assert!(e.contains("高低压影响"));
        assert!(e.contains("体验气压"));
    }
}
