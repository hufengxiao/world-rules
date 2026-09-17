//! 尊重裁判与判罚
//!
//! 比赛中对裁判、判罚的尊重与正确沟通礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RefereeRespectRules,
    name: "尊重裁判与判罚",
    desc: "比赛中对裁判、判罚的尊重与正确沟通礼仪",
    origin: "国际",
    tags: ["体育", "裁判", "判罚", "尊重", "竞技"]
}

impl RefereeRespectRules {
    /// 服从判罚
    pub fn accept_ruling(&self) -> Vec<&'static str> {
        vec![
            "尊重裁判的最终判罚",
            "不围攻或威胁裁判",
            "用合规方式提出异议",
            "情绪激动先保持距离冷却",
        ]
    }

    /// 沟通方式
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "有问题规则允许时有序反映",
            "队长或代表提出异议",
            "不使用过激言语",
            "尊重换位理解裁判视角",
        ]
    }

    /// 赛后态度
    pub fn after_game(&self) -> Vec<&'static str> {
        vec![
            "与裁判友好握手致意",
            "对争议判罚赛后理性复盘",
            "不公开贴标签诽谤裁判",
            "尊重裁判是尊重比赛",
        ]
    }

    /// 执裁自律
    pub fn referee_self(&self) -> Vec<&'static str> {
        vec![
            "裁判履职保持公正",
            "如实执裁不偏袒",
            "判罚依据规则清晰说明",
            "遇冲突维持秩序及时处理",
        ]
    }
}

impl Rule for RefereeRespectRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("referee_respect")
    }

    fn explain(&self) -> String {
        format!(
            "【尊重裁判与判罚】\n{}",
            [
                format!(
                    "服从判罚：\\n{}",
                    self.accept_ruling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沟通方式：\\n{}",
                    self.communication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赛后态度：\\n{}",
                    self.after_game()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "执裁自律：\\n{}",
                    self.referee_self()
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
    fn test_refereerespectrules_basic() {
        let rules = RefereeRespectRules::new();
        assert_eq!(rules.metadata().name, "尊重裁判与判罚");
        assert!(!rules.accept_ruling().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.after_game().is_empty());
        assert!(!rules.referee_self().is_empty());
    }

    #[test]
    fn test_refereerespectrules_validation() {
        let rules = RefereeRespectRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("referee_respect"));
    }

    #[test]
    fn test_refereerespectrules_explain() {
        let rules = RefereeRespectRules::new();
        let e = rules.explain();
        assert!(e.contains("服从判罚"));
        assert!(e.contains("沟通方式"));
        assert!(e.contains("赛后态度"));
    }
}
