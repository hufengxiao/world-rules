//! 团队冲突调解
//!
//! 同事间矛盾冲突时的居中调解与化解分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TeamConflictMediationRules,
    name: "团队冲突调解",
    desc: "同事间矛盾冲突时的居中调解与化解分寸",
    origin: "职场通用",
    tags: ["职场", "冲突", "调解"]
}

impl TeamConflictMediationRules {
    /// 了解情况
    pub fn understanding(&self) -> Vec<&'static str> {
        vec![
            "分别听取双方陈述",
            "不偏听偏信一方",
            "厘清事实与误会",
            "关注诉求而非立场",
        ]
    }

    /// 居中调解
    pub fn mediating(&self) -> Vec<&'static str> {
        vec![
            "保持中立不站队",
            "引导换位思考",
            "聚焦共同目标",
            "避免当众指责任何一方",
        ]
    }

    /// 化解方法
    pub fn resolution(&self) -> Vec<&'static str> {
        vec![
            "把问题与人分开谈",
            "寻找双方可接受的方案",
            "明确各自改进承诺",
            "必要时请上级介入",
        ]
    }

    /// 后续跟进
    pub fn followup(&self) -> Vec<&'static str> {
        vec![
            "约定观察与复盘时间",
            "私下关注双方状态",
            "不向外传播冲突细节",
            "防止旧怨再起",
        ]
    }
}

impl Rule for TeamConflictMediationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("team_conflict_mediation")
    }

    fn explain(&self) -> String {
        format!(
            "【团队冲突调解】\n{}",
            [
                format!(
                    "了解情况：\\n{}",
                    self.understanding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "居中调解：\\n{}",
                    self.mediating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "化解方法：\\n{}",
                    self.resolution()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "后续跟进：\\n{}",
                    self.followup()
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
    fn test_teamconflictmediationrules_basic() {
        let rules = TeamConflictMediationRules::new();
        assert_eq!(rules.metadata().name, "团队冲突调解");
        assert!(!rules.understanding().is_empty());
        assert!(!rules.mediating().is_empty());
        assert!(!rules.resolution().is_empty());
        assert!(!rules.followup().is_empty());
    }

    #[test]
    fn test_teamconflictmediationrules_validation() {
        let rules = TeamConflictMediationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("team_conflict_mediation")
        );
    }

    #[test]
    fn test_teamconflictmediationrules_explain() {
        let rules = TeamConflictMediationRules::new();
        let e = rules.explain();
        assert!(e.contains("了解情况"));
        assert!(e.contains("居中调解"));
        assert!(e.contains("化解方法"));
    }
}
