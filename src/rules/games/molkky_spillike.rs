//! 芬兰木棋
//!
//! 投掷木棒击倒编号木柱的北欧户外游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MolkkySpillikeRules,
    name: "芬兰木棋",
    desc: "投掷木棒击倒编号木柱的北欧户外游戏",
    origin: "芬兰",
    tags: ["户外", "投掷", "北欧"]
}

impl MolkkySpillikeRules {
    /// 场地布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "十二根编号木柱",
            "编号一至十二",
            "木柱立于圆圈内",
            "备一根投掷木棒",
        ]
    }

    /// 投掷规则
    pub fn throwing(&self) -> Vec<&'static str> {
        vec![
            "从数米外投掷木棒",
            "击倒木柱得分",
            "倒柱被移出游戏",
            "立起木柱继续",
        ]
    }

    /// 计分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "按倒柱编号累计得分",
            "一次击倒多柱加总分",
            "得分超五十则归二十五",
            "须正好达五十分",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "正好击倒至五十分者胜",
            "三次未击倒即出局",
            "可多人轮流对战",
            "考验精准与策略",
        ]
    }
}

impl Rule for MolkkySpillikeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("molkky_spillike")
    }

    fn explain(&self) -> String {
        format!(
            "【芬兰木棋】\n{}",
            [
                format!(
                    "场地布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "投掷规则：\\n{}",
                    self.throwing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分方式：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_molkkyspillikerules_basic() {
        let rules = MolkkySpillikeRules::new();
        assert_eq!(rules.metadata().name, "芬兰木棋");
        assert!(!rules.setup().is_empty());
        assert!(!rules.throwing().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_molkkyspillikerules_validation() {
        let rules = MolkkySpillikeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("molkky_spillike"));
    }

    #[test]
    fn test_molkkyspillikerules_explain() {
        let rules = MolkkySpillikeRules::new();
        let e = rules.explain();
        assert!(e.contains("场地布置"));
        assert!(e.contains("投掷规则"));
        assert!(e.contains("计分方式"));
    }
}
