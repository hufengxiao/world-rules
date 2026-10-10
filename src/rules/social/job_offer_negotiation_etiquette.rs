//! 谈录用与入职礼仪
//!
//! 收到录用通知后沟通与入职的礼仪规则，包括及时回应、诚恳谈条件、诚信守约与入职报到礼节，维护职业形象。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JobOfferNegotiationEtiquetteRules,
    name: "谈录用与入职礼仪",
    desc: "收到录用通知后沟通与入职的礼仪规则，包括及时回应、诚恳谈条件、诚信守约与入职报到礼节，维护职业形象。",
    origin: "职场求职与录用沟通场景",
    tags: ["社交", "职场", "求职", "礼仪"]
}

impl JobOfferNegotiationEtiquetteRules {
    /// 及时回应
    pub fn response(&self) -> Vec<&'static str> {
        vec![
            "收到通知尽快答复",
            "考虑期内主动沟通进度",
            "不拖延占用他人名额",
            "婉拒也须礼貌说明",
        ]
    }

    /// 谈条件
    pub fn negotiation(&self) -> Vec<&'static str> {
        vec![
            "在合理范围提出诉求",
            "语气诚恳态度专业",
            "不过度讨价还价",
            "达成的条件书面确认",
        ]
    }

    /// 诚信守约
    pub fn integrity(&self) -> Vec<&'static str> {
        vec![
            "答应入职不随意反悔",
            "确实变卦尽早告知",
            "不脚踏多船隐瞒实情",
            "尊重双方时间成本",
        ]
    }

    /// 入职报到
    pub fn onboarding(&self) -> Vec<&'static str> {
        vec![
            "按时报到不迟到",
            "备齐所需材料",
            "主动向同事自我介绍",
            "对接待者表示感谢",
        ]
    }
}

impl Rule for JobOfferNegotiationEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("job_offer")
    }

    fn explain(&self) -> String {
        format!(
            "【谈录用与入职礼仪】\n{}",
            [
                format!(
                    "及时回应：\\n{}",
                    self.response()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "谈条件：\\n{}",
                    self.negotiation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "诚信守约：\\n{}",
                    self.integrity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "入职报到：\\n{}",
                    self.onboarding()
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
    fn test_joboffernegotiationetiquetterules_basic() {
        let rules = JobOfferNegotiationEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "谈录用与入职礼仪");
        assert!(!rules.response().is_empty());
        assert!(!rules.negotiation().is_empty());
        assert!(!rules.integrity().is_empty());
        assert!(!rules.onboarding().is_empty());
    }

    #[test]
    fn test_joboffernegotiationetiquetterules_validation() {
        let rules = JobOfferNegotiationEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("job_offer"));
    }

    #[test]
    fn test_joboffernegotiationetiquetterules_explain() {
        let rules = JobOfferNegotiationEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("及时回应"));
        assert!(e.contains("谈条件"));
        assert!(e.contains("诚信守约"));
    }
}
