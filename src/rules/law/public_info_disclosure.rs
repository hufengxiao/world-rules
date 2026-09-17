//! 信息公开申请
//!
//! 企事业单位信息公开的申请、范围与使用要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PublicInfoDisclosureRules,
    name: "信息公开申请",
    desc: "企事业单位信息公开的申请、范围与使用要点",
    origin: "中国",
    tags: ["法律", "信息公开", "申请"]
}

impl PublicInfoDisclosureRules {
    /// 申请前提
    pub fn premise(&self) -> Vec<&'static str> {
        vec![
            "了解依法应公开范围",
            "公开有利于知情",
            "有下列正当需求申请",
            "依法提出请求",
        ]
    }

    /// 申请方式
    pub fn apply(&self) -> Vec<&'static str> {
        vec!["书面申请清楚", "写明所需信息", "留下联系方式", "按程序递交"]
    }

    /// 办理回应
    pub fn response(&self) -> Vec<&'static str> {
        vec![
            "机关应予回复",
            "依时限答复",
            "不全或不清告知补正",
            "公开与否有依据",
        ]
    }

    /// 救济与使用
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "无正当不公开可异议",
            "用于正当目的",
            "不凭信息扰",
            "依规获取信息",
        ]
    }
}

impl Rule for PublicInfoDisclosureRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("info_disclosure")
    }

    fn explain(&self) -> String {
        format!(
            "【信息公开申请】\n{}",
            [
                format!(
                    "申请前提：\\n{}",
                    self.premise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "申请方式：\\n{}",
                    self.apply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "办理回应：\\n{}",
                    self.response()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "救济与使用：\\n{}",
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
    fn test_publicinfodisclosurerules_basic() {
        let rules = PublicInfoDisclosureRules::new();
        assert_eq!(rules.metadata().name, "信息公开申请");
        assert!(!rules.premise().is_empty());
        assert!(!rules.apply().is_empty());
        assert!(!rules.response().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_publicinfodisclosurerules_validation() {
        let rules = PublicInfoDisclosureRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("info_disclosure"));
    }

    #[test]
    fn test_publicinfodisclosurerules_explain() {
        let rules = PublicInfoDisclosureRules::new();
        let e = rules.explain();
        assert!(e.contains("申请前提"));
        assert!(e.contains("申请方式"));
        assert!(e.contains("办理回应"));
    }
}
