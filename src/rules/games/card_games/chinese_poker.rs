//! 中式扑克规则
//!
//! Chinese Poker (十三张/拼牌) 是流行的扑克变体，
//! 玩家需要将13张牌分成3墩牌（前3中5后5），
//! 每墩必须符合特定牌力要求。

use super::cards::{Card, Rank, Suit};
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 中式扑克牌墩配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChinesePokerHand {
    /// 前墩 (3张牌，必须≤中墩)
    pub front: Vec<Card>,
    /// 中墩 (5张牌，必须≤后墩)
    pub middle: Vec<Card>,
    /// 后墩 (5张牌，最强)
    pub back: Vec<Card>,
    /// 是否合法配置
    pub valid: bool,
}

/// 中式扑克规则
pub struct ChinesePokerRules {
    metadata: RuleMetadata,
}

impl ChinesePokerRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中式扑克规则", "Chinese Poker 十三张拼牌规则")
                .with_origin("中国")
                .with_tags(vec!["扑克".into(), "中式".into(), "拼牌".into()]),
        }
    }

    /// 评估13张牌并尝试分配到3墩
    pub fn arrange_cards(cards: &[Card]) -> Option<ChinesePokerHand> {
        if cards.len() != 13 {
            return None;
        }

        // 尝试找到合法的牌墩分配
        // 使用贪心策略：后墩最强，中墩次之，前墩最弱
        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort_by_key(|c| std::cmp::Reverse(c.value()));

        // 尝试从最强牌中组成后墩
        let back_candidates = Self::find_best_5_cards(&sorted_cards);

        if let Some((back, remaining)) = back_candidates {
            // 从剩余牌中组成中墩
            let middle_candidates = Self::find_best_5_cards(&remaining);

            if let Some((middle, front)) = middle_candidates {
                // 检查牌力顺序是否合法
                let valid = Self::validate_hand_order(&front, &middle, &back);

                return Some(ChinesePokerHand {
                    front,
                    middle,
                    back,
                    valid,
                });
            }
        }

        None
    }

    /// 从牌堆中找出最佳5张牌组合
    fn find_best_5_cards(cards: &[Card]) -> Option<(Vec<Card>, Vec<Card>)> {
        if cards.len() < 5 {
            return None;
        }

        let mut best_combo: Option<Vec<Card>> = None;
        let mut best_rank: Option<super::poker::HandRank> = None;

        // 尝试所有C(n,5)组合
        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                for k in (j + 1)..cards.len() {
                    for l in (k + 1)..cards.len() {
                        for m in (l + 1)..cards.len() {
                            let combo = vec![
                                cards[i].clone(),
                                cards[j].clone(),
                                cards[k].clone(),
                                cards[l].clone(),
                                cards[m].clone(),
                            ];

                            let eval = super::poker::TexasHoldemRules::evaluate_hand(&combo);
                            let rank = eval.rank;

                            let should_update = match best_rank {
                                None => true,
                                Some(current) => rank > current,
                            };

                            if should_update {
                                best_combo = Some(combo.clone());
                                best_rank = Some(rank);
                            }
                        }
                    }
                }
            }
        }

        best_combo.map(|combo| {
            let remaining: Vec<Card> = cards
                .iter()
                .filter(|c| !combo.contains(c))
                .cloned()
                .collect();
            (combo, remaining)
        })
    }

    /// 评估3张牌的牌力（简化版）
    fn evaluate_3_cards(cards: &[Card]) -> u8 {
        if cards.len() != 3 {
            return 0;
        }

        // 检查三条
        if cards[0].rank == cards[1].rank && cards[1].rank == cards[2].rank {
            return 6; // 三条（最强）
        }

        // 检查对子
        if cards[0].rank == cards[1].rank || cards[1].rank == cards[2].rank {
            return 2; // 对子
        }

        // 高牌
        let max_value = cards.iter().map(|c| c.value()).max().unwrap_or(0);
        if max_value >= 14 {
            return 1; // 有 Ace
        }

        0
    }

    /// 验证牌墩顺序是否合法（前≤中≤后）
    fn validate_hand_order(front: &[Card], middle: &[Card], back: &[Card]) -> bool {
        let front_strength = Self::evaluate_3_cards(front);
        let middle_strength = super::poker::TexasHoldemRules::evaluate_hand(middle).rank as u8;
        let back_strength = super::poker::TexasHoldemRules::evaluate_hand(back).rank as u8;

        front_strength <= middle_strength && middle_strength <= back_strength
    }

    /// 检查是否为特殊牌型
    pub fn check_special_hands(cards: &[Card]) -> Option<String> {
        if cards.len() != 13 {
            return None;
        }

        // 检查十三水（全部13张同花）
        let suits: Vec<Suit> = cards.iter().map(|c| c.suit).collect();
        if suits.iter().all(|s| *s == suits[0]) {
            return Some("十三水".to_string());
        }

        // 检查一条龙（A-K连续13张）
        let values: std::collections::HashSet<u8> = cards.iter().map(|c| c.value()).collect();
        if (2..=14).all(|v| values.contains(&v)) {
            return Some("一条龙".to_string());
        }

        // 检查六对半（至少6个对子）
        let mut rank_counts: std::collections::HashMap<Rank, u8> = std::collections::HashMap::new();
        for card in cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }
        let pair_count = rank_counts.values().filter(|&c| *c >= 2).count();
        if pair_count >= 6 {
            return Some("六对半".to_string());
        }

        None
    }

    /// 计算得分（与对手比较）
    pub fn compare_hands(hand1: &ChinesePokerHand, hand2: &ChinesePokerHand) -> i32 {
        if !hand1.valid || !hand2.valid {
            // 不合法配置自动输3墩
            return if hand1.valid { 3 } else { -3 };
        }

        let mut score = 0;

        // 前墩比较
        let front1 = Self::evaluate_3_cards(&hand1.front);
        let front2 = Self::evaluate_3_cards(&hand2.front);
        score += if front1 > front2 {
            1
        } else if front1 < front2 {
            -1
        } else {
            0
        };

        // 中墩比较
        let mid1 = super::poker::TexasHoldemRules::evaluate_hand(&hand1.middle);
        let mid2 = super::poker::TexasHoldemRules::evaluate_hand(&hand2.middle);
        score += match super::poker::TexasHoldemRules::compare_hands(&mid1, &mid2) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        };

        // 后墩比较
        let back1 = super::poker::TexasHoldemRules::evaluate_hand(&hand1.back);
        let back2 = super::poker::TexasHoldemRules::evaluate_hand(&hand2.back);
        score += match super::poker::TexasHoldemRules::compare_hands(&back1, &back2) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        };

        score
    }
}

