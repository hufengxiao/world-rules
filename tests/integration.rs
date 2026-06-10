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

// ===== 斗地主牌型识别集成测试 =====

#[test]
fn ddz_all_patterns_recognized() {
    use world_rules::rules::games::doudizhu::{recognize_pattern, CardPattern, DdzCard, DdzSuit};

    let c = |rank: u8| DdzCard::new(rank, DdzSuit::Spade);

    // 单张
    let (pat, _) = recognize_pattern(&[c(3)]).unwrap();
    assert_eq!(pat, CardPattern::Single);

    // 对子
    let (pat, _) = recognize_pattern(&[c(5), c(5)]).unwrap();
    assert_eq!(pat, CardPattern::Pair);

    // 三张
    let (pat, _) = recognize_pattern(&[c(7), c(7), c(7)]).unwrap();
    assert_eq!(pat, CardPattern::Triple);

    // 三带一
    let (pat, _) = recognize_pattern(&[c(8), c(8), c(8), c(3)]).unwrap();
    assert_eq!(pat, CardPattern::TripleWithOne);

    // 三带二
    let (pat, _) = recognize_pattern(&[c(9), c(9), c(9), c(4), c(4)]).unwrap();
    assert_eq!(pat, CardPattern::TripleWithPair);

    // 顺子
    let (pat, rank) = recognize_pattern(&[c(3), c(4), c(5), c(6), c(7)]).unwrap();
    assert_eq!(pat, CardPattern::Straight);
    assert_eq!(rank, 7);

    // 连对
    let (pat, _) = recognize_pattern(&[c(3), c(3), c(4), c(4), c(5), c(5)]).unwrap();
    assert_eq!(pat, CardPattern::DoubleStraight);

    // 飞机
    let (pat, _) = recognize_pattern(&[c(3), c(3), c(3), c(4), c(4), c(4)]).unwrap();
    assert_eq!(pat, CardPattern::Plane);

    // 飞机带翅膀
    let (pat, _) = recognize_pattern(&[c(3), c(3), c(3), c(4), c(4), c(4), c(5), c(6)]).unwrap();
    assert_eq!(pat, CardPattern::PlaneWithWings);

    // 炸弹
    let (pat, _) = recognize_pattern(&[c(10), c(10), c(10), c(10)]).unwrap();
    assert_eq!(pat, CardPattern::Bomb);

    // 王炸
    let (pat, _) = recognize_pattern(&[DdzCard::joker_small(), DdzCard::joker_big()]).unwrap();
    assert_eq!(pat, CardPattern::Rocket);
}

#[test]
fn ddz_invalid_patterns_return_none() {
    use world_rules::rules::games::doudizhu::{recognize_pattern, DdzCard, DdzSuit};

    let c = |rank: u8| DdzCard::new(rank, DdzSuit::Spade);

    // 空牌
    assert!(recognize_pattern(&[]).is_none());

    // 两张不同牌 (不是对子)
    assert!(recognize_pattern(&[c(3), c(5)]).is_none());

    // 含2的"顺子"不合法
    assert!(recognize_pattern(&[c(10), c(11), c(12), c(13), c(15)]).is_none());
}

// ===== 扑克牌型比较集成测试 =====

#[test]
fn poker_compare_flush_vs_straight() {
    use world_rules::rules::games::card_games::poker::TexasHoldemRules;
    use world_rules::rules::games::card_games::{Card, Rank, Suit};

    let flush = TexasHoldemRules::evaluate_hand(&[
        Card::new(Suit::Heart, Rank::Ace),
        Card::new(Suit::Heart, Rank::Ten),
        Card::new(Suit::Heart, Rank::Eight),
        Card::new(Suit::Heart, Rank::Six),
        Card::new(Suit::Heart, Rank::Three),
    ]);
    let straight = TexasHoldemRules::evaluate_hand(&[
        Card::new(Suit::Spade, Rank::Nine),
        Card::new(Suit::Heart, Rank::Eight),
        Card::new(Suit::Diamond, Rank::Seven),
        Card::new(Suit::Club, Rank::Six),
        Card::new(Suit::Spade, Rank::Five),
    ]);
    assert!(
        TexasHoldemRules::compare_hands(&flush, &straight) == std::cmp::Ordering::Greater,
        "同花应大于顺子"
    );
}

