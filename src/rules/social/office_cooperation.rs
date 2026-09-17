//! 同事协作
//!
//! 同事间配合、沟通与合作的职场关系

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OfficeCooperationRules,
    name: "同事协作",
    desc: "同事间配合、沟通与合作的职场关系",
    origin: "国际",
    tags: ["职场", "协作", "合作", "同事"]
}

impl OfficeCooperationRules {
    /// 主动分担
    pub fn share(&self) -> Vec<&'static str> {
        vec!["任务分工明确", "主动协助队友", "不推诿拖延", "责任心担当"]
    }

    /// 沟通清楚
    pub fn communicate(&self) -> Vec<&'static str> {
        vec!["需求要讲明白", "及时同步进度", "问题尽早提", "换位理解"]
    }

    /// 意见协作
    pub fn opinion(&self) -> Vec<&'static str> {
        vec!["尊重不同观点", "和谐讨论", "说服还是协商", "不独断专行"]
    }

    /// 共同成长
    pub fn grow(&self) -> Vec<&'static str> {
        vec!["分享经验数据", "互相鼓励", "成果归功团队", "团结合作"]
    }
}

impl Rule for OfficeCooperationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("office_coop")
    }

    fn explain(&self) -> String {
        format!(
            "【同事协作】\n{}",
            [
                format!(
                    "主动分担：\\n{}",
                    self.share()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沟通清楚：\\n{}",
                    self.communicate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "意见协作：\\n{}",
                    self.opinion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共同成长：\\n{}",
                    self.grow()
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
    fn test_officecooperationrules_basic() {
        let rules = OfficeCooperationRules::new();
        assert_eq!(rules.metadata().name, "同事协作");
        assert!(!rules.share().is_empty());
        assert!(!rules.communicate().is_empty());
        assert!(!rules.opinion().is_empty());
        assert!(!rules.grow().is_empty());
    }

    #[test]
    fn test_officecooperationrules_validation() {
        let rules = OfficeCooperationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("office_coop"));
    }

    #[test]
    fn test_officecooperationrules_explain() {
        let rules = OfficeCooperationRules::new();
        let e = rules.explain();
        assert!(e.contains("主动分担"));
        assert!(e.contains("沟通清楚"));
        assert!(e.contains("意见协作"));
    }
}
