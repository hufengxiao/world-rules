//! 德州扑克规则

use super::cards::{Card, Rank, Suit};
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};
use std::collections::HashMap;

/// 从字符串解析扑克牌列表
///
/// 格式: 用空格分隔，如 "Ah Kd Qs Jc 10h 9d 8s"
/// 花色: h=红心 d=方块 s=黑桃 c=梅花
fn parse_poker_cards(s: &str) -> Result<Vec<Card>, String> {
    let mut cards = Vec::new();
    for part in s.split_whitespace() {
        let card = parse_single_card(part)?;
        cards.push(card);
    }
    Ok(cards)
}

fn parse_single_card(s: &str) -> Result<Card, String> {
    let s = s.trim();
    if s.len() < 2 {
        return Err(format!("无法解析: {}", s));
    }
    let (rank_str, suit_char) = if s.starts_with("10") {
        ("10", &s[2..])
    } else {
        (&s[..s.len() - 1], &s[s.len() - 1..])
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
    Ok(Card::new(suit, rank))
}

/// 牌型等级
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
            metadata: RuleMetadata::new("德州扑克规则", "Texas Hold'em 标准规则")
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
            let suit_cards: Vec<Card> = cards.iter().filter(|c| c.suit == suit).cloned().collect();

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
                let four_cards: Vec<Card> =
                    cards.iter().filter(|c| c.rank == rank).cloned().collect();

                let kicker = cards
                    .iter()
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

        let three_rank = counts.iter().find(|(_, &c)| c == 3).map(|(&r, _)| r);

        let pair_rank = counts
            .iter()
            .filter(|(_, &c)| c == 2 || c == 3)
            .filter(|(r, _)| Some(*r) != three_rank.as_ref())
            .max_by_key(|(&r, _)| r.value())
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
            let suit_cards: Vec<Card> = cards.iter().filter(|c| c.suit == suit).cloned().collect();

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
        Self::find_straight(cards).map(|straight| HandEvaluation {
            rank: HandRank::Straight,
            cards: straight.clone(),
            tiebreaker: vec![straight[0].value()],
        })
    }

    fn find_straight(cards: &[Card]) -> Option<Vec<Card>> {
        let mut values: Vec<u8> = cards.iter().map(|c| c.value()).collect();
        values.sort();
        values.dedup();

        // 检查 A-2-3-4-5 (小顺子)
        if values.contains(&14)
            && values.contains(&2)
            && values.contains(&3)
            && values.contains(&4)
            && values.contains(&5)
        {
            let straight: Vec<Card> = cards
                .iter()
                .filter(|c| {
                    matches!(
                        c.rank,
                        Rank::Ace | Rank::Two | Rank::Three | Rank::Four | Rank::Five
                    )
                })
                .cloned()
                .collect();
            return Some(straight);
        }

        // 检查普通顺子（至少需要5张不同牌面）
        if values.len() < 5 {
            return None;
        }
        for i in 0..values.len() - 4 {
            if values[i + 4] - values[i] == 4 {
                let start = values[i];
                let straight: Vec<Card> = cards
                    .iter()
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
                let three_cards: Vec<Card> =
                    cards.iter().filter(|c| c.rank == rank).cloned().collect();

                let kickers: Vec<Card> = cards.iter().filter(|c| c.rank != rank).cloned().collect();

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

        let pairs: Vec<Rank> = counts
            .iter()
            .filter(|(_, &c)| c == 2)
            .map(|(&r, _)| r)
            .collect();

        if pairs.len() >= 2 {
            let mut sorted_pairs = pairs;
            sorted_pairs.sort_by(|a, b| b.value().cmp(&a.value()));

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
                let pair_cards: Vec<Card> =
                    cards.iter().filter(|c| c.rank == rank).cloned().collect();

                let kickers: Vec<Card> = cards.iter().filter(|c| c.rank != rank).cloned().collect();

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
        // 解析牌面并评估牌型
        let cards = match parse_poker_cards(context) {
            Ok(c) => c,
            Err(_) => return Ok(false),
        };
        if cards.len() < 5 {
            return Ok(false);
        }
        let eval = Self::evaluate_hand(&cards);
        // 只要能评估出牌型就有效
        Ok(eval.rank != HandRank::HighCard || cards.len() >= 5)
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

    fn c(suit: Suit, rank: Rank) -> Card {
        Card::new(suit, rank)
    }

    // ==================== 牌型等级排序 ====================

    #[test]
    fn test_hand_rank_order() {
        assert!(HandRank::RoyalFlush > HandRank::StraightFlush);
        assert!(HandRank::StraightFlush > HandRank::FourOfAKind);
        assert!(HandRank::FourOfAKind > HandRank::FullHouse);
        assert!(HandRank::FullHouse > HandRank::Flush);
        assert!(HandRank::Flush > HandRank::Straight);
        assert!(HandRank::Straight > HandRank::ThreeOfAKind);
        assert!(HandRank::ThreeOfAKind > HandRank::TwoPair);
        assert!(HandRank::TwoPair > HandRank::OnePair);
        assert!(HandRank::OnePair > HandRank::HighCard);
    }

    #[test]
    fn test_texas_holdem_rules() {
        let rules = TexasHoldemRules::new();
        assert_eq!(rules.metadata().name, "德州扑克规则");
    }

    // ==================== 皇家同花顺 ====================

    #[test]
    fn test_royal_flush() {
        let cards = vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Spade, Rank::King),
            c(Suit::Spade, Rank::Queen),
            c(Suit::Spade, Rank::Jack),
            c(Suit::Spade, Rank::Ten),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::RoyalFlush);
    }

    #[test]
    fn test_royal_flush_all_suits() {
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let cards = vec![
                c(suit, Rank::Ace),
                c(suit, Rank::King),
                c(suit, Rank::Queen),
                c(suit, Rank::Jack),
                c(suit, Rank::Ten),
            ];
            assert_eq!(
                TexasHoldemRules::evaluate_hand(&cards).rank,
                HandRank::RoyalFlush
            );
        }
    }

    // ==================== 同花顺 ====================

    #[test]
    fn test_straight_flush() {
        let cards = vec![
            c(Suit::Heart, Rank::Nine),
            c(Suit::Heart, Rank::Eight),
            c(Suit::Heart, Rank::Seven),
            c(Suit::Heart, Rank::Six),
            c(Suit::Heart, Rank::Five),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::StraightFlush);
    }

    #[test]
    fn test_straight_flush_low_ace() {
        // A-2-3-4-5 同花顺（小同花顺）
        let cards = vec![
            c(Suit::Diamond, Rank::Ace),
            c(Suit::Diamond, Rank::Two),
            c(Suit::Diamond, Rank::Three),
            c(Suit::Diamond, Rank::Four),
            c(Suit::Diamond, Rank::Five),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::StraightFlush);
    }

    // ==================== 四条 ====================

    #[test]
    fn test_four_of_a_kind() {
        let cards = vec![
            c(Suit::Spade, Rank::Seven),
            c(Suit::Heart, Rank::Seven),
            c(Suit::Diamond, Rank::Seven),
            c(Suit::Club, Rank::Seven),
            c(Suit::Spade, Rank::Two),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::FourOfAKind);
    }

    #[test]
    fn test_four_of_a_kind_aces() {
        let cards = vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Heart, Rank::Ace),
            c(Suit::Diamond, Rank::Ace),
            c(Suit::Club, Rank::Ace),
            c(Suit::Spade, Rank::King),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::FourOfAKind);
    }

    // ==================== 满堂红 ====================

    #[test]
    fn test_full_house() {
        let cards = vec![
            c(Suit::Spade, Rank::Ten),
            c(Suit::Heart, Rank::Ten),
            c(Suit::Diamond, Rank::Ten),
            c(Suit::Club, Rank::Four),
            c(Suit::Spade, Rank::Four),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::FullHouse);
    }

    #[test]
    fn test_full_house_kings_over_twos() {
        let cards = vec![
            c(Suit::Spade, Rank::King),
            c(Suit::Heart, Rank::King),
            c(Suit::Diamond, Rank::King),
            c(Suit::Club, Rank::Two),
            c(Suit::Spade, Rank::Two),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::FullHouse);
    }

    // ==================== 同花 ====================

    #[test]
    fn test_flush() {
        let cards = vec![
            c(Suit::Club, Rank::Two),
            c(Suit::Club, Rank::Five),
            c(Suit::Club, Rank::Eight),
            c(Suit::Club, Rank::Jack),
            c(Suit::Club, Rank::Ace),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::Flush);
    }

    // ==================== 顺子 ====================

    #[test]
    fn test_straight() {
        let cards = vec![
            c(Suit::Spade, Rank::Six),
            c(Suit::Heart, Rank::Seven),
            c(Suit::Diamond, Rank::Eight),
            c(Suit::Club, Rank::Nine),
            c(Suit::Spade, Rank::Ten),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::Straight);
    }

    #[test]
    fn test_straight_low_ace() {
        // A-2-3-4-5 顺子
        let cards = vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Heart, Rank::Two),
            c(Suit::Diamond, Rank::Three),
            c(Suit::Club, Rank::Four),
            c(Suit::Spade, Rank::Five),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::Straight);
    }

    #[test]
    fn test_straight_high_ace() {
        // 10-J-Q-K-A 顺子（不是同花）
        let cards = vec![
            c(Suit::Spade, Rank::Ten),
            c(Suit::Heart, Rank::Jack),
            c(Suit::Diamond, Rank::Queen),
            c(Suit::Club, Rank::King),
            c(Suit::Spade, Rank::Ace),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::Straight);
    }

    // ==================== 三条 ====================

    #[test]
    fn test_three_of_a_kind() {
        let cards = vec![
            c(Suit::Spade, Rank::Nine),
            c(Suit::Heart, Rank::Nine),
            c(Suit::Diamond, Rank::Nine),
            c(Suit::Club, Rank::Five),
            c(Suit::Spade, Rank::Three),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::ThreeOfAKind);
    }

    // ==================== 两对 ====================

    #[test]
    fn test_two_pair() {
        let cards = vec![
            c(Suit::Spade, Rank::Jack),
            c(Suit::Heart, Rank::Jack),
            c(Suit::Diamond, Rank::Five),
            c(Suit::Club, Rank::Five),
            c(Suit::Spade, Rank::Three),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::TwoPair);
    }

    #[test]
    fn test_two_pair_aces_and_kings() {
        let cards = vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Heart, Rank::Ace),
            c(Suit::Diamond, Rank::King),
            c(Suit::Club, Rank::King),
            c(Suit::Spade, Rank::Two),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::TwoPair);
    }

    // ==================== 一对 ====================

    #[test]
    fn test_one_pair() {
        let cards = vec![
            c(Suit::Spade, Rank::Queen),
            c(Suit::Heart, Rank::Queen),
            c(Suit::Diamond, Rank::Nine),
            c(Suit::Club, Rank::Six),
            c(Suit::Spade, Rank::Three),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::OnePair);
    }

    // ==================== 高牌 ====================

    #[test]
    fn test_high_card() {
        let cards = vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Heart, Rank::Queen),
            c(Suit::Diamond, Rank::Nine),
            c(Suit::Club, Rank::Six),
            c(Suit::Spade, Rank::Three),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::HighCard);
    }

    // ==================== compare_hands 比较逻辑 ====================

    #[test]
    fn test_compare_different_ranks() {
        let royal = TexasHoldemRules::evaluate_hand(&vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Spade, Rank::King),
            c(Suit::Spade, Rank::Queen),
            c(Suit::Spade, Rank::Jack),
            c(Suit::Spade, Rank::Ten),
        ]);
        let high = TexasHoldemRules::evaluate_hand(&vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Heart, Rank::Queen),
            c(Suit::Diamond, Rank::Nine),
            c(Suit::Club, Rank::Six),
            c(Suit::Spade, Rank::Three),
        ]);
        assert_eq!(
            TexasHoldemRules::compare_hands(&royal, &high),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            TexasHoldemRules::compare_hands(&high, &royal),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_compare_same_rank_different_tiebreaker() {
        // 一对A vs 一对K
        let pair_aces = TexasHoldemRules::evaluate_hand(&vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Heart, Rank::Ace),
            c(Suit::Diamond, Rank::Five),
            c(Suit::Club, Rank::Three),
            c(Suit::Spade, Rank::Two),
        ]);
        let pair_kings = TexasHoldemRules::evaluate_hand(&vec![
            c(Suit::Spade, Rank::King),
            c(Suit::Heart, Rank::King),
            c(Suit::Diamond, Rank::Five),
            c(Suit::Club, Rank::Three),
            c(Suit::Spade, Rank::Two),
        ]);
        assert_eq!(
            TexasHoldemRules::compare_hands(&pair_aces, &pair_kings),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_compare_equal_hands() {
        let hand1 = TexasHoldemRules::evaluate_hand(&vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Heart, Rank::Ace),
            c(Suit::Diamond, Rank::Five),
            c(Suit::Club, Rank::Three),
            c(Suit::Spade, Rank::Two),
        ]);
        let hand2 = TexasHoldemRules::evaluate_hand(&vec![
            c(Suit::Diamond, Rank::Ace),
            c(Suit::Club, Rank::Ace),
            c(Suit::Spade, Rank::Five),
            c(Suit::Heart, Rank::Three),
            c(Suit::Diamond, Rank::Two),
        ]);
        assert_eq!(
            TexasHoldemRules::compare_hands(&hand1, &hand2),
            std::cmp::Ordering::Equal
        );
    }

    // ==================== 7张牌中选最佳5张 ====================

    #[test]
    fn test_7_cards_finds_best_hand() {
        // 7张牌中包含同花顺
        let cards = vec![
            c(Suit::Heart, Rank::Five),
            c(Suit::Heart, Rank::Six),
            c(Suit::Heart, Rank::Seven),
            c(Suit::Heart, Rank::Eight),
            c(Suit::Heart, Rank::Nine),
            c(Suit::Spade, Rank::Ace),
            c(Suit::Diamond, Rank::King),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::StraightFlush);
    }

    #[test]
    fn test_7_cards_flush_beats_straight() {
        let cards = vec![
            c(Suit::Club, Rank::Two),
            c(Suit::Club, Rank::Five),
            c(Suit::Club, Rank::Eight),
            c(Suit::Club, Rank::Jack),
            c(Suit::Club, Rank::Ace),
            c(Suit::Spade, Rank::Six),
            c(Suit::Heart, Rank::Seven),
        ];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        assert_eq!(eval.rank, HandRank::Flush);
    }

    // ==================== 边界: 同花顺 vs 四条 ====================

    #[test]
    fn test_straight_flush_beats_four_of_a_kind() {
        let sf = TexasHoldemRules::evaluate_hand(&vec![
            c(Suit::Heart, Rank::Five),
            c(Suit::Heart, Rank::Six),
            c(Suit::Heart, Rank::Seven),
            c(Suit::Heart, Rank::Eight),
            c(Suit::Heart, Rank::Nine),
        ]);
        let fk = TexasHoldemRules::evaluate_hand(&vec![
            c(Suit::Spade, Rank::Ace),
            c(Suit::Heart, Rank::Ace),
            c(Suit::Diamond, Rank::Ace),
            c(Suit::Club, Rank::Ace),
            c(Suit::Spade, Rank::King),
        ]);
        assert!(sf.rank > fk.rank);
    }

    // ==================== validate() 真实逻辑测试 ====================

    #[test]
    fn test_validate_royal_flush() {
        let rules = TexasHoldemRules::new();
        let result = rules.validate("Ah Kh Qh Jh 10h");
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_validate_two_pair() {
        let rules = TexasHoldemRules::new();
        let result = rules.validate("Ah Ad Ks Kc 5h");
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_validate_invalid_cards() {
        let rules = TexasHoldemRules::new();
        let result = rules.validate("Xx Yy Zz");
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_validate_too_few_cards() {
        let rules = TexasHoldemRules::new();
        let result = rules.validate("Ah Kd");
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_validate_empty() {
        let rules = TexasHoldemRules::new();
        let result = rules.validate("");
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_validate_seven_cards() {
        let rules = TexasHoldemRules::new();
        let result = rules.validate("Ah Kh Qh Jh 10h 9d 8s");
        assert_eq!(result.unwrap(), true);
    }
}
