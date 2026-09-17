//! 四子连珠棋
//!
//! 四子棋竖放落子、连四获胜的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ConnectFourHourRules,
    name: "四子连珠棋",
    desc: "四子棋竖放落子、连四获胜的规则",
    origin: "国际",
    tags: ["游戏", "四子棋", "棋类"]
}

impl ConnectFourHourRules {
    /// 棋盘列落
    pub fn drop(&self) -> Vec<&'static str> {
        vec!["竖立网格棋盘", "棋子从顶下落", "选列置子", "落底即停"]
    }

    /// 连四获胜
    pub fn win(&self) -> Vec<&'static str> {
        vec!["四子直线相连", "横竖斜皆算", "先连先胜", "防守对方连四"]
    }

    /// 策略思考
    pub fn strategy(&self) -> Vec<&'static str> {
        vec!["占中列优势", "威胁得分", "兼顾防守敌", "预判几步"]
    }

    /// 平局终止
    pub fn draw(&self) -> Vec<&'static str> {
        vec!["盘满无人连四则和", "重新开局", "公平轮流", "快乐对弈"]
    }
}

impl Rule for ConnectFourHourRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("connect_four")
    }

    fn explain(&self) -> String {
        format!(
            "【四子连珠棋】\n{}",
            [
                format!(
                    "棋盘列落：\\n{}",
                    self.drop()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "连四获胜：\\n{}",
                    self.win()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "策略思考：\\n{}",
                    self.strategy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "平局终止：\\n{}",
                    self.draw()
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
    fn test_connectfourhourrules_basic() {
        let rules = ConnectFourHourRules::new();
        assert_eq!(rules.metadata().name, "四子连珠棋");
        assert!(!rules.drop().is_empty());
        assert!(!rules.win().is_empty());
        assert!(!rules.strategy().is_empty());
        assert!(!rules.draw().is_empty());
    }

    #[test]
    fn test_connectfourhourrules_validation() {
        let rules = ConnectFourHourRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("connect_four"));
    }

    #[test]
    fn test_connectfourhourrules_explain() {
        let rules = ConnectFourHourRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘列落"));
        assert!(e.contains("连四获胜"));
        assert!(e.contains("策略思考"));
    }
}
