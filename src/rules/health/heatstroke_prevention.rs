//! 中暑预防与处理
//!
//! 高温天气中暑的预防、识别与应急处理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HeatstrokePreventionRules,
    name: "中暑预防与处理",
    desc: "高温天气中暑的预防、识别与应急处理规则",
    origin: "医学",
    tags: ["健康", "中暑", "高温", "急救", "预防"]
}

impl HeatstrokePreventionRules {
    /// 预防原则
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "高温时段避免长时间在户外",
            "及时补充水分与电解质",
            "穿宽松透气浅色衣物",
            "合理安排在日照强的时段外出",
        ]
    }

    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "认识头晕、恶心、口渴等症状",
            "大汗、皮肤湿冷提示中暑",
            "高热、意识模糊视为重症",
            "出现症状立即停下休息",
        ]
    }

    /// 应急处理
    pub fn first_aid(&self) -> Vec<&'static str> {
        vec![
            "移至阴凉通风处休息",
            "解开衣领降温",
            "用湿毛巾冷敷并适度补水",
            "重症或昏迷立即拨打急救",
        ]
    }

    /// 特殊人群
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "老人儿童更易中暑需重点看护",
            "慢性病者高温减少外出",
            "孕妇避免酷热长时间活动",
            "运动员在高温训练量力",
        ]
    }
}

impl Rule for HeatstrokePreventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("heatstroke")
    }

    fn explain(&self) -> String {
        format!(
            "【中暑预防与处理】\n{}",
            [
                format!(
                    "预防原则：\\n{}",
                    self.prevention()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应急处理：\\n{}",
                    self.first_aid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊人群：\\n{}",
                    self.special()
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
    fn test_heatstrokepreventionrules_basic() {
        let rules = HeatstrokePreventionRules::new();
        assert_eq!(rules.metadata().name, "中暑预防与处理");
        assert!(!rules.prevention().is_empty());
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.first_aid().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_heatstrokepreventionrules_validation() {
        let rules = HeatstrokePreventionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("heatstroke"));
    }

    #[test]
    fn test_heatstrokepreventionrules_explain() {
        let rules = HeatstrokePreventionRules::new();
        let e = rules.explain();
        assert!(e.contains("预防原则"));
        assert!(e.contains("症状识别"));
        assert!(e.contains("应急处理"));
    }
}
