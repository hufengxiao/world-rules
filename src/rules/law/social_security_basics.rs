//! 社会保险基础
//!
//! 养老医保等社会保险的缴纳、权益与使用要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SocialSecurityBasicsRules,
    name: "社会保险基础",
    desc: "养老医保等社会保险的缴纳、权益与使用要点",
    origin: "中国",
    tags: ["法律", "社保", "医保", "养老"]
}

impl SocialSecurityBasicsRules {
    /// 参保义务
    pub fn enroll(&self) -> Vec<&'static str> {
        vec![
            "了解五险组成",
            "单位应当依规参保缴费",
            "个人可缴对应比例",
            "灵活就业可参相应保险",
        ]
    }

    /// 权益认识
    pub fn rights(&self) -> Vec<&'static str> {
        vec![
            "了解医保报销范围",
            "养老待遇与缴费相关",
            "失业工伤生育各有保障",
            "权益受法律保护",
        ]
    }

    /// 查询维护
    pub fn query(&self) -> Vec<&'static str> {
        vec![
            "定期查询个人账户",
            "核对缴费记录",
            "妥善保管社保卡",
            "信息变动及时办理",
        ]
    }

    /// 转移接续
    pub fn transfer(&self) -> Vec<&'static str> {
        vec![
            "跨地工作办理转移",
            "了解接续手续",
            "保留缴费证明",
            "足额缴费利长远",
        ]
    }
}

impl Rule for SocialSecurityBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("social_security")
    }

    fn explain(&self) -> String {
        format!(
            "【社会保险基础】\n{}",
            [
                format!(
                    "参保义务：\\n{}",
                    self.enroll()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "权益认识：\\n{}",
                    self.rights()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "查询维护：\\n{}",
                    self.query()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "转移接续：\\n{}",
                    self.transfer()
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
    fn test_socialsecuritybasicsrules_basic() {
        let rules = SocialSecurityBasicsRules::new();
        assert_eq!(rules.metadata().name, "社会保险基础");
        assert!(!rules.enroll().is_empty());
        assert!(!rules.rights().is_empty());
        assert!(!rules.query().is_empty());
        assert!(!rules.transfer().is_empty());
    }

    #[test]
    fn test_socialsecuritybasicsrules_validation() {
        let rules = SocialSecurityBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("social_security"));
    }

    #[test]
    fn test_socialsecuritybasicsrules_explain() {
        let rules = SocialSecurityBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("参保义务"));
        assert!(e.contains("权益认识"));
        assert!(e.contains("查询维护"));
    }
}
