//! 吹牛骰
//!
//! 隐藏骰盅互相叫点，质疑者开盅验真假的博弈骰戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LiarsDiceRules,
    name: "吹牛骰",
    desc: "隐藏骰盅互相叫点，质疑者开盅验真假的博弈骰戏",
    origin: "欧美",
    tags: ["骰子", "博弈", "心理"]
}

impl LiarsDiceRules {
    /// 开局准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "每人五枚骰子一只骰盅",
            "摇盅后私下看自己的骰",
            "一点通常为万能点",
            "先商定一点是否万能",
        ]
    }

    /// 叫点方式
    pub fn bidding(&self) -> Vec<&'static str> {
        vec![
            "叫出某点数至少有多少枚",
            "叫数须大于上家",
            "同数量须叫更大点数",
            "可加数量叫任意点数",
        ]
    }

    /// 质疑开盅
    pub fn challenge(&self) -> Vec<&'static str> {
        vec![
            "认为上家吹牛可喊开",
            "全桌亮出所有骰子",
            "统计叫点数实际枚数",
            "一点按约定计入",
        ]
    }

    /// 判罚规则
    pub fn penalty(&self) -> Vec<&'static str> {
        vec![
            "数量不足则叫者输",
            "数量达标则质疑者输",
            "输家罚去一枚骰",
            "骰子清零者出局",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "最后持有骰子者获胜",
            "亦可约定达局数决胜",
            "叫数须诚实计数",
            "严禁偷看他人骰盅",
        ]
    }
}

impl Rule for LiarsDiceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("liars_dice")
    }

    fn explain(&self) -> String {
        format!(
            "【吹牛骰】\n{}",
            [
                format!(
                    "开局准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "叫点方式：\\n{}",
                    self.bidding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "质疑开盅：\\n{}",
                    self.challenge()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "判罚规则：\\n{}",
                    self.penalty()
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
    fn test_liarsdicerules_basic() {
        let rules = LiarsDiceRules::new();
        assert_eq!(rules.metadata().name, "吹牛骰");
        assert!(!rules.setup().is_empty());
        assert!(!rules.bidding().is_empty());
        assert!(!rules.challenge().is_empty());
        assert!(!rules.penalty().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_liarsdicerules_validation() {
        let rules = LiarsDiceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("liars_dice"));
    }

    #[test]
    fn test_liarsdicerules_explain() {
        let rules = LiarsDiceRules::new();
        let e = rules.explain();
        assert!(e.contains("开局准备"));
        assert!(e.contains("叫点方式"));
        assert!(e.contains("质疑开盅"));
    }
}
