//! 开会发言礼貌
//!
//! 会议发言、倾听与表达的职场礼貌

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MeetingSpeechPolitenessRules,
    name: "开会发言礼貌",
    desc: "会议发言、倾听与表达的职场礼貌",
    origin: "国际",
    tags: ["职场", "会议", "发言", "礼貌"]
}

impl MeetingSpeechPolitenessRules {
    /// 会前准备
    pub fn prepare(&self) -> Vec<&'static str> {
        vec!["准时到会", "备好材料", "手机静音", "不迟到不闲聊"]
    }

    /// 表达观点
    pub fn speak(&self) -> Vec<&'static str> {
        vec![
            "简明扼要说要点",
            "按议程发言",
            "等他人说完再讲",
            "不跑题不抢话",
        ]
    }

    /// 倾听尊重
    pub fn listen(&self) -> Vec<&'static str> {
        vec!["认真听他人意见", "不打断别人", "记下要点", "尊重不同观点"]
    }

    /// 会后执行
    pub fn follow(&self) -> Vec<&'static str> {
        vec!["明确分工落实", "及时反馈", "不拖延", "让会议有成效"]
    }
}

impl Rule for MeetingSpeechPolitenessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("meeting_politeness")
    }

    fn explain(&self) -> String {
        format!(
            "【开会发言礼貌】\n{}",
            [
                format!(
                    "会前准备：\\n{}",
                    self.prepare()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表达观点：\\n{}",
                    self.speak()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "倾听尊重：\\n{}",
                    self.listen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "会后执行：\\n{}",
                    self.follow()
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
    fn test_meetingspeechpolitenessrules_basic() {
        let rules = MeetingSpeechPolitenessRules::new();
        assert_eq!(rules.metadata().name, "开会发言礼貌");
        assert!(!rules.prepare().is_empty());
        assert!(!rules.speak().is_empty());
        assert!(!rules.listen().is_empty());
        assert!(!rules.follow().is_empty());
    }

    #[test]
    fn test_meetingspeechpolitenessrules_validation() {
        let rules = MeetingSpeechPolitenessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("meeting_politeness"));
    }

    #[test]
    fn test_meetingspeechpolitenessrules_explain() {
        let rules = MeetingSpeechPolitenessRules::new();
        let e = rules.explain();
        assert!(e.contains("会前准备"));
        assert!(e.contains("表达观点"));
        assert!(e.contains("倾听尊重"));
    }
}
