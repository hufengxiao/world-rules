//! 失眠调理
//!
//! 入睡困难与睡眠质量差的日常调理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InsomniaCareRules,
    name: "失眠调理",
    desc: "入睡困难与睡眠质量差的日常调理方法",
    origin: "中国",
    tags: ["睡眠", "调理", "心理"]
}

impl InsomniaCareRules {
    /// 作息规律
    pub fn routine(&self) -> Vec<&'static str> {
        vec![
            "固定时间上床起床",
            "周末也不睡懒觉",
            "午睡不超过半小时",
            "困了才上床",
        ]
    }

    /// 睡前准备
    pub fn bedtime(&self) -> Vec<&'static str> {
        vec![
            "睡前两小时不进食",
            "避免咖啡因与浓茶",
            "睡前放松不做剧烈运动",
            "营造安静昏暗环境",
        ]
    }

    /// 心理调节
    pub fn mental(&self) -> Vec<&'static str> {
        vec![
            "不强迫自己入睡",
            "躺久难眠可起身放松",
            "写日记清空思绪",
            "减少对失眠的焦虑",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "长期失眠影响生活就医",
            "伴情绪低落应就诊",
            "勿长期自行服安眠药",
            "排查潜在身体疾病",
        ]
    }
}

impl Rule for InsomniaCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("insomnia_care")
    }

    fn explain(&self) -> String {
        format!(
            "【失眠调理】\n{}",
            [
                format!(
                    "作息规律：\\n{}",
                    self.routine()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "睡前准备：\\n{}",
                    self.bedtime()
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
    fn test_insomniacarerules_basic() {
        let rules = InsomniaCareRules::new();
        assert_eq!(rules.metadata().name, "失眠调理");
        assert!(!rules.routine().is_empty());
        assert!(!rules.bedtime().is_empty());
        assert!(!rules.mental().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_insomniacarerules_validation() {
        let rules = InsomniaCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("insomnia_care"));
    }

    #[test]
    fn test_insomniacarerules_explain() {
        let rules = InsomniaCareRules::new();
        let e = rules.explain();
        assert!(e.contains("作息规律"));
        assert!(e.contains("睡前准备"));
        assert!(e.contains("心理调节"));
    }
}
