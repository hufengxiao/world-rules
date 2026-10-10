//! 噩梦安抚
//!
//! 夜间噩梦惊醒后的安抚与长期改善方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NightmareSoothingRules,
    name: "噩梦安抚",
    desc: "夜间噩梦惊醒后的安抚与长期改善方法",
    origin: "中国",
    tags: ["睡眠", "心理", "安抚"]
}

impl NightmareSoothingRules {
    /// 惊醒安抚
    pub fn soothing(&self) -> Vec<&'static str> {
        vec![
            "轻声安抚陪伴",
            "开小灯增加安全感",
            "缓慢深呼吸平复",
            "不急于追问梦境",
        ]
    }

    /// 儿童应对
    pub fn children(&self) -> Vec<&'static str> {
        vec![
            "耐心安抚勿责备",
            "给予拥抱与陪伴",
            "白天避免恐怖内容",
            "睡前讲温馨故事",
        ]
    }

    /// 长期改善
    pub fn long_term(&self) -> Vec<&'static str> {
        vec![
            "规律作息减少疲劳",
            "睡前放松减压",
            "避免睡前看刺激内容",
            "白天适度运动",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "频繁噩梦影响生活就医",
            "创伤后反复噩梦应求助",
            "伴情绪问题需心理疏导",
            "儿童持续噩梦宜就诊",
        ]
    }
}

impl Rule for NightmareSoothingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nightmare_soothing")
    }

    fn explain(&self) -> String {
        format!(
            "【噩梦安抚】\n{}",
            [
                format!(
                    "惊醒安抚：\\n{}",
                    self.soothing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "儿童应对：\\n{}",
                    self.children()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长期改善：\\n{}",
                    self.long_term()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_nightmaresoothingrules_basic() {
        let rules = NightmareSoothingRules::new();
        assert_eq!(rules.metadata().name, "噩梦安抚");
        assert!(!rules.soothing().is_empty());
        assert!(!rules.children().is_empty());
        assert!(!rules.long_term().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_nightmaresoothingrules_validation() {
        let rules = NightmareSoothingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("nightmare_soothing"));
    }

    #[test]
    fn test_nightmaresoothingrules_explain() {
        let rules = NightmareSoothingRules::new();
        let e = rules.explain();
        assert!(e.contains("惊醒安抚"));
        assert!(e.contains("儿童应对"));
        assert!(e.contains("长期改善"));
    }
}
