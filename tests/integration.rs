//! 集成测试 - 测试库 API 的实际行为

use world_rules::prelude::*;
use world_rules::rules::games::card_games::poker::TexasHoldemRules;
use world_rules::rules::games::mahjong::{Dragon, Hand, Tile, Wind};

// ===== 麻将胡牌算法测试 =====

#[test]
fn mahjong_standard_win() {
    let mut hand = Hand::new();
    // 1万 1万 1万  2万 2万 2万  3万 3万 3万  4万 4万  5万 5万 5万
    for _ in 0..3 {
        hand.add_tile(Tile::wan(1));
        hand.add_tile(Tile::wan(2));
        hand.add_tile(Tile::wan(3));
    }
    for _ in 0..2 {
        hand.add_tile(Tile::wan(4));
    }
    for _ in 0..3 {
        hand.add_tile(Tile::wan(5));
    }
    assert!(hand.can_win(), "标准胡牌应通过");
}

#[test]
fn mahjong_seven_pairs() {
    let mut hand = Hand::new();
    // 七对子: 1万×2 2万×2 3万×2 4万×2 5万×2 6万×2 7万×2
    for n in 1..=7 {
        hand.add_tile(Tile::wan(n));
        hand.add_tile(Tile::wan(n));
    }
    assert!(hand.can_win(), "七对子应通过");
}

#[test]
fn mahjong_not_win_13_tiles() {
    let mut hand = Hand::new();
    for n in 1..=9 {
        hand.add_tile(Tile::wan(n));
    }
    hand.add_tile(Tile::tiao(1));
    hand.add_tile(Tile::tiao(2));
    hand.add_tile(Tile::tiao(3));
    assert!(!hand.can_win(), "13张牌不应胡牌");
}

#[test]
fn mahjong_empty_hand_not_win() {
    let hand = Hand::new();
    assert!(!hand.can_win(), "空手牌不应胡牌");
}

#[test]
fn mahjong_waiting_tiles() {
    let mut hand = Hand::new();
    // 1万 1万 1万  2万 2万 2万  3万 3万 3万  4万 4万  5万 5万  (13张，听 4万)
    for _ in 0..3 {
        hand.add_tile(Tile::wan(1));
        hand.add_tile(Tile::wan(2));
        hand.add_tile(Tile::wan(3));
    }
    hand.add_tile(Tile::wan(4));
    hand.add_tile(Tile::wan(4));
    for _ in 0..2 {
        hand.add_tile(Tile::wan(5));
    }
    let waiting = hand.find_waiting_tiles();
    assert!(!waiting.is_empty(), "13张应有听牌");
}

#[test]
fn mahjong_feng_tiles() {
    let mut hand = Hand::new();
    // 东×2 南×2 西×2 北×2 中×2 发×2 白×2
    for _ in 0..2 {
        hand.add_tile(Tile::feng(Wind::Dong));
        hand.add_tile(Tile::feng(Wind::Nan));
        hand.add_tile(Tile::feng(Wind::Xi));
        hand.add_tile(Tile::feng(Wind::Bei));
        hand.add_tile(Tile::jian(Dragon::HongZhong));
        hand.add_tile(Tile::jian(Dragon::FaCai));
        hand.add_tile(Tile::jian(Dragon::BaiBan));
    }
    assert!(hand.can_win(), "字牌七对子应通过");
}

// ===== 德州扑克牌型评估测试 =====

#[test]
fn poker_royal_flush() {
    use world_rules::rules::games::card_games::{Card, Rank, Suit};
    let cards = vec![
        Card::new(Suit::Spade, Rank::Ace),
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Spade, Rank::Queen),
        Card::new(Suit::Spade, Rank::Jack),
        Card::new(Suit::Spade, Rank::Ten),
    ];
    let eval = TexasHoldemRules::evaluate_hand(&cards);
    assert_eq!(eval.rank.name(), "皇家同花顺");
}

