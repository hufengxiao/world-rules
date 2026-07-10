// 麻将规则性能基准测试
// 测试核心麻将算法的性能表现

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use world_rules::rules::core::{Rule, ValidateContext};
use world_rules::rules::games::mahjong::{Hand, Tile};
use world_rules::rules::games::{RiichiMahjongRules, SichuanMahjongRules};

/// 基准测试：胡牌检测性能
/// 测试标准胡牌算法在不同手牌情况下的性能
fn bench_mahjong_can_win(c: &mut Criterion) {
    let mut group = c.benchmark_group("mahjong_can_win");

    // 测试顺子胡牌
    let mut hand_straight = Hand::new();
    for tile in [
        Tile::wan(1),
        Tile::wan(2),
        Tile::wan(3),
        Tile::wan(4),
        Tile::wan(5),
        Tile::wan(6),
        Tile::wan(7),
        Tile::wan(8),
        Tile::wan(9),
        Tile::tiao(1),
        Tile::tiao(2),
        Tile::tiao(3),
        Tile::tong(5),
        Tile::tong(5),
    ] {
        hand_straight.add_tile(tile);
    }

    group.bench_function("straight_win", |b| {
        b.iter(|| black_box(hand_straight.can_win()))
    });

    // 测试对对胡
    let mut hand_pairs = Hand::new();
    for tile in [
        Tile::wan(1),
        Tile::wan(1),
        Tile::wan(1),
        Tile::wan(5),
        Tile::wan(5),
        Tile::wan(5),
        Tile::tiao(3),
        Tile::tiao(3),
        Tile::tiao(3),
        Tile::tong(7),
        Tile::tong(7),
        Tile::tong(7),
        Tile::wan(9),
        Tile::wan(9),
    ] {
        hand_pairs.add_tile(tile);
    }

    group.bench_function("pairs_win", |b| b.iter(|| black_box(hand_pairs.can_win())));

    // 测试七对子
    let mut hand_seven_pairs = Hand::new();
    for tile in [
        Tile::wan(1),
        Tile::wan(1),
        Tile::wan(3),
        Tile::wan(3),
        Tile::tiao(5),
        Tile::tiao(5),
        Tile::tiao(7),
        Tile::tiao(7),
        Tile::tong(2),
        Tile::tong(2),
        Tile::tong(4),
        Tile::tong(4),
        Tile::wan(9),
        Tile::wan(9),
    ] {
        hand_seven_pairs.add_tile(tile);
    }

    group.bench_function("seven_pairs_win", |b| {
        b.iter(|| black_box(hand_seven_pairs.can_win()))
    });

    group.finish();
}

/// 基准测试：规则验证性能
/// 测试不同麻将变体的规则验证性能
fn bench_mahjong_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("mahjong_validate");

    let sichuan_rules = SichuanMahjongRules::new();
    let riichi_rules = RiichiMahjongRules::new();

    // 使用正确的 ValidateContext API
    let test_hand = ValidateContext::mahjong_tiles("1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 2条 3条 4条 4条");

    group.bench_function("sichuan_validate", |b| {
        b.iter(|| black_box(sichuan_rules.validate(&test_hand)))
    });

    group.bench_function("riichi_validate", |b| {
        b.iter(|| black_box(riichi_rules.validate(&test_hand)))
    });

    group.finish();
}

/// 基准测试：手牌操作性能
/// 测试添加牌和移除牌的性能
fn bench_mahjong_hand_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("mahjong_hand_ops");

    // 测试添加牌
    group.bench_function("add_tile", |b| {
        b.iter(|| {
            let mut hand = Hand::new();
            for i in 1..=9 {
                hand.add_tile(Tile::wan(i));
            }
            black_box(hand)
        })
    });

    // 测试创建新手牌（替代 clear）
    let hand_template = Hand::from_tiles(vec![
        Tile::wan(1), Tile::wan(2), Tile::wan(3), Tile::wan(4), Tile::wan(5),
        Tile::wan(6), Tile::wan(7), Tile::wan(8), Tile::wan(9), Tile::tiao(1),
        Tile::tiao(2), Tile::tiao(3), Tile::tong(5), Tile::tong(5),
    ]);

    group.bench_function("recreate_hand", |b| {
        b.iter(|| {
            let h = hand_template.clone();
            black_box(h)
        })
    });

    group.finish();
}

/// 基准测试：听牌检测性能
/// 测试判断是否听牌的算法性能
fn bench_mahjong_waiting_tiles(c: &mut Criterion) {
    // 创建一个听牌的手牌
    let mut hand_waiting = Hand::new();
    for tile in [
        Tile::wan(1),
        Tile::wan(2),
        Tile::wan(3),
        Tile::wan(4),
        Tile::wan(5),
        Tile::wan(6),
        Tile::wan(7),
        Tile::wan(8),
        Tile::wan(9),
        Tile::tiao(1),
        Tile::tiao(2),
        Tile::tiao(3),
        Tile::tong(5),
    ] {
        hand_waiting.add_tile(tile);
    }

    c.bench_function("detect_waiting", |b| {
        b.iter(|| black_box(hand_waiting.can_win()))
    });
}

criterion_group!(
    benches,
    bench_mahjong_can_win,
    bench_mahjong_validate,
    bench_mahjong_hand_operations,
    bench_mahjong_waiting_tiles,
);
criterion_main!(benches);