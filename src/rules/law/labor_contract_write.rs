//! 劳动合同要点
//!
//! 劳动合同的订立、内容与续签要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LaborContractWriteRules,
    name: "劳动合同要点",
    desc: "劳动合同的订立、内容与续签要点",
    origin: "中国",
    tags: ["法律", "劳动合同", "劳动"]
}

impl LaborContractWriteRules {
    /// 合同订立
    pub fn sign(&self) -> Vec<&'static str> {
        vec![
            "入职即订立合同",
            "双方主体明确",
            "书面合同留存",
            "不签空白合同",
        ]
    }

    /// 内容要素
    pub fn content(&self) -> Vec<&'static str> {
        vec!["载明岗位报酬", "明确工时休假", "列社保条款", "工作地点清楚"]
    }

    /// 期限与试用
    pub fn duration(&self) -> Vec<&'static str> {
        vec!["固定期限有约", "试用期依法定", "合同内容合法", "变更需书面"]
    }

    /// 权益落实
    pub fn enjoy(&self) -> Vec<&'static str> {
        vec![
            "保留合同证据",
            "条款违规可主张",
            "查阅社保缴纳",
            "协商调解维权",
        ]
    }
}

impl Rule for LaborContractWriteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor_contract")
    }

    fn explain(&self) -> String {
        format!(
            "【劳动合同要点】\n{}",
            [
                format!(
                    "合同订立：\\n{}",
                    self.sign()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "内容要素：\\n{}",
                    self.content()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "期限与试用：\\n{}",
                    self.duration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "权益落实：\\n{}",
                    self.enjoy()
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
    fn test_laborcontractwriterules_basic() {
        let rules = LaborContractWriteRules::new();
        assert_eq!(rules.metadata().name, "劳动合同要点");
        assert!(!rules.sign().is_empty());
        assert!(!rules.content().is_empty());
        assert!(!rules.duration().is_empty());
        assert!(!rules.enjoy().is_empty());
    }

    #[test]
    fn test_laborcontractwriterules_validation() {
        let rules = LaborContractWriteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("labor_contract"));
    }

    #[test]
    fn test_laborcontractwriterules_explain() {
        let rules = LaborContractWriteRules::new();
        let e = rules.explain();
        assert!(e.contains("合同订立"));
        assert!(e.contains("内容要素"));
        assert!(e.contains("期限与试用"));
    }
}
