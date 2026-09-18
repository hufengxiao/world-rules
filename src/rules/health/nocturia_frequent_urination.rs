//! 夜尿频繁护理
//!
//! 夜间排尿次数增多的原因与生活习惯调整

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NocturiaFrequentUrinationRules,
    name: "夜尿频繁护理",
    desc: "夜间排尿次数增多的原因与生活习惯调整",
    origin: "中国",
    tags: ["健康", "夜尿", "泌尿", "睡眠"]
}

impl NocturiaFrequentUrinationRules {
    /// 原因辨析
    pub fn cause(&self) -> Vec<&'static str> {
        vec!["睡前饮水过多", "咖啡茶利尿", "药物影响", "前列腺或疾病因素"]
    }

    /// 饮水调整
    pub fn fluid(&self) -> Vec<&'static str> {
        vec!["睡前减少饮水", "不饮浓茶咖啡", "白天合理补水", "晚餐少咸食"]
    }

    /// 睡前习惯
    pub fn routine(&self) -> Vec<&'static str> {
        vec!["睡前去一次厕所", "避免憋尿入睡", "睡前放松", "保持规律作息"]
    }

    /// 就医注意
    pub fn seek_help(&self) -> Vec<&'static str> {
        vec!["持续多尿消瘦", "饮多尿多口渴", "尿痛血尿", "及时就医检查"]
    }
}

impl Rule for NocturiaFrequentUrinationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nocturia")
    }

    fn explain(&self) -> String {
        format!(
            "【夜尿频繁护理】\n{}",
            [
                format!(
                    "原因辨析：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮水调整：\\n{}",
                    self.fluid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "睡前习惯：\\n{}",
                    self.routine()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医注意：\\n{}",
                    self.seek_help()
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
    fn test_nocturiafrequenturinationrules_basic() {
        let rules = NocturiaFrequentUrinationRules::new();
        assert_eq!(rules.metadata().name, "夜尿频繁护理");
        assert!(!rules.cause().is_empty());
        assert!(!rules.fluid().is_empty());
        assert!(!rules.routine().is_empty());
        assert!(!rules.seek_help().is_empty());
    }

    #[test]
    fn test_nocturiafrequenturinationrules_validation() {
        let rules = NocturiaFrequentUrinationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("nocturia"));
    }

    #[test]
    fn test_nocturiafrequenturinationrules_explain() {
        let rules = NocturiaFrequentUrinationRules::new();
        let e = rules.explain();
        assert!(e.contains("原因辨析"));
        assert!(e.contains("饮水调整"));
        assert!(e.contains("睡前习惯"));
    }
}
