use criterion::{black_box, criterion_group, criterion_main, Criterion};
use world_rules::prelude::*;
use world_rules::rules::games::card_games::poker::TexasHoldemRules;
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::mahjong::Hand;

fn bench_mahjong_can_win(c: &mut Criterion) {
    let mut hand = Hand::new();
    for tile in [
        world_rules::rules::games::mahjong::Tile::wan(1),
        world_rules::rules::games::mahjong::Tile::wan(2),
        world_rules::rules::games::mahjong::Tile::wan(3),
        world_rules::rules::games::mahjong::Tile::wan(4),
        world_rules::rules::games::mahjong::Tile::wan(5),
        world_rules::rules::games::mahjong::Tile::wan(6),
        world_rules::rules::games::mahjong::Tile::wan(7),
        world_rules::rules::games::mahjong::Tile::wan(8),
        world_rules::rules::games::mahjong::Tile::wan(9),
        world_rules::rules::games::mahjong::Tile::tiao(1),
        world_rules::rules::games::mahjong::Tile::tiao(2),
        world_rules::rules::games::mahjong::Tile::tiao(3),
        world_rules::rules::games::mahjong::Tile::tiao(4),
        world_rules::rules::games::mahjong::Tile::tiao(4),
    ] {
        hand.add_tile(tile);
    }

    c.bench_function("mahjong_can_win", |b| b.iter(|| black_box(hand.can_win())));
}

fn bench_mahjong_validate(c: &mut Criterion) {
    let rules = SichuanMahjongRules::new();
    c.bench_function("mahjong_validate", |b| {
        b.iter(|| {
            black_box(rules.validate("1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 2条 3条 4条 4条"))
        })
    });
}

fn bench_poker_evaluate(c: &mut Criterion) {
    let cards = vec![
        Card::new(Suit::Heart, Rank::Ace),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Heart, Rank::Queen),
        Card::new(Suit::Heart, Rank::Jack),
        Card::new(Suit::Heart, Rank::Ten),
    ];

    c.bench_function("poker_evaluate_royal_flush", |b| {
        b.iter(|| black_box(TexasHoldemRules::evaluate_hand(&cards)))
    });
}

fn bench_poker_validate(c: &mut Criterion) {
    let rules = TexasHoldemRules::new();
    c.bench_function("poker_validate", |b| {
        b.iter(|| black_box(rules.validate("Ah Kh Qh Jh 10h")))
    });
}

fn bench_rule_explain(c: &mut Criterion) {
    let rules = FootballRules::new();
    c.bench_function("rule_explain", |b| b.iter(|| black_box(rules.explain())));
}

criterion_group!(
    benches,
    bench_mahjong_can_win,
    bench_mahjong_validate,
    bench_poker_evaluate,
    bench_poker_validate,
    bench_rule_explain
);
criterion_main!(benches);
