//! 游戏验证示例 - 展示所有 validate 功能
//!
//! 运行: cargo run --example validate_games

use world_rules::prelude::*;
use world_rules::rules::games::card_games::poker::TexasHoldemRules;
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::doudizhu::{recognize_pattern, DdzCard, DdzSuit};
use world_rules::rules::games::mahjong::{Hand, Tile};

fn main() {
    println!("=== 世界规则库 - 游戏验证示例 ===\n");

    demonstrate_mahjong();
    demonstrate_poker();
    demonstrate_doudizhu();
    demonstrate_chess();
}

fn demonstrate_mahjong() {
    println!("--- 麻将胡牌验证 ---");

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

    println!("手牌: {}张", hand.tiles().len());
    if hand.can_win() {
        println!("✅ 可以胡牌！\n");
    } else {
        println!("❌ 不能胡牌\n");
    }
}

fn demonstrate_poker() {
    println!("--- 德州扑克牌型评估 ---");

    let hands = vec![
        (
            "皇家同花顺",
            vec![
                Card::new(Suit::Spade, Rank::Ace),
                Card::new(Suit::Spade, Rank::King),
                Card::new(Suit::Spade, Rank::Queen),
                Card::new(Suit::Spade, Rank::Jack),
                Card::new(Suit::Spade, Rank::Ten),
            ],
        ),
        (
            "满堂红",
            vec![
                Card::new(Suit::Spade, Rank::King),
                Card::new(Suit::Heart, Rank::King),
                Card::new(Suit::Diamond, Rank::King),
                Card::new(Suit::Spade, Rank::Queen),
                Card::new(Suit::Heart, Rank::Queen),
            ],
        ),
    ];

    for (desc, cards) in hands {
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        println!(
            "{}: {} ({})",
            desc,
            eval.rank.name(),
            eval.rank.english_name()
        );
    }
    println!();
}

fn demonstrate_doudizhu() {
    println!("--- 斗地主牌型识别 ---");

    let patterns = vec![
        ("单张", vec![DdzCard::new(3, DdzSuit::Spade)]),
        (
            "对子",
            vec![
                DdzCard::new(5, DdzSuit::Spade),
                DdzCard::new(5, DdzSuit::Heart),
            ],
        ),
        (
            "炸弹",
            vec![
                DdzCard::new(8, DdzSuit::Spade),
                DdzCard::new(8, DdzSuit::Heart),
                DdzCard::new(8, DdzSuit::Diamond),
                DdzCard::new(8, DdzSuit::Club),
            ],
        ),
        ("王炸", vec![DdzCard::joker_small(), DdzCard::joker_big()]),
    ];

    for (desc, cards) in patterns {
        match recognize_pattern(&cards) {
            Some((pat, _)) => println!("{}: {} (优先级{})", desc, pat.name(), pat.priority()),
            None => println!("{}: 无法识别", desc),
        }
    }
    println!();
}

fn demonstrate_chess() {
    println!("--- 中国象棋走法验证 ---");

    let moves = vec![
        ("车 0,0 0,5", "车纵向移动"),
        ("车 0,0 1,1", "车斜向移动"),
        ("马 1,0 2,2", "马走日"),
        ("马 1,0 1,2", "马走直线"),
    ];

    for (move_str, desc) in moves {
        let rules = ChineseChessRules::new();
        let valid = rules.validate(move_str).unwrap_or(false);
        println!("{}: {} {}", desc, move_str, if valid { "✅" } else { "❌" });
    }
    println!();
}
