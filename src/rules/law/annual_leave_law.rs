//! 带薪年假法律
//!
//! 职工带薪年休假的天数与安排规定

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AnnualLeaveLawRules,
    name: "带薪年假法律",
    desc: "职工带薪年休假的天数与安排规定",
    origin: "中国",
    tags: ["法律", "劳动", "休假"]
}

impl AnnualLeaveLawRules {
    /// 年假天数
    pub fn days(&self) -> Vec<&'static str> {
        vec![
            "工作满一年不满十年五天",
            "满十年不满二十年十天",
            "满二十年十五天",
            "依累计工龄计算",
        ]
    }

    /// 安排方式
    pub fn arrangement(&self) -> Vec<&'static str> {
        vec![
            "单位统筹安排年假",
            "考虑职工本人意愿",
            "可分段安排",
            "一般不跨年安排",
        ]
    }

    /// 未休补偿
    pub fn compensation(&self) -> Vec<&'static str> {
        vec![
            "应休未休按日工资三倍支付",
            "含正常工作期间工资",
            "因本人原因不休除外",
            "须书面确认放弃",
        ]
    }

    /// 不享受情形
    pub fn exclusions(&self) -> Vec<&'static str> {
        vec![
            "请病假超规定天数不享",
            "寒暑假天数多于年假不享",
            "事假累计过多不享",
            "依累计工作年限判断",
        ]
    }
}

impl Rule for AnnualLeaveLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("annual_leave_law")
    }

    fn explain(&self) -> String {
        format!(
            "【带薪年假法律】\n{}",
            [
                format!(
                    "年假天数：\\n{}",
                    self.days()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安排方式：\\n{}",
                    self.arrangement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "未休补偿：\\n{}",
                    self.compensation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "不享受情形：\\n{}",
                    self.exclusions()
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
    fn test_annualleavelawrules_basic() {
        let rules = AnnualLeaveLawRules::new();
        assert_eq!(rules.metadata().name, "带薪年假法律");
        assert!(!rules.days().is_empty());
        assert!(!rules.arrangement().is_empty());
        assert!(!rules.compensation().is_empty());
        assert!(!rules.exclusions().is_empty());
    }

    #[test]
    fn test_annualleavelawrules_validation() {
        let rules = AnnualLeaveLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("annual_leave_law"));
    }

    #[test]
    fn test_annualleavelawrules_explain() {
        let rules = AnnualLeaveLawRules::new();
        let e = rules.explain();
        assert!(e.contains("年假天数"));
        assert!(e.contains("安排方式"));
        assert!(e.contains("未休补偿"));
    }
}
