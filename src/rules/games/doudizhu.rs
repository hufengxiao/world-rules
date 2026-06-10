//! 斗地主规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};
use std::collections::HashMap;

/// 斗地主牌面花色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DdzSuit {
    /// 黑桃
    Spade,
    /// 红心
    Heart,
    /// 方块
    Diamond,
    /// 梅花
    Club,
}

/// 斗地主牌
///
/// 通过 `rank` (点数) 和 `suit` (花色) 表示一张牌。
/// 王牌无花色，rank=16 为小王，rank=17 为大王。
///
/// # 点数映射
/// - 3-10: 对应数字
/// - J=11, Q=12, K=13, A=14, 2=15
/// - 小王=16, 大王=17
///
/// # 示例
/// ```
/// use world_rules::rules::games::doudizhu::{DdzCard, DdzSuit};
///
/// let card = DdzCard::new(3, DdzSuit::Spade);
/// assert_eq!(card.rank, 3);
///
/// let joker = DdzCard::joker_big();
/// assert_eq!(joker.rank, 17);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DdzCard {
    /// 点数 (3=3, 4=4, ..., 10=10, J=11, Q=12, K=13, A=14, 2=15, 小王=16, 大王=17)
    pub rank: u8,
    /// 花色 (王牌无花色)
    pub suit: Option<DdzSuit>,
}

impl DdzCard {
    pub fn new(rank: u8, suit: DdzSuit) -> Self {
        Self {
            rank: rank.clamp(3, 15),
            suit: Some(suit),
        }
    }

    pub fn joker_small() -> Self {
        Self {
            rank: 16,
            suit: None,
        }
    }

    pub fn joker_big() -> Self {
        Self {
            rank: 17,
            suit: None,
        }
    }

    /// 从字符串解析 (如 "3s", "10h", "Jd", "2c", "X"=小王, "D"=大王)
    pub fn parse_card(s: &str) -> Result<Self, String> {
        let s = s.trim();
        match s {
            "X" | "x" | "小王" => Ok(Self::joker_small()),
            "D" | "d" | "大王" => Ok(Self::joker_big()),
            _ => {
                if s.len() < 2 {
                    return Err(format!("无法解析: {}", s));
                }
                let (rank_str, suit_char) = if let Some(rest) = s.strip_prefix("10") {
                    ("10", rest)
                } else {
                    (&s[..s.len() - 1], &s[s.len() - 1..])
                };
                let rank = match rank_str.to_uppercase().as_str() {
                    "3" => 3,
                    "4" => 4,
                    "5" => 5,
                    "6" => 6,
                    "7" => 7,
                    "8" => 8,
                    "9" => 9,
                    "10" => 10,
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    "A" => 14,
                    "2" => 15,
                    _ => return Err(format!("无效点数: {}", rank_str)),
                };
                let suit = match suit_char.to_lowercase().as_str() {
                    "s" | "♠" => DdzSuit::Spade,
                    "h" | "♥" => DdzSuit::Heart,
                    "d" | "♦" => DdzSuit::Diamond,
                    "c" | "♣" => DdzSuit::Club,
                    _ => return Err(format!("无效花色: {}", suit_char)),
                };
                Ok(Self::new(rank, suit))
            }
        }
    }

    /// 解析多张牌 (空格分隔)
    pub fn parse_many(s: &str) -> Result<Vec<Self>, String> {
        s.split_whitespace().map(Self::parse_card).collect()
    }
}

impl std::fmt::Display for DdzCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.rank {
            16 => write!(f, "小王"),
            17 => write!(f, "大王"),
            _ => {
                let rank_str = match self.rank {
                    11 => "J".to_string(),
                    12 => "Q".to_string(),
                    13 => "K".to_string(),
                    14 => "A".to_string(),
                    15 => "2".to_string(),
                    n => n.to_string(),
                };
                let suit_str = match self.suit {
                    Some(DdzSuit::Spade) => "♠",
                    Some(DdzSuit::Heart) => "♥",
                    Some(DdzSuit::Diamond) => "♦",
                    Some(DdzSuit::Club) => "♣",
                    None => "",
                };
                write!(f, "{}{}", rank_str, suit_str)
            }
        }
    }
}

