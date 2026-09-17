//! 行政处罚救济
//!
//! 行政处罚的异议、复议与依法救济要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AdministrativePenaltyRules,
    name: "行政处罚救济",
    desc: "行政处罚的异议、复议与依法救济要点",
    origin: "中国",
    tags: ["法律", "行政处罚", "复议"]
}

impl AdministrativePenaltyRules {
    /// 处罚了解
    pub fn understand(&self) -> Vec<&'static str> {
        vec![
            "了解处罚种类",
            "了解处罚决定书",
            "有权知悉查处依据",
            "程序依规",
        ]
    }

    /// 不服救济
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "不服可申请复议",
            "可提起行政诉讼",
            "有法定期限",
            "按程序申请",
        ]
    }

    /// 证据与主张
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "保留处罚决定书",
            "收集事实证据",
            "理性说明申辩",
            "不无端抗拒执法",
        ]
    }

    /// 合法处理
    pub fn proper(&self) -> Vec<&'static str> {
        vec![
            "依法接受合法处罚",
            "对不合法处理依法异议",
            "不暴力抗拒执法",
            "尊重程序正义",
        ]
    }
}

impl Rule for AdministrativePenaltyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("admin_penalty")
    }

    fn explain(&self) -> String {
        format!(
            "【行政处罚救济】\n{}",
            [
                format!(
                    "处罚了解：\\n{}",
                    self.understand()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "不服救济：\\n{}",
                    self.remedy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "证据与主张：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合法处理：\\n{}",
                    self.proper()
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
    fn test_administrativepenaltyrules_basic() {
        let rules = AdministrativePenaltyRules::new();
        assert_eq!(rules.metadata().name, "行政处罚救济");
        assert!(!rules.understand().is_empty());
        assert!(!rules.remedy().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.proper().is_empty());
    }

    #[test]
    fn test_administrativepenaltyrules_validation() {
        let rules = AdministrativePenaltyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("admin_penalty"));
    }

    #[test]
    fn test_administrativepenaltyrules_explain() {
        let rules = AdministrativePenaltyRules::new();
        let e = rules.explain();
        assert!(e.contains("处罚了解"));
        assert!(e.contains("不服救济"));
        assert!(e.contains("证据与主张"));
    }
}
