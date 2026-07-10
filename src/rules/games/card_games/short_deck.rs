//! 短牌扑克规则
//!
//! Short Deck (Six Plus Hold'em) 使用36张牌（去掉2-5），
//! 牌型评估与标准扑克略有不同：同花大于满堂红。

use super::cards::{Card, Rank, Suit};
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 短牌牌型等级
///
/// 注意: 短牌中同花 > 满堂红（因为同花更难凑）
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum ShortDeckHandRank {
    /// 高牌
    HighCard,
    /// 一对
    OnePair,
    /// 两对
    TwoPair,
    /// 三条
    ThreeOfAKind,
    /// 顺子（A-6-7-8-9 也算顺子）
    Straight,
    /// 满堂红（三条+一对）
    FullHouse,
    /// 同花（在短牌中 > 满堂红）
    Flush,
    /// 四条
    FourOfAKind,
    /// 同花顺
    StraightFlush,
    /// 皇家同花顺
    RoyalFlush,
}

impl ShortDeckHandRank {
    pub fn name(&self) -> &'static str {
        match self {
            ShortDeckHandRank::HighCard => "高牌",
            ShortDeckHandRank::OnePair => "一对",
            ShortDeckHandRank::TwoPair => "两对",
            ShortDeckHandRank::ThreeOfAKind => "三条",
            ShortDeckHandRank::Straight => "顺子",
            ShortDeckHandRank::FullHouse => "满堂红",
            ShortDeckHandRank::Flush => "同花",
            ShortDeckHandRank::FourOfAKind => "四条",
            ShortDeckHandRank::StraightFlush => "同花顺",
            ShortDeckHandRank::RoyalFlush => "皇家同花顺",
        }
    }
}

/// 短牌扑克手牌评估
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShortDeckEvaluation {
    pub rank: ShortDeckHandRank,
    pub cards: Vec<Card>,
    pub tiebreaker: Vec<u8>,
}

/// 短牌扑克规则
pub struct ShortDeckRules {
    metadata: RuleMetadata,
}