/// 识别牌型，返回 (牌型, 关键牌点数用于比较)
///
/// 关键牌点数: 顺子/连对取最大牌, 飞机取最大三张, 炸弹/王炸取牌面值
pub fn recognize_pattern(cards: &[DdzCard]) -> Option<(CardPattern, u8)> {
    let n = cards.len();
    if n == 0 {
        return None;
    }

    let mut counts: HashMap<u8, u8> = HashMap::new();
    for c in cards {
        *counts.entry(c.rank).or_insert(0) += 1;
    }

    // 王炸: 大小王
    if n == 2 && counts.contains_key(&16) && counts.contains_key(&17) {
        return Some((CardPattern::Rocket, 17));
    }

    let mut by_count: HashMap<u8, Vec<u8>> = HashMap::new();
    for (&rank, &count) in &counts {
        by_count.entry(count).or_default().push(rank);
    }
    for v in by_count.values_mut() {
        v.sort();
    }

    // 炸弹: 4张相同
    if n == 4 {
        if let Some(ranks) = by_count.get(&4) {
            if ranks.len() == 1 {
                return Some((CardPattern::Bomb, ranks[0]));
            }
        }
    }

    // 单张
    if n == 1 {
        return Some((CardPattern::Single, cards[0].rank));
    }

    // 对子
    if n == 2 {
        if let Some(ranks) = by_count.get(&2) {
            if ranks.len() == 1 {
                return Some((CardPattern::Pair, ranks[0]));
            }
        }
    }

    // 三张
    if n == 3 {
        if let Some(ranks) = by_count.get(&3) {
            if ranks.len() == 1 {
                return Some((CardPattern::Triple, ranks[0]));
            }
        }
    }

    // 三带一
    if n == 4 {
        if let Some(ranks) = by_count.get(&3) {
            if ranks.len() == 1 && by_count.get(&1).is_some_and(|v| v.len() == 1) {
                return Some((CardPattern::TripleWithOne, ranks[0]));
            }
        }
    }

    // 三带二
    if n == 5 {
        if let Some(ranks) = by_count.get(&3) {
            if ranks.len() == 1 && by_count.get(&2).is_some_and(|v| v.len() == 1) {
                return Some((CardPattern::TripleWithPair, ranks[0]));
            }
        }
    }

    // 顺子: 5+张连续单牌 (3-A, 不含2和王)
    if n >= 5 && counts.values().all(|&c| c == 1) {
        let mut ranks: Vec<u8> = counts.keys().copied().collect();
        ranks.sort();
        if ranks.iter().all(|&r| (3..=14).contains(&r)) && is_consecutive(&ranks) {
            return Some((CardPattern::Straight, *ranks.last().unwrap()));
        }
    }

    // 连对: 3+对连续对子
    if n >= 6 && n.is_multiple_of(2) && counts.values().all(|&c| c == 2) {
        let mut ranks: Vec<u8> = counts.keys().copied().collect();
        ranks.sort();
        if ranks.len() >= 3
            && ranks.iter().all(|&r| (3..=14).contains(&r))
            && is_consecutive(&ranks)
        {
            return Some((CardPattern::DoubleStraight, *ranks.last().unwrap()));
        }
    }

    // 飞机 (不带翅膀): 2+个连续三张
    if n >= 6 && n.is_multiple_of(3) {
        if let Some(ranks) = by_count.get(&3) {
            if ranks.len() == n / 3
                && ranks.iter().all(|&r| (3..=14).contains(&r))
                && is_consecutive(ranks)
            {
                return Some((CardPattern::Plane, *ranks.last().unwrap()));
            }
        }
    }

    // 飞机带翅膀 (带单牌)
    if n >= 8 {
        if let Some(triple_ranks) = by_count.get(&3) {
            let triple_count = triple_ranks.len();
            if triple_count >= 2
                && triple_ranks.iter().all(|&r| (3..=14).contains(&r))
                && is_consecutive(triple_ranks)
            {
                let remaining = n - triple_count * 3;
                if remaining == triple_count {
                    // 带单牌
                    return Some((CardPattern::PlaneWithWings, *triple_ranks.last().unwrap()));
                }
            }
        }
    }

    // 飞机带翅膀 (带对子)
    if n >= 10 {
        if let Some(triple_ranks) = by_count.get(&3) {
            let triple_count = triple_ranks.len();
            if triple_count >= 2
                && triple_ranks.iter().all(|&r| (3..=14).contains(&r))
                && is_consecutive(triple_ranks)
            {
                let remaining = n - triple_count * 3;
                if remaining == triple_count * 2 {
                    // 检查剩余是否全是对子
                    let pair_count = by_count.get(&2).map_or(0, |v| v.len());
                    if pair_count == triple_count {
                        return Some((CardPattern::PlaneWithWings, *triple_ranks.last().unwrap()));
                    }
                }
            }
        }
    }

    // 四带二 (带2张单牌)
    if n == 6 {
        if let Some(ranks) = by_count.get(&4) {
            if ranks.len() == 1 {
                return Some((CardPattern::FourWithTwo, ranks[0]));
            }
        }
    }

    // 四带二对
    if n == 8 {
        if let Some(ranks) = by_count.get(&4) {
            if ranks.len() == 1 {
                let pair_count = by_count.get(&2).map_or(0, |v| v.len());
                if pair_count == 2 {
                    return Some((CardPattern::FourWithTwo, ranks[0]));
                }
            }
        }
    }

    None
}

