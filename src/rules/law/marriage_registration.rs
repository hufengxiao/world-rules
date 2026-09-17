//! 婚姻登记与法定婚龄
//!
//! 结婚登记、法定婚龄与婚姻关系成立要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MarriageRegistrationRules,
    name: "婚姻登记与法定婚龄",
    desc: "结婚登记、法定婚龄与婚姻关系成立要点",
    origin: "中国",
    tags: ["法律", "婚姻", "登记", "结婚"]
}

impl MarriageRegistrationRules {
    /// 法定条件
    pub fn condition(&self) -> Vec<&'static str> {
        vec![
            "达到法定结婚年龄",
            "双方自愿结婚",
            "无近亲或禁止情形",
            "具备完全民事行为能力",
        ]
    }

    /// 登记程序
    pub fn process(&self) -> Vec<&'static str> {
        vec![
            "携证件前往登记机关",
            "本人到场申请",
            "填写结婚登记申请",
            "领取结婚证",
        ]
    }

    /// 婚姻效力
    pub fn effect(&self) -> Vec<&'static str> {
        vec![
            "登记日起婚姻成立",
            "确立财产与权利义务",
            "诚实忠诚义务",
            "依法缔结受保护",
        ]
    }

    /// 特殊情况
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "无效婚姻如实了解",
            "可撤销情形依法主张",
            "虚假登记不受保护",
            "复杂情形咨询法律",
        ]
    }
}

impl Rule for MarriageRegistrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("marriage")
    }

    fn explain(&self) -> String {
        format!(
            "【婚姻登记与法定婚龄】\n{}",
            [
                format!(
                    "法定条件：\\n{}",
                    self.condition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "登记程序：\\n{}",
                    self.process()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "婚姻效力：\\n{}",
                    self.effect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊情况：\\n{}",
                    self.special()
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
    fn test_marriageregistrationrules_basic() {
        let rules = MarriageRegistrationRules::new();
        assert_eq!(rules.metadata().name, "婚姻登记与法定婚龄");
        assert!(!rules.condition().is_empty());
        assert!(!rules.process().is_empty());
        assert!(!rules.effect().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_marriageregistrationrules_validation() {
        let rules = MarriageRegistrationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("marriage"));
    }

    #[test]
    fn test_marriageregistrationrules_explain() {
        let rules = MarriageRegistrationRules::new();
        let e = rules.explain();
        assert!(e.contains("法定条件"));
        assert!(e.contains("登记程序"));
        assert!(e.contains("婚姻效力"));
    }
}
