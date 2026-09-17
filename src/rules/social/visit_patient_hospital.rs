//! 探访病人
//!
//! 探访病人的时机、交谈与礼貌分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VisitPatientHospitalRules,
    name: "探访病人",
    desc: "探访病人的时机、交谈与礼貌分寸",
    origin: "中国",
    tags: ["社交", "探病", "探望", "病房"]
}

impl VisitPatientHospitalRules {
    /// 择时前往
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "先联系家人问时间",
            "避开治疗休息",
            "时长不宜太长",
            "身体不适缓去",
        ]
    }

    /// 询问体贴
    pub fn words(&self) -> Vec<&'static str> {
        vec![
            "说宽慰的话",
            "不问病况隐私",
            "不多谈病情担忧",
            "多陪伴少谈忧",
        ]
    }

    /// 带礼合适
    pub fn gift(&self) -> Vec<&'static str> {
        vec!["带水果或鲜花", "先问忌口如实", "食物携带合适", "心意为主"]
    }

    /// 顾及休养
    pub fn respect(&self) -> Vec<&'static str> {
        vec!["轻声细语", "坐一会即辞", "不打扰休息", "祝福康复"]
    }
}

impl Rule for VisitPatientHospitalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("visit_patient")
    }

    fn explain(&self) -> String {
        format!(
            "【探访病人】\n{}",
            [
                format!(
                    "择时前往：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "询问体贴：\\n{}",
                    self.words()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "带礼合适：\\n{}",
                    self.gift()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "顾及休养：\\n{}",
                    self.respect()
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
    fn test_visitpatienthospitalrules_basic() {
        let rules = VisitPatientHospitalRules::new();
        assert_eq!(rules.metadata().name, "探访病人");
        assert!(!rules.timing().is_empty());
        assert!(!rules.words().is_empty());
        assert!(!rules.gift().is_empty());
        assert!(!rules.respect().is_empty());
    }

    #[test]
    fn test_visitpatienthospitalrules_validation() {
        let rules = VisitPatientHospitalRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("visit_patient"));
    }

    #[test]
    fn test_visitpatienthospitalrules_explain() {
        let rules = VisitPatientHospitalRules::new();
        let e = rules.explain();
        assert!(e.contains("择时前往"));
        assert!(e.contains("询问体贴"));
        assert!(e.contains("带礼合适"));
    }
}
