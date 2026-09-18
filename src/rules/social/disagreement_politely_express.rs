//! 得体表达异议
//!
//! 不同意见时温和表达异议也维护彼此尊重的方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DisagreementPolitelyExpressRules,
    name: "得体表达异议",
    desc: "不同意见时温和表达异议也维护彼此尊重的方法",
    origin: "中国",
    tags: ["社交", "异议", "沟通", "尊重"]
}

impl DisagreementPolitelyExpressRules {
    /// 先理解
    pub fn listen(&self) -> Vec<&'static str> {
        vec!["听完对方说完", "肯定对方立场", "找共同点", "再谈分歧"]
    }

    /// 温和表达
    pub fn wording(&self) -> Vec<&'static str> {
        vec![
            "我的看法略有不同",
            "从另一角度想",
            "你考虑得周到",
            "是否也可考虑",
        ]
    }

    /// 就事论事
    pub fn fair(&self) -> Vec<&'static str> {
        vec!["对事不对人", "不人身攻击", "摆事实讲理据", "冷静不抬杠"]
    }

    /// 友好收场
    pub fn close(&self) -> Vec<&'static str> {
        vec!["不强行争对错", "求同存异", "尊重最后决定", "保持关系融洽"]
    }
}

impl Rule for DisagreementPolitelyExpressRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("disagreement")
    }

    fn explain(&self) -> String {
        format!(
            "【得体表达异议】\n{}",
            [
                format!(
                    "先理解：\\n{}",
                    self.listen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "温和表达：\\n{}",
                    self.wording()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就事论事：\\n{}",
                    self.fair()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "友好收场：\\n{}",
                    self.close()
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
    fn test_disagreementpolitelyexpressrules_basic() {
        let rules = DisagreementPolitelyExpressRules::new();
        assert_eq!(rules.metadata().name, "得体表达异议");
        assert!(!rules.listen().is_empty());
        assert!(!rules.wording().is_empty());
        assert!(!rules.fair().is_empty());
        assert!(!rules.close().is_empty());
    }

    #[test]
    fn test_disagreementpolitelyexpressrules_validation() {
        let rules = DisagreementPolitelyExpressRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("disagreement"));
    }

    #[test]
    fn test_disagreementpolitelyexpressrules_explain() {
        let rules = DisagreementPolitelyExpressRules::new();
        let e = rules.explain();
        assert!(e.contains("先理解"));
        assert!(e.contains("温和表达"));
        assert!(e.contains("就事论事"));
    }
}
