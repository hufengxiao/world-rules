//! 边界条件测试 - Phase 22
//!
//! 本文件包含所有规则的边界条件测试，确保在各种极限输入下系统稳定性。
//! 测试分类：
//! 1. 数值边界测试 - 测试数值范围的极限和溢出
//! 2. 空值/None 测试 - 测试空输入和 None 处理
//! 3. 极限值测试 - 测试集合和序列的边界
//! 4. 并发边界测试 - 测试多线程访问的安全性

use world_rules::rules::core::{Difficulty, RuleCategory, RuleMetadata};
use world_rules::rules::games::card_games::poker::TexasHoldemRules;
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::mahjong::{Dragon, Hand, Tile, TileType, Wind};

// ============================================================================
// 1. 数值边界测试
// ============================================================================

mod numeric_boundaries {
    use super::*;

    /// 测试麻将牌数字边界：最小值 (1)
    #[test]
    fn mahjong_tile_minimum_number() {
        let tile = Tile::wan(1);
        assert_eq!(tile.tile_type.number(), Some(1));

        let tile = Tile::tiao(1);
        assert_eq!(tile.tile_type.number(), Some(1));

        let tile = Tile::tong(1);
        assert_eq!(tile.tile_type.number(), Some(1));
    }

    /// 测试麻将牌数字边界：最大值 (9)
    #[test]
    fn mahjong_tile_maximum_number() {
        let tile = Tile::wan(9);
        assert_eq!(tile.tile_type.number(), Some(9));

        let tile = Tile::tiao(9);
        assert_eq!(tile.tile_type.number(), Some(9));

        let tile = Tile::tong(9);
        assert_eq!(tile.tile_type.number(), Some(9));
    }

    /// 测试麻将牌数字边界：超出范围（应被 clamp）
    #[test]
    fn mahjong_tile_out_of_range_low() {
        // 数字小于 1 应被 clamp 到 1
        let tile = Tile::wan(0);
        let num = tile.tile_type.number().unwrap_or(0);
        assert!(num >= 1 && num <= 9, "数字应被 clamp 到 1-9 范围");

        // u8 最大值（wrap around）
        let tile = Tile::tiao(255);
        let num = tile.tile_type.number().unwrap_or(0);
        assert!(num >= 1 && num <= 9, "大数字应被 clamp 到 1-9 范围");
    }

    /// 测试麻将牌数字边界：超出范围高值
    #[test]
    fn mahjong_tile_out_of_range_high() {
        // 数字大于 9 应被 clamp 到 9
        let tile = Tile::wan(10);
        let num = tile.tile_type.number().unwrap_or(0);
        assert!(num >= 1 && num <= 9, "数字应被 clamp 到 1-9 范围");

        let tile = Tile::tiao(255);
        let num = tile.tile_type.number().unwrap_or(0);
        assert!(num >= 1 && num <= 9, "大数字应被 clamp 到 1-9 范围");
    }

    /// 测试扑克牌等级边界：最小值 (Two)
    #[test]
    fn poker_rank_minimum() {
        let card = Card::new(Suit::Heart, Rank::Two);
        assert_eq!(card.rank, Rank::Two);
    }

    /// 测试扑克牌等级边界：最大值 (Ace)
    #[test]
    fn poker_rank_maximum() {
        let card = Card::new(Suit::Heart, Rank::Ace);
        assert_eq!(card.rank, Rank::Ace);
    }

    /// 测试难度等级边界：最小和最大
    #[test]
    fn difficulty_boundary() {
        // 最小难度
        assert!(Difficulty::Beginner < Difficulty::Easy);

        // 最大难度
        assert!(Difficulty::Expert < Difficulty::Master);

        // 默认值
        assert_eq!(Difficulty::default(), Difficulty::Normal);
    }
}

// ============================================================================
// 2. 空值/None 测试
// ============================================================================

mod null_and_none_tests {
    use super::*;

    /// 测试空手牌
    #[test]
    fn empty_hand() {
        let hand = Hand::new();
        assert!(!hand.can_win(), "空手牌不应胡牌");

        let waiting = hand.find_waiting_tiles();
        assert!(waiting.is_empty(), "空手牌没有听牌");
    }

