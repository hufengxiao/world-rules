//! 拖欠工资维权
//!
//! 用人单位拖欠工资的追讨途径与法律依据

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WageArrearsLawRules,
    name: "拖欠工资维权",
    desc: "用人单位拖欠工资的追讨途径与法律依据",
    origin: "中国",
    tags: ["法律", "劳动", "工资"]
}

impl WageArrearsLawRules {
    /// 欠薪认定
    pub fn definition(&self) -> Vec<&'static str> {
        vec![
            "未按时足额发放工资",
            "无故克扣属违法",
            "低于最低工资违法",
            "加班费不发亦属欠薪",
        ]
    }

    /// 证据收集
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "保留劳动合同",
            "保存工资条与转账记录",
            "留存考勤记录",
            "保存催讨沟通记录",
        ]
    }

    /// 追讨途径
    pub fn channels(&self) -> Vec<&'static str> {
        vec![
            "先与单位协商",
            "向劳动监察部门投诉",
            "申请劳动仲裁",
            "依法向法院起诉",
        ]
    }

    /// 法律后果
    pub fn liability(&self) -> Vec<&'static str> {
        vec![
            "责令限期支付",
            "逾期加付赔偿金",
            "恶意欠薪或涉刑事",
            "可申请财产保全",
        ]
    }
}

impl Rule for WageArrearsLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("wage_arrears_law")
    }

    fn explain(&self) -> String {
        format!(
            "【拖欠工资维权】\n{}",
            [
                format!(
                    "欠薪认定：\\n{}",
                    self.definition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "证据收集：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "追讨途径：\\n{}",
                    self.channels()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "法律后果：\\n{}",
                    self.liability()
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
    fn test_wagearrearslawrules_basic() {
        let rules = WageArrearsLawRules::new();
        assert_eq!(rules.metadata().name, "拖欠工资维权");
        assert!(!rules.definition().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.channels().is_empty());
        assert!(!rules.liability().is_empty());
    }

    #[test]
    fn test_wagearrearslawrules_validation() {
        let rules = WageArrearsLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("wage_arrears_law"));
    }

    #[test]
    fn test_wagearrearslawrules_explain() {
        let rules = WageArrearsLawRules::new();
        let e = rules.explain();
        assert!(e.contains("欠薪认定"));
        assert!(e.contains("证据收集"));
        assert!(e.contains("追讨途径"));
    }
}
