//! 库布木块
//!
//! 瑞典传统投掷木块游戏，击倒王块者胜

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KubbGameRules,
    name: "库布木块",
    desc: "瑞典传统投掷木块游戏，击倒王块者胜",
    origin: "瑞典",
    tags: ["户外", "投掷", "传统"]
}

impl KubbGameRules {
    /// 场地布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "两队分列场地两端",
            "中间摆放五块木块",
            "两端各立王块",
            "备六根投掷木棒",
        ]
    }

    /// 投掷规则
    pub fn throwing(&self) -> Vec<&'static str> {
        vec![
            "用木棒投击对方木块",
            "须从己方底线投掷",
            "击倒木块可夺取",
            "投掷方式不限",
        ]
    }

    /// 进阶玩法
    pub fn advance(&self) -> Vec<&'static str> {
        vec![
            "先击倒对方全部木块",
            "再将木块投入对方半场",
            "最后击倒王块取胜",
            "投掷立起木块亦可",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "击倒王块即获胜利",
            "王块未清场前倒则败",
            "须先击倒所有场块",
            "再攻王块方为有效",
        ]
    }
}

impl Rule for KubbGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("kubb_game")
    }

    fn explain(&self) -> String {
        format!(
            "【库布木块】\n{}",
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
                    "进阶玩法：\\n{}",
                    self.advance()
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
    fn test_kubbgamerules_basic() {
        let rules = KubbGameRules::new();
        assert_eq!(rules.metadata().name, "库布木块");
        assert!(!rules.setup().is_empty());
        assert!(!rules.throwing().is_empty());
        assert!(!rules.advance().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_kubbgamerules_validation() {
        let rules = KubbGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("kubb_game"));
    }

    #[test]
    fn test_kubbgamerules_explain() {
        let rules = KubbGameRules::new();
        let e = rules.explain();
        assert!(e.contains("场地布置"));
        assert!(e.contains("投掷规则"));
        assert!(e.contains("进阶玩法"));
    }
}