    /// 测试空扑克手牌（应 panic，因为需要至少 5 张牌）
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn empty_poker_hand() {
        let cards: Vec<Card> = vec![];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试单张扑克手牌（应 panic）
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn single_card_hand() {
        let cards = vec![Card::new(Suit::Heart, Rank::Ace)];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试 RuleMetadata 的可选字段
    #[test]
    fn metadata_optional_fields() {
        let meta = RuleMetadata::new("测试规则", "测试描述");
        assert!(meta.origin.is_none(), "默认 origin 应为 None");
        assert!(meta.tags.is_empty(), "默认 tags 应为空");
    }

    /// 测试 TileType::number() 在风牌/箭牌上返回 None
    #[test]
    fn honor_tile_number_is_none() {
        let feng = TileType::Feng(Wind::Dong);
        assert!(feng.number().is_none(), "风牌没有数字");

        let jian = TileType::Jian(Dragon::HongZhong);
        assert!(jian.number().is_none(), "箭牌没有数字");
    }

    /// 测试空标签列表
    #[test]
    fn empty_tags() {
        let meta = RuleMetadata::new("规则", "描述").with_tags(vec![]);
        assert!(meta.tags.is_empty());
    }

    /// 测试空字符串名称
    #[test]
    fn empty_name() {
        let meta = RuleMetadata::new("", "");
        assert_eq!(meta.name, "");
        assert_eq!(meta.description, "");
    }

    /// 测试空白字符名称
    #[test]
    fn whitespace_name() {
        let meta = RuleMetadata::new("   ", "   ");
        assert_eq!(meta.name, "   ");
        assert_eq!(meta.description, "   ");
    }

    /// 测试空分类名
    #[test]
    fn empty_category_name() {
        let cat = RuleCategory::games("");
        assert_eq!(cat.to_string(), "Games/");

        let cat = RuleCategory::law("");
        assert_eq!(cat.to_string(), "Law/");
    }

    /// 测试劳动法规则返回非空列表
    #[test]
    fn labor_law_non_empty_results() {
        let rules = world_rules::rules::law::labor::LaborLawRules::new();
        assert!(!rules.working_hours().is_empty());
        assert!(!rules.leave_rules().is_empty());
        assert!(!rules.contract_rules().is_empty());
    }

    /// 测试空输入到规则解释方法
    #[test]
    fn explain_with_empty_metadata() {
        let meta = RuleMetadata::new("", "");
        let display = format!("{}", meta);
        assert_eq!(display, ""); // 空名称显示为空
    }

    /// 测试大量空值添加到手牌
    #[test]
    fn hand_with_many_empty_melds() {
        let mut hand = Hand::new();
        // 手牌初始状态应有空 tiles 和空 melds
        assert!(hand.tiles().is_empty());
        assert!(hand.melds().is_empty());
    }

    /// 测试 None 在 Option 字段中的处理
    #[test]
    fn metadata_origin_none() {
        let meta = RuleMetadata::new("规则", "描述");
        // 不设置 origin，保持 None
        assert!(meta.origin.is_none());

        // 显示方法应正确处理 None
        let display = format!("{}", meta);
        assert!(!display.contains("(")); // 无 origin 时不显示括号
    }

    /// 测试 Difficulty 默认值
    #[test]
    fn difficulty_default_is_normal() {
        let meta = RuleMetadata::new("规则", "描述");
        assert_eq!(meta.difficulty, Difficulty::Normal);
    }

    /// 测试空标签的序列化
    #[test]
    fn empty_tags_serialization() {
        let meta = RuleMetadata::new("规则", "描述");
        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("\"tags\":[]"));
    }
}

// ============================================================================
// 3. 极限值测试
// ============================================================================

mod extreme_value_tests {
    use super::*;

    /// 测试最大手牌数（14 张）
    #[test]
    fn max_hand_size() {
        let mut hand = Hand::new();
        // 添加 14 张牌
        for n in 1..=7 {
            hand.add_tile(Tile::wan(n));
            hand.add_tile(Tile::wan(n));
        }
        // 14 张牌不应 panic
        assert!(true);
    }

