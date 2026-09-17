//! 夫妻财产关系要点
//!
//! 婚姻关系中的共同财产、债务与约定的常识要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MarriagePropertyBasicsRules,
    name: "夫妻财产关系要点",
    desc: "婚姻关系中的共同财产、债务与约定的常识要点",
    origin: "国际",
    tags: ["法律", "婚姻", "财产", "家庭", "权益"]
}

impl MarriagePropertyBasicsRules {
    /// 共同财产
    pub fn community(&self) -> Vec<&'static str> {
        vec![
            "了解法定共同财产范围",
            "婚后工资收入常属共同",
            "个人财产与共有产分开看",
            "涉重大资产咨询专业人士",
        ]
    }

    /// 财产约定
    pub fn agreement(&self) -> Vec<&'static str> {
        vec![
            "可依法约定财产归属",
            "约定须书面并真实自愿",
            "婚前财产可经约定归属",
            "保留约定的书面记录",
        ]
    }

    /// 债务承担
    pub fn debt(&self) -> Vec<&'static str> {
        vec![
            "区分共同债务与个人债务",
            "共同签字或用于家计多为共同",
            "个人债务属个人承担",
            "夫妻共同举债共同负担",
        ]
    }

    /// 权利保障
    pub fn rights(&self) -> Vec<&'static str> {
        vec![
            "尊重彼此财产知情权",
            "处分共同财产需协商",
            "遇到争议依法沟通解决",
            "涉及大额或房产要谨慎并合法合规",
        ]
    }
}

impl Rule for MarriagePropertyBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("marriage_property")
    }

    fn explain(&self) -> String {
        format!(
            "【夫妻财产关系要点】\n{}",
            [
                format!(
                    "共同财产：\\n{}",
                    self.community()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "财产约定：\\n{}",
                    self.agreement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "债务承担：\\n{}",
                    self.debt()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "权利保障：\\n{}",
                    self.rights()
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
    fn test_marriagepropertybasicsrules_basic() {
        let rules = MarriagePropertyBasicsRules::new();
        assert_eq!(rules.metadata().name, "夫妻财产关系要点");
        assert!(!rules.community().is_empty());
        assert!(!rules.agreement().is_empty());
        assert!(!rules.debt().is_empty());
        assert!(!rules.rights().is_empty());
    }

    #[test]
    fn test_marriagepropertybasicsrules_validation() {
        let rules = MarriagePropertyBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("marriage_property"));
    }

    #[test]
    fn test_marriagepropertybasicsrules_explain() {
        let rules = MarriagePropertyBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("共同财产"));
        assert!(e.contains("财产约定"));
        assert!(e.contains("债务承担"));
    }
}