fn is_consecutive(sorted_ranks: &[u8]) -> bool {
    for i in 1..sorted_ranks.len() {
        if sorted_ranks[i] != sorted_ranks[i - 1] + 1 {
            return false;
        }
    }
    true
}

/// 判断当前出牌是否能压过上家
///
/// 规则:
/// - 炸弹可以压任何非炸弹牌型
/// - 王炸可以压任何牌型
/// - 同类型牌型比较关键牌点数
pub fn can_beat(current: &[(CardPattern, u8)], previous: &[(CardPattern, u8)]) -> bool {
    if current.len() != 1 || previous.len() != 1 {
        return false;
    }
    let (cur_pat, cur_rank) = &current[0];
    let (prev_pat, prev_rank) = &previous[0];

    // 王炸最大
    if *cur_pat == CardPattern::Rocket {
        return true;
    }

    // 炸弹压非炸弹
    if *cur_pat == CardPattern::Bomb
        && *prev_pat != CardPattern::Bomb
        && *prev_pat != CardPattern::Rocket
    {
        return true;
    }

    // 同类型比较
    if cur_pat == prev_pat {
        return cur_rank > prev_rank;
    }

    // 炸弹对炸弹比较
    if *cur_pat == CardPattern::Bomb && *prev_pat == CardPattern::Bomb {
        return cur_rank > prev_rank;
    }

    false
}

/// 斗地主牌型
///
/// 表示斗地主中所有合法的出牌组合。
/// 优先级从低到高：单张(1) → 王炸(12)。
/// 炸弹可以压任何非炸弹牌型，王炸可以压任何牌型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardPattern {
    /// 单张
    Single,
    /// 对子
    Pair,
    /// 三张
    Triple,
    /// 三带一
    TripleWithOne,
    /// 三带二
    TripleWithPair,
    /// 顺子 (至少5张)
    Straight,
    /// 连对 (至少3对)
    DoubleStraight,
    /// 飞机 (至少2个三张)
    Plane,
    /// 飞机带翅膀
    PlaneWithWings,
    /// 四带二
    FourWithTwo,
    /// 炸弹
    Bomb,
    /// 王炸
    Rocket,
}

impl CardPattern {
    pub fn name(&self) -> &'static str {
        match self {
            CardPattern::Single => "单张",
            CardPattern::Pair => "对子",
            CardPattern::Triple => "三张",
            CardPattern::TripleWithOne => "三带一",
            CardPattern::TripleWithPair => "三带二",
            CardPattern::Straight => "顺子",
            CardPattern::DoubleStraight => "连对",
            CardPattern::Plane => "飞机",
            CardPattern::PlaneWithWings => "飞机带翅膀",
            CardPattern::FourWithTwo => "四带二",
            CardPattern::Bomb => "炸弹",
            CardPattern::Rocket => "王炸",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            CardPattern::Single => 1,
            CardPattern::Pair => 2,
            CardPattern::Triple => 3,
            CardPattern::TripleWithOne => 4,
            CardPattern::TripleWithPair => 5,
            CardPattern::Straight => 6,
            CardPattern::DoubleStraight => 7,
            CardPattern::Plane => 8,
            CardPattern::PlaneWithWings => 9,
            CardPattern::FourWithTwo => 10,
            CardPattern::Bomb => 11,
            CardPattern::Rocket => 12,
        }
    }
}

