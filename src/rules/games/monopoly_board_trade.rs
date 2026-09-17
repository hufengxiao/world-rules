//! 大富翁交易
//!
//! 大富翁买地建屋、交易与破产规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MonopolyBoardTradeRules,
    name: "大富翁交易",
    desc: "大富翁买地建屋、交易与破产规则",
    origin: "国际",
    tags: ["游戏", "大富翁", "桌游", "交易"]
}

impl MonopolyBoardTradeRules {
    /// 基本玩法
    pub fn basic(&self) -> Vec<&'static str> {
        vec!["掷骰走格", "落脚地产可买", "建屋涨租", "收租积累财富"]
    }

    /// 交易谈价
    pub fn trade(&self) -> Vec<&'static str> {
        vec!["可买卖换地", "自由协商价格", "现金或地互换", "谈妥记录"]
    }

    /// 付租责任
    pub fn rent(&self) -> Vec<&'static str> {
        vec![
            "踩他人地付租",
            "欠款无力宣告破产",
            "清算财产退出",
            "遵守承诺",
        ]
    }

    /// 公平愉快
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["不赖账不作弊", "大小朋友共乐", "注重参与", "输赢看得开"]
    }
}

impl Rule for MonopolyBoardTradeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("monopoly")
    }

    fn explain(&self) -> String {
        format!(
            "【大富翁交易】\n{}",
            [
                format!(
                    "基本玩法：\\n{}",
                    self.basic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交易谈价：\\n{}",
                    self.trade()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "付租责任：\\n{}",
                    self.rent()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公平愉快：\\n{}",
                    self.fun()
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
    fn test_monopolyboardtraderules_basic() {
        let rules = MonopolyBoardTradeRules::new();
        assert_eq!(rules.metadata().name, "大富翁交易");
        assert!(!rules.basic().is_empty());
        assert!(!rules.trade().is_empty());
        assert!(!rules.rent().is_empty());
        assert!(!rules.fun().is_empty());
    }

    #[test]
    fn test_monopolyboardtraderules_validation() {
        let rules = MonopolyBoardTradeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("monopoly"));
    }

    #[test]
    fn test_monopolyboardtraderules_explain() {
        let rules = MonopolyBoardTradeRules::new();
        let e = rules.explain();
        assert!(e.contains("基本玩法"));
        assert!(e.contains("交易谈价"));
        assert!(e.contains("付租责任"));
    }
}
