//! 社区与会议礼仪
//!
//! 业主会、社区会议等参会的倾听与发言礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CommunityMeetingRules,
    name: "社区与会议礼仪",
    desc: "业主会、社区会议等参会的倾听与发言礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "会议", "社区", "参与"]
}

impl CommunityMeetingRules {
    /// 准时参会
    pub fn punctual(&self) -> Vec<&'static str> {
        vec![
            "准时到达会议地点",
            "了解议题提前准备",
            "登记或落座有序",
            "不因个人私事占队变为焦点",
        ]
    }

    /// 认真倾听
    pub fn listening(&self) -> Vec<&'static str> {
        vec![
            "听取他人意见不随意插话",
            "尊重主持人引导",
            "手机静音",
            "不明之处事后请教",
        ]
    }

    /// 文明发言
    pub fn speech(&self) -> Vec<&'static str> {
        vec![
            "按顺序或相机获得发言",
            "就有议题提具体意见",
            "不人身攻击或大声争吵",
            "控制时长勿独占话题",
        ]
    }

    /// 结果尊重
    pub fn outcome(&self) -> Vec<&'static str> {
        vec![
            "尊重集体与表决结果",
            "与会决议后续配合执行",
            "分歧以沟通化解",
            "会后按分工跟进",
        ]
    }
}

impl Rule for CommunityMeetingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("community_meeting")
    }

    fn explain(&self) -> String {
        format!(
            "【社区与会议礼仪】\n{}",
            [
                format!(
                    "准时参会：\\n{}",
                    self.punctual()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "认真倾听：\\n{}",
                    self.listening()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "文明发言：\\n{}",
                    self.speech()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结果尊重：\\n{}",
                    self.outcome()
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
    fn test_communitymeetingrules_basic() {
        let rules = CommunityMeetingRules::new();
        assert_eq!(rules.metadata().name, "社区与会议礼仪");
        assert!(!rules.punctual().is_empty());
        assert!(!rules.listening().is_empty());
        assert!(!rules.speech().is_empty());
        assert!(!rules.outcome().is_empty());
    }

    #[test]
    fn test_communitymeetingrules_validation() {
        let rules = CommunityMeetingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("community_meeting"));
    }

    #[test]
    fn test_communitymeetingrules_explain() {
        let rules = CommunityMeetingRules::new();
        let e = rules.explain();
        assert!(e.contains("准时参会"));
        assert!(e.contains("认真倾听"));
        assert!(e.contains("文明发言"));
    }
}
