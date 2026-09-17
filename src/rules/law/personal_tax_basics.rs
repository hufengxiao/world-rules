//! 个人所得税基础
//!
//! 应税所得、专项扣除与主动纳税的常识要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PersonalTaxBasicsRules,
    name: "个人所得税基础",
    desc: "应税所得、专项扣除与主动纳税的常识要点",
    origin: "中国",
    tags: ["法律", "个税", "税务", "纳税"]
}

impl PersonalTaxBasicsRules {
    /// 纳税义务
    pub fn obligation(&self) -> Vec<&'static str> {
        vec![
            "了解应税所得范围",
            "工资劳务经营所得多属应税",
            "收入达到标准需申报",
            "依法如实申报",
        ]
    }

    /// 扣除项
    pub fn deduction(&self) -> Vec<&'static str> {
        vec![
            "熟悉专项附加扣除",
            "如实申报符合的扣除",
            "保存相关凭证",
            "规范享受政策",
        ]
    }

    /// 合规申报
    pub fn filing(&self) -> Vec<&'static str> {
        vec![
            "按期限完成申报",
            "保留收入与凭证",
            "正确适用税率",
            "不虚报瞒报收入",
        ]
    }

    /// 异议维权
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "对申报有疑主动核对",
            "遇错误申请更正",
            "不明规定咨询税务",
            "诚信纳税受保护",
        ]
    }
}

impl Rule for PersonalTaxBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("personal_tax")
    }

    fn explain(&self) -> String {
        format!(
            "【个人所得税基础】\n{}",
            [
                format!(
                    "纳税义务：\\n{}",
                    self.obligation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "扣除项：\\n{}",
                    self.deduction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合规申报：\\n{}",
                    self.filing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "异议维权：\\n{}",
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
    fn test_personaltaxbasicsrules_basic() {
        let rules = PersonalTaxBasicsRules::new();
        assert_eq!(rules.metadata().name, "个人所得税基础");
        assert!(!rules.obligation().is_empty());
        assert!(!rules.deduction().is_empty());
        assert!(!rules.filing().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_personaltaxbasicsrules_validation() {
        let rules = PersonalTaxBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("personal_tax"));
    }

    #[test]
    fn test_personaltaxbasicsrules_explain() {
        let rules = PersonalTaxBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("纳税义务"));
        assert!(e.contains("扣除项"));
        assert!(e.contains("合规申报"));
    }
}
