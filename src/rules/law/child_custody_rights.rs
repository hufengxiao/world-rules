//! 子女抚养权要点
//!
//! 离婚时子女抚养权归属、探视与抚养费

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChildCustodyRightsRules,
    name: "子女抚养权要点",
    desc: "离婚时子女抚养权归属、探视与抚养费",
    origin: "中国",
    tags: ["法律", "抚养权", "子女", "探视"]
}

impl ChildCustodyRightsRules {
    /// 归属原则
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "以子女利益为重",
            "考虑双方抚养能力",
            "年幼子女常随母或酌情",
            "尊重较大子女意愿",
        ]
    }

    /// 抚养费
    pub fn support_fee(&self) -> Vec<&'static str> {
        vec![
            "不抚养方付抚养费",
            "金额兼顾所需与能力",
            "可协议或依法确定",
            "可依情况变更",
        ]
    }

    /// 探视权利
    pub fn visitation(&self) -> Vec<&'static str> {
        vec![
            "不抚养方有探视权",
            "另一方应配合",
            "探视方式可约定",
            "保障亲子联系",
        ]
    }

    /// 变更与维权
    pub fn change(&self) -> Vec<&'static str> {
        vec![
            "情形变化可申请变更",
            "不配合可依法主张",
            "损害子女利益可争取",
            "以子女健康成长为先",
        ]
    }
}

impl Rule for ChildCustodyRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("custody")
    }

    fn explain(&self) -> String {
        format!(
            "【子女抚养权要点】\n{}",
            [
                format!(
                    "归属原则：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抚养费：\\n{}",
                    self.support_fee()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "探视权利：\\n{}",
                    self.visitation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "变更与维权：\\n{}",
                    self.change()
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
    fn test_childcustodyrightsrules_basic() {
        let rules = ChildCustodyRightsRules::new();
        assert_eq!(rules.metadata().name, "子女抚养权要点");
        assert!(!rules.principle().is_empty());
        assert!(!rules.support_fee().is_empty());
        assert!(!rules.visitation().is_empty());
        assert!(!rules.change().is_empty());
    }

    #[test]
    fn test_childcustodyrightsrules_validation() {
        let rules = ChildCustodyRightsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("custody"));
    }

    #[test]
    fn test_childcustodyrightsrules_explain() {
        let rules = ChildCustodyRightsRules::new();
        let e = rules.explain();
        assert!(e.contains("归属原则"));
        assert!(e.contains("抚养费"));
        assert!(e.contains("探视权利"));
    }
}