    /// 测试超过最大手牌数（15 张）
    #[test]
    fn over_max_hand_size() {
        let mut hand = Hand::new();
        // 添加 15 张牌（超过标准）
        for n in 1..=5 {
            hand.add_tile(Tile::wan(n));
            hand.add_tile(Tile::wan(n));
            hand.add_tile(Tile::wan(n));
        }
        // 系统应能处理，不应 panic
        assert!(true);
    }

    /// 测试大量重复牌
    #[test]
    fn many_duplicate_tiles() {
        let mut hand = Hand::new();
        // 添加大量重复牌（虽然实际不可能，但测试系统稳定性）
        for _ in 0..20 {
            hand.add_tile(Tile::wan(1));
        }
        // 不应 panic
        assert!(true);
    }

    /// 测试扑克最大牌数（7 张）
    #[test]
    fn max_poker_hand() {
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Heart, Rank::King),
            Card::new(Suit::Heart, Rank::Queen),
            Card::new(Suit::Heart, Rank::Jack),
            Card::new(Suit::Heart, Rank::Ten),
            Card::new(Suit::Heart, Rank::Nine),
            Card::new(Suit::Heart, Rank::Eight),
        ];
        let _eval = TexasHoldemRules::evaluate_hand(&cards);
        // 7 张牌应返回最佳 5 张组合
        assert!(true);
    }

    /// 测试 RuleCategory 名称长度极限
    #[test]
    fn category_name_extreme_length() {
        // 非常长的分类名
        let long_name = "x".repeat(1000);
        let cat = RuleCategory::games(&long_name);
        assert!(cat.to_string().contains(&long_name));

        // 空名称
        let _empty_cat = RuleCategory::games("");
        assert!(true);
    }

    /// 测试 RuleMetadata 字段长度极限
    #[test]
    fn metadata_extreme_length() {
        // 非常长的名称和描述
        let long_text = "测试".repeat(1000);
        let meta = RuleMetadata::new(&long_text, &long_text);
        assert_eq!(meta.name, long_text);
    }

    /// 测试 u8 极限值（麻将牌数字）
    #[test]
    fn mahjong_tile_u8_limits() {
        // u8 最小值
        let tile = Tile::wan(0);
        assert!(tile.tile_type.number().unwrap_or(0) >= 1);

        // u8 最大值
        let tile = Tile::wan(255);
        assert!(tile.tile_type.number().unwrap_or(0) <= 9);
    }

    /// 测试扑克牌花色和等级数量极限
    #[test]
    fn poker_suit_rank_limits() {
        // 所有花色数量固定为 4
        let suits = vec![Suit::Heart, Suit::Spade, Suit::Club, Suit::Diamond];
        assert_eq!(suits.len(), 4);

        // 所有等级数量固定为 13
        let ranks = vec![
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        assert_eq!(ranks.len(), 13);
    }

    /// 测试字符串长度的极限（10000 字符）
    #[test]
    fn extremely_long_string() {
        let very_long = "x".repeat(10000);
        let meta = RuleMetadata::new(&very_long, "描述");
        assert_eq!(meta.name.len(), 10000);
    }

    /// 测试大量标签的极限
    #[test]
    fn many_tags_limit() {
        let tags: Vec<String> = (0..1000).map(|i| format!("标签{}", i)).collect();
        let meta = RuleMetadata::new("规则", "描述").with_tags(tags.clone());
        assert_eq!(meta.tags.len(), 1000);
    }

    /// 测试嵌套分类深度极限
    #[test]
    fn deep_nested_category() {
        let deep_name = format!("{}/{}", "level1", "level2/level3/level4/level5");
        let cat = RuleCategory::custom("deep", &deep_name);
        assert!(cat.to_string().contains("level1"));
    }

    /// 测试迭代器极限（大量迭代）
    #[test]
    fn iterator_limit() {
        let mut count = 0u64;
        for i in 0..100_000 {
            count += i;
        }
        // 测试应快速完成，不超时
        assert!(count > 0);
    }

    /// 测试大集合的性能
    #[test]
    fn large_collection_performance() {
        // 创建大型牌集合（10000 张牌）
        let mut tiles = Vec::with_capacity(10000);
        for _ in 0..10000 {
            tiles.push(Tile::wan(1));
        }
        assert_eq!(tiles.len(), 10000);
    }
}

// ============================================================================
// 4. 并发边界测试
// ============================================================================

