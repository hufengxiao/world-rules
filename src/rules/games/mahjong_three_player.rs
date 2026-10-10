//! 三人麻将
//!
//! 三人成局的麻将玩法，缺一门并调整牌墙与番型

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MahjongThreePlayerRules,
    name: "三人麻将",
    desc: "三人成局的麻将玩法，缺一门并调整牌墙与番型",
    origin: "中国",
    tags: ["麻将", "变体", "三人"]
}

impl MahjongThreePlayerRules {
    /// 牌张调整
    pub fn tiles(&self) -> Vec<&'static str> {
        vec![
            "移除万筒索其中一门",
            "常用缺一门凑三人牌",
            "保留字牌与风牌",
            "牌墙相应缩短",
        ]
    }

    /// 发牌流程
    pub fn dealing(&self) -> Vec<&'static str> {
        vec![
            "庄家掷骰定开牌位",
            "每人十三张牌",
            "轮庄依逆时针方向",
            "留底牌作杠花用",
        ]
    }

    /// 胡牌规则
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "四人牌型规则同样适用",
            "三缺一亦可自摸点炮",
            "番型按约定计算",
            "一炮多响须预先商定",
        ]
    }

    /// 计分调整
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "三家付分制结算",
            "自摸三家共付",
            "点炮由放炮者付",
            "番数翻倍另计",
        ]
    }
}

impl Rule for MahjongThreePlayerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_three_player")
    }

    fn explain(&self) -> String {
        format!(
            "【三人麻将】\n{}",
            [
                format!(
                    "牌张调整：\\n{}",
                    self.tiles()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发牌流程：\\n{}",
                    self.dealing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胡牌规则：\\n{}",
                    self.winning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分调整：\\n{}",
                    self.scoring()
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
    fn test_mahjongthreeplayerrules_basic() {
        let rules = MahjongThreePlayerRules::new();
        assert_eq!(rules.metadata().name, "三人麻将");
        assert!(!rules.tiles().is_empty());
        assert!(!rules.dealing().is_empty());
        assert!(!rules.winning().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_mahjongthreeplayerrules_validation() {
        let rules = MahjongThreePlayerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::games("mahjong_three_player")
        );
    }

    #[test]
    fn test_mahjongthreeplayerrules_explain() {
        let rules = MahjongThreePlayerRules::new();
        let e = rules.explain();
        assert!(e.contains("牌张调整"));
        assert!(e.contains("发牌流程"));
        assert!(e.contains("胡牌规则"));
    }
}
