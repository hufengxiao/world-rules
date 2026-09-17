//! 记忆力与健脑
//!
//! 健脑活动、记忆技巧与大脑保健方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MemoryBrainTrainingRules,
    name: "记忆力与健脑",
    desc: "健脑活动、记忆技巧与大脑保健方法",
    origin: "心理学",
    tags: ["健康", "记忆", "健脑", "认知"]
}

impl MemoryBrainTrainingRules {
    /// 记忆技巧
    pub fn method(&self) -> Vec<&'static str> {
        vec!["联想助记", "分段记忆", "重复巩固", "理解后再记"]
    }

    /// 健脑活动
    pub fn brain(&self) -> Vec<&'static str> {
        vec!["读书学习新知", "玩益智游戏", "练习新技能", "社交健脑"]
    }

    /// 作息护脑
    pub fn rest(&self) -> Vec<&'static str> {
        vec![
            "充足睡眠助记忆巩固",
            "规律作息",
            "适度运动增血流",
            "管理压力",
        ]
    }

    /// 注意信号
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "健忘明显影响生活就医",
            "遗忘进度骤变检查",
            "不轻视认知变化",
            "定期脑力关怀",
        ]
    }
}

impl Rule for MemoryBrainTrainingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("memory")
    }

    fn explain(&self) -> String {
        format!(
            "【记忆力与健脑】\n{}",
            [
                format!(
                    "记忆技巧：\\n{}",
                    self.method()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健脑活动：\\n{}",
                    self.brain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "作息护脑：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "注意信号：\\n{}",
                    self.care()
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
    fn test_memorybraintrainingrules_basic() {
        let rules = MemoryBrainTrainingRules::new();
        assert_eq!(rules.metadata().name, "记忆力与健脑");
        assert!(!rules.method().is_empty());
        assert!(!rules.brain().is_empty());
        assert!(!rules.rest().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_memorybraintrainingrules_validation() {
        let rules = MemoryBrainTrainingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("memory"));
    }

    #[test]
    fn test_memorybraintrainingrules_explain() {
        let rules = MemoryBrainTrainingRules::new();
        let e = rules.explain();
        assert!(e.contains("记忆技巧"));
        assert!(e.contains("健脑活动"));
        assert!(e.contains("作息护脑"));
    }
}
