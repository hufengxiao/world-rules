//! 羽毛球发球
//!
//! 羽毛球发球握拍、到位与单打规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BadmintonServeSinglesRules,
    name: "羽毛球发球",
    desc: "羽毛球发球握拍、到位与单打规则",
    origin: "国际",
    tags: ["体育", "羽毛球", "发球"]
}

impl BadmintonServeSinglesRules {
    /// 正确发球
    pub fn serve(&self) -> Vec<&'static str> {
        vec![
            "球低于腰部击打",
            "手在肩下拍球",
            "先落地己方后越网",
            "不得过腰发球",
        ]
    }

    /// 发球落点
    pub fn placement(&self) -> Vec<&'static str> {
        vec!["单打发到发球区", "不得越线", "发短球或后场", "变化落点"]
    }

    /// 计分轮换
    pub fn scoring(&self) -> Vec<&'static str> {
        vec!["得分方发球", "单打发双方轮流", "双数右单数左", "得分继续"]
    }

    /// 连击注意
    pub fn fault(&self) -> Vec<&'static str> {
        vec!["二次击球犯规", "触网为失误", "过腰发球犯规", "遵守规范"]
    }
}

impl Rule for BadmintonServeSinglesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("badminton_serve")
    }

    fn explain(&self) -> String {
        format!(
            "【羽毛球发球】\n{}",
            [
                format!(
                    "正确发球：\\n{}",
                    self.serve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发球落点：\\n{}",
                    self.placement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分轮换：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "连击注意：\\n{}",
                    self.fault()
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
    fn test_badmintonservesinglesrules_basic() {
        let rules = BadmintonServeSinglesRules::new();
        assert_eq!(rules.metadata().name, "羽毛球发球");
        assert!(!rules.serve().is_empty());
        assert!(!rules.placement().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.fault().is_empty());
    }

    #[test]
    fn test_badmintonservesinglesrules_validation() {
        let rules = BadmintonServeSinglesRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("badminton_serve"));
    }

    #[test]
    fn test_badmintonservesinglesrules_explain() {
        let rules = BadmintonServeSinglesRules::new();
        let e = rules.explain();
        assert!(e.contains("正确发球"));
        assert!(e.contains("发球落点"));
        assert!(e.contains("计分轮换"));
    }
}