/// 斗地主规则
pub struct DouDiZhuRules {
    metadata: RuleMetadata,
}

impl DouDiZhuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("斗地主规则", "斗地主标准规则")
                .with_origin("中国")
                .with_tags(vec!["游戏".into(), "扑克".into(), "斗地主".into()]),
        }
    }

    /// 牌数分配
    pub fn card_distribution(&self) -> (u8, u8, u8, u8) {
        (17, 17, 17, 3) // 三人各17张，3张底牌
    }

    /// 底牌数量
    pub fn landlord_cards(&self) -> u8 {
        20 // 地主20张 (17+3底牌)
    }

    /// 农民牌数
    pub fn farmer_cards(&self) -> u8 {
        17
    }

    /// 牌型说明
    pub fn pattern_descriptions(&self) -> Vec<(CardPattern, &'static str)> {
        vec![
            (CardPattern::Single, "任意一张单牌"),
            (CardPattern::Pair, "两张点数相同的牌"),
            (CardPattern::Triple, "三张点数相同的牌"),
            (CardPattern::TripleWithOne, "三张+任意一张单牌"),
            (CardPattern::TripleWithPair, "三张+任意一对"),
            (CardPattern::Straight, "五张或更多连续单牌（不含2和王）"),
            (
                CardPattern::DoubleStraight,
                "三对或更多连续对子（不含2和王）",
            ),
            (CardPattern::Plane, "两个或更多连续三张（不含2和王）"),
            (CardPattern::PlaneWithWings, "飞机+同数量的单牌或对子"),
            (CardPattern::FourWithTwo, "四张+任意两张单牌或两对"),
            (CardPattern::Bomb, "四张点数相同的牌，可打任何非炸弹牌型"),
            (CardPattern::Rocket, "大王+小王，最大的牌型"),
        ]
    }

    /// 牌的大小顺序 (从小到大)
    pub fn card_order(&self) -> Vec<&'static str> {
        vec![
            "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A", "2", "小王", "大王",
        ]
    }

    /// 叫地主规则
    pub fn call_rules(&self) -> Vec<&'static str> {
        vec![
            "每人轮流叫分: 1分、2分、3分",
            "叫3分者直接成为地主",
            "无人叫分则重新发牌",
            "地主获得3张底牌",
        ]
    }

    /// 出牌规则
    pub fn play_rules(&self) -> Vec<&'static str> {
        vec![
            "地主先出牌",
            "按顺序轮流出牌",
            "必须出比上家大的同类型牌，或炸弹",
            "可以选择不出（过牌）",
            "直到有人出完所有牌",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "地主赢: 底分×2（春天×3）",
            "地主输: 底分×2（反春×3）",
            "炸弹翻倍: 每出一个炸弹翻倍",
        ]
    }
}

