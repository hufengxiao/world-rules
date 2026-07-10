//! 七张牌 Stud 扑克规则
//!
//! Seven Card Stud 是经典的扑克变体，玩家获得7张牌，
//! 从中选取最佳5张组成牌型。

use super::cards::{Card, Rank, Suit};
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 七张牌 Stud 游戏阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudPhase {
    /// 第三街 - 每人3张牌（2暗1明）
    ThirdStreet,
    /// 第四街 - 第4张牌（明牌）
    FourthStreet,
    /// 第五街 - 第5张牌（明牌）
    FifthStreet,
    /// 第六街 - 第6张牌（明牌）
    SixthStreet,
    /// 第七街 - 第7张牌（暗牌）
    SeventhStreet,
}

/// 七张牌 Stud 手牌评估
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StudHandEvaluation {
    /// 牌型等级
    pub rank: super::poker::HandRank,
    /// 最佳5张牌组合
    pub cards: Vec<Card>,
    /// 原始7张牌中的索引
    pub indices: Vec<usize>,
}

/// 七张牌 Stud 规则
pub struct SevenCardStudRules {
    metadata: RuleMetadata,
}

impl SevenCardStudRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("七张牌 Stud 规则", "Seven Card Stud 经典规则")
                .with_origin("美国")
                .with_tags(vec!["扑克".into(), "Stud".into(), "经典".into()]),
        }
    }

    /// 从7张牌中找出最佳5张牌组合
    pub fn evaluate_stud_hand(cards: &[Card]) -> Option<StudHandEvaluation> {
        if cards.len() < 5 {
            return None;
        }

        if cards.len() <= 5 {
            // 直接评估这5张或更少的牌
            let eval = super::poker::TexasHoldemRules::evaluate_hand(cards);
            return Some(StudHandEvaluation {
                rank: eval.rank,
                cards: eval.cards.clone(),
                indices: (0..cards.len()).collect(),
            });
        }

        // 7张牌时，尝试所有C(7,5)=21种组合
        let mut best_eval: Option<StudHandEvaluation> = None;

        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                for k in (j + 1)..cards.len() {
                    for l in (k + 1)..cards.len() {
                        for m in (l + 1)..cards.len() {
                            let selected = vec![
                                cards[i].clone(),
                                cards[j].clone(),
                                cards[k].clone(),
                                cards[l].clone(),
                                cards[m].clone(),
                            ];

                            let eval = super::poker::TexasHoldemRules::evaluate_hand(&selected);

                            let should_update = match &best_eval {
                                None => true,
                                Some(current) => {
                                    super::poker::TexasHoldemRules::compare_hands(
                                        &super::poker::HandEvaluation {
                                            rank: eval.rank,
                                            cards: eval.cards.clone(),
                                            tiebreaker: eval.tiebreaker.clone(),
                                        },
                                        &super::poker::HandEvaluation {
                                            rank: current.rank,
                                            cards: current.cards.clone(),
                                            tiebreaker: vec![],
                                        },
                                    ) == std::cmp::Ordering::Greater
                                }
                            };

                            if should_update {
                                best_eval = Some(StudHandEvaluation {
                                    rank: eval.rank,
                                    cards: selected,
                                    indices: vec![i, j, k, l, m],
                                });
                            }
                        }
                    }
                }
            }
        }

        best_eval
    }

    /// 根据明牌评估起始手牌强度
    pub fn evaluate_starting_hand(up_cards: &[Card]) -> String {
        if up_cards.is_empty() {
            return "未知强度".to_string();
        }

        // 统计明牌中的对子和同花情况
        let mut rank_counts: std::collections::HashMap<Rank, u8> = std::collections::HashMap::new();
        let mut suit_counts: std::collections::HashMap<Suit, u8> = std::collections::HashMap::new();

        for card in up_cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
            *suit_counts.entry(card.suit).or_insert(0) += 1;
        }

        // 检查是否有高牌明牌
        let has_high_card = up_cards
            .iter()
            .any(|c| matches!(c.rank, Rank::Ace | Rank::King | Rank::Queen | Rank::Jack));

        // 检查明牌对子
        let has_pair = rank_counts.values().any(|&c| c >= 2);

        // 检查同花潜力
        let has_flush_draw = suit_counts.values().any(|&c| c >= 3);

        if has_pair && has_high_card {
            "强牌 - 高牌对子"
        } else if has_pair {
            "中等 - 有对子"
        } else if has_flush_draw {
            "中等 - 同花潜力"
        } else if has_high_card {
            "弱牌 - 仅高牌"
        } else {
            "弱牌 - 无优势"
        }
        .to_string()
    }
}

