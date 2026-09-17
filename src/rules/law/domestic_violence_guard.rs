//! 家庭暴力防护
//!
//! 识别家庭暴力、求助保护与依法维权要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DomesticViolenceGuardRules,
    name: "家庭暴力防护",
    desc: "识别家庭暴力、求助保护与依法维权要点",
    origin: "中国",
    tags: ["法律", "家暴", "防护", "维权"]
}

impl DomesticViolenceGuardRules {
    /// 识别家暴
    pub fn identify(&self) -> Vec<&'static str> {
        vec![
            "身体暴力是伤害",
            "精神冷暴力也属暴力",
            "经济控制亦属",
            "反复伤害须重视",
        ]
    }

    /// 保护自己
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "遭遇侵害立即脱离",
            "向亲友或机构求助",
            "可申请人身保护令",
            "必要时报警",
        ]
    }

    /// 证据保留
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "保留伤情照片",
            "就医记录佐证",
            "保存相关沟通记录",
            "目击者证言",
        ]
    }

    /// 法律维权
    pub fn rights(&self) -> Vec<&'static str> {
        vec![
            "家暴应受法律制裁",
            "可依情离婚追责",
            "保护令具强制力",
            "寻求专业法律扶助",
        ]
    }
}

impl Rule for DomesticViolenceGuardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("domestic_violence")
    }

    fn explain(&self) -> String {
        format!(
            "【家庭暴力防护】\n{}",
            [
                format!(
                    "识别家暴：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保护自己：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "证据保留：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "法律维权：\\n{}",
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
    fn test_domesticviolenceguardrules_basic() {
        let rules = DomesticViolenceGuardRules::new();
        assert_eq!(rules.metadata().name, "家庭暴力防护");
        assert!(!rules.identify().is_empty());
        assert!(!rules.protect().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.rights().is_empty());
    }

    #[test]
    fn test_domesticviolenceguardrules_validation() {
        let rules = DomesticViolenceGuardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("domestic_violence"));
    }

    #[test]
    fn test_domesticviolenceguardrules_explain() {
        let rules = DomesticViolenceGuardRules::new();
        let e = rules.explain();
        assert!(e.contains("识别家暴"));
        assert!(e.contains("保护自己"));
        assert!(e.contains("证据保留"));
    }
}
