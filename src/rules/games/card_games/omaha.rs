//! 奥马哈扑克规则
//!
//! 奥马哈扑克是德州扑克的变体，玩家获得4张底牌，
//! 必须使用其中2张与公共牌中的3张组成最佳牌型。

use super::cards::{Card, Rank, Suit};
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 奥马哈扑克手牌评估结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OmahaHandEvaluation {
    /// 牌型等级
    pub rank: super::poker::HandRank,
    /// 最佳5张牌组合
    pub cards: Vec<Card>,
    /// 底牌使用的索引（必须恰好2张）
    pub hole_cards_used: Vec<usize>,
    /// 公共牌使用的索引（必须恰好3张）
    pub community_cards_used: Vec<usize>,
}

/// 奥马哈扑克规则
pub struct OmahaRules {
    metadata: RuleMetadata,
}

impl OmahaRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥马哈扑克规则", "Omaha Hold'em 标准规则")
                .with_origin("美国")
                .with_tags(vec!["扑克".into(), "奥马哈".into(), "竞技".into()]),
        }
    }

    /// 评估奥马哈牌型
    ///
    /// 必须从4张底牌中选恰好2张，从公共牌中选恰好3张
    pub fn evaluate_omaha_hand(
        hole_cards: &[Card],
        community_cards: &[Card],
    ) -> Option<OmahaHandEvaluation> {
        if hole_cards.len() != 4 || community_cards.len() < 3 {
            return None;
        }

        let mut best_eval: Option<OmahaHandEvaluation> = None;

        // 尝试所有底牌组合（4选2 = 6种）
        for i in 0..hole_cards.len() {
            for j in (i + 1)..hole_cards.len() {
                let selected_hole = vec![hole_cards[i].clone(), hole_cards[j].clone()];
                let hole_indices = vec![i, j];

                // 尝试所有公共牌组合（至少5选3）
                for c1 in 0..community_cards.len() {
                    for c2 in (c1 + 1)..community_cards.len() {
                        for c3 in (c2 + 1)..community_cards.len() {
                            let selected_community = vec![
                                community_cards[c1].clone(),
                                community_cards[c2].clone(),
                                community_cards[c3].clone(),
                            ];
                            let community_indices = vec![c1, c2, c3];

                            // 组合成5张牌
                            let combined: Vec<Card> =
                                [selected_hole.clone(), selected_community.clone()].concat();

                            // 使用德州扑克评估逻辑
                            let eval = super::poker::TexasHoldemRules::evaluate_hand(&combined);

                            // 比较是否是更好的牌型
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
                                            tiebreaker: vec![], // 奥马哈比较时只看牌型
                                        },
                                    ) == std::cmp::Ordering::Greater
                                }
                            };

                            if should_update {
                                best_eval = Some(OmahaHandEvaluation {
                                    rank: eval.rank,
                                    cards: combined,
                                    hole_cards_used: hole_indices.clone(),
                                    community_cards_used: community_indices,
                                });
                            }
                        }
                    }
                }
            }
        }

        best_eval
    }
}

impl Default for OmahaRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OmahaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("omaha")
    }

    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        let input = match context {
            ValidateContext::PokerCards(s) => s.as_str(),
            ValidateContext::Generic(s) => s.as_str(),
            _ => return Ok(false),
        };

        // 解析输入格式: "底牌:As Kh Qd Jc 公共:10s 9h 8d 7c 6s"
        let parts: Vec<&str> = input.split("公共:").collect();
        if parts.len() != 2 {
            return Ok(false);
        }

        let hole_str = parts[0].trim();
        let hole_str = hole_str.strip_prefix("底牌:").unwrap_or(hole_str);

        // 简单验证: 底牌必须4张，公共牌至少3张
        let hole_count = hole_str.split_whitespace().count();
        let community_count = parts[1].split_whitespace().count();

        Ok(hole_count == 4 && community_count >= 3)
    }

    fn explain(&self) -> String {
        "【奥马哈扑克规则】\n\n\
        每位玩家获得4张底牌，共5张公共牌。\n\
        必须从底牌中选恰好2张，公共牌中选恰好3张组成最佳牌型。\n\n\
        牌型等级与德州扑克相同:\n\
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
        关键区别: 必须使用2张底牌，不能使用少于或多于2张"
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
    fn test_omaha_rules_metadata() {
        let rules = OmahaRules::new();
        assert_eq!(rules.metadata().name, "奥马哈扑克规则");
        assert!(rules.metadata().origin.is_some());
    }

    #[test]
    fn test_omaha_evaluate_valid() {
        let hole = vec![
            card(Suit::Spade, Rank::Ace),
            card(Suit::Heart, Rank::Ace),
            card(Suit::Diamond, Rank::King),
            card(Suit::Club, Rank::King),
        ];
        let community = vec![
            card(Suit::Spade, Rank::King),
            card(Suit::Heart, Rank::Ace),
            card(Suit::Diamond, Rank::Five),
            card(Suit::Club, Rank::Three),
            card(Suit::Spade, Rank::Two),
        ];

        let eval = OmahaRules::evaluate_omaha_hand(&hole, &community);
        assert!(eval.is_some());
        // 应该能组成四条A或四条K
        let result = eval.unwrap();
        assert_eq!(result.hole_cards_used.len(), 2);
        assert_eq!(result.community_cards_used.len(), 3);
    }

    #[test]
    fn test_omaha_invalid_hole_count() {
        let hole = vec![card(Suit::Spade, Rank::Ace)]; // 只有1张底牌
        let community = vec![
            card(Suit::Heart, Rank::King),
            card(Suit::Diamond, Rank::Queen),
            card(Suit::Club, Rank::Jack),
        ];

        let eval = OmahaRules::evaluate_omaha_hand(&hole, &community);
        assert!(eval.is_none());
    }

    #[test]
    fn test_omaha_validate_format() {
        let rules = OmahaRules::new();

        // 正确格式
        let result = rules.validate(&ValidateContext::Generic(
            "底牌:As Kh Qd Jc 公共:10s 9h 8d 7c 6s".to_string(),
        ));
        assert!(result.unwrap());

        // 底牌数量错误
        let result = rules.validate(&ValidateContext::Generic(
            "底牌:As Kh 公共:10s 9h 8d 7c 6s".to_string(),
        ));
        assert!(!result.unwrap());
    }

    #[test]
    fn test_omaha_explain() {
        let rules = OmahaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("必须使用2张底牌"));
        assert!(explanation.contains("4张底牌"));
    }
}