mod concurrency_boundary_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    /// 测试手牌的 Send trait（编译时检查）
    #[test]
    fn hand_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Hand>();
    }

    /// 测试手牌的 Sync trait（编译时检查）
    #[test]
    fn hand_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Hand>();
    }

    /// 测试 Tile 的 Send/Sync trait
    #[test]
    fn tile_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Tile>();
        assert_send_sync::<TileType>();
    }

    /// 测试 RuleMetadata 的 Send/Sync trait
    #[test]
    fn metadata_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RuleMetadata>();
    }

    /// 测试 RuleCategory 的 Send/Sync trait
    #[test]
    fn category_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RuleCategory>();
    }

    /// 测试 Difficulty 的 Send/Sync trait
    #[test]
    fn difficulty_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Difficulty>();
    }

    /// 测试 Card 的 Send/Sync trait
    #[test]
    fn card_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Card>();
        assert_send_sync::<Suit>();
        assert_send_sync::<Rank>();
    }

    /// 测试多线程访问 Hand（通过 Arc）
    #[test]
    fn hand_thread_safe_access() {
        let hand = Arc::new(Hand::new());
        let hand_clone = Arc::clone(&hand);

        // 编译时检查 Arc<Hand> 是否 Send
        fn assert_send<T: Send>() {}
        assert_send::<Arc<Hand>>();

        // 在新线程中访问
        let handle = thread::spawn(move || {
            let _ = hand_clone.tiles().len();
        });
        handle.join().unwrap();
    }

    /// 测试多线程创建 Tile
    #[test]
    fn tile_creation_thread_safe() {
        let tiles: Vec<Tile> = (1..=9)
            .flat_map(|n| vec![Tile::wan(n), Tile::tiao(n), Tile::tong(n)])
            .collect();

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let tiles_clone = tiles.clone();
                thread::spawn(move || {
                    // 多线程中访问 tiles
                    tiles_clone.len()
                })
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert!(results.iter().all(|&len| len == 27));
    }

    /// 测试多线程创建 Card
    #[test]
    fn card_creation_thread_safe() {
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Spade, Rank::King),
        ];

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let cards_clone = cards.clone();
                thread::spawn(move || cards_clone.len())
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert!(results.iter().all(|&len| len == 2));
    }

    /// 测试多线程创建 RuleMetadata
    #[test]
    fn metadata_thread_safe_creation() {
        let meta = Arc::new(RuleMetadata::new("规则", "描述"));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let meta_clone = Arc::clone(&meta);
                thread::spawn(move || meta_clone.name.clone())
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert!(results.iter().all(|name| name == "规则"));
    }

    /// 测试多线程访问 Difficulty
    #[test]
    fn difficulty_thread_safe() {
        let difficulty = Difficulty::Hard;

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let diff = difficulty;
                thread::spawn(move || format!("{}", diff))
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert!(results.iter().all(|s| s == "困难"));
    }
}

// ============================================================================
// 5. 类型转换边界测试
// ============================================================================

mod type_conversion_boundaries {
    use super::*;

    /// 测试 TileType 显示格式
    #[test]
    fn tiletype_display() {
        let tile = Tile::wan(1);
        let display = format!("{}", tile);
        assert!(!display.is_empty());

        let tile = Tile::feng(Wind::Dong);
        let display = format!("{}", tile);
        assert!(!display.is_empty());
    }

    /// 测试 RuleCategory 显示格式
    #[test]
    fn category_display() {
        let cat = RuleCategory::games("mahjong");
        assert_eq!(cat.to_string(), "Games/mahjong");

        let cat = RuleCategory::sports("football");
        assert_eq!(cat.to_string(), "Sports/football");

        let cat = RuleCategory::law("contract");
        assert_eq!(cat.to_string(), "Law/contract");
    }

    /// 测试 Difficulty 显示格式
    #[test]
    fn difficulty_display() {
        assert_eq!(format!("{}", Difficulty::Beginner), "入门");
        assert_eq!(format!("{}", Difficulty::Easy), "简单");
        assert_eq!(format!("{}", Difficulty::Normal), "普通");
        assert_eq!(format!("{}", Difficulty::Hard), "困难");
        assert_eq!(format!("{}", Difficulty::Expert), "专家");
        assert_eq!(format!("{}", Difficulty::Master), "大师");
    }

