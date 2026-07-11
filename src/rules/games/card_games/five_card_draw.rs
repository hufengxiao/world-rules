//! 五张抽牌扑克规则
//!
//! Five Card Draw 是最经典的扑克变体之一，
//! 玩家获得5张暗牌，可以丢弃部分牌并重新抽牌。

use super::cards::{Card, Rank};
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 五张抽牌游戏阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawPhase {
    /// 发牌阶段 - 获得5张暗牌
    Deal,
    /// 第一次抽牌 - 可以丢弃最多5张牌
    FirstDraw,
    /// 第二次抽牌（某些变体）
    SecondDraw,
}

/// 五张抽牌手牌评估
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DrawHandEvaluation {
    /// 牌型等级
    pub rank: super::poker::HandRank,
    /// 5张牌组合
    pub cards: Vec<Card>,
    /// 建议保留的牌索引
    pub keep_indices: Vec<usize>,
    /// 建议丢弃的牌索引
    pub discard_indices: Vec<usize>,
}

/// 五张抽牌扑克规则
pub struct FiveCardDrawRules {
    metadata: RuleMetadata,
}

impl FiveCardDrawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("五张抽牌扑克规则", "Five Card Draw 经典规则")
                .with_origin("美国")
                .with_tags(vec!["扑克".into(), "Draw".into(), "经典".into()]),
        }
    }

    /// 评估5张牌的牌型
    pub fn evaluate_draw_hand(cards: &[Card]) -> Option<DrawHandEvaluation> {
        if cards.len() != 5 {
            return None;
        }

        let eval = super::poker::TexasHoldemRules::evaluate_hand(cards);

        // 计算建议保留/丢弃策略
        let (keep_indices, discard_indices) = Self::suggest_strategy(cards, &eval.rank);

        Some(DrawHandEvaluation {
            rank: eval.rank,
            cards: eval.cards.clone(),
            keep_indices,
            discard_indices,
        })
    }

    /// 根据牌型建议保留策略
    fn suggest_strategy(cards: &[Card], rank: &super::poker::HandRank) -> (Vec<usize>, Vec<usize>) {
        match rank {
            // 已成牌型 - 全部保留
            super::poker::HandRank::RoyalFlush
            | super::poker::HandRank::StraightFlush
            | super::poker::HandRank::FourOfAKind
            | super::poker::HandRank::FullHouse
            | super::poker::HandRank::Flush
            | super::poker::HandRank::Straight => ((0..5).collect(), vec![]),

            // 三条 - 保留三条，丢弃2张
            super::poker::HandRank::ThreeOfAKind => Self::find_keep_indices_for_same_rank(cards, 3),

            // 两对 - 保留两对，丢弃1张
            super::poker::HandRank::TwoPair => Self::find_keep_indices_for_pairs(cards),

            // 一对 - 保留对子，丢弃3张（除非有高牌）
            super::poker::HandRank::OnePair => Self::find_keep_indices_for_pair_with_kickers(cards),

            // 高牌 - 保留高牌或尝试抽同花/顺子
            super::poker::HandRank::HighCard => Self::find_keep_indices_for_high_card(cards),
        }
    }

    /// 找出三条应保留的索引
    fn find_keep_indices_for_same_rank(
        cards: &[Card],
        target_count: u8,
    ) -> (Vec<usize>, Vec<usize>) {
        let mut rank_counts: std::collections::HashMap<Rank, Vec<usize>> =
            std::collections::HashMap::new();

        for (i, card) in cards.iter().enumerate() {
            rank_counts.entry(card.rank).or_default().push(i);
        }

        for (_, indices) in rank_counts {
            if indices.len() == target_count as usize {
                let discard_indices: Vec<usize> = (0..5).filter(|i| !indices.contains(i)).collect();
                return (indices, discard_indices);
            }
        }

        ((0..5).collect(), vec![])
    }

    /// 找出两对应保留的索引
    fn find_keep_indices_for_pairs(cards: &[Card]) -> (Vec<usize>, Vec<usize>) {
        let mut rank_counts: std::collections::HashMap<Rank, Vec<usize>> =
            std::collections::HashMap::new();

        for (i, card) in cards.iter().enumerate() {
            rank_counts.entry(card.rank).or_default().push(i);
        }

        let mut keep_indices = Vec::new();
        for (_, indices) in rank_counts {
            if indices.len() == 2 {
                keep_indices.extend(indices);
            }
        }

        let discard_indices: Vec<usize> = (0..5).filter(|i| !keep_indices.contains(i)).collect();
        (keep_indices, discard_indices)
    }

    /// 找出一对应保留的索引（考虑 kicker）
    fn find_keep_indices_for_pair_with_kickers(cards: &[Card]) -> (Vec<usize>, Vec<usize>) {
        let mut rank_counts: std::collections::HashMap<Rank, Vec<usize>> =
            std::collections::HashMap::new();

        for (i, card) in cards.iter().enumerate() {
            rank_counts.entry(card.rank).or_default().push(i);
        }

        let mut keep_indices = Vec::new();

        // 保留对子
        for indices in rank_counts.values() {
            if indices.len() == 2 {
                keep_indices.extend(indices.clone());
                break;
            }
        }

        // 检查是否有 Ace kicker
        for (i, card) in cards.iter().enumerate() {
            if !keep_indices.contains(&i) && card.rank == Rank::Ace {
                keep_indices.push(i);
            }
        }

        let discard_indices: Vec<usize> = (0..5).filter(|i| !keep_indices.contains(i)).collect();
        (keep_indices, discard_indices)
    }

    /// 找出高牌应保留的索引
    fn find_keep_indices_for_high_card(cards: &[Card]) -> (Vec<usize>, Vec<usize>) {
        // 保留 Ace 和 King
        let keep_indices: Vec<usize> = cards
            .iter()
            .enumerate()
            .filter(|(_, c)| matches!(c.rank, Rank::Ace | Rank::King))
            .map(|(i, _)| i)
            .collect();

        let discard_indices: Vec<usize> = (0..5).filter(|i| !keep_indices.contains(i)).collect();
        (keep_indices, discard_indices)
    }

    /// 计算抽牌后的期望牌型概率
    pub fn calculate_draw_probability(discard_count: usize) -> String {
        match discard_count {
            1 => "抽1张: 约20%提升牌型",
            2 => "抽2张: 约15%提升至对子或更好",
            3 => "抽3张: 约10%提升至对子",
            4 => "抽4张: 约8%提升至对子",
            5 => "抽5张: 约50%至少获得一对",
            _ => "无效抽牌数量",
        }
        .to_string()
    }
}

