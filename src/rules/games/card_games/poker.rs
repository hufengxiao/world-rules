//! 德州扑克规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};
use super::cards::{Card, Suit, Rank};
use std::collections::HashMap;

/// 牌型等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    /// 高牌
    HighCard,
    /// 一对
    OnePair,
    /// 两对
    TwoPair,
    /// 三条
    ThreeOfAKind,
    /// 顺子
    Straight,
    /// 同花
    Flush,
    /// 满堂红 (三条+一对)
    FullHouse,
    /// 四条
    FourOfAKind,
    /// 同花顺
    StraightFlush,
    /// 皇家同花顺
    RoyalFlush,
}

impl HandRank {
    pub fn name(&self) -> &'static str {
        match self {
            HandRank::HighCard => "高牌",
            HandRank::OnePair => "一对",
            HandRank::TwoPair => "两对",
            HandRank::ThreeOfAKind => "三条",
            HandRank::Straight => "顺子",
            HandRank::Flush => "同花",
            HandRank::FullHouse => "满堂红",
            HandRank::FourOfAKind => "四条",
            HandRank::StraightFlush => "同花顺",
            HandRank::RoyalFlush => "皇家同花顺",
        }
    }

    pub fn english_name(&self) -> &'static str {
        match self {
            HandRank::HighCard => "High Card",
            HandRank::OnePair => "One Pair",
            HandRank::TwoPair => "Two Pair",
            HandRank::ThreeOfAKind => "Three of a Kind",
            HandRank::Straight => "Straight",
            HandRank::Flush => "Flush",
            HandRank::FullHouse => "Full House",
            HandRank::FourOfAKind => "Four of a Kind",
            HandRank::StraightFlush => "Straight Flush",
            HandRank::RoyalFlush => "Royal Flush",
        }
    }
}

/// 德州扑克手牌评估结果
#[derive(Debug, Clone)]
pub struct HandEvaluation {
    pub rank: HandRank,
    pub cards: Vec<Card>,
    pub tiebreaker: Vec<u8>,
}

/// 德州扑克规则
pub struct TexasHoldemRules {
    metadata: RuleMetadata,
}