#[test]
fn poker_four_of_a_kind() {
    use world_rules::rules::games::card_games::{Card, Rank, Suit};
    let cards = vec![
        Card::new(Suit::Spade, Rank::Ace),
        Card::new(Suit::Heart, Rank::Ace),
        Card::new(Suit::Diamond, Rank::Ace),
        Card::new(Suit::Club, Rank::Ace),
        Card::new(Suit::Spade, Rank::King),
    ];
    let eval = TexasHoldemRules::evaluate_hand(&cards);
    assert_eq!(eval.rank.name(), "四条");
}

#[test]
fn poker_full_house() {
    use world_rules::rules::games::card_games::{Card, Rank, Suit};
    let cards = vec![
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Diamond, Rank::King),
        Card::new(Suit::Spade, Rank::Queen),
        Card::new(Suit::Heart, Rank::Queen),
    ];
    let eval = TexasHoldemRules::evaluate_hand(&cards);
    assert_eq!(eval.rank.name(), "满堂红");
}

#[test]
fn poker_flush() {
    use world_rules::rules::games::card_games::{Card, Rank, Suit};
    let cards = vec![
        Card::new(Suit::Heart, Rank::Ace),
        Card::new(Suit::Heart, Rank::Ten),
        Card::new(Suit::Heart, Rank::Eight),
        Card::new(Suit::Heart, Rank::Six),
        Card::new(Suit::Heart, Rank::Three),
    ];
    let eval = TexasHoldemRules::evaluate_hand(&cards);
    assert_eq!(eval.rank.name(), "同花");
}

#[test]
fn poker_straight() {
    use world_rules::rules::games::card_games::{Card, Rank, Suit};
    let cards = vec![
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Heart, Rank::Eight),
        Card::new(Suit::Diamond, Rank::Seven),
        Card::new(Suit::Club, Rank::Six),
        Card::new(Suit::Spade, Rank::Five),
    ];
    let eval = TexasHoldemRules::evaluate_hand(&cards);
    assert_eq!(eval.rank.name(), "顺子");
}

#[test]
fn poker_high_card() {
    use world_rules::rules::games::card_games::{Card, Rank, Suit};
    let cards = vec![
        Card::new(Suit::Spade, Rank::Ace),
        Card::new(Suit::Heart, Rank::Ten),
        Card::new(Suit::Diamond, Rank::Eight),
        Card::new(Suit::Club, Rank::Six),
        Card::new(Suit::Spade, Rank::Three),
    ];
    let eval = TexasHoldemRules::evaluate_hand(&cards);
    assert_eq!(eval.rank.name(), "高牌");
}

// ===== 核心 API 测试 =====

#[test]
fn rule_trait_metadata() {
    let rule = SichuanMahjongRules::new();
    assert!(!rule.metadata().name.is_empty());
    assert!(!rule.metadata().description.is_empty());
    assert_eq!(rule.metadata().origin.as_deref(), Some("四川麻将"));
}

#[test]
fn rule_trait_explain_not_empty() {
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(SichuanMahjongRules::new()),
        Box::new(FootballRules::new()),
        Box::new(PhysicsLaws::new()),
        Box::new(NutritionRules::new()),
    ];
    for rule in &rules {
        assert!(
            !rule.explain().is_empty(),
            "{} explain 为空",
            rule.metadata().name
        );
    }
}

#[test]
fn rule_category_consistency() {
    let mahjong = SichuanMahjongRules::new();
    assert!(matches!(mahjong.category(), RuleCategory::Games(_)));

    let football = FootballRules::new();
    assert!(matches!(football.category(), RuleCategory::Sports(_)));
}

#[test]
fn math_pythagorean() {
    let result = MathRules::pythagorean(3.0, 4.0);
    assert!((result - 5.0).abs() < 1e-10);
}

#[test]
fn math_fibonacci() {
    let fib = MathRules::fibonacci(10);
    assert_eq!(fib.len(), 10);
    assert_eq!(fib[0], 1);
    assert_eq!(fib[1], 1);
    assert_eq!(fib[9], 55);
}