impl Default for FiveCardDrawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FiveCardDrawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("draw")
    }

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        let input = match context {
            ValidateContext::PokerCards(s) => s.as_str(),
            ValidateContext::Generic(s) => s.as_str(),
            _ => return Ok(false),
        };

        // Five Card Draw 必须恰好5张牌
        let parts: Vec<&str> = input.split_whitespace().collect();
        Ok(parts.len() == 5)
    }

    fn explain(&self) -> String {
        "【五张抽牌扑克规则】\n\n\
        最经典的扑克形式，每人5张暗牌。\n\
        第一轮下注后，可丢弃最多5张牌并重新抽取。\n\n\
        游戏流程:\n\
        1. 发牌 - 每人5张暗牌\n\
        2. 第一轮下注\n\
        3. 抽牌 - 可丢弃0-5张牌\n\
        4. 第二轮下注\n\
        5. 比牌\n\n\
        牌型等级:\n\
        1. 皇家同花顺\n\
        2. 同花顺\n\
        3. 四条\n\
        4. 满堂红\n\
        5. 同花\n\
        6. 顺子\n\
        7. 三条\n\
        8. 两对\n\
        9. 一对\n\
        10. 高牌\n\n\
        策略建议:\n\
        - 已成牌型(同花顺/四条等): 全部保留\n\
        - 三条: 保留三条，抽2张\n\
        - 两对: 保留两对，抽1张\n\
        - 一对+高牌: 保留对子和高牌\n\
        - 高牌: 保留Ace/King或尝试抽新牌"
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
    fn test_draw_rules_metadata() {
        let rules = FiveCardDrawRules::new();
        assert_eq!(rules.metadata().name, "五张抽牌扑克规则");
    }

    #[test]
    fn test_draw_evaluate_royal_flush() {
        let cards = vec![
            card(Suit::Spade, Rank::Ace),
            card(Suit::Spade, Rank::King),
            card(Suit::Spade, Rank::Queen),
            card(Suit::Spade, Rank::Jack),
            card(Suit::Spade, Rank::Ten),
        ];

        let eval = FiveCardDrawRules::evaluate_draw_hand(&cards);
        assert!(eval.is_some());
        let result = eval.unwrap();
        assert_eq!(result.rank, super::poker::HandRank::RoyalFlush);
        assert_eq!(result.keep_indices.len(), 5); // 全部保留
        assert_eq!(result.discard_indices.len(), 0);
    }

    #[test]
    fn test_draw_evaluate_three_of_kind() {
        let cards = vec![
            card(Suit::Spade, Rank::Seven),
            card(Suit::Heart, Rank::Seven),
            card(Suit::Diamond, Rank::Seven),
            card(Suit::Club, Rank::King),
            card(Suit::Heart, Rank::Two),
        ];

        let eval = FiveCardDrawRules::evaluate_draw_hand(&cards);
        assert!(eval.is_some());
        let result = eval.unwrap();
        assert_eq!(result.rank, super::poker::HandRank::ThreeOfAKind);
        assert_eq!(result.keep_indices.len(), 3);
        assert_eq!(result.discard_indices.len(), 2);
    }

    #[test]
    fn test_draw_evaluate_two_pair() {
        let cards = vec![
            card(Suit::Spade, Rank::Ace),
            card(Suit::Heart, Rank::Ace),
            card(Suit::Diamond, Rank::King),
            card(Suit::Club, Rank::King),
            card(Suit::Heart, Rank::Two),
        ];

        let eval = FiveCardDrawRules::evaluate_draw_hand(&cards);
        assert!(eval.is_some());
        let result = eval.unwrap();
        assert_eq!(result.rank, super::poker::HandRank::TwoPair);
        assert_eq!(result.keep_indices.len(), 4);
        assert_eq!(result.discard_indices.len(), 1);
    }

    #[test]
    fn test_draw_evaluate_one_pair() {
        let cards = vec![
            card(Suit::Spade, Rank::Queen),
            card(Suit::Heart, Rank::Queen),
            card(Suit::Diamond, Rank::Nine),
            card(Suit::Club, Rank::Six),
            card(Suit::Heart, Rank::Three),
        ];

        let eval = FiveCardDrawRules::evaluate_draw_hand(&cards);
        assert!(eval.is_some());
        let result = eval.unwrap();
        assert_eq!(result.rank, super::poker::HandRank::OnePair);
        assert_eq!(result.keep_indices.len(), 2); // 只保留对子
    }

    #[test]
    fn test_draw_evaluate_one_pair_with_ace() {
        let cards = vec![
            card(Suit::Spade, Rank::Seven),
            card(Suit::Heart, Rank::Seven),
            card(Suit::Diamond, Rank::Ace),
            card(Suit::Club, Rank::Six),
            card(Suit::Heart, Rank::Three),
        ];

        let eval = FiveCardDrawRules::evaluate_draw_hand(&cards);
        assert!(eval.is_some());
        let result = eval.unwrap();
        assert_eq!(result.rank, super::poker::HandRank::OnePair);
        assert_eq!(result.keep_indices.len(), 3); // 对子 + Ace
    }

    #[test]
    fn test_draw_evaluate_wrong_count() {
        let cards = vec![
            card(Suit::Spade, Rank::Ace),
            card(Suit::Heart, Rank::King),
            card(Suit::Diamond, Rank::Queen),
        ];

        let eval = FiveCardDrawRules::evaluate_draw_hand(&cards);
        assert!(eval.is_none());
    }

    #[test]
    fn test_draw_validate_correct() {
        let rules = FiveCardDrawRules::new();
        let result = rules.validate(&ValidateContext::Generic("As Kh Qd Jc 10s".to_string()));
        assert!(result.unwrap());
    }

    #[test]
    fn test_draw_validate_wrong_count() {
        let rules = FiveCardDrawRules::new();
        let result = rules.validate(&ValidateContext::Generic("As Kh Qd".to_string()));
        assert!(!result.unwrap());
    }

    #[test]
    fn test_draw_probability() {
        assert!(FiveCardDrawRules::calculate_draw_probability(1).contains("20%"));
        assert!(FiveCardDrawRules::calculate_draw_probability(5).contains("50%"));
    }

    #[test]
    fn test_draw_explain() {
        let rules = FiveCardDrawRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("丢弃最多5张"));
        assert!(explanation.contains("策略建议"));
    }
}
