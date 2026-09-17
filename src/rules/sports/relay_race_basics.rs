//! 接力赛基本规则
//!
//! 田径接力棒的传递、区域与交接规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RelayRaceBasicsRules,
    name: "接力赛基本规则",
    desc: "田径接力棒的传递、区域与交接规则",
    origin: "国际",
    tags: ["体育", "接力", "田径"]
}

impl RelayRaceBasicsRules {
    /// 交接棒
    pub fn baton(&self) -> Vec<&'static str> {
        vec![
            "在接力区完成交接",
            "接棒者起跑配合",
            "传递不掉棒",
            "换手交接须熟练",
        ]
    }

    /// 区域规则
    pub fn zone(&self) -> Vec<&'static str> {
        vec![
            "接力区有明确长度",
            "交棒须在区内完成",
            "超出区域判违例",
            "越区借力不允许",
        ]
    }

    /// 团队配合
    pub fn team(&self) -> Vec<&'static str> {
        vec![
            "按速度排棒次",
            "呼应起跑时机",
            "默契减少失误",
            "全队协作完赛",
        ]
    }

    /// 竞逐礼仪
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "不在跑道妨碍他人",
            "赛后回棒交接",
            "尊重判罚",
            "拼搏友谊并重",
        ]
    }
}

impl Rule for RelayRaceBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("relay")
    }

    fn explain(&self) -> String {
        format!(
            "【接力赛基本规则】\n{}",
            [
                format!(
                    "交接棒：\\n{}",
                    self.baton()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "区域规则：\\n{}",
                    self.zone()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "团队配合：\\n{}",
                    self.team()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "竞逐礼仪：\\n{}",
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
    fn test_relayracebasicsrules_basic() {
        let rules = RelayRaceBasicsRules::new();
        assert_eq!(rules.metadata().name, "接力赛基本规则");
        assert!(!rules.baton().is_empty());
        assert!(!rules.zone().is_empty());
        assert!(!rules.team().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_relayracebasicsrules_validation() {
        let rules = RelayRaceBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("relay"));
    }

    #[test]
    fn test_relayracebasicsrules_explain() {
        let rules = RelayRaceBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("交接棒"));
        assert!(e.contains("区域规则"));
        assert!(e.contains("团队配合"));
    }
}