impl ShortDeckRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("短牌扑克规则", "Short Deck / Six Plus Hold'em 规则")
                .with_origin("亚洲")
                .with_tags(vec!["扑克".into(), "短牌".into(), "六加".into()]),
        }
    }

    /// 检查牌是否在短牌范围内（6-A）
    fn is_short_deck_card(card: &Card) -> bool {
        matches!(
            card.rank,
            Rank::Six
                | Rank::Seven
                | Rank::Eight
                | Rank::Nine
                | Rank::Ten
                | Rank::Jack
                | Rank::Queen
                | Rank::King
                | Rank::Ace
        )
    }

    /// 获取短牌点数值（6=6, 7=7, ..., A=14）
    fn short_deck_value(card: &Card) -> u8 {
        card.value()
    }

    /// 评估短牌手牌
    pub fn evaluate_hand(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        // 验证所有牌都在短牌范围内
        if !cards.iter().all(Self::is_short_deck_card) {
            return None;
        }

        if cards.len() < 5 {
            return None;
        }

        // 短牌顺子规则：A-6-7-8-9 也算顺子（最小顺子）
        // 尝试找出最佳牌型（注意顺序：同花 > 满堂红）

        if let Some(eval) = Self::check_royal_flush(cards) {
            return Some(eval);
        }
        if let Some(eval) = Self::check_straight_flush(cards) {
            return Some(eval);
        }
        if let Some(eval) = Self::check_four_of_a_kind(cards) {
            return Some(eval);
        }
        // 注意：同花在满堂红之前检查
        if let Some(eval) = Self::check_flush(cards) {
            return Some(eval);
        }
        if let Some(eval) = Self::check_full_house(cards) {
            return Some(eval);
        }
        if let Some(eval) = Self::check_straight_short_deck(cards) {
            return Some(eval);
        }
        if let Some(eval) = Self::check_three_of_a_kind(cards) {
            return Some(eval);
        }
        if let Some(eval) = Self::check_two_pair(cards) {
            return Some(eval);
        }
        if let Some(eval) = Self::check_one_pair(cards) {
            return Some(eval);
        }

        Some(Self::check_high_card(cards))
    }

    fn check_royal_flush(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let royal_cards: Vec<Card> = cards
                .iter()
                .filter(|c| c.suit == suit)
                .filter(|c| {
                    matches!(
                        c.rank,
                        Rank::Ten | Rank::Jack | Rank::Queen | Rank::King | Rank::Ace
                    )
                })
                .cloned()
                .collect();

            if royal_cards.len() == 5 {
                return Some(ShortDeckEvaluation {
                    rank: ShortDeckHandRank::RoyalFlush,
                    cards: royal_cards,
                    tiebreaker: vec![14],
                });
            }
        }
        None
    }

    fn check_straight_flush(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let suit_cards: Vec<Card> = cards.iter().filter(|c| c.suit == suit).cloned().collect();

            if suit_cards.len() >= 5 {
                if let Some(straight) = Self::find_straight_short_deck(&suit_cards) {
                    return Some(ShortDeckEvaluation {
                        rank: ShortDeckHandRank::StraightFlush,
                        cards: straight,
                        tiebreaker: vec![Self::short_deck_value(&straight[0])],
                    });
                }
            }
        }
        None
    }

    fn check_four_of_a_kind(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        let counts = Self::count_ranks(cards);

        for (&rank, &count) in &counts {
            if count == 4 {
                let four_cards: Vec<Card> =
                    cards.iter().filter(|c| c.rank == rank).cloned().collect();

                let kicker = cards
                    .iter()
                    .filter(|c| c.rank != rank)
                    .max_by_key(|c| Self::short_deck_value(c))
                    .cloned();

                let mut result = four_cards;
                if let Some(k) = kicker {
                    result.push(k);
                }

                return Some(ShortDeckEvaluation {
                    rank: ShortDeckHandRank::FourOfAKind,
                    cards: result,
                    tiebreaker: vec![Self::short_deck_value(&Card::new(Suit::Spade, rank))],
                });
            }
        }
        None
    }

    fn check_flush(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let suit_cards: Vec<Card> = cards.iter().filter(|c| c.suit == suit).cloned().collect();

            if suit_cards.len() >= 5 {
                let mut sorted = suit_cards;
                sorted.sort_by_key(|c| std::cmp::Reverse(Self::short_deck_value(c)));
                sorted.truncate(5);

                return Some(ShortDeckEvaluation {
                    rank: ShortDeckHandRank::Flush,
                    cards: sorted,
                    tiebreaker: sorted.iter().map(Self::short_deck_value).collect(),
                });
            }
        }
        None
    }

    fn check_full_house(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        let counts = Self::count_ranks(cards);

        let three_rank = counts.iter().find(|(_, &c)| c == 3).map(|(&r, _)| r);
        let pair_rank = counts
            .iter()
            .filter(|(_, &c)| c == 2 || c == 3)
            .filter(|(r, _)| Some(*r) != three_rank.as_ref())
            .max_by_key(|(&r, _)| Self::short_deck_value(&Card::new(Suit::Spade, r)))
            .map(|(&r, _)| r);

        if let (Some(three), Some(pair)) = (three_rank, pair_rank) {
            let three_cards: Vec<Card> =
                cards.iter().filter(|c| c.rank == three).cloned().collect();
            let pair_cards: Vec<Card> = cards
                .iter()
                .filter(|c| c.rank == pair)
                .take(2)
                .cloned()
                .collect();

            return Some(ShortDeckEvaluation {
                rank: ShortDeckHandRank::FullHouse,
                cards: [three_cards, pair_cards].concat(),
                tiebreaker: vec![
                    Self::short_deck_value(&Card::new(Suit::Spade, three)),
                    Self::short_deck_value(&Card::new(Suit::Spade, pair)),
                ],
            });
        }
        None
    }

    fn check_straight_short_deck(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        Self::find_straight_short_deck(cards).map(|straight| ShortDeckEvaluation {
            rank: ShortDeckHandRank::Straight,
            cards: straight.clone(),
            tiebreaker: vec![Self::short_deck_value(&straight[0])],
        })
    }

    fn find_straight_short_deck(cards: &[Card]) -> Option<Vec<Card>> {
        let mut values: Vec<u8> = cards.iter().map(Self::short_deck_value).collect();
        values.sort();
        values.dedup();

        // 短牌特殊顺子：A-6-7-8-9（最小顺子，A算5）
        if values.contains(&14)
            && values.contains(&6)
            && values.contains(&7)
            && values.contains(&8)
            && values.contains(&9)
        {
            let straight: Vec<Card> = cards
                .iter()
                .filter(|c| {
                    matches!(
                        c.rank,
                        Rank::Ace | Rank::Six | Rank::Seven | Rank::Eight | Rank::Nine
                    )
                })
                .cloned()
                .collect();
            return Some(straight);
        }

        // 普通顺子（至少需要5张不同牌面）
        if values.len() < 5 {
            return None;
        }

        for i in 0..values.len() - 4 {
            if values[i + 4] - values[i] == 4 {
                let start = values[i];
                let straight: Vec<Card> = cards
                    .iter()
                    .filter(|c| {
                        Self::short_deck_value(c) >= start && Self::short_deck_value(c) <= start + 4
                    })
                    .cloned()
                    .collect();
                return Some(straight);
            }
        }

        None
    }

    fn check_three_of_a_kind(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        let counts = Self::count_ranks(cards);

        for (&rank, &count) in &counts {
            if count == 3 {
                let three_cards: Vec<Card> =
                    cards.iter().filter(|c| c.rank == rank).cloned().collect();
                let kickers: Vec<Card> = cards.iter().filter(|c| c.rank != rank).cloned().collect();

                let mut sorted_kickers = kickers;
                sorted_kickers.sort_by_key(|c| std::cmp::Reverse(Self::short_deck_value(c)));
                sorted_kickers.truncate(2);

                return Some(ShortDeckEvaluation {
                    rank: ShortDeckHandRank::ThreeOfAKind,
                    cards: [three_cards, sorted_kickers].concat(),
                    tiebreaker: vec![Self::short_deck_value(&Card::new(Suit::Spade, rank))],
                });
            }
        }
        None
    }

    fn check_two_pair(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        let counts = Self::count_ranks(cards);

        let pairs: Vec<Rank> = counts
            .iter()
            .filter(|(_, &c)| c == 2)
            .map(|(&r, _)| r)
            .collect();

        if pairs.len() >= 2 {
            let mut sorted_pairs = pairs;
            sorted_pairs.sort_by_key(|r| {
                std::cmp::Reverse(Self::short_deck_value(&Card::new(Suit::Spade, *r)))
            });

            let pair1_cards: Vec<Card> = cards
                .iter()
                .filter(|c| c.rank == sorted_pairs[0])
                .cloned()
                .collect();
            let pair2_cards: Vec<Card> = cards
                .iter()
                .filter(|c| c.rank == sorted_pairs[1])
                .cloned()
                .collect();

            let kicker = cards
                .iter()
                .filter(|c| !sorted_pairs.contains(&c.rank))
                .max_by_key(|c| Self::short_deck_value(c))
                .cloned();

            let mut result = [pair1_cards, pair2_cards].concat();
            if let Some(k) = kicker {
                result.push(k);
            }

            return Some(ShortDeckEvaluation {
                rank: ShortDeckHandRank::TwoPair,
                cards: result,
                tiebreaker: vec![
                    Self::short_deck_value(&Card::new(Suit::Spade, sorted_pairs[0])),
                    Self::short_deck_value(&Card::new(Suit::Spade, sorted_pairs[1])),
                ],
            });
        }
        None
    }

    fn check_one_pair(cards: &[Card]) -> Option<ShortDeckEvaluation> {
        let counts = Self::count_ranks(cards);

        for (&rank, &count) in &counts {
            if count == 2 {
                let pair_cards: Vec<Card> =
                    cards.iter().filter(|c| c.rank == rank).cloned().collect();
                let kickers: Vec<Card> = cards.iter().filter(|c| c.rank != rank).cloned().collect();

                let mut sorted_kickers = kickers;
                sorted_kickers.sort_by_key(|c| std::cmp::Reverse(Self::short_deck_value(c)));
                sorted_kickers.truncate(3);

                return Some(ShortDeckEvaluation {
                    rank: ShortDeckHandRank::OnePair,
                    cards: [pair_cards, sorted_kickers].concat(),
                    tiebreaker: vec![Self::short_deck_value(&Card::new(Suit::Spade, rank))],
                });
            }
        }
        None
    }

    fn check_high_card(cards: &[Card]) -> ShortDeckEvaluation {
        let mut sorted: Vec<Card> = cards.to_vec();
        sorted.sort_by_key(|c| std::cmp::Reverse(Self::short_deck_value(c)));
        sorted.truncate(5);

        ShortDeckEvaluation {
            rank: ShortDeckHandRank::HighCard,
            cards: sorted.clone(),
            tiebreaker: sorted.iter().map(Self::short_deck_value).collect(),
        }
    }

    fn count_ranks(cards: &[Card]) -> std::collections::HashMap<Rank, u8> {
        let mut counts = std::collections::HashMap::new();
        for card in cards {
            *counts.entry(card.rank).or_insert(0) += 1;
        }
        counts
    }

    /// 比较两手短牌牌型
    pub fn compare_hands(
        hand1: &ShortDeckEvaluation,
        hand2: &ShortDeckEvaluation,
    ) -> std::cmp::Ordering {
        hand1.rank.cmp(&hand2.rank).then_with(|| {
            for (a, b) in hand1.tiebreaker.iter().zip(hand2.tiebreaker.iter()) {
                match a.cmp(b) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }
            std::cmp::Ordering::Equal
        })
    }
}