impl Default for SevenCardStudRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SevenCardStudRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("stud")
    }

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        let input = match context {
            ValidateContext::PokerCards(s) => s.as_str(),
            ValidateContext::Generic(s) => s.as_str(),
            _ => return Ok(false),
        };

        // 验证牌面格式和数量
        let parts: Vec<&str> = input.split_whitespace().collect();

        // Stud 需要5-7张牌
        Ok(parts.len() >= 5 && parts.len() <= 7)
    }

    fn explain(&self) -> String {
        "【七张牌 Stud 规则】\n\n\
        经典扑克变体，无公共牌，每位玩家获得7张牌。\n\
        从7张牌中选取最佳5张组成牌型。\n\n\
        发牌流程:\n\
        1. 第三街 (Third Street): 2张暗牌 + 1张明牌\n\
        2. 第四街 (Fourth Street): 第4张明牌\n\
        3. 第五街 (Fifth Street): 第5张明牌\n\
        4. 第六街 (Sixth Street): 第6张明牌\n\
        5. 第七街 (Seventh Street): 第7张暗牌\n\n\
        牌型等级与德州扑克相同:\n\
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
        特殊规则:\n\
        - 明牌对子触发双倍下注\n\
        - 最低明牌开始行动"
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
    fn test_stud_rules_metadata() {
        let rules = SevenCardStudRules::new();
        assert_eq!(rules.metadata().name, "七张牌 Stud 规则");
    }

    #[test]
    fn test_stud_evaluate_7_cards() {
        let cards = vec![
            card(Suit::Spade, Rank::Ace),
            card(Suit::Spade, Rank::King),
            card(Suit::Spade, Rank::Queen),
            card(Suit::Spade, Rank::Jack),
            card(Suit::Spade, Rank::Ten),
            card(Suit::Heart, Rank::Two),
            card(Suit::Diamond, Rank::Three),
        ];

        let eval = SevenCardStudRules::evaluate_stud_hand(&cards);
        assert!(eval.is_some());
        let result = eval.unwrap();
        assert_eq!(result.rank, super::poker::HandRank::RoyalFlush);
        assert_eq!(result.cards.len(), 5);
    }

    #[test]
    fn test_stud_evaluate_5_cards() {
        let cards = vec![
            card(Suit::Heart, Rank::Ace),
            card(Suit::Heart, Rank::King),
            card(Suit::Heart, Rank::Queen),
            card(Suit::Heart, Rank::Jack),
            card(Suit::Heart, Rank::Ten),
        ];

        let eval = SevenCardStudRules::evaluate_stud_hand(&cards);
        assert!(eval.is_some());
        assert_eq!(eval.unwrap().rank, super::poker::HandRank::RoyalFlush);
    }

    #[test]
    fn test_stud_evaluate_insufficient_cards() {
        let cards = vec![card(Suit::Spade, Rank::Ace)];
        let eval = SevenCardStudRules::evaluate_stud_hand(&cards);
        assert!(eval.is_none());
    }

    #[test]
    fn test_stud_starting_hand_high_pair() {
        let up_cards = vec![card(Suit::Spade, Rank::Ace), card(Suit::Heart, Rank::Ace)];
        let strength = SevenCardStudRules::evaluate_starting_hand(&up_cards);
        assert!(strength.contains("强牌"));
    }

    #[test]
    fn test_stud_starting_hand_pair() {
        let up_cards = vec![
            card(Suit::Spade, Rank::Seven),
            card(Suit::Heart, Rank::Seven),
        ];
        let strength = SevenCardStudRules::evaluate_starting_hand(&up_cards);
        assert!(strength.contains("中等"));
    }

    #[test]
    fn test_stud_validate_correct() {
        let rules = SevenCardStudRules::new();
        let result = rules.validate(&ValidateContext::Generic(
            "As Kh Qd Jc 10s 9h 8d".to_string(),
        ));
        assert!(result.unwrap());
    }

    #[test]
    fn test_stud_validate_wrong_count() {
        let rules = SevenCardStudRules::new();
        // 太少
        let result = rules.validate(&ValidateContext::Generic("As Kh Qd".to_string()));
        assert!(!result.unwrap());
        // 太多
        let result = rules.validate(&ValidateContext::Generic(
            "As Kh Qd Jc 10s 9h 8d 7c 6s".to_string(),
        ));
        assert!(!result.unwrap());
    }

    #[test]
    fn test_stud_explain() {
        let rules = SevenCardStudRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("第三街"));
        assert!(explanation.contains("第七街"));
        assert!(explanation.contains("7张牌"));
    }
}
