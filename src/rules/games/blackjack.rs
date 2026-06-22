//! 21点规则

use super::card_games::cards::{Card, Rank, Suit};
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

fn parse_blackjack_cards(s: &str) -> Result<Vec<Card>, String> {
    let mut cards = Vec::new();
    for part in s.split_whitespace() {
        let part = part.trim();
        if part.len() < 2 {
            return Err(format!("无法解析: {}", part));
        }
        let (rank_str, suit_char) = if let Some(rest) = part.strip_prefix("10") {
            ("10", rest)
        } else {
            (&part[..part.len() - 1], &part[part.len() - 1..])
        };
        let rank = match rank_str.to_uppercase().as_str() {
            "A" => Rank::Ace,
            "K" => Rank::King,
            "Q" => Rank::Queen,
            "J" => Rank::Jack,
            "10" => Rank::Ten,
            "9" => Rank::Nine,
            "8" => Rank::Eight,
            "7" => Rank::Seven,
            "6" => Rank::Six,
            "5" => Rank::Five,
            "4" => Rank::Four,
            "3" => Rank::Three,
            "2" => Rank::Two,
            _ => return Err(format!("无效点数: {}", rank_str)),
        };
        let suit = match suit_char.to_lowercase().as_str() {
            "h" | "♥" => Suit::Heart,
            "d" | "♦" => Suit::Diamond,
            "s" | "♠" => Suit::Spade,
            "c" | "♣" => Suit::Club,
            _ => return Err(format!("无效花色: {}", suit_char)),
        };
        cards.push(Card::new(suit, rank));
    }
    Ok(cards)
}

/// 21点规则
pub struct BlackjackRules {
    metadata: RuleMetadata,
}

impl BlackjackRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("21点规则", "Blackjack 标准规则")
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
                    value += card.rank.value();
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
        vec!["A: 可算1或11点", "2-10: 按牌面点数", "J、Q、K: 算10点"]
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

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        let cards_str = match context {
            ValidateContext::PokerCards(s) => s.as_str(),
            ValidateContext::Generic(s) => s.as_str(),
            _ => return Ok(false),
        };

        // 解析牌面并验证21点手牌
        let cards = match parse_blackjack_cards(cards_str) {
            Ok(c) => c,
            Err(_) => return Ok(false),
        };
        if cards.is_empty() {
            return Ok(false);
        }
        // 有效手牌: 至少有牌，且点数不超过21（或正好是bust也算"有效状态"）
        let value = Self::calculate_hand_value(&cards);
        Ok(value <= 21 || Self::is_bust(&cards))
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
            self.card_values()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.player_actions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dealer_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.payout_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
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

    #[test]
    fn test_validate_blackjack_hand() {
        let rules = BlackjackRules::new();
        // 21点
        assert!(rules.validate("Kh As").unwrap());
    }

    #[test]
    fn test_validate_under_21() {
        let rules = BlackjackRules::new();
        assert!(rules.validate("5h 6d 7s").unwrap());
    }

    #[test]
    fn test_validate_bust() {
        let rules = BlackjackRules::new();
        // K+Q+2 = 22, bust
        assert!(rules.validate("Ks Qh 2d").unwrap()); // bust 也算有效状态
    }

    #[test]
    fn test_validate_invalid() {
        let rules = BlackjackRules::new();
        assert!(!rules.validate("Xx Yy").unwrap());
        assert!(!rules.validate("").unwrap());
    }
}
