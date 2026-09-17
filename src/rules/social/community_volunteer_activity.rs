//! 社区活动参与
//!
//! 参加社区活动、志愿服务的礼仪与配合

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CommunityVolunteerActivityRules,
    name: "社区活动参与",
    desc: "参加社区活动、志愿服务的礼仪与配合",
    origin: "中国",
    tags: ["社交", "社区", "志愿", "活动"]
}

impl CommunityVolunteerActivityRules {
    /// 活动参与
    pub fn participate(&self) -> Vec<&'static str> {
        vec!["关注社区通知", "报名按时参加", "带好所需物", "尊重组织者"]
    }

    /// 志愿服务
    pub fn volunteer(&self) -> Vec<&'static str> {
        vec!["热忱报名志愿", "尽责守岗", "服从安排", "乐于奉献"]
    }

    /// 配合和谐
    pub fn cooperate(&self) -> Vec<&'static str> {
        vec!["不拥挤争抢", "照看老人小孩", "保持场地整洁", "礼貌参与"]
    }

    /// 回馈社区
    pub fn reciprocity(&self) -> Vec<&'static str> {
        vec!["分享经验心得", "带动邻里", "感谢组织方", "共建温暖"]
    }
}

impl Rule for CommunityVolunteerActivityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("community")
    }

    fn explain(&self) -> String {
        format!(
            "【社区活动参与】\n{}",
            [
                format!(
                    "活动参与：\\n{}",
                    self.participate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "志愿服务：\\n{}",
                    self.volunteer()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "配合和谐：\\n{}",
                    self.cooperate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回馈社区：\\n{}",
                    self.reciprocity()
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
    fn test_communityvolunteeractivityrules_basic() {
        let rules = CommunityVolunteerActivityRules::new();
        assert_eq!(rules.metadata().name, "社区活动参与");
        assert!(!rules.participate().is_empty());
        assert!(!rules.volunteer().is_empty());
        assert!(!rules.cooperate().is_empty());
        assert!(!rules.reciprocity().is_empty());
    }

    #[test]
    fn test_communityvolunteeractivityrules_validation() {
        let rules = CommunityVolunteerActivityRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("community"));
    }

    #[test]
    fn test_communityvolunteeractivityrules_explain() {
        let rules = CommunityVolunteerActivityRules::new();
        let e = rules.explain();
        assert!(e.contains("活动参与"));
        assert!(e.contains("志愿服务"));
        assert!(e.contains("配合和谐"));
    }
}
