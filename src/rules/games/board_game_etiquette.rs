//! 桌游游戏礼仪
//!
//! 桌游的读牌、轮流与合作礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BoardGameEtiquetteRules,
    name: "桌游游戏礼仪",
    desc: "桌游的读牌、轮流与合作礼仪",
    origin: "国际",
    tags: ["游戏", "桌游", "礼仪"]
}

impl BoardGameEtiquetteRules {
    /// 读规参与
    pub fn read(&self) -> Vec<&'static str> {
        vec!["先了解规则", "不懂就问", "参与不只观望", "欢迎新手加入"]
    }

    /// 轮流操作
    pub fn turn_taking(&self) -> Vec<&'static str> {
        vec!["按顺序行动", "不抢他人步", "耐心等自己", "节奏合理通过"]
    }

    /// 输赢心态
    pub fn sportsmanship(&self) -> Vec<&'static str> {
        vec!["不因输生气", "不悔棋改步", "输赢皆游戏", "重在乐趣"]
    }

    /// 场合友好
    pub fn friendly(&self) -> Vec<&'static str> {
        vec!["语言和善", "不嘲讽新手", "照顾旁观", "开心共聚"]
    }
}

impl Rule for BoardGameEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("board_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【桌游游戏礼仪】\n{}",
            [
                format!(
                    "读规参与：\\n{}",
                    self.read()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "轮流操作：\\n{}",
                    self.turn_taking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "输赢心态：\\n{}",
                    self.sportsmanship()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场合友好：\\n{}",
                    self.friendly()
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
    fn test_boardgameetiquetterules_basic() {
        let rules = BoardGameEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "桌游游戏礼仪");
        assert!(!rules.read().is_empty());
        assert!(!rules.turn_taking().is_empty());
        assert!(!rules.sportsmanship().is_empty());
        assert!(!rules.friendly().is_empty());
    }

    #[test]
    fn test_boardgameetiquetterules_validation() {
        let rules = BoardGameEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("board_etiquette"));
    }

    #[test]
    fn test_boardgameetiquetterules_explain() {
        let rules = BoardGameEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("读规参与"));
        assert!(e.contains("轮流操作"));
        assert!(e.contains("输赢心态"));
    }
}