impl Default for ChinesePokerRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChinesePokerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chinese_poker")
    }

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        let input = match context {
            ValidateContext::PokerCards(s) => s.as_str(),
            ValidateContext::Generic(s) => s.as_str(),
            _ => return Ok(false),
        };

        // Chinese Poker 必须恰好13张牌
        let parts: Vec<&str> = input.split_whitespace().collect();
        Ok(parts.len() == 13)
    }

    fn explain(&self) -> String {
        "【中式扑克规则】\n\n\
        Chinese Poker (十三张/拼牌) 是将13张牌分成3墩的游戏。\n\
        玩家需将牌分成前(3张)、中(5张)、后(5张)三墩。\n\n\
        牌力要求:\n\
        - 后墩 ≥ 中墩 ≥ 前墩（必须遵守）\n\
        - 前墩: 最多三条，无顺子/同花\n\
        - 中墩/后墩: 标准扑克牌型\n\n\
        特殊牌型:\n\
        - 十三水: 13张同花色，自动赢3墩×3倍\n\
        - 一条龙: A-K连续13张，自动赢3墩×3倍\n\
        - 六对半: 至少6个对子，自动赢3墩×2倍\n\n\
        计分规则:\n\
        - 每墩赢1分，输1分\n\
        - 全赢(3墩)额外+3分\n\
        - 全输(3墩)额外-3分"
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
    fn test_chinese_poker_metadata() {
        let rules = ChinesePokerRules::new();
        assert_eq!(rules.metadata().name, "中式扑克规则");
        assert_eq!(rules.metadata().origin, Some("中国".to_string()));
    }

    #[test]
    fn test_chinese_poker_arrange_13_cards() {
        // 创建一个合法的13张牌配置
        let cards = vec![
            // 后墩候选（同花）
            card(Suit::Spade, Rank::Ace),
            card(Suit::Spade, Rank::King),
            card(Suit::Spade, Rank::Queen),
            card(Suit::Spade, Rank::Jack),
            card(Suit::Spade, Rank::Ten),
            // 中墩候选（满堂红）
            card(Suit::Heart, Rank::Nine),
            card(Suit::Diamond, Rank::Nine),
            card(Suit::Club, Rank::Nine),
            card(Suit::Heart, Rank::Two),
            card(Suit::Diamond, Rank::Two),
            // 前墩候选（对子）
            card(Suit::Club, Rank::Seven),
            card(Suit::Heart, Rank::Seven),
            card(Suit::Diamond, Rank::Three),
        ];

        let hand = ChinesePokerRules::arrange_cards(&cards);
        assert!(hand.is_some());
        let result = hand.unwrap();
        assert_eq!(result.front.len(), 3);
        assert_eq!(result.middle.len(), 5);
        assert_eq!(result.back.len(), 5);
    }

    #[test]
    fn test_chinese_poker_invalid_count() {
        let cards = vec![card(Suit::Spade, Rank::Ace), card(Suit::Heart, Rank::King)];
        let hand = ChinesePokerRules::arrange_cards(&cards);
        assert!(hand.is_none());
    }

    #[test]
    fn test_chinese_poker_special_thirteen_water() {
        // 13张同花色
        let cards: Vec<Card> = (2..=14)
            .map(|v| {
                let rank = match v {
                    14 => Rank::Ace,
                    13 => Rank::King,
                    12 => Rank::Queen,
                    11 => Rank::Jack,
                    10 => Rank::Ten,
                    9 => Rank::Nine,
                    8 => Rank::Eight,
                    7 => Rank::Seven,
                    6 => Rank::Six,
                    5 => Rank::Five,
                    4 => Rank::Four,
                    3 => Rank::Three,
                    2 => Rank::Two,
                    _ => Rank::Two,
                };
                card(Suit::Spade, rank)
            })
            .collect();

        let special = ChinesePokerRules::check_special_hands(&cards);
        assert!(special.is_some());
        assert_eq!(special.unwrap(), "十三水");
    }

    #[test]
    fn test_chinese_poker_special_dragon() {
        // A-K连续
        let mut cards = Vec::new();
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
        for (i, rank) in ranks.iter().enumerate() {
            let suit = match i % 4 {
                0 => Suit::Spade,
                1 => Suit::Heart,
                2 => Suit::Diamond,
                3 => Suit::Club,
                _ => Suit::Spade,
            };
            cards.push(card(suit, *rank));
        }

        let special = ChinesePokerRules::check_special_hands(&cards);
        assert!(special.is_some());
        assert_eq!(special.unwrap(), "一条龙");
    }

    #[test]
    fn test_chinese_poker_evaluate_3_cards() {
        // 三条
        let triple = vec![
            card(Suit::Spade, Rank::Seven),
            card(Suit::Heart, Rank::Seven),
            card(Suit::Diamond, Rank::Seven),
        ];
        assert_eq!(ChinesePokerRules::evaluate_3_cards(&triple), 6);

        // 对子
        let pair = vec![
            card(Suit::Spade, Rank::Seven),
            card(Suit::Heart, Rank::Seven),
            card(Suit::Diamond, Rank::King),
        ];
        assert_eq!(ChinesePokerRules::evaluate_3_cards(&pair), 2);
    }

    #[test]
    fn test_chinese_poker_validate() {
        let rules = ChinesePokerRules::new();

        // 正确数量
        let input = "As Kh Qd Jc 10s 9h 8d 7c 6s 5h 4d 3c 2s";
        let result = rules.validate(&ValidateContext::Generic(input.to_string()));
        assert!(result.unwrap());

        // 数量错误
        let input = "As Kh Qd";
        let result = rules.validate(&ValidateContext::Generic(input.to_string()));
        assert!(!result.unwrap());
    }

    #[test]
    fn test_chinese_poker_compare() {
        let hand1 = ChinesePokerHand {
            front: vec![
                card(Suit::Spade, Rank::Seven),
                card(Suit::Heart, Rank::Seven),
                card(Suit::Diamond, Rank::Three),
            ],
            middle: vec![
                card(Suit::Heart, Rank::Nine),
                card(Suit::Diamond, Rank::Nine),
                card(Suit::Club, Rank::Nine),
                card(Suit::Heart, Rank::Two),
                card(Suit::Diamond, Rank::Two),
            ],
            back: vec![
                card(Suit::Spade, Rank::Ace),
                card(Suit::Spade, Rank::King),
                card(Suit::Spade, Rank::Queen),
                card(Suit::Spade, Rank::Jack),
                card(Suit::Spade, Rank::Ten),
            ],
            valid: true,
        };

        let hand2 = ChinesePokerHand {
            front: vec![
                card(Suit::Club, Rank::Five),
                card(Suit::Heart, Rank::Five),
                card(Suit::Diamond, Rank::Two),
            ],
            middle: vec![
                card(Suit::Club, Rank::Eight),
                card(Suit::Spade, Rank::Eight),
                card(Suit::Heart, Rank::Eight),
                card(Suit::Diamond, Rank::King),
                card(Suit::Club, Rank::King),
            ],
            back: vec![
                card(Suit::Heart, Rank::Ace),
                card(Suit::Diamond, Rank::Ace),
                card(Suit::Club, Rank::Ace),
                card(Suit::Spade, Rank::Ace),
                card(Suit::Heart, Rank::King),
            ],
            valid: true,
        };

        let score = ChinesePokerRules::compare_hands(&hand1, &hand2);
        // 后墩：同花顺 vs 四条，同花顺赢
        // 中墩：满堂红9-2 vs 满堂红8-K，9赢
        // 前墩：对子7 vs 对子5，7赢
        assert!(score > 0); // hand1 应该赢
    }

    #[test]
    fn test_chinese_poker_explain() {
        let rules = ChinesePokerRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("十三水"));
        assert!(explanation.contains("一条龙"));
        assert!(explanation.contains("前墩"));
    }
}
