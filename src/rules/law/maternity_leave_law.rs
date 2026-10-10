//! 产假法律
//!
//! 女职工产假与生育待遇的法律规定

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MaternityLeaveLawRules,
    name: "产假法律",
    desc: "女职工产假与生育待遇的法律规定",
    origin: "中国",
    tags: ["法律", "劳动", "女性"]
}

impl MaternityLeaveLawRules {
    /// 产假天数
    pub fn duration(&self) -> Vec<&'static str> {
        vec![
            "法定基础产假九十八天",
            "各地另有延长假期",
            "难产可增加产假",
            "多胞胎每多一婴增加假期",
        ]
    }

    /// 生育待遇
    pub fn benefits(&self) -> Vec<&'static str> {
        vec![
            "产假期间享生育津贴",
            "由生育保险基金支付",
            "未参保由单位支付",
            "产检时间计入劳动",
        ]
    }

    /// 劳动保护
    pub fn protection(&self) -> Vec<&'static str> {
        vec![
            "孕期不得随意解雇",
            "不得安排禁忌劳动",
            "哺乳期享有哺乳时间",
            "三期女职工特殊保护",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "权益受损可协商",
            "向劳动监察投诉",
            "申请劳动仲裁",
            "保留休假与工资凭证",
        ]
    }
}

impl Rule for MaternityLeaveLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("maternity_leave_law")
    }

    fn explain(&self) -> String {
        format!(
            "【产假法律】\n{}",
            [
                format!(
                    "产假天数：\\n{}",
                    self.duration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生育待遇：\\n{}",
                    self.benefits()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "劳动保护：\\n{}",
                    self.protection()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权途径：\\n{}",
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
    fn test_maternityleavelawrules_basic() {
        let rules = MaternityLeaveLawRules::new();
        assert_eq!(rules.metadata().name, "产假法律");
        assert!(!rules.duration().is_empty());
        assert!(!rules.benefits().is_empty());
        assert!(!rules.protection().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_maternityleavelawrules_validation() {
        let rules = MaternityLeaveLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("maternity_leave_law"));
    }

    #[test]
    fn test_maternityleavelawrules_explain() {
        let rules = MaternityLeaveLawRules::new();
        let e = rules.explain();
        assert!(e.contains("产假天数"));
        assert!(e.contains("生育待遇"));
        assert!(e.contains("劳动保护"));
    }
}