impl TexasHoldemRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "德州扑克规则",
                "Texas Hold'em 标准规则"
            )
            .with_origin("美国")
            .with_tags(vec!["扑克".into(), "德州".into(), "竞技".into()]),
        }
    }

    /// 评估最佳5张牌组合
    pub fn evaluate_hand(cards: &[Card]) -> HandEvaluation {
        assert!(cards.len() >= 5);

        // 尝试找出最佳牌型
        if let Some(eval) = Self::check_royal_flush(cards) {
            return eval;
        }
        if let Some(eval) = Self::check_straight_flush(cards) {
            return eval;
        }
        if let Some(eval) = Self::check_four_of_a_kind(cards) {
            return eval;
        }
        if let Some(eval) = Self::check_full_house(cards) {
            return eval;
        }
        if let Some(eval) = Self::check_flush(cards) {
            return eval;
        }
        if let Some(eval) = Self::check_straight(cards) {
            return eval;
        }
        if let Some(eval) = Self::check_three_of_a_kind(cards) {
            return eval;
        }
        if let Some(eval) = Self::check_two_pair(cards) {
            return eval;
        }
        if let Some(eval) = Self::check_one_pair(cards) {
            return eval;
        }

        Self::check_high_card(cards)
    }

    fn check_royal_flush(cards: &[Card]) -> Option<HandEvaluation> {
        // 检查是否存在 A-K-Q-J-10 同花
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let royal_cards: Vec<Card> = cards.iter()
                .filter(|c| c.suit == suit)
                .filter(|c| matches!(c.rank, Rank::Ten | Rank::Jack | Rank::Queen | Rank::King | Rank::Ace))
                .cloned()
                .collect();

            if royal_cards.len() == 5 {
                return Some(HandEvaluation {
                    rank: HandRank::RoyalFlush,
                    cards: royal_cards,
                    tiebreaker: vec![14],
                });
            }
        }
        None
    }

    fn check_straight_flush(cards: &[Card]) -> Option<HandEvaluation> {
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let suit_cards: Vec<Card> = cards.iter()
                .filter(|c| c.suit == suit)
                .cloned()
                .collect();

            if suit_cards.len() >= 5 {
                if let Some(straight) = Self::find_straight(&suit_cards) {
                    let tiebreaker_val = straight[0].value();
                    return Some(HandEvaluation {
                        rank: HandRank::StraightFlush,
                        cards: straight,
                        tiebreaker: vec![tiebreaker_val],
                    });
                }
            }
        }
        None
    }

    fn check_four_of_a_kind(cards: &[Card]) -> Option<HandEvaluation> {
        let counts = Self::count_ranks(cards);

        for (&rank, &count) in &counts {
            if count == 4 {
                let four_cards: Vec<Card> = cards.iter()
                    .filter(|c| c.rank == rank)
                    .cloned()
                    .collect();

                let kicker = cards.iter()
                    .filter(|c| c.rank != rank)
                    .max_by_key(|c| c.value())
                    .cloned();

                let mut result = four_cards;
                if let Some(k) = kicker {
                    result.push(k);
                }

                return Some(HandEvaluation {
                    rank: HandRank::FourOfAKind,
                    cards: result,
                    tiebreaker: vec![rank.value()],
                });
            }
        }
        None
    }

    fn check_full_house(cards: &[Card]) -> Option<HandEvaluation> {
        let counts = Self::count_ranks(cards);

        let three_rank = counts.iter()
            .find(|(_, &c)| c == 3)
            .map(|(&r, _)| r);

        let pair_rank = counts.iter()
            .filter(|(_, &c)| c == 2 || c == 3)
            .filter(|(r, _)| Some(*r) != three_rank.as_ref())
            .max_by_key(|(&r, _)| r.value())
            .map(|(&r, _)| r);

        if let (Some(three), Some(pair)) = (three_rank, pair_rank) {
            let three_cards: Vec<Card> = cards.iter()
                .filter(|c| c.rank == three)
                .cloned()
                .collect();

            let pair_cards: Vec<Card> = cards.iter()
                .filter(|c| c.rank == pair)
                .take(2)
                .cloned()
                .collect();

            return Some(HandEvaluation {
                rank: HandRank::FullHouse,
                cards: [three_cards, pair_cards].concat(),
                tiebreaker: vec![three.value(), pair.value()],
            });
        }
        None
    }

    fn check_flush(cards: &[Card]) -> Option<HandEvaluation> {
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let suit_cards: Vec<Card> = cards.iter()
                .filter(|c| c.suit == suit)
                .cloned()
                .collect();

            if suit_cards.len() >= 5 {
                let mut sorted = suit_cards;
                sorted.sort_by(|a, b| b.value().cmp(&a.value()));
                sorted.truncate(5);
                let tiebreaker: Vec<u8> = sorted.iter().map(|c| c.value()).collect();

                return Some(HandEvaluation {
                    rank: HandRank::Flush,
                    cards: sorted,
                    tiebreaker,
                });
            }
        }
        None
    }

    fn check_straight(cards: &[Card]) -> Option<HandEvaluation> {
        Self::find_straight(cards).map(|straight| {
            HandEvaluation {
                rank: HandRank::Straight,
                cards: straight.clone(),
                tiebreaker: vec![straight[0].value()],
            }
        })
    }

    fn find_straight(cards: &[Card]) -> Option<Vec<Card>> {
        let mut values: Vec<u8> = cards.iter()
            .map(|c| c.value())
            .collect();
        values.sort();
        values.dedup();

        // 检查 A-2-3-4-5 (小顺子)
        if values.contains(&14) && values.contains(&2) && values.contains(&3)
            && values.contains(&4) && values.contains(&5) {
            let straight: Vec<Card> = cards.iter()
                .filter(|c| matches!(c.rank, Rank::Ace | Rank::Two | Rank::Three | Rank::Four | Rank::Five))
                .cloned()
                .collect();
            return Some(straight);
        }

        // 检查普通顺子
        for i in 0..values.len() - 4 {
            if values[i + 4] - values[i] == 4 {
                let start = values[i];
                let straight: Vec<Card> = cards.iter()
                    .filter(|c| c.value() >= start && c.value() <= start + 4)
                    .cloned()
                    .collect();
                return Some(straight);
            }
        }
        None
    }

    fn check_three_of_a_kind(cards: &[Card]) -> Option<HandEvaluation> {
        let counts = Self::count_ranks(cards);

        for (&rank, &count) in &counts {
            if count == 3 {
                let three_cards: Vec<Card> = cards.iter()
                    .filter(|c| c.rank == rank)
                    .cloned()
                    .collect();

                let kickers: Vec<Card> = cards.iter()
                    .filter(|c| c.rank != rank)
                    .cloned()
                    .collect();

                let mut sorted_kickers = kickers;
                sorted_kickers.sort_by(|a, b| b.value().cmp(&a.value()));
                sorted_kickers.truncate(2);

                return Some(HandEvaluation {
                    rank: HandRank::ThreeOfAKind,
                    cards: [three_cards, sorted_kickers].concat(),
                    tiebreaker: vec![rank.value()],
                });
            }
        }
        None
    }

    fn check_two_pair(cards: &[Card]) -> Option<HandEvaluation> {
        let counts = Self::count_ranks(cards);

        let pairs: Vec<Rank> = counts.iter()
            .filter(|(_, &c)| c == 2)
            .map(|(&r, _)| r)
            .collect();

        if pairs.len() >= 2 {
            let mut sorted_pairs = pairs;
            sorted_pairs.sort_by(|a, b| b.value().cmp(&a.value()));

            let pair1_cards: Vec<Card> = cards.iter()
                .filter(|c| c.rank == sorted_pairs[0])
                .cloned()
                .collect();

            let pair2_cards: Vec<Card> = cards.iter()
                .filter(|c| c.rank == sorted_pairs[1])
                .cloned()
                .collect();

            let kicker = cards.iter()
                .filter(|c| !sorted_pairs.contains(&c.rank))
                .max_by_key(|c| c.value())
                .cloned();

            let mut result = [pair1_cards, pair2_cards].concat();
            if let Some(k) = kicker {
                result.push(k);
            }

            return Some(HandEvaluation {
                rank: HandRank::TwoPair,
                cards: result,
                tiebreaker: vec![sorted_pairs[0].value(), sorted_pairs[1].value()],
            });
        }
        None
    }

    fn check_one_pair(cards: &[Card]) -> Option<HandEvaluation> {
        let counts = Self::count_ranks(cards);

        for (&rank, &count) in &counts {
            if count == 2 {
                let pair_cards: Vec<Card> = cards.iter()
                    .filter(|c| c.rank == rank)
                    .cloned()
                    .collect();

                let kickers: Vec<Card> = cards.iter()
                    .filter(|c| c.rank != rank)
                    .cloned()
                    .collect();

                let mut sorted_kickers = kickers;
                sorted_kickers.sort_by(|a, b| b.value().cmp(&a.value()));
                sorted_kickers.truncate(3);

                return Some(HandEvaluation {
                    rank: HandRank::OnePair,
                    cards: [pair_cards, sorted_kickers].concat(),
                    tiebreaker: vec![rank.value()],
                });
            }
        }
        None
    }

    fn check_high_card(cards: &[Card]) -> HandEvaluation {
        let mut sorted: Vec<Card> = cards.to_vec();
        sorted.sort_by(|a, b| b.value().cmp(&a.value()));
        sorted.truncate(5);

        HandEvaluation {
            rank: HandRank::HighCard,
            cards: sorted.clone(),
            tiebreaker: sorted.iter().map(|c| c.value()).collect(),
        }
    }

    fn count_ranks(cards: &[Card]) -> HashMap<Rank, u8> {
        let mut counts = HashMap::new();
        for card in cards {
            *counts.entry(card.rank).or_insert(0) += 1;
        }
        counts
    }

    /// 比较两手牌大小
    pub fn compare_hands(hand1: &HandEvaluation, hand2: &HandEvaluation) -> std::cmp::Ordering {
        hand1.rank.cmp(&hand2.rank)
            .then_with(|| {
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

impl Default for TexasHoldemRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TexasHoldemRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("poker")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【德州扑克规则】\n\n\
            每位玩家获得2张底牌，共5张公共牌。\n\
            最佳5张牌组合决定胜负。\n\n\
            牌型等级 (从高到低):\n\
            1. 皇家同花顺 - 同花色的 A-K-Q-J-10\n\
            2. 同花顺 - 同花色的连续5张牌\n\
            3. 四条 - 4张相同牌面\n\
            4. 满堂红 - 3张相同 + 1对\n\
            5. 同花 - 5张同花色\n\
            6. 顺子 - 连续5张牌\n\
            7. 三条 - 3张相同牌面\n\
            8. 两对 - 2组对子\n\
            9. 一对 - 1组对子\n\
            10. 高牌 - 无以上牌型\n\n\
            游戏流程:\n\
            - 翻牌前 (Preflop): 发底牌\n\
            - 翻牌 (Flop): 3张公共牌\n\
            - 转牌 (Turn): 第4张公共牌\n\
            - 河牌 (River): 第5张公共牌"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_rank_order() {
        assert!(HandRank::RoyalFlush > HandRank::StraightFlush);
        assert!(HandRank::FullHouse > HandRank::Flush);
    }

    #[test]
    fn test_texas_holdem_rules() {
        let rules = TexasHoldemRules::new();
        assert_eq!(rules.metadata().name, "德州扑克规则");
    }
}