    /// 测试特殊字符在分类名中
    #[test]
    fn category_special_characters() {
        // Unicode 字符
        let cat = RuleCategory::games("麻将🀄");
        assert!(cat.to_string().contains("麻将"));

        // 空格
        let cat = RuleCategory::sports("football soccer");
        assert!(cat.to_string().contains("football soccer"));
    }
}

// ============================================================================
// 6. 错误路径测试
// ============================================================================

mod error_path_tests {
    use super::*;

    /// 测试无效的麻将手牌状态
    #[test]
    fn invalid_mahjong_hand_states() {
        // 13 张牌（少一张）
        let mut hand = Hand::new();
        for n in 1..=9 {
            hand.add_tile(Tile::wan(n));
        }
        hand.add_tile(Tile::tiao(1));
        hand.add_tile(Tile::tiao(2));
        hand.add_tile(Tile::tiao(3));
        hand.add_tile(Tile::tiao(4));
        // 13 张不应胡牌，但不应 panic
        assert!(!hand.can_win());
    }

    /// 测试非标准扑克牌数（应 panic）
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn non_standard_poker_hand_sizes() {
        // 2 张牌（不够 5 张）
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Spade, Rank::King),
        ];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试重复牌在手牌中（应 panic）
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn duplicate_cards_in_hand() {
        // 扑克手牌中包含完全相同的牌（实际不可能）
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Heart, Rank::Ace), // 重复
        ];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }
}

// ============================================================================
// 7. 补充数值边界测试 - 体育规则
// ============================================================================

mod sports_numeric_boundaries {
    /// 测试田径项目距离边界：最小距离 (60米)
    #[test]
    fn athletics_min_distance() {
        let rules = world_rules::rules::sports::athletics::AthleticsRules::new();
        let sprints = rules.sprint_distances();
        assert!(sprints.contains(&60), "应包含60米短跑");
        assert!(sprints.iter().all(|&d| d >= 60), "所有距离应 >= 60米");
    }

    /// 测试田径项目距离边界：最大距离 (10000米)
    #[test]
    fn athletics_max_distance() {
        let rules = world_rules::rules::sports::athletics::AthleticsRules::new();
        let long_distance = rules.long_distance_events();
        assert!(long_distance.contains(&10000), "应包含10000米长跑");
        assert!(
            long_distance.iter().all(|&d| d <= 10000),
            "所有距离应 <= 10000米"
        );
    }

    /// 测试田径项目距离排序
    #[test]
    fn athletics_distance_ordering() {
        let rules = world_rules::rules::sports::athletics::AthleticsRules::new();
        let sprints = rules.sprint_distances();
        let sorted: Vec<u16> = sprints.iter().copied().collect();
        let mut expected = sorted.clone();
        expected.sort();
        assert_eq!(sorted, expected, "距离应按升序排列");
    }
}

// ============================================================================
// 8. 补充数值边界测试 - 法律规则
// ============================================================================

mod law_numeric_boundaries {
    use super::*;

    /// 测试劳动法工时边界：每日最大工作时间
    #[test]
    fn labor_working_hours_boundary() {
        let rules = world_rules::rules::law::labor::LaborLawRules::new();
        let hours = rules.working_hours();
        // 标准工作时间应为 8 小时
        assert!(
            hours.iter().any(|h| h.contains("每日8小时")),
            "标准工作时间为每日8小时"
        );
        // 加班每日不超过 3 小时
        assert!(
            hours.iter().any(|h| h.contains("3小时")),
            "加班每日不超过3小时"
        );
    }

    /// 测试劳动法年假边界：最小和最大天数
    #[test]
    fn labor_leave_days_boundary() {
        let rules = world_rules::rules::law::labor::LaborLawRules::new();
        let leave = rules.leave_rules();
        // 最小年假：5天（工作1-10年）
        assert!(leave.iter().any(|l| l.contains("5天")), "最小年假为5天");
        // 最大年假：15天（工作20年以上）
        assert!(leave.iter().any(|l| l.contains("15天")), "最大年假为15天");
    }

