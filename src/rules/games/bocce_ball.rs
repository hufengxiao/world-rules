//! 地掷球
//!
//! 意大利传统滚球游戏，靠近目标球者得分

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BocceBallRules,
    name: "地掷球",
    desc: "意大利传统滚球游戏，靠近目标球者得分",
    origin: "意大利",
    tags: ["户外", "滚球", "传统"]
}

impl BocceBallRules {
    /// 场地准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "平整场地划出球道",
            "备一个大目标球与八球",
            "两队各持四球",
            "约定投掷线与边界",
        ]
    }

    /// 投掷规则
    pub fn throwing(&self) -> Vec<&'static str> {
        vec![
            "先抛目标球入场地",
            "轮流投掷己方球",
            "可用滚或抛方式",
            "尽量靠近目标球",
        ]
    }

    /// 计分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "距目标球最近一方得分",
            "每有一球近于对方即计一分",
            "得分方下轮先投",
            "对方球可被击开",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "先达十二分者胜",
            "可分单人双人团体赛",
            "轮流交替投掷",
            "策略兼顾进攻防守",
        ]
    }
}

impl Rule for BocceBallRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("bocce_ball")
    }

    fn explain(&self) -> String {
        format!(
            "【地掷球】\n{}",
            [
                format!(
                    "场地准备：\\n{}",
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
    fn test_bocceballrules_basic() {
        let rules = BocceBallRules::new();
        assert_eq!(rules.metadata().name, "地掷球");
        assert!(!rules.setup().is_empty());
        assert!(!rules.throwing().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_bocceballrules_validation() {
        let rules = BocceBallRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("bocce_ball"));
    }

    #[test]
    fn test_bocceballrules_explain() {
        let rules = BocceBallRules::new();
        let e = rules.explain();
        assert!(e.contains("场地准备"));
        assert!(e.contains("投掷规则"));
        assert!(e.contains("计分方式"));
    }
}
