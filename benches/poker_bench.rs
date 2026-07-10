// 扑克牌型评估性能基准测试
// 测试德州扑克、奥马哈等扑克变体的牌型评估性能

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use world_rules::rules::core::Rule;
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::{PokerOmahaRules, TexasHoldemRules};

/// 基准测试：牌型评估性能
/// 测试不同牌型的评估速度
fn bench_poker_evaluate(c: &mut Criterion) {
    let mut group = c.benchmark_group("poker_evaluate");

    // 皇家同花顺
    let royal_flush = vec![
        Card::new(Suit::Heart, Rank::Ace),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Heart, Rank::Queen),
        Card::new(Suit::Heart, Rank::Jack),
        Card::new(Suit::Heart, Rank::Ten),
    ];

    group.bench_function("royal_flush", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&royal_flush)))
    });

    // 同花顺
    let straight_flush = vec![
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Spade, Rank::Eight),
        Card::new(Suit::Spade, Rank::Seven),
        Card::new(Suit::Spade, Rank::Six),
        Card::new(Suit::Spade, Rank::Five),
    ];

    group.bench_function("straight_flush", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&straight_flush)))
    });

    // 四条
    let four_of_a_kind = vec![
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Diamond, Rank::King),
        Card::new(Suit::Club, Rank::King),
        Card::new(Suit::Heart, Rank::Two),
    ];

    group.bench_function("four_of_a_kind", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&four_of_a_kind)))
    });

    // 满堂红
    let full_house = vec![
        Card::new(Suit::Heart, Rank::Queen),
        Card::new(Suit::Spade, Rank::Queen),
        Card::new(Suit::Diamond, Rank::Queen),
        Card::new(Suit::Club, Rank::Jack),
        Card::new(Suit::Heart, Rank::Jack),
    ];

    group.bench_function("full_house", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&full_house)))
    });

    // 同花
    let flush = vec![
        Card::new(Suit::Diamond, Rank::Ace),
        Card::new(Suit::Diamond, Rank::Jack),
        Card::new(Suit::Diamond, Rank::Nine),
        Card::new(Suit::Diamond, Rank::Four),
        Card::new(Suit::Diamond, Rank::Two),
    ];

    group.bench_function("flush", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&flush)))
    });

    // 顺子
    let straight = vec![
        Card::new(Suit::Heart, Rank::Ten),
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Diamond, Rank::Eight),
        Card::new(Suit::Club, Rank::Seven),
        Card::new(Suit::Heart, Rank::Six),
    ];

    group.bench_function("straight", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&straight)))
    });

    // 三条
    let three_of_a_kind = vec![
        Card::new(Suit::Heart, Rank::Seven),
        Card::new(Suit::Spade, Rank::Seven),
        Card::new(Suit::Diamond, Rank::Seven),
        Card::new(Suit::Club, Rank::King),
        Card::new(Suit::Heart, Rank::Queen),
    ];

    group.bench_function("three_of_a_kind", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&three_of_a_kind)))
    });

    // 两对
    let two_pairs = vec![
        Card::new(Suit::Heart, Rank::Jack),
        Card::new(Suit::Spade, Rank::Jack),
        Card::new(Suit::Diamond, Rank::Ten),
        Card::new(Suit::Club, Rank::Ten),
        Card::new(Suit::Heart, Rank::Ace),
    ];

    group.bench_function("two_pairs", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&two_pairs)))
    });

    // 一对
    let one_pair = vec![
        Card::new(Suit::Heart, Rank::Nine),
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Diamond, Rank::Eight),
        Card::new(Suit::Club, Rank::Seven),
        Card::new(Suit::Heart, Rank::Three),
    ];

    group.bench_function("one_pair", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&one_pair)))
    });

    // 高牌
    let high_card = vec![
        Card::new(Suit::Heart, Rank::Ace),
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Diamond, Rank::Queen),
        Card::new(Suit::Club, Rank::Nine),
        Card::new(Suit::Heart, Rank::Four),
    ];

    group.bench_function("high_card", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&high_card)))
    });

    group.finish();
}

/// 基准测试：规则验证性能
/// 测试德州扑克和奥马哈的规则验证性能
fn bench_poker_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("poker_validate");

    let texas_rules = TexasHoldemRules::new();
    let omaha_rules = PokerOmahaRules::new();

    let test_hand = "Ah Kh Qh Jh 10h";

    group.bench_function("texas_holdem_validate", |b| {
        b.iter(|| black_box(texas_rules.validate(test_hand)))
    });

    group.bench_function("omaha_validate", |b| {
        b.iter(|| black_box(omaha_rules.validate(test_hand)))
    });

    group.finish();
}

/// 基准测试：牌比较性能
/// 测试牌的大小比较操作性能
fn bench_poker_card_compare(c: &mut Criterion) {
    let card1 = Card::new(Suit::Heart, Rank::Ace);
    let card2 = Card::new(Suit::Spade, Rank::King);

    c.bench_function("card_compare", |b| {
        b.iter(|| {
            let rank1 = black_box(card1.rank);
            let rank2 = black_box(card2.rank);
            rank1 > rank2
        })
    });
}

/// 基准测试：手牌排序性能
/// 测试对一手牌进行排序的性能
fn bench_poker_sort_hand(c: &mut Criterion) {
    let mut hand = vec![
        Card::new(Suit::Heart, Rank::Seven),
        Card::new(Suit::Spade, Rank::Two),
        Card::new(Suit::Diamond, Rank::King),
        Card::new(Suit::Club, Rank::Five),
        Card::new(Suit::Heart, Rank::Ten),
        Card::new(Suit::Spade, Rank::Ace),
        Card::new(Suit::Diamond, Rank::Three),
    ];

    c.bench_function("sort_hand", |b| {
        b.iter(|| {
            let mut h = hand.clone();
            h.sort_by(|a, b| b.rank.cmp(&a.rank));
            black_box(h)
        })
    });
}

criterion_group!(
    benches,
    bench_poker_evaluate,
    bench_poker_validate,
    bench_poker_card_compare,
    bench_poker_sort_hand,
);
criterion_main!(benches);
