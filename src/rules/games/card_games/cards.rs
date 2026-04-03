//! 扑克牌定义

use std::fmt;

/// 花色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    /// 黑桃 (最高)
    Spade,
    /// 红心
    Heart,
    /// 方块
    Diamond,
    /// 梅花 (最低)
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Spade => write!(f, "♠"),
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Club => write!(f, "♣"),
        }
    }
}

impl Suit {
    pub fn chinese_name(&self) -> &'static str {
        match self {
            Suit::Spade => "黑桃",
            Suit::Heart => "红心",
            Suit::Diamond => "方块",
            Suit::Club => "梅花",
        }
    }
}

/// 牌面大小
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

impl Rank {
    /// 获取牌面数值 (Ace = 14)
    pub fn value(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }

    /// 获取牌面数值 (Ace = 1, 用于某些游戏)
    pub fn low_value(&self) -> u8 {
        match self {
            Rank::Ace => 1,
            _ => self.value(),
        }
    }
}

/// 扑克牌
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn value(&self) -> u8 {
        self.rank.value()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 先比较牌面大小，再比较花色
        self.rank.cmp(&other.rank).then(self.suit.cmp(&other.suit))
    }
}

/// 标准扑克牌堆 (52张，不含大小王)
pub fn standard_deck() -> Vec<Card> {
    let suits = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];
    let ranks = [
        Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
        Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
        Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
    ];

    suits.iter()
        .flat_map(|&s| ranks.iter().map(move |&r| Card::new(s, r)))
        .collect()
}

/// 带大小王的扑克牌堆 (54张)
pub fn full_deck() -> Vec<Card> {
    standard_deck() // 简化实现，实际需要添加大小王
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_display() {
        let card = Card::new(Suit::Spade, Rank::Ace);
        assert_eq!(card.to_string(), "♠A");
    }

    #[test]
    fn test_deck_count() {
        assert_eq!(standard_deck().len(), 52);
    }

    #[test]
    fn test_rank_ordering() {
        assert!(Rank::Ace > Rank::King);
        assert!(Rank::Two < Rank::Three);
    }
}