//! 桌上足球规则
//!
//! 桌上足球的操作、进球与轮流规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TableSoccerRules,
    name: "桌上足球规则",
    desc: "桌上足球的操作、进球与轮流规则",
    origin: "国际",
    tags: ["体育", "桌上足球", "规则"]
}

impl TableSoccerRules {
    /// 操作方式
    pub fn control(&self) -> Vec<&'static str> {
        vec![
            "转动操纵杆控球",
            "控制己方球员",
            "射门传球配合",
            "不转杆过猛",
        ]
    }

    /// 发球与死球
    pub fn restart(&self) -> Vec<&'static str> {
        vec![
            "进球后重新开球",
            "出界发球恢复",
            "死球时按规矩发",
            "公平轮流开球",
        ]
    }

    /// 记分胜负
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球入对方门得分",
            "先达规定分获胜",
            "某些计三局两胜",
            "比分公开记录",
        ]
    }

    /// 场边礼仪
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["不拍击球台", "轮流不抢杆", "尊重队手动作", "绅士比赛"]
    }
}

impl Rule for TableSoccerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("table_soccer")
    }

    fn explain(&self) -> String {
        format!(
            "【桌上足球规则】\n{}",
            [
                format!(
                    "操作方式：\\n{}",
                    self.control()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发球与死球：\\n{}",
                    self.restart()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "记分胜负：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场边礼仪：\\n{}",
                    self.manner()
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
    fn test_tablesoccerrules_basic() {
        let rules = TableSoccerRules::new();
        assert_eq!(rules.metadata().name, "桌上足球规则");
        assert!(!rules.control().is_empty());
        assert!(!rules.restart().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_tablesoccerrules_validation() {
        let rules = TableSoccerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("table_soccer"));
    }

    #[test]
    fn test_tablesoccerrules_explain() {
        let rules = TableSoccerRules::new();
        let e = rules.explain();
        assert!(e.contains("操作方式"));
        assert!(e.contains("发球与死球"));
        assert!(e.contains("记分胜负"));
    }
}
