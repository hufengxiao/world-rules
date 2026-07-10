//! 扑克规则属性测试
//!
//! 使用 proptest 对扑克核心算法进行属性测试，
//! 确保在各种输入条件下不会 panic 并保持正确性。

use proptest::prelude::*;
use world_rules::rules::games::card_games::{Card, Rank, Suit};

/// 生成任意花色
prop_compose! {
    fn any_suit()(suit_idx in 0u8..=3) -> Suit {
        match suit_idx {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            _ => Suit::Club,
        }
    }
}

/// 生成任意牌面（不含大小王）
prop_compose! {
    fn any_rank_no_joker()(rank_idx in 0u8..=12) -> Rank {
        match rank_idx {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            _ => Rank::Ace,
        }
    }
}

/// 生成任意牌面（包含大小王）
prop_compose! {
    fn any_rank_with_joker()(rank_idx in 0u8..=13) -> Rank {
        match rank_idx {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => Rank::Joker,
        }
    }
}

/// 生成任意扑克牌
prop_compose! {
    fn any_card()(suit in any_suit(), rank in any_rank_no_joker()) -> Card {
        Card::new(suit, rank)
    }
}

/// 生成包含大小王的任意牌
prop_compose! {
    fn any_card_with_joker()(suit in any_suit(), rank in any_rank_with_joker()) -> Card {
        Card::new(suit, rank)
    }
}

/// 生成扑克手牌（最多7张）
prop_compose! {
    fn any_poker_hand()(cards in prop::collection::vec(any_card(), 0..7)) -> Vec<Card> {
        cards
    }
}

// ==================== Card 创建测试 ====================

proptest! {
    /// 测试扑克牌创建不 panic
    #[test]
    fn test_card_creation_no_panic(suit in any_suit(), rank in any_rank_with_joker()) {
        let card = Card::new(suit, rank);
        prop_assert!(true);
    }

    /// 测试牌面数值在有效范围
    #[test]
    fn test_rank_value_valid(rank in any_rank_with_joker()) {
        let value = rank.value();
        prop_assert!(value >= 2 && value <= 15);
    }

    /// 测试 Ace 的值是 14（不含大小王时）
    #[test]
    fn test_ace_value_is_14() {
        prop_assert_eq!(Rank::Ace.value(), 14);
    }

    /// 测试牌面顺序正确
    #[test]
    fn test_rank_ordering(rank1 in any_rank_no_joker(), rank2 in any_rank_no_joker()) {
        // 比较牌面大小应该符合枚举定义的顺序
        if rank1 < rank2 {
            prop_assert!(rank1.value() < rank2.value());
        } else if rank1 > rank2 {
            prop_assert!(rank1.value() > rank2.value());
        }
    }
}

// ==================== Suit 属性测试 ====================

proptest! {
    /// 测试花色显示不 panic
    #[test]
    fn test_suit_display_no_panic(suit in any_suit()) {
        let display = format!("{}", suit);
        prop_assert!(!display.is_empty());
    }

    /// 测试花色中文名不 panic
    #[test]
    fn test_suit_chinese_name_no_panic(suit in any_suit()) {
        let chinese = suit.chinese_name();
        prop_assert!(!chinese.is_empty());
    }
}

// ==================== Rank 属性测试 ====================

proptest! {
    /// 测试牌面显示不 panic
    #[test]
    fn test_rank_display_no_panic(rank in any_rank_with_joker()) {
        let display = format!("{}", rank);
        prop_assert!(!display.is_empty());
    }

    /// 测试非大小王牌面的数值正确
    #[test]
    fn test_rank_values(rank in any_rank_no_joker()) {
        let value = rank.value();
        prop_assert!(value >= 2 && value <= 14);
    }
}

// ==================== Card 显示和属性测试 ====================

proptest! {
    /// 测试扑克牌显示不 panic
    #[test]
    fn test_card_display_no_panic(card in any_card()) {
        let display = format!("{}", card);
        prop_assert!(!display.is_empty());
    }

    /// 测试扑克牌 Debug 不 panic
    #[test]
    fn test_card_debug_no_panic(card in any_card()) {
        let debug = format!("{:?}", card);
        prop_assert!(!debug.is_empty());
    }
}

// ==================== 手牌验证测试 ====================

proptest! {
    /// 测试手牌集合操作不 panic
    #[test]
    fn test_hand_operations_no_panic(cards in any_poker_hand()) {
        // 各种操作不应该 panic
        let _ = cards.len();
        let _ = cards.clone();
        prop_assert!(true);
    }

    /// 测试空手牌情况
    #[test]
    fn test_empty_hand() {
        let empty: Vec<Card> = vec![];
        prop_assert!(empty.is_empty());
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_proptest_config() {
        proptest!(|(suit in any_suit(), rank in any_rank_no_joker())| {
            let card = Card::new(suit, rank);
            assert!(card.suit() == suit);
            assert!(card.rank() == rank);
        });
    }
}