//! 加班与加班费
//!
//! 加班安排、加班费计算与加班权益要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OvertimeCompensationRules,
    name: "加班与加班费",
    desc: "加班安排、加班费计算与加班权益要点",
    origin: "中国",
    tags: ["法律", "加班", "加班费", "劳动"]
}

impl OvertimeCompensationRules {
    /// 加班安排
    pub fn schedule(&self) -> Vec<&'static str> {
        vec![
            "加班有工作需要的约定",
            "了解单位加班流程",
            "不强迫超时加班",
            "保障合理休息",
        ]
    }

    /// 加班费计算
    pub fn pay(&self) -> Vec<&'static str> {
        vec![
            "平时加班有加班工资",
            "休息日加班安排补助",
            "法定节日加班有补助",
            "按时足额领取",
        ]
    }

    /// 记录证据
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "保留加班记录凭证",
            "留存加班通知",
            "记录工作时长",
            "维权时有据可依",
        ]
    }

    /// 维权主张
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "有异议先沟通",
            "可向劳动部门反映",
            "依法主张补偿",
            "举证责任依规",
        ]
    }
}

impl Rule for OvertimeCompensationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("overtime")
    }

    fn explain(&self) -> String {
        format!(
            "【加班与加班费】\n{}",
            [
                format!(
                    "加班安排：\\n{}",
                    self.schedule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "加班费计算：\\n{}",
                    self.pay()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "记录证据：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权主张：\\n{}",
                    self.remedy()
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
    fn test_overtimecompensationrules_basic() {
        let rules = OvertimeCompensationRules::new();
        assert_eq!(rules.metadata().name, "加班与加班费");
        assert!(!rules.schedule().is_empty());
        assert!(!rules.pay().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_overtimecompensationrules_validation() {
        let rules = OvertimeCompensationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("overtime"));
    }

    #[test]
    fn test_overtimecompensationrules_explain() {
        let rules = OvertimeCompensationRules::new();
        let e = rules.explain();
        assert!(e.contains("加班安排"));
        assert!(e.contains("加班费计算"));
        assert!(e.contains("记录证据"));
    }
}
