//! 领土牌组构筑
//!
//! 以购买卡牌构筑牌库的开创性构筑类桌游

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DominionDeckBuilderRules,
    name: "领土牌组构筑",
    desc: "以购买卡牌构筑牌库的开创性构筑类桌游",
    origin: "美国",
    tags: ["桌游", "构筑", "策略"]
}

impl DominionDeckBuilderRules {
    /// 开局布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "每人起始铜币与庄园牌",
            "公共牌堆按局摆放",
            "选定本局使用的王国卡",
            "确定起始玩家",
        ]
    }

    /// 回合阶段
    pub fn phases(&self) -> Vec<&'static str> {
        vec![
            "行动阶段出行动卡",
            "购买阶段用钱币购卡",
            "清理阶段弃牌补牌",
            "每回合抽满五张",
        ]
    }

    /// 购买卡牌
    pub fn buying(&self) -> Vec<&'static str> {
        vec![
            "以钱币购买新卡",
            "新卡入弃牌堆",
            "可购行动卡财宝卡",
            "或购胜利分卡",
        ]
    }

    /// 牌库循环
    pub fn cycling(&self) -> Vec<&'static str> {
        vec![
            "抽牌堆用尽洗弃牌堆",
            "构筑优质牌库是核心",
            "淘汰低效起始牌",
            "追求连招效率",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "省份牌堆耗尽即终局",
            "或三类牌堆同时耗尽",
            "统计胜利分卡总分",
            "分高者获胜",
        ]
    }
}

impl Rule for DominionDeckBuilderRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("dominion_deck_builder")
    }

    fn explain(&self) -> String {
        format!(
            "【领土牌组构筑】\n{}",
            [
                format!(
                    "开局布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合阶段：\\n{}",
                    self.phases()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "购买卡牌：\\n{}",
                    self.buying()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "牌库循环：\\n{}",
                    self.cycling()
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
    fn test_dominiondeckbuilderrules_basic() {
        let rules = DominionDeckBuilderRules::new();
        assert_eq!(rules.metadata().name, "领土牌组构筑");
        assert!(!rules.setup().is_empty());
        assert!(!rules.phases().is_empty());
        assert!(!rules.buying().is_empty());
        assert!(!rules.cycling().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_dominiondeckbuilderrules_validation() {
        let rules = DominionDeckBuilderRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::games("dominion_deck_builder")
        );
    }

    #[test]
    fn test_dominiondeckbuilderrules_explain() {
        let rules = DominionDeckBuilderRules::new();
        let e = rules.explain();
        assert!(e.contains("开局布置"));
        assert!(e.contains("回合阶段"));
        assert!(e.contains("购买卡牌"));
    }
}