#[test]
fn poker_compare_same_rank_by_tiebreaker() {
    use world_rules::rules::games::card_games::poker::TexasHoldemRules;
    use world_rules::rules::games::card_games::{Card, Rank, Suit};

    let pair_aces = TexasHoldemRules::evaluate_hand(&[
        Card::new(Suit::Spade, Rank::Ace),
        Card::new(Suit::Heart, Rank::Ace),
        Card::new(Suit::Diamond, Rank::Five),
        Card::new(Suit::Club, Rank::Three),
        Card::new(Suit::Spade, Rank::Two),
    ]);
    let pair_kings = TexasHoldemRules::evaluate_hand(&[
        Card::new(Suit::Spade, Rank::King),
        Card::new(Suit::Heart, Rank::King),
        Card::new(Suit::Diamond, Rank::Queen),
        Card::new(Suit::Club, Rank::Jack),
        Card::new(Suit::Spade, Rank::Ten),
    ]);
    assert!(
        TexasHoldemRules::compare_hands(&pair_aces, &pair_kings) == std::cmp::Ordering::Greater,
        "AA一对应大于KK一对"
    );
}

// ===== 麻将明牌 (吃碰杠) 集成测试 =====

#[test]
fn mahjong_meld_chi() {
    use world_rules::rules::games::mahjong::Meld;

    let chi = Meld::Shunzi(Tile::wan(1), Tile::wan(2), Tile::wan(3));
    assert_eq!(chi.tiles().len(), 3);
    assert!(chi.is_shunzi());
    assert!(!chi.is_kezi());
}

#[test]
fn mahjong_meld_pon() {
    use world_rules::rules::games::mahjong::Meld;

    let pon = Meld::Kezi(Tile::tiao(5));
    assert_eq!(pon.tiles().len(), 3);
    assert!(pon.is_kezi());
}

#[test]
fn mahjong_meld_kan() {
    use world_rules::rules::games::mahjong::Meld;

    let kan = Meld::Gangzi(Tile::tong(7));
    assert_eq!(kan.tiles().len(), 4);
}

#[test]
fn mahjong_hand_with_melds() {
    use world_rules::rules::games::mahjong::Meld;

    let mut hand = Hand::new();
    // 加一个明牌顺子
    hand.add_meld(Meld::Shunzi(Tile::wan(1), Tile::wan(2), Tile::wan(3)));
    // 手中 11 张: 4万×3 5万×2 6万×3 7万×2 8万×1 → 加到14张看能否胡
    for _ in 0..2 {
        hand.add_tile(Tile::wan(4));
        hand.add_tile(Tile::wan(5));
        hand.add_tile(Tile::wan(6));
        hand.add_tile(Tile::wan(7));
    }
    hand.add_tile(Tile::wan(4));
    hand.add_tile(Tile::wan(6));
    // 11张手牌 + 明牌不算入 can_win (can_win 只看 tiles)
    // can_win 检查 self.tiles.len() == 14
    assert!(!hand.can_win(), "11张手牌不应胡");
    assert_eq!(hand.melds().len(), 1);
}

// ===== 中国象棋走子验证 =====

#[test]
fn chinese_chess_move_validation() {
    use world_rules::rules::games::board_games::chinese_chess::{
        ChineseChessRules, Piece, PieceType,
    };

    let rules = ChineseChessRules::new();

    // 车在 (0,0) 可以直线移动到 (0,5)
    let rook = Piece {
        piece_type: PieceType::Rook,
        is_red: true,
        position: (0, 0),
    };
    assert!(rules.is_valid_move(&rook, (0, 5))); // 纵向
    assert!(rules.is_valid_move(&rook, (5, 0))); // 横向
    assert!(!rules.is_valid_move(&rook, (1, 1))); // 斜向不行

    // 马在 (1,0) 走日
    let knight = Piece {
        piece_type: PieceType::Horse,
        is_red: true,
        position: (1, 0),
    };
    assert!(rules.is_valid_move(&knight, (2, 2)));
    assert!(rules.is_valid_move(&knight, (0, 2)));
    assert!(!rules.is_valid_move(&knight, (1, 2))); // 直线不行
}

// ===== 五子棋胜负判定 =====

#[test]
fn gomoku_horizontal_win() {
    use world_rules::rules::games::board_games::gomoku::{GomokuRules, GomokuVariant};

    let rules = GomokuRules::new(GomokuVariant::Standard);
    let mut board = [[None; 15]; 15];

    // 黑子横排5个
    for col in 3..8 {
        board[7][col] = Some(true); // true = 黑
    }

    let result = rules.check_win(&board, (7, 7));
    assert_eq!(result, Some(true), "横排5子应判黑胜");
}

#[test]
fn gomoku_no_win() {
    use world_rules::rules::games::board_games::gomoku::{GomokuRules, GomokuVariant};

    let rules = GomokuRules::new(GomokuVariant::Standard);
    let mut board = [[None; 15]; 15];

    // 只有4个
    for col in 3..7 {
        board[7][col] = Some(true);
    }

    let result = rules.check_win(&board, (7, 6));
    assert_eq!(result, None, "4子不应判胜");
}
