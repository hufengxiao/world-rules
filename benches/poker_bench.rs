// 扑克牌型评估性能基准测试
// 测试德州扑克牌型描述和验证性能

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use world_rules::rules::core::{Rule, ValidateContext};
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::{PokerOmahaRules, TexasHoldemRules};

/// 基准测试：牌创建性能
/// 测试创建扑克牌的性能
fn bench_poker_card_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("poker_card_creation");

    // 测试创建单个牌
    group.bench_function("create_single_card", |b| {
        b.iter(|| black_box(Card::new(Suit::Heart, Rank::Ace)))
    });

    // 测试创建一组牌
    group.bench_function("create_hand", |b| {
        b.iter(|| {
            let hand = vec![
                Card::new(Suit::Heart, Rank::Ace),
                Card::new(Suit::Heart, Rank::King),
                Card::new(Suit::Heart, Rank::Queen),
                Card::new(Suit::Heart, Rank::Jack),
                Card::new(Suit::Heart, Rank::Ten),
            ];
            black_box(hand)
        })
    });

    group.finish();
}

/// 基准测试：规则验证性能
/// 测试德州扑克和奥马哈的规则验证性能
fn bench_poker_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("poker_validate");

    let texas_rules = TexasHoldemRules::new();
    let omaha_rules = PokerOmahaRules::new();

    // 使用正确的 ValidateContext API
    let test_hand = ValidateContext::poker_cards("Ah Kh Qh Jh 10h");

    group.bench_function("texas_holdem_validate", |b| {
        b.iter(|| black_box(texas_rules.validate(&test_hand)))
    });

    group.bench_function("omaha_validate", |b| {
        b.iter(|| black_box(omaha_rules.validate(&test_hand)))
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

/// 基准测试：牌型规则说明性能
/// 测试获取牌型排名说明的性能
fn bench_poker_hand_rankings(c: &mut Criterion) {
    let texas_rules = TexasHoldemRules::new();

    c.bench_function("hand_rankings", |b| {
        b.iter(|| black_box(texas_rules.hand_rankings()))
    });
}

/// 基准测试：下注行动说明性能
/// 测试获取下注行动说明的性能
fn bench_poker_betting_actions(c: &mut Criterion) {
    let texas_rules = TexasHoldemRules::new();

    c.bench_function("betting_actions", |b| {
        b.iter(|| black_box(texas_rules.betting_actions()))
    });
}

/// 基准测试：手牌排序性能
/// 测试对一手牌进行排序的性能
fn bench_poker_sort_hand(c: &mut Criterion) {
    let hand = vec![
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
    bench_poker_card_creation,
    bench_poker_validate,
    bench_poker_card_compare,
    bench_poker_hand_rankings,
    bench_poker_betting_actions,
    bench_poker_sort_hand,
);
criterion_main!(benches);