impl Default for DouDiZhuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DouDiZhuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("doudizhu")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        let cards = match DdzCard::parse_many(context) {
            Ok(c) => c,
            Err(_) => return Ok(false),
        };
        Ok(recognize_pattern(&cards).is_some())
    }

    fn explain(&self) -> String {
        let patterns = self.pattern_descriptions();
        format!(
            "【斗地主规则】\n\n\
            牌数: 地主{}张，农民{}张\n\n\
            叫地主:\n{}\n\n\
            牌型:\n{}\n\n\
            出牌规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.landlord_cards(),
            self.farmer_cards(),
            self.call_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            patterns
                .iter()
                .map(|(p, d)| format!("  • {}: {}", p.name(), d))
                .collect::<Vec<_>>()
                .join("\n"),
            self.play_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_rules()
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

    #[test]
    fn test_doudizhu_rules() {
        let rules = DouDiZhuRules::new();
        assert_eq!(rules.landlord_cards(), 20);
        assert_eq!(rules.farmer_cards(), 17);
    }

    #[test]
    fn test_pattern_priority() {
        assert!(CardPattern::Rocket.priority() > CardPattern::Bomb.priority());
    }

    // ===== 牌型识别测试 =====

    fn c(rank: u8) -> DdzCard {
        DdzCard::new(rank, DdzSuit::Spade)
    }

    #[test]
    fn test_recognize_single() {
        let cards = vec![c(3)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::Single);
        assert_eq!(rank, 3);
    }

    #[test]
    fn test_recognize_pair() {
        let cards = vec![c(5), c(5)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::Pair);
        assert_eq!(rank, 5);
    }

    #[test]
    fn test_recognize_triple() {
        let cards = vec![c(7), c(7), c(7)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::Triple);
        assert_eq!(rank, 7);
    }

    #[test]
    fn test_recognize_triple_with_one() {
        let cards = vec![c(8), c(8), c(8), c(3)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::TripleWithOne);
        assert_eq!(rank, 8);
    }

    #[test]
    fn test_recognize_triple_with_pair() {
        let cards = vec![c(9), c(9), c(9), c(4), c(4)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::TripleWithPair);
        assert_eq!(rank, 9);
    }

    #[test]
    fn test_recognize_straight() {
        // 3-4-5-6-7
        let cards = vec![c(3), c(4), c(5), c(6), c(7)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::Straight);
        assert_eq!(rank, 7);
    }

    #[test]
    fn test_recognize_straight_long() {
        // 3-4-5-6-7-8-9-10-J-Q-K-A (12张)
        let cards: Vec<DdzCard> = (3..=14).map(c).collect();
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::Straight);
        assert_eq!(rank, 14); // A
    }

    #[test]
    fn test_straight_rejects_2() {
        // 含2的"顺子"不合法
        let cards = vec![c(10), c(11), c(12), c(13), c(15)]; // 10-J-Q-K-2
        assert!(recognize_pattern(&cards).is_none());
    }

    #[test]
    fn test_recognize_double_straight() {
        // 33-44-55
        let cards = vec![c(3), c(3), c(4), c(4), c(5), c(5)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::DoubleStraight);
        assert_eq!(rank, 5);
    }

    #[test]
    fn test_recognize_bomb() {
        let cards = vec![c(8), c(8), c(8), c(8)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::Bomb);
        assert_eq!(rank, 8);
    }

    #[test]
    fn test_recognize_rocket() {
        let cards = vec![DdzCard::joker_small(), DdzCard::joker_big()];
        let (pat, _) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::Rocket);
    }

    #[test]
    fn test_recognize_plane() {
        // 333-444
        let cards = vec![c(3), c(3), c(3), c(4), c(4), c(4)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::Plane);
        assert_eq!(rank, 4);
    }

    #[test]
    fn test_recognize_plane_with_wings() {
        // 333-444 + 5-6
        let cards = vec![c(3), c(3), c(3), c(4), c(4), c(4), c(5), c(6)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::PlaneWithWings);
        assert_eq!(rank, 4);
    }

    // ===== can_beat 测试 =====

    #[test]
    fn test_beat_single() {
        let prev = vec![(CardPattern::Single, 5)];
        let cur = vec![(CardPattern::Single, 8)];
        assert!(can_beat(&cur, &prev));
        assert!(!can_beat(&prev, &cur));
    }

    #[test]
    fn test_bomb_beats_straight() {
        let prev = vec![(CardPattern::Straight, 10)];
        let cur = vec![(CardPattern::Bomb, 3)];
        assert!(can_beat(&cur, &prev));
    }

    #[test]
    fn test_rocket_beats_bomb() {
        let prev = vec![(CardPattern::Bomb, 15)]; // 2的炸弹
        let cur = vec![(CardPattern::Rocket, 17)];
        assert!(can_beat(&cur, &prev));
    }

    #[test]
    fn test_cannot_beat_different_type() {
        let prev = vec![(CardPattern::Pair, 8)];
        let cur = vec![(CardPattern::Single, 14)]; // 单张A不能压对子
        assert!(!can_beat(&cur, &prev));
    }

    // ===== DdzCard Display 测试 =====

    #[test]
    fn test_card_display_number() {
        let card = DdzCard::new(3, DdzSuit::Spade);
        assert_eq!(format!("{}", card), "3♠");
    }

    #[test]
    fn test_card_display_face() {
        let card = DdzCard::new(11, DdzSuit::Heart);
        assert_eq!(format!("{}", card), "J♥");
    }

    #[test]
    fn test_card_display_ace() {
        let card = DdzCard::new(14, DdzSuit::Diamond);
        assert_eq!(format!("{}", card), "A♦");
    }

    #[test]
    fn test_card_display_two() {
        let card = DdzCard::new(15, DdzSuit::Club);
        assert_eq!(format!("{}", card), "2♣");
    }

    #[test]
    fn test_card_display_jokers() {
        assert_eq!(format!("{}", DdzCard::joker_small()), "小王");
        assert_eq!(format!("{}", DdzCard::joker_big()), "大王");
    }

    // ===== DdzCard 解析测试 =====

    #[test]
    fn test_parse_card_basic() {
        let card = DdzCard::parse_card("3s").unwrap();
        assert_eq!(card.rank, 3);
        assert_eq!(card.suit, Some(DdzSuit::Spade));
    }

    #[test]
    fn test_parse_card_ten() {
        let card = DdzCard::parse_card("10h").unwrap();
        assert_eq!(card.rank, 10);
        assert_eq!(card.suit, Some(DdzSuit::Heart));
    }

    #[test]
    fn test_parse_card_joker() {
        let small = DdzCard::parse_card("X").unwrap();
        assert_eq!(small.rank, 16);
        let big = DdzCard::parse_card("D").unwrap();
        assert_eq!(big.rank, 17);
    }

    #[test]
    fn test_parse_card_chinese_joker() {
        let small = DdzCard::parse_card("小王").unwrap();
        assert_eq!(small.rank, 16);
        let big = DdzCard::parse_card("大王").unwrap();
        assert_eq!(big.rank, 17);
    }

    #[test]
    fn test_parse_many() {
        let cards = DdzCard::parse_many("3s 3h 3d").unwrap();
        assert_eq!(cards.len(), 3);
        assert!(cards.iter().all(|c| c.rank == 3));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(DdzCard::parse_card("Z").is_err());
        assert!(DdzCard::parse_card("1s").is_err()); // 1不是合法点数
    }

    // ===== validate 通过 Rule trait 测试 =====

    #[test]
    fn test_validate_valid_pattern() {
        use crate::rules::core::Rule as _;
        let rules = DouDiZhuRules::new();
        // 炸弹
        assert!(rules.validate("3s 3h 3d 3c").unwrap());
        // 单张
        assert!(rules.validate("5s").unwrap());
        // 王炸
        assert!(rules.validate("X D").unwrap());
    }

    #[test]
    fn test_validate_invalid_pattern() {
        use crate::rules::core::Rule as _;
        let rules = DouDiZhuRules::new();
        // 两张不同牌
        assert!(!rules.validate("3s 5h").unwrap());
        // 空字符串
        assert!(!rules.validate("").unwrap());
        // 无效输入
        assert!(!rules.validate("abc").unwrap());
    }

    // ===== 更多边界测试 =====

    #[test]
    fn test_recognize_four_with_two_singles() {
        // 四带二 (4张+2张单牌)
        let cards = vec![c(8), c(8), c(8), c(8), c(3), c(5)];
        let (pat, rank) = recognize_pattern(&cards).unwrap();
        assert_eq!(pat, CardPattern::FourWithTwo);
        assert_eq!(rank, 8);
    }

    #[test]
    fn test_straight_must_be_at_least_5() {
        // 4张连续不是顺子
        let cards = vec![c(3), c(4), c(5), c(6)];
        assert!(recognize_pattern(&cards).is_none());
    }

    #[test]
    fn test_double_straight_must_be_at_least_3_pairs() {
        // 2对连续不是连对
        let cards = vec![c(3), c(3), c(4), c(4)];
        assert!(recognize_pattern(&cards).is_none());
    }

    #[test]
    fn test_bomb_priority_order() {
        // 3的炸弹 < 2的炸弹
        let bomb3 = recognize_pattern(&[c(3), c(3), c(3), c(3)]).unwrap();
        let bomb2 = recognize_pattern(&[c(15), c(15), c(15), c(15)]).unwrap();
        assert!(can_beat(
            std::slice::from_ref(&bomb2),
            std::slice::from_ref(&bomb3)
        ));
        assert!(!can_beat(&[bomb3], &[bomb2]));
    }
}