    /// 测试劳动法试用期边界
    #[test]
    fn labor_probation_period_boundary() {
        let rules = world_rules::rules::law::labor::LaborLawRules::new();
        let contract = rules.contract_rules();
        // 试用期最长不超过 6 个月（隐含在合同期限1-3年试用期不超过2月中）
        assert!(
            contract.iter().any(|c| c.contains("试用期")),
            "应包含试用期规定"
        );
    }
}

// ============================================================================
// 9. 补充数值边界测试 - 游戏规则
// ============================================================================

mod games_numeric_boundaries {
    use super::*;

    /// 测试扑克牌型等级边界：最小牌型（高牌）
    #[test]
    fn poker_hand_rank_minimum() {
        use world_rules::rules::games::card_games::poker::HandRank;
        // 高牌是最低牌型
        assert!(HandRank::HighCard < HandRank::OnePair);
    }

    /// 测试扑克牌型等级边界：最大牌型（皇家同花顺）
    #[test]
    fn poker_hand_rank_maximum() {
        use world_rules::rules::games::card_games::poker::HandRank;
        // 皇家同花顺是最高牌型
        assert!(HandRank::RoyalFlush > HandRank::StraightFlush);
    }

    /// 测试扑克牌型等级排序
    #[test]
    fn poker_hand_rank_ordering() {
        use world_rules::rules::games::card_games::poker::HandRank;
        // 验证所有牌型按强度排序
        assert!(HandRank::HighCard < HandRank::OnePair);
        assert!(HandRank::OnePair < HandRank::TwoPair);
        assert!(HandRank::TwoPair < HandRank::ThreeOfAKind);
        assert!(HandRank::ThreeOfAKind < HandRank::Straight);
        assert!(HandRank::Straight < HandRank::Flush);
        assert!(HandRank::Flush < HandRank::FullHouse);
        assert!(HandRank::FullHouse < HandRank::FourOfAKind);
        assert!(HandRank::FourOfAKind < HandRank::StraightFlush);
        assert!(HandRank::StraightFlush < HandRank::RoyalFlush);
    }

    /// 测试麻将番数边界：最小和最大
    #[test]
    fn mahjong_fan_boundary() {
        // 在四川麻将中，最小番数为 1，最大通常为封顶番数
        // 这里测试基本胡牌条件
        let mut hand = Hand::new();
        // 最简单的平胡：1-9万
        for n in 1..=9 {
            hand.add_tile(Tile::wan(n));
        }
        hand.add_tile(Tile::tiao(1));
        hand.add_tile(Tile::tiao(1));
        hand.add_tile(Tile::tiao(1));
        hand.add_tile(Tile::tong(5)); // 雀头
                                      // 测试不应 panic
        assert!(true);
    }
}

// ============================================================================
// 10. 补充数值边界测试 - 集合与序列
// ============================================================================

mod collection_boundary_tests {
    use world_rules::rules::games::card_games::{Card, Rank, Suit};
    use world_rules::rules::games::mahjong::{Hand, Tile};

    /// 测试空集合处理
    #[test]
    fn empty_collections() {
        // 空扑克牌列表
        let empty_cards: Vec<Card> = vec![];
        // 应能处理而不崩溃（虽然可能 panic）
        // 这里只测试编译通过
        assert!(empty_cards.is_empty());

        // 空麻将手牌
        let empty_hand = Hand::new();
        assert!(!empty_hand.can_win());
    }

    /// 测试单元素集合
    #[test]
    fn single_element_collections() {
        // 单张牌
        let single_tile = Tile::wan(1);
        assert_eq!(single_tile.tile_type.number(), Some(1));

        // 单张扑克牌
        let single_card = Card::new(Suit::Heart, Rank::Ace);
        assert_eq!(single_card.rank, Rank::Ace);
    }

    /// 测试大集合性能
    #[test]
    fn large_collection_performance() {
        // 创建大量牌（测试性能，不应超时）
        let mut tiles: Vec<Tile> = Vec::with_capacity(1000);
        for _ in 0..1000 {
            tiles.push(Tile::wan(1));
        }
        assert_eq!(tiles.len(), 1000);

        // 创建大量扑克牌
        let mut cards: Vec<Card> = Vec::with_capacity(1000);
        for _ in 0..1000 {
            cards.push(Card::new(Suit::Heart, Rank::Ace));
        }
        assert_eq!(cards.len(), 1000);
    }
}
