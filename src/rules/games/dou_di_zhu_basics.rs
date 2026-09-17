//! 斗地主基本规则
//!
//! 斗地主叫地主、牌型与胜负的基本规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DouDiZhuBasicsRules,
    name: "斗地主基本规则",
    desc: "斗地主叫地主、牌型与胜负的基本规则",
    origin: "中国",
    tags: ["游戏", "斗地主", "纸牌"]
}

impl DouDiZhuBasicsRules {
    /// 发牌叫分
    pub fn deal(&self) -> Vec<&'static str> {
        vec!["三人游戏一副牌", "留三张底牌", "抢叫地主", "叫分高者当地主"]
    }

    /// 牌型
    pub fn patterns(&self) -> Vec<&'static str> {
        vec!["单张对子三张", "顺子需至少五张", "连对三顺", "飞机带翅膀"]
    }

    /// 炸弹王炸
    pub fn bomb(&self) -> Vec<&'static str> {
        vec![
            "四张同点可炸",
            "双王为王炸最大",
            "炸弹大过普通牌",
            "慎重使用炸弹",
        ]
    }

    /// 胜负出牌
    pub fn finish(&self) -> Vec<&'static str> {
        vec![
            "先出完手牌者胜",
            "地主一人对两家",
            "农民齐心协力",
            "一波清完或倍数计分",
        ]
    }
}

impl Rule for DouDiZhuBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("dou_di_zhu")
    }

    fn explain(&self) -> String {
        format!(
            "【斗地主基本规则】\n{}",
            [
                format!(
                    "发牌叫分：\\n{}",
                    self.deal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "牌型：\\n{}",
                    self.patterns()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "炸弹王炸：\\n{}",
                    self.bomb()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负出牌：\\n{}",
                    self.finish()
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
    fn test_doudizhubasicsrules_basic() {
        let rules = DouDiZhuBasicsRules::new();
        assert_eq!(rules.metadata().name, "斗地主基本规则");
        assert!(!rules.deal().is_empty());
        assert!(!rules.patterns().is_empty());
        assert!(!rules.bomb().is_empty());
        assert!(!rules.finish().is_empty());
    }

    #[test]
    fn test_doudizhubasicsrules_validation() {
        let rules = DouDiZhuBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("dou_di_zhu"));
    }

    #[test]
    fn test_doudizhubasicsrules_explain() {
        let rules = DouDiZhuBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("发牌叫分"));
        assert!(e.contains("牌型"));
        assert!(e.contains("炸弹王炸"));
    }
}
