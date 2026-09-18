//! 回应聚会邀请
//!
//! 收到邀约如何及时得体地答应或婉拒

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PartyInvitationReplyRules,
    name: "回应聚会邀请",
    desc: "收到邀约如何及时得体地答应或婉拒",
    origin: "中国",
    tags: ["社交", "邀请", "回礼", "聚会"]
}

impl PartyInvitationReplyRules {
    /// 及时回应
    pub fn prompt(&self) -> Vec<&'static str> {
        vec!["尽早答复", "不拖到最后", "让主人早安排", "回复要明确"]
    }

    /// 答应出席
    pub fn accept(&self) -> Vec<&'static str> {
        vec![
            "欣然答应",
            "问清时间地点",
            "需要带什么提前问",
            "如期而赴不缺席",
        ]
    }

    /// 婉拒不赴
    pub fn decline(&self) -> Vec<&'static str> {
        vec![
            "尽早告知抱歉",
            "说明真正原因",
            "真诚致谢邀请",
            "不提他人替代",
        ]
    }

    /// 临时变故
    pub fn change(&self) -> Vec<&'static str> {
        vec!["临时有变早通知", "诚恳道歉", "改日再聚", "不失信于人"]
    }
}

impl Rule for PartyInvitationReplyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("invitation_reply")
    }

    fn explain(&self) -> String {
        format!(
            "【回应聚会邀请】\n{}",
            [
                format!(
                    "及时回应：\\n{}",
                    self.prompt()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "答应出席：\\n{}",
                    self.accept()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "婉拒不赴：\\n{}",
                    self.decline()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "临时变故：\\n{}",
                    self.change()
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
    fn test_partyinvitationreplyrules_basic() {
        let rules = PartyInvitationReplyRules::new();
        assert_eq!(rules.metadata().name, "回应聚会邀请");
        assert!(!rules.prompt().is_empty());
        assert!(!rules.accept().is_empty());
        assert!(!rules.decline().is_empty());
        assert!(!rules.change().is_empty());
    }

    #[test]
    fn test_partyinvitationreplyrules_validation() {
        let rules = PartyInvitationReplyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("invitation_reply"));
    }

    #[test]
    fn test_partyinvitationreplyrules_explain() {
        let rules = PartyInvitationReplyRules::new();
        let e = rules.explain();
        assert!(e.contains("及时回应"));
        assert!(e.contains("答应出席"));
        assert!(e.contains("婉拒不赴"));
    }
}
