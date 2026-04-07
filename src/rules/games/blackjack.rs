//! 21点规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};
use super::card_games::cards::{Card, Rank};

/// 21点规则
pub struct BlackjackRules {
    metadata: RuleMetadata,
}

impl BlackjackRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "21点规则",
                "Blackjack 标准规则"
            )
            .with_origin("美国")
            .with_tags(vec!["游戏".into(), "扑克".into(), "21点".into()]),
        }
    }

    /// 目标点数
    pub fn target_value(&self) -> u8 {
        21
    }

    /// 计算手牌点数
    pub fn calculate_hand_value(cards: &[Card]) -> u8 {
        let mut value = 0;
        let mut ace_count = 0;

        for card in cards {
            match card.rank {
                Rank::Ace => {
                    ace_count += 1;
                    value += 11; // A默认算11
                }
                Rank::Jack | Rank::Queen | Rank::King => {
                    value += 10;
                }
                _ => {
                    value += card.rank.value() as u8;
                }
            }
        }

        // 如果爆牌，A算1
        while value > 21 && ace_count > 0 {
            value -= 10;
            ace_count -= 1;
        }

        value
    }

    /// 判断是否爆牌
    pub fn is_bust(cards: &[Card]) -> bool {
        Self::calculate_hand_value(cards) > 21
    }

    /// 判断是否Blackjack (21点)
    pub fn is_blackjack(cards: &[Card]) -> bool {
        cards.len() == 2 && Self::calculate_hand_value(cards) == 21
    }

    /// 牌值说明
    pub fn card_values(&self) -> Vec<&'static str> {
        vec![
            "A: 可算1或11点",
            "2-10: 按牌面点数",
            "J、Q、K: 算10点",
        ]
    }

    /// 玩家操作
    pub fn player_actions(&self) -> Vec<&'static str> {
        vec![
            "Hit (拿牌): 再拿一张牌",
            "Stand (停牌): 不再拿牌",
            "Double Down (加倍): 加倍赌注，只能再拿一张牌",
            "Split (分牌): 两张相同牌可分成两手",
            "Surrender (投降): 输一半赌注",
        ]
    }

    /// 庄家规则
    pub fn dealer_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家必须在17点或以上停牌",
            "庄家必须在16点或以下继续拿牌",
            "庄家不能分牌或加倍",
            "庄家17点的A算11 (软17)",
        ]
    }

    /// 赔率说明
    pub fn payout_rules(&self) -> Vec<&'static str> {
        vec![
            "Blackjack: 1.5倍赌注 (3:2)",
            "普通赢: 1倍赌注",
            "和局: 退还赌注",
            "爆牌输: 输掉赌注",
        ]
    }
}

impl Default for BlackjackRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BlackjackRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("blackjack")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【21点规则】\n\n\
            目标: 手牌点数尽可能接近{}点但不爆牌\n\n\
            牌值计算:\n{}\n\n\
            玩家操作:\n{}\n\n\
            庄家规则:\n{}\n\n\
            赔率:\n{}\n",
            self.target_value(),
            self.card_values().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.player_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.dealer_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.payout_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::games::card_games::cards::Suit;

    #[test]
    fn test_hand_value() {
        let cards = vec![
            Card::new(Suit::Spade, Rank::King),
            Card::new(Suit::Heart, Rank::Ace),
        ];
        assert_eq!(BlackjackRules::calculate_hand_value(&cards), 21);
        assert!(BlackjackRules::is_blackjack(&cards));
    }

    #[test]
    fn test_bust() {
        let cards = vec![
            Card::new(Suit::Spade, Rank::King),
            Card::new(Suit::Heart, Rank::Queen),
            Card::new(Suit::Diamond, Rank::Two),
        ];
        assert!(BlackjackRules::is_bust(&cards));
    }
}