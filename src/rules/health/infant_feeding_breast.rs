//! 母乳喂养与婴儿喂哺
//!
//! 母乳喂养的姿势、频率与喂养卫生

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InfantFeedingBreastRules,
    name: "母乳喂养与婴儿喂哺",
    desc: "母乳喂养的姿势、频率与喂养卫生",
    origin: "医学",
    tags: ["健康", "母乳", "婴儿", "喂养"]
}

impl InfantFeedingBreastRules {
    /// 喂奶姿势
    pub fn posture(&self) -> Vec<&'static str> {
        vec!["抱稳婴儿含乳", "含住乳晕", "自然舒适姿势", "不堵鼻不挤"]
    }

    /// 按需喂养
    pub fn feeding(&self) -> Vec<&'static str> {
        vec!["婴儿饿了即喂", "按需不强行定时", "夜奶正常", "观察饱足"]
    }

    /// 卫生护理
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec!["喂前清洁", "保持乳头干爽", "拍嗝防吐奶", "奶具消毒"]
    }

    /// 母乳与配方
    pub fn choice(&self) -> Vec<&'static str> {
        vec![
            "母乳优先最好",
            "母乳不足可添配方",
            "选合适段奶粉",
            "遵医嘱调整",
        ]
    }
}

impl Rule for InfantFeedingBreastRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("breastfeeding")
    }

    fn explain(&self) -> String {
        format!(
            "【母乳喂养与婴儿喂哺】\n{}",
            [
                format!(
                    "喂奶姿势：\\n{}",
                    self.posture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "按需喂养：\\n{}",
                    self.feeding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生护理：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "母乳与配方：\\n{}",
                    self.choice()
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
    fn test_infantfeedingbreastrules_basic() {
        let rules = InfantFeedingBreastRules::new();
        assert_eq!(rules.metadata().name, "母乳喂养与婴儿喂哺");
        assert!(!rules.posture().is_empty());
        assert!(!rules.feeding().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.choice().is_empty());
    }

    #[test]
    fn test_infantfeedingbreastrules_validation() {
        let rules = InfantFeedingBreastRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("breastfeeding"));
    }

    #[test]
    fn test_infantfeedingbreastrules_explain() {
        let rules = InfantFeedingBreastRules::new();
        let e = rules.explain();
        assert!(e.contains("喂奶姿势"));
        assert!(e.contains("按需喂养"));
        assert!(e.contains("卫生护理"));
    }
}