impl Default for ShortDeckRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShortDeckRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("short_deck")
    }

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        let input = match context {
            ValidateContext::PokerCards(s) => s.as_str(),
            ValidateContext::Generic(s) => s.as_str(),
            _ => return Ok(false),
        };

        // 验证牌面格式和范围（必须是6-A）
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() < 5 {
            return Ok(false);
        }

        // 简单检查：确保没有2-5
        for part in &parts {
            let rank_str = if let Some(rest) = part.strip_prefix("10") {
                "10"
            } else {
                &part[..part.len() - 1]
            };

            // 2-5不在短牌范围内
            if matches!(rank_str, "2" | "3" | "4" | "5") {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn explain(&self) -> String {
        "【短牌扑克规则】\n\n\
        Short Deck (Six Plus) 使用36张牌（去掉2-5）。\n\
        牌型评估与标准扑克不同：同花 > 满堂红。\n\n\
        使用牌面范围: 6, 7, 8, 9, 10, J, Q, K, A\n\n\
        牌型等级（从高到低）:\n\
        1. 皇家同花顺 - A-K-Q-J-10 同花\n\
        2. 同花顺 - 同花连续5张\n\
        3. 四条 - 4张相同\n\
        4. 同花 - 5张同花（比标准扑克更强）\n\
        5. 满堂红 - 三条+对子\n\
        6. 顺子 - 连续5张（含A-6-7-8-9最小顺子）\n\
        7. 三条\n\
        8. 两对\n\
        9. 一对\n\
        10. 高牌\n\n\
        关键区别:\n\
        - 同花更难凑（5/36 vs 标准13/52），故大于满堂红\n\
        - A可以组成最小顺子 A-6-7-8-9\n\
        - 三条更容易出现，牌力相对下降"
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    fn card(suit: Suit, rank: Rank) -> Card {
        Card::new(suit, rank)
    }

    #[test]
    fn test_short_deck_metadata() {
        let rules = ShortDeckRules::new();
        assert_eq!(rules.metadata().name, "短牌扑克规则");
    }

    #[test]
    fn test_short_deck_royal_flush() {
        let cards = vec![
            card(Suit::Spade, Rank::Ace),
            card(Suit::Spade, Rank::King),
            card(Suit::Spade, Rank::Queen),
            card(Suit::Spade, Rank::Jack),
            card(Suit::Spade, Rank::Ten),
        ];

        let eval = ShortDeckRules::evaluate_hand(&cards);
        assert!(eval.is_some());
        assert_eq!(eval.unwrap().rank, ShortDeckHandRank::RoyalFlush);
    }

    #[test]
    fn test_short_deck_flush_beats_full_house() {
        // 同花
        let flush_cards = vec![
            card(Suit::Heart, Rank::Ace),
            card(Suit::Heart, Rank::King),
            card(Suit::Heart, Rank::Nine),
            card(Suit::Heart, Rank::Eight),
            card(Suit::Heart, Rank::Seven),
        ];

        // 满堂红
        let fullhouse_cards = vec![
            card(Suit::Spade, Rank::King),
            card(Suit::Heart, Rank::King),
            card(Suit::Diamond, Rank::King),
            card(Suit::Club, Rank::Nine),
            card(Suit::Spade, Rank::Nine),
        ];

        let flush = ShortDeckRules::evaluate_hand(&flush_cards).unwrap();
        let fullhouse = ShortDeckRules::evaluate_hand(&fullhouse_cards).unwrap();

        // 短牌中同花 > 满堂红
        assert!(flush.rank > fullhouse.rank);
    }

    #[test]
    fn test_short_deck_minimum_straight() {
        // A-6-7-8-9 最小顺子
        let cards = vec![
            card(Suit::Spade, Rank::Ace),
            card(Suit::Heart, Rank::Six),
            card(Suit::Diamond, Rank::Seven),
            card(Suit::Club, Rank::Eight),
            card(Suit::Spade, Rank::Nine),
        ];

        let eval = ShortDeckRules::evaluate_hand(&cards);
        assert!(eval.is_some());
        assert_eq!(eval.unwrap().rank, ShortDeckHandRank::Straight);
    }

    #[test]
    fn test_short_deck_invalid_card() {
        // 包含不在短牌范围的牌（2-5）
        let cards = vec![
            card(Suit::Spade, Rank::Two),
            card(Suit::Heart, Rank::Three),
            card(Suit::Diamond, Rank::Four),
            card(Suit::Club, Rank::Five),
            card(Suit::Spade, Rank::Six),
        ];

        let eval = ShortDeckRules::evaluate_hand(&cards);
        assert!(eval.is_none()); // 2-5不在短牌范围
    }

    #[test]
    fn test_short_deck_validate_range() {
        let rules = ShortDeckRules::new();

        // 合法的短牌
        let result = rules.validate(&ValidateContext::Generic("As Kh Qd Jc 10s".to_string()));
        assert!(result.unwrap());

        // 包含2-5（不合法）
        let result = rules.validate(&ValidateContext::Generic("As 2h 3d 4c 5s".to_string()));
        assert!(!result.unwrap());
    }

    #[test]
    fn test_short_deck_four_of_kind() {
        let cards = vec![
            card(Suit::Spade, Rank::Nine),
            card(Suit::Heart, Rank::Nine),
            card(Suit::Diamond, Rank::Nine),
            card(Suit::Club, Rank::Nine),
            card(Suit::Spade, Rank::Ace),
        ];

        let eval = ShortDeckRules::evaluate_hand(&cards);
        assert!(eval.is_some());
        assert_eq!(eval.unwrap().rank, ShortDeckHandRank::FourOfAKind);
    }

    #[test]
    fn test_short_deck_compare_equal() {
        let hand1 = ShortDeckRules::evaluate_hand(&[
            card(Suit::Spade, Rank::Ace),
            card(Suit::Heart, Rank::Ace),
            card(Suit::Diamond, Rank::King),
            card(Suit::Club, Rank::Nine),
            card(Suit::Spade, Rank::Eight),
        ])
        .unwrap();

        let hand2 = ShortDeckRules::evaluate_hand(&[
            card(Suit::Heart, Rank::Ace),
            card(Suit::Diamond, Rank::Ace),
            card(Suit::Spade, Rank::King),
            card(Suit::Heart, Rank::Nine),
            card(Suit::Club, Rank::Eight),
        ])
        .unwrap();

        assert_eq!(
            ShortDeckRules::compare_hands(&hand1, &hand2),
            std::cmp::Ordering::Equal
        );
    }

    #[test]
    fn test_short_deck_explain() {
        let rules = ShortDeckRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("同花 > 满堂红"));
        assert!(explanation.contains("36张牌"));
        assert!(explanation.contains("A-6-7-8-9"));
    }
}
