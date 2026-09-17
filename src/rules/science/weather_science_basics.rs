//! 天气与气象常识
//!
//! 认识天气系统、云雨循环与气象安全常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WeatherScienceBasicsRules,
    name: "天气与气象常识",
    desc: "认识天气系统、云雨循环与气象安全常识",
    origin: "国际",
    tags: ["科学", "天气", "气象", "气候"]
}

impl WeatherScienceBasicsRules {
    /// 天气系统
    pub fn systems(&self) -> Vec<&'static str> {
        vec![
            "低压高压系统影响天气",
            "锋面过境带来天气变化",
            "气团性质决定冷暖",
            "气压梯度力驱动风",
        ]
    }

    /// 云与降水
    pub fn cloud_rain(&self) -> Vec<&'static str> {
        vec![
            "积云对流产生雷雨",
            "层云带来持续阴雨",
            "云滴凝结成雨雪",
            "地形抬升促成降水",
        ]
    }

    /// 气象安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "雷暴时避免户外空旷",
            "雷雨天远离高处与金属",
            "寒潮大风注意防护",
            "暴雨洪涝低洼处避险",
        ]
    }

    /// 观测预判
    pub fn observe(&self) -> Vec<&'static str> {
        vec![
            "关注气象预警信号",
            "按气象预报安排出行",
            "理解风力等级",
            "关注极端天气提醒",
        ]
    }
}

impl Rule for WeatherScienceBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("weather")
    }

    fn explain(&self) -> String {
        format!(
            "【天气与气象常识】\n{}",
            [
                format!(
                    "天气系统：\\n{}",
                    self.systems()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "云与降水：\\n{}",
                    self.cloud_rain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "气象安全：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观测预判：\\n{}",
                    self.observe()
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
    fn test_weathersciencebasicsrules_basic() {
        let rules = WeatherScienceBasicsRules::new();
        assert_eq!(rules.metadata().name, "天气与气象常识");
        assert!(!rules.systems().is_empty());
        assert!(!rules.cloud_rain().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.observe().is_empty());
    }

    #[test]
    fn test_weathersciencebasicsrules_validation() {
        let rules = WeatherScienceBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("weather"));
    }

    #[test]
    fn test_weathersciencebasicsrules_explain() {
        let rules = WeatherScienceBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("天气系统"));
        assert!(e.contains("云与降水"));
        assert!(e.contains("气象安全"));
    }
}
