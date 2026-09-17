//! 交通事故与违章处理
//!
//! 发生交通事故或违章时的处理要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TrafficViolationHandlingRules,
    name: "交通事故与违章处理",
    desc: "发生交通事故或违章时的处理要点",
    origin: "中国",
    tags: ["法律", "交通", "事故", "违章"]
}

impl TrafficViolationHandlingRules {
    /// 事故处置
    pub fn accident(&self) -> Vec<&'static str> {
        vec![
            "发生事故先确保安全",
            "及时停车开启警示",
            "有人受伤立即求助",
            "不逃逸不破坏现场",
        ]
    }

    /// 责任认定
    pub fn responsibility(&self) -> Vec<&'static str> {
        vec![
            "配合交警勘查",
            "如实陈述经过",
            "依据证据定责",
            "有异议依法复核",
        ]
    }

    /// 违章处理
    pub fn violation(&self) -> Vec<&'static str> {
        vec![
            "查询处理违章记录",
            "依规缴纳罚款",
            "留意记分规则",
            "不找代办逃避责任",
        ]
    }

    /// 理赔协商
    pub fn claim(&self) -> Vec<&'static str> {
        vec![
            "投保理赔依保险",
            "保留事故现场凭证",
            "协商不伤人和",
            "疑难依法处理",
        ]
    }
}

impl Rule for TrafficViolationHandlingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("traffic_violation")
    }

    fn explain(&self) -> String {
        format!(
            "【交通事故与违章处理】\n{}",
            [
                format!(
                    "事故处置：\\n{}",
                    self.accident()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "责任认定：\\n{}",
                    self.responsibility()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "违章处理：\\n{}",
                    self.violation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "理赔协商：\\n{}",
                    self.claim()
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
    fn test_trafficviolationhandlingrules_basic() {
        let rules = TrafficViolationHandlingRules::new();
        assert_eq!(rules.metadata().name, "交通事故与违章处理");
        assert!(!rules.accident().is_empty());
        assert!(!rules.responsibility().is_empty());
        assert!(!rules.violation().is_empty());
        assert!(!rules.claim().is_empty());
    }

    #[test]
    fn test_trafficviolationhandlingrules_validation() {
        let rules = TrafficViolationHandlingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("traffic_violation"));
    }

    #[test]
    fn test_trafficviolationhandlingrules_explain() {
        let rules = TrafficViolationHandlingRules::new();
        let e = rules.explain();
        assert!(e.contains("事故处置"));
        assert!(e.contains("责任认定"));
        assert!(e.contains("违章处理"));
    }
}
