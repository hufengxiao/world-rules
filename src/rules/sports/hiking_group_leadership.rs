//! 徒步团带队协作
//!
//! 户外徒步队伍中领队与队员的意识与协作规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HikingGroupLeadershipRules,
    name: "徒步团带队协作",
    desc: "户外徒步队伍中领队与队员的意识与协作规则",
    origin: "国际",
    tags: ["体育", "徒步", "领队", "团队", "户外"]
}

impl HikingGroupLeadershipRules {
    /// 线路规划
    pub fn route(&self) -> Vec<&'static str> {
        vec![
            "勘察路线与难易程度",
            "评估队伍整体体能",
            "告知行程与注意事项",
            "掌握基本信息与应急",
        ]
    }

    /// 队伍组织
    pub fn organization(&self) -> Vec<&'static str> {
        vec![
            "安排经验者前后照应",
            "保持队形与沟通",
            "定期清点人数",
            "速度以最慢者为准",
        ]
    }

    /// 领队责任
    pub fn responsibility(&self) -> Vec<&'static str> {
        vec![
            "及时调整节奏",
            "关注队员状态",
            "遇险果断处置",
            "确保安全第一",
        ]
    }

    /// 队员配合
    pub fn members(&self) -> Vec<&'static str> {
        vec![
            "听从领队安排",
            "不擅自离队",
            "如实报告自身状况",
            "互相扶持照应",
        ]
    }
}

impl Rule for HikingGroupLeadershipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hiking_leadership")
    }

    fn explain(&self) -> String {
        format!(
            "【徒步团带队协作】\n{}",
            [
                format!(
                    "线路规划：\\n{}",
                    self.route()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "队伍组织：\\n{}",
                    self.organization()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "领队责任：\\n{}",
                    self.responsibility()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "队员配合：\\n{}",
                    self.members()
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
    fn test_hikinggroupleadershiprules_basic() {
        let rules = HikingGroupLeadershipRules::new();
        assert_eq!(rules.metadata().name, "徒步团带队协作");
        assert!(!rules.route().is_empty());
        assert!(!rules.organization().is_empty());
        assert!(!rules.responsibility().is_empty());
        assert!(!rules.members().is_empty());
    }

    #[test]
    fn test_hikinggroupleadershiprules_validation() {
        let rules = HikingGroupLeadershipRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("hiking_leadership"));
    }

    #[test]
    fn test_hikinggroupleadershiprules_explain() {
        let rules = HikingGroupLeadershipRules::new();
        let e = rules.explain();
        assert!(e.contains("线路规划"));
        assert!(e.contains("队伍组织"));
        assert!(e.contains("领队责任"));
    }
}
