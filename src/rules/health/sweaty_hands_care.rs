//! 手汗护理
//!
//! 手部多汗的日常应对与护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SweatyHandsCareRules,
    name: "手汗护理",
    desc: "手部多汗的日常应对与护理方法",
    origin: "中国",
    tags: ["皮肤", "多汗", "护理"]
}

impl SweatyHandsCareRules {
    /// 日常应对
    pub fn daily(&self) -> Vec<&'static str> {
        vec![
            "随身备手帕纸巾",
            "勤洗手保持清洁",
            "避免紧张加重出汗",
            "适度使用止汗产品",
        ]
    }

    /// 护理方法
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "用温和洗手液清洁",
            "洗后涂护手霜",
            "保持手部干爽",
            "避免频繁用酒精搓手",
        ]
    }

    /// 心理调节
    pub fn mental(&self) -> Vec<&'static str> {
        vec![
            "学会放松减压",
            "深呼吸缓解紧张",
            "不必过度焦虑",
            "接纳身体反应",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "严重影响生活可就医",
            "可选药物或物理治疗",
            "排查甲状腺等疾病",
            "遵医嘱选择方案",
        ]
    }
}

impl Rule for SweatyHandsCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sweaty_hands_care")
    }

    fn explain(&self) -> String {
        format!(
            "【手汗护理】\n{}",
            [
                format!(
                    "日常应对：\\n{}",
                    self.daily()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "护理方法：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "心理调节：\\n{}",
                    self.mental()
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
    fn test_sweatyhandscarerules_basic() {
        let rules = SweatyHandsCareRules::new();
        assert_eq!(rules.metadata().name, "手汗护理");
        assert!(!rules.daily().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.mental().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_sweatyhandscarerules_validation() {
        let rules = SweatyHandsCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sweaty_hands_care"));
    }

    #[test]
    fn test_sweatyhandscarerules_explain() {
        let rules = SweatyHandsCareRules::new();
        let e = rules.explain();
        assert!(e.contains("日常应对"));
        assert!(e.contains("护理方法"));
        assert!(e.contains("心理调节"));
    }
}
