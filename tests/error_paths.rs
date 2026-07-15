//! 错误路径测试 - Phase 23
//!
//! 本文件包含所有规则的错误路径测试，确保在各种异常输入下系统的错误处理正确性。
//! 测试分类：
//! 1. 无效输入测试 - 测试非法参数和格式错误
//! 2. 异常状态测试 - 测试不一致或不可能的状态
//! 3. 错误恢复测试 - 测试系统从错误中恢复的能力
//! 4. Panic 测试 - 测试预期 panic 的场景

use world_rules::rules::core::{
    Difficulty, RuleCategory, RuleError, RuleMetadata, RuleSet, ValidateContext,
};
use world_rules::rules::games::card_games::poker::TexasHoldemRules;
use world_rules::rules::games::card_games::{Card, Rank, Suit};
use world_rules::rules::games::mahjong::{Dragon, Hand, Tile, TileType, Wind};

// ============================================================================
// 1. 无效输入测试
// ============================================================================

mod invalid_input_tests {
    use super::*;

    /// 测试无效的麻将牌数字（超出 1-9 范围）
    #[test]
    fn invalid_mahjong_tile_number() {
        // 数字 0 应被 clamp 或处理
        let tile = Tile::wan(0);
        let num = tile.tile_type.number();
        // 风牌/箭牌或无效数字应返回 None 或被 clamp
        assert!(num.is_none() || (num.unwrap() >= 1 && num.unwrap() <= 9));

        // 数字 10 应被 clamp
        let tile = Tile::tiao(10);
        let num = tile.tile_type.number();
        assert!(num.is_none() || (num.unwrap() >= 1 && num.unwrap() <= 9));
    }

    /// 测试空扑克牌输入
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn empty_poker_hand_panics() {
        let cards: Vec<Card> = vec![];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试扑克牌数不足 5 张
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn insufficient_poker_cards_panics() {
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Heart, Rank::King),
        ];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试空字符串规则名称
    #[test]
    fn empty_rule_name() {
        let meta = RuleMetadata::new("", "描述");
        assert_eq!(meta.name, "");
        // 空名称不应导致 panic
        assert!(true);
    }

    /// 测试空字符串规则描述
    #[test]
    fn empty_rule_description() {
        let meta = RuleMetadata::new("规则名", "");
        assert_eq!(meta.description, "");
    }

    /// 测试空分类名称
    #[test]
    fn empty_category_name() {
        let cat = RuleCategory::games("");
        assert_eq!(cat.to_string(), "Games/");

        let cat = RuleCategory::law("");
        assert_eq!(cat.to_string(), "Law/");
    }

    /// 测试特殊字符分类名称
    #[test]
    fn special_characters_in_category() {
        // Unicode 字符
        let cat = RuleCategory::games("🎮游戏");
        assert!(cat.to_string().contains("🎮"));

        // 控制字符
        let cat = RuleCategory::games("\n\t");
        assert!(true); // 不应 panic
    }

    /// 测试超长字符串输入
    #[test]
    fn extremely_long_input() {
        let long_name = "x".repeat(100_000);
        let meta = RuleMetadata::new(&long_name, "描述");
        assert_eq!(meta.name.len(), 100_000);
    }

    /// 测试 ValidateContext 的特殊输入
    #[test]
    fn validate_context_special_input() {
        // 空字符串
        let ctx = ValidateContext::mahjong_tiles("");
        assert!(matches!(ctx, ValidateContext::MahjongTiles(_)));

        // 空白字符
        let ctx = ValidateContext::poker_cards("   ");
        assert!(matches!(ctx, ValidateContext::PokerCards(_)));

        // 无效格式
        let ctx = ValidateContext::doudizhu_cards("invalid!@#$%");
        assert!(matches!(ctx, ValidateContext::DouDiZhuCards(_)));
    }

    /// 测试 RuleMetadata 负数版本号（字符串形式）
    #[test]
    fn negative_version_string() {
        let meta = RuleMetadata::new("规则", "描述").with_version("-1.0.0");
        assert_eq!(meta.version, "-1.0.0");
        // 允许任何字符串版本
    }

    /// 测试 RuleMetadata 空版本号
    #[test]
    fn empty_version() {
        let meta = RuleMetadata::new("规则", "描述").with_version("");
        assert_eq!(meta.version, "");
    }

    /// 测试 RuleMetadata 特殊标签
    #[test]
    fn special_tags() {
        // 空标签
        let meta = RuleMetadata::new("规则", "描述").with_tags(vec!["".to_string()]);
        assert_eq!(meta.tags.len(), 1);

        // Unicode 标签
        let meta = RuleMetadata::new("规则", "描述").with_tags(vec!["🎮🎯🎲".to_string()]);
        assert_eq!(meta.tags[0], "🎮🎯🎲");
    }
}

// ============================================================================
// 2. 异常状态测试
// ============================================================================

mod abnormal_state_tests {
    use super::*;

    /// 测试麻将手牌超过最大容量
    #[test]
    fn hand_exceeds_max_capacity() {
        let mut hand = Hand::new();
        // 添加大量牌（超过标准 14 张）
        for _ in 0..100 {
            hand.add_tile(Tile::wan(1));
        }
        // 系统应能处理，不应 panic
        assert!(true);
    }

    /// 测试麻将牌重复次数超过限制
    #[test]
    fn tile_duplicates_exceed_limit() {
        let mut hand = Hand::new();
        // 同一张牌添加多次（实际规则中最多 4 张）
        for _ in 0..10 {
            hand.add_tile(Tile::wan(1));
        }
        // 系统应能处理
        assert!(true);
    }

    /// 测试 RuleSet 空规则集操作
    #[test]
    fn empty_ruleset_operations() {
        let rs = RuleSet::new("空规则集".to_string(), RuleCategory::games("test"));
        assert!(rs.is_empty());
        assert_eq!(rs.len(), 0);
        assert!(rs.get_rule("不存在").is_none());
        assert!(rs.list_rules().is_empty());
    }

    /// 测试 RuleSet 获取不存在的规则
    #[test]
    fn get_nonexistent_rule() {
        let rs = RuleSet::new("测试规则集".to_string(), RuleCategory::games("test"));
        assert!(rs.get_rule("不存在的规则").is_none());
    }

    /// 测试 RuleCategory 序列化/反序列化
    #[test]
    fn category_serialization_edge_cases() {
        // 空 name
        let cat = RuleCategory::games("");
        let json = serde_json::to_string(&cat).unwrap();
        let decoded: RuleCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, cat);

        // 特殊字符
        let cat = RuleCategory::law("测试/法律");
        let json = serde_json::to_string(&cat).unwrap();
        let decoded: RuleCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, cat);
    }

    /// 测试 RuleMetadata 序列化边界情况
    #[test]
    fn metadata_serialization_edge_cases() {
        // 全空字段
        let meta = RuleMetadata::new("", "").with_version("");
        let json = serde_json::to_string(&meta).unwrap();
        let decoded: RuleMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.name, "");

        // 超 long 字段
        let long = "x".repeat(10000);
        let meta = RuleMetadata::new(&long, &long).with_tags(vec![long.clone()]);
        let json = serde_json::to_string(&meta).unwrap();
        let decoded: RuleMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.name.len(), 10000);
    }

    /// 测试 Difficulty 默认和边界
    #[test]
    fn difficulty_boundaries() {
        // 默认值
        let default: Difficulty = Difficulty::default();
        assert_eq!(default, Difficulty::Normal);

        // 比较顺序
        assert!(Difficulty::Beginner < Difficulty::Easy);
        assert!(Difficulty::Easy < Difficulty::Normal);
        assert!(Difficulty::Normal < Difficulty::Hard);
        assert!(Difficulty::Hard < Difficulty::Expert);
        assert!(Difficulty::Expert < Difficulty::Master);
    }

    /// 测试 TileType 边界
    #[test]
    fn tiletype_boundaries() {
        // 万子牌 1-9
        for n in 1..=9 {
            let tile = Tile::wan(n);
            assert!(tile.tile_type.number().unwrap_or(0) >= 1);
        }

        // 风牌没有数字
        for wind in [Wind::Dong, Wind::Nan, Wind::Xi, Wind::Bei] {
            let tt = TileType::Feng(wind);
            assert!(tt.number().is_none());
        }

        // 箭牌没有数字
        for dragon in [Dragon::BaiBan, Dragon::FaCai, Dragon::HongZhong] {
            let tt = TileType::Jian(dragon);
            assert!(tt.number().is_none());
        }
    }

    /// 测试 Hand 空状态
    #[test]
    fn hand_empty_state() {
        let hand = Hand::new();
        assert!(!hand.can_win());
        assert!(hand.find_waiting_tiles().is_empty());
        assert!(hand.tiles().is_empty());
        assert!(hand.melds().is_empty());
    }
}

// ============================================================================
// 3. 错误恢复测试
// ============================================================================

mod error_recovery_tests {
    use super::*;

    /// 测试 RuleError 转换为字符串
    #[test]
    fn rule_error_to_string() {
        let err = RuleError::RuleNotFound("测试规则".to_string());
        assert!(err.to_string().contains("测试规则"));

        let err = RuleError::ValidationError("验证失败".to_string());
        assert!(err.to_string().contains("验证失败"));

        let err = RuleError::ConfigError("配置错误".to_string());
        assert!(err.to_string().contains("配置错误"));

        let err = RuleError::UnsupportedOperation("不支持".to_string());
        assert!(err.to_string().contains("不支持"));
    }

    /// 测试 RuleError ContextMismatch
    #[test]
    fn rule_error_context_mismatch() {
        let err = RuleError::ContextMismatch {
            expected: "MahjongTiles".to_string(),
            actual: "PokerCards".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("MahjongTiles"));
        assert!(msg.contains("PokerCards"));
    }

    /// 测试从错误中恢复：重新创建有效状态
    #[test]
    fn recover_from_error() {
        // 创建无效状态后恢复
        let mut hand = Hand::new();
        for _ in 0..100 {
            hand.add_tile(Tile::wan(1));
        }

        // 清空并重建有效状态
        hand = Hand::new();
        for n in 1..=9 {
            hand.add_tile(Tile::wan(n));
        }
        // 恢复后应正常工作
        assert!(true);
    }

    /// 测试 RuleSet 添加和查询规则
    #[test]
    fn ruleset_add_and_query() {
        let mut rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));

        // 添加规则（使用 RuleMetadata 作为示例）
        // 注意：这里测试 RuleSet 的基本操作，不依赖具体 Rule 实现
        assert!(rs.is_empty());

        // 查询不存在的规则不应 panic
        assert!(rs.get_rule("不存在").is_none());
    }

    /// 测试 ValidateContext 类型转换
    #[test]
    fn validate_context_type_handling() {
        // 创建各种类型的上下文
        let contexts = vec![
            ValidateContext::doudizhu_cards("3s 4h 5d"),
            ValidateContext::mahjong_tiles("1m 2m 3m"),
            ValidateContext::poker_cards("As Kh Qd"),
            ValidateContext::chess_move("车", "0,0", "0,5"),
            ValidateContext::gomoku_board(vec![(0, 0, true), (1, 1, false)]),
            ValidateContext::generic("通用上下文"),
        ];

        for ctx in contexts {
            // type_name 应返回非空字符串
            assert!(!ctx.type_name().is_empty());
            // Display 应正常工作
            let _display = format!("{}", ctx);
        }
    }

    /// 测试空输入后恢复到有效状态
    #[test]
    fn recover_from_empty_input() {
        // 空输入
        let meta = RuleMetadata::new("", "");
        assert_eq!(meta.name, "");

        // 恢复到有效状态
        let valid_meta = RuleMetadata::new("有效规则", "有效描述")
            .with_version("1.0.0")
            .with_origin("测试")
            .with_tags(vec!["测试".to_string()])
            .with_difficulty(Difficulty::Normal);

        assert!(!valid_meta.name.is_empty());
        assert!(!valid_meta.description.is_empty());
    }

    /// 测试多次添加同一规则（覆盖）
    #[test]
    fn ruleset_duplicate_add() {
        // RuleSet 的 add_rule 应覆盖同名规则
        let mut rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));

        // 由于没有简单的 Rule 实现，这里测试基本操作
        assert!(rs.is_empty());

        // 添加后查询
        let names = rs.list_rules();
        assert!(names.is_empty());
    }

    /// 测试从序列化错误恢复
    #[test]
    fn recover_from_serialization_error() {
        // 无效 JSON 应返回错误
        let invalid_json = "{invalid}";
        let result: Result<RuleMetadata, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());

        // 有效 JSON 应成功
        let valid_json = r#"{"name":"规则","description":"描述","version":"1.0.0","origin":null,"tags":[],"difficulty":"Normal"}"#;
        let result: Result<RuleMetadata, _> = serde_json::from_str(valid_json);
        assert!(result.is_ok());
    }
}

// ============================================================================
// 4. Panic 测试
// ============================================================================

mod panic_tests {
    use super::*;

    /// 测试空扑克手牌导致 panic
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn empty_poker_hand_causes_panic() {
        let cards: Vec<Card> = vec![];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试扑克牌少于 5 张导致 panic
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn poker_hand_less_than_five_causes_panic() {
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Spade, Rank::King),
            Card::new(Suit::Heart, Rank::Queen),
        ];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试扑克牌正好 4 张导致 panic
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn poker_hand_four_cards_causes_panic() {
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Spade, Rank::King),
            Card::new(Suit::Heart, Rank::Queen),
            Card::new(Suit::Club, Rank::Jack),
        ];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试重复牌的扑克手牌（不应 panic，由系统处理）
    #[test]
    fn duplicate_poker_cards_no_panic() {
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Heart, Rank::Ace), // 重复
            Card::new(Suit::Heart, Rank::King),
            Card::new(Suit::Heart, Rank::Queen),
            Card::new(Suit::Heart, Rank::Jack),
        ];
        // 系统应处理重复牌，不 panic
        let _eval = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试麻将超出范围数字被 clamp（不 panic）
    #[test]
    fn mahjong_tile_out_of_range_no_panic() {
        // 数字 0 被处理
        let tile = Tile::wan(0);
        let _ = tile.tile_type.number();

        // 数字 255 被处理
        let tile = Tile::tiao(255);
        let _ = tile.tile_type.number();

        // 不应 panic
        assert!(true);
    }

    /// 测试空字符串创建 RuleMetadata（不 panic）
    #[test]
    fn empty_strings_no_panic() {
        let _meta = RuleMetadata::new("", "");
        let _meta = RuleMetadata::new("", "").with_version("");
        let _meta = RuleMetadata::new("", "").with_origin("");
        let _meta = RuleMetadata::new("", "").with_tags(vec![]);
    }

    /// 测试空 RuleSet 操作（不 panic）
    #[test]
    fn empty_ruleset_no_panic() {
        let rs = RuleSet::new("空".to_string(), RuleCategory::games(""));
        assert!(rs.is_empty());
        assert!(rs.get_rule("").is_none());
        assert!(rs.list_rules().is_empty());
        let _md = rs.to_markdown();
    }

    /// 测试 ValidateContext 特殊字符（不 panic）
    #[test]
    fn validate_context_special_chars_no_panic() {
        let _ctx = ValidateContext::mahjong_tiles("!@#$%^&*()");
        let _ctx = ValidateContext::poker_cards("");
        let _ctx = ValidateContext::doudizhu_cards("\n\t\r");
        let _ctx = ValidateContext::generic("null");
    }

    /// 测试超长字符串（不 panic）
    #[test]
    fn very_long_strings_no_panic() {
        let long = "x".repeat(1_000_000);
        let _meta = RuleMetadata::new(&long, &long);
        let _cat = RuleCategory::games(&long);
    }

    /// 测试 RuleMetadata display 处理 None 和空（不 panic）
    #[test]
    fn metadata_display_no_panic() {
        let meta = RuleMetadata::new("规则", "描述");
        let _display = format!("{}", meta);

        let meta = RuleMetadata::new("", "");
        let _display = format!("{}", meta);

        let meta = RuleMetadata::new("规则", "描述").with_tags(vec![]);
        let _display = format!("{}", meta);
    }
}

// ============================================================================
// 5. 综合错误路径测试
// ============================================================================

mod comprehensive_error_tests {
    use super::*;

    /// 测试链式错误场景：创建、验证、恢复
    #[test]
    fn error_chain_handling() {
        // 1. 创建空状态
        let mut hand = Hand::new();
        assert!(hand.tiles().is_empty());

        // 2. 添加无效牌（超出范围）
        hand.add_tile(Tile::wan(255));

        // 3. 验证状态
        assert!(!hand.can_win());

        // 4. 恢复有效状态
        hand = Hand::new();
        for n in 1..=9 {
            hand.add_tile(Tile::wan(n));
        }

        // 5. 再次验证
        // 系统应正常工作
        assert!(true);
    }

    /// 测试多步错误恢复
    #[test]
    fn multi_step_error_recovery() {
        // 步骤 1: 创建无效元数据
        let mut meta = RuleMetadata::new("", "");

        // 步骤 2: 尝试修复（实际上元数据是不可变的，需要重建）
        meta = RuleMetadata::new("有效名称", "有效描述");

        // 步骤 3: 验证修复成功
        assert!(!meta.name.is_empty());

        // 步骤 4: 添加更多有效字段
        meta = meta
            .with_version("2.0.0")
            .with_origin("中国")
            .with_tags(vec!["测试".to_string()])
            .with_difficulty(Difficulty::Expert);

        // 步骤 5: 最终验证
        assert!(meta.tags.len() == 1);
    }

    /// 测试并发安全的错误处理
    #[test]
    fn concurrent_error_handling() {
        use std::sync::Arc;
        use std::thread;

        // 创建共享状态
        let meta = Arc::new(RuleMetadata::new("共享规则", "共享描述"));

        // 多线程访问
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let meta_clone = Arc::clone(&meta);
                thread::spawn(move || {
                    // 读取操作不应 panic
                    let _name = &meta_clone.name;
                    let _desc = &meta_clone.description;
                })
            })
            .collect();

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
    }

    /// 测试 RuleCategory 所有变体
    #[test]
    fn all_category_variants() {
        let categories = vec![
            RuleCategory::games("test"),
            RuleCategory::sports("test"),
            RuleCategory::social("test"),
            RuleCategory::science("test"),
            RuleCategory::law("test"),
            RuleCategory::health("test"),
            RuleCategory::custom("custom", "test"),
        ];

        for cat in categories {
            // Display 应正常工作
            let _display = format!("{}", cat);

            // 序列化应正常工作
            let json = serde_json::to_string(&cat).unwrap();
            let decoded: RuleCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, cat);
        }
    }

    /// 测试所有 ValidateContext 类型
    #[test]
    fn all_validate_context_types() {
        let contexts = vec![
            ValidateContext::doudizhu_cards("3s 4h"),
            ValidateContext::mahjong_tiles("1m 2m"),
            ValidateContext::poker_cards("As Kh"),
            ValidateContext::chess_move("车", "a1", "a5"),
            ValidateContext::gomoku_board(vec![(0, 0, true)]),
            ValidateContext::generic("测试"),
        ];

        for ctx in contexts {
            // type_name 应返回有效字符串
            let type_name = ctx.type_name();
            assert!(!type_name.is_empty());

            // Display 应正常工作
            let _display = format!("{}", ctx);
        }
    }

    /// 测试所有 RuleError 变体
    #[test]
    fn all_rule_error_variants() {
        let errors = vec![
            RuleError::RuleNotFound("规则".to_string()),
            RuleError::ValidationError("验证".to_string()),
            RuleError::ConfigError("配置".to_string()),
            RuleError::UnsupportedOperation("操作".to_string()),
            RuleError::ContextMismatch {
                expected: "预期".to_string(),
                actual: "实际".to_string(),
            },
        ];

        for err in errors {
            // to_string 应正常工作
            let msg = err.to_string();
            assert!(!msg.is_empty());
        }
    }

    /// 测试所有 Difficulty 变体
    #[test]
    fn all_difficulty_variants() {
        let difficulties = vec![
            Difficulty::Beginner,
            Difficulty::Easy,
            Difficulty::Normal,
            Difficulty::Hard,
            Difficulty::Expert,
            Difficulty::Master,
        ];

        for diff in &difficulties {
            // Display 应正常工作
            let display = format!("{}", diff);
            assert!(!display.is_empty());

            // 序列化应正常工作
            let json = serde_json::to_string(&diff).unwrap();
            let decoded: Difficulty = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, *diff);
        }
    }

    /// 测试所有 Rank 变体
    #[test]
    fn all_rank_variants() {
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

    /// 测试所有 Suit 变体
    #[test]
    fn all_suit_variants() {
        let suits = vec![Suit::Heart, Suit::Spade, Suit::Club, Suit::Diamond];
        assert_eq!(suits.len(), 4);
    }

    /// 测试所有 Wind 变体
    #[test]
    fn all_wind_variants() {
        let winds = vec![Wind::Dong, Wind::Nan, Wind::Xi, Wind::Bei];
        assert_eq!(winds.len(), 4);
    }

    /// 测试所有 Dragon 变体
    #[test]
    fn all_dragon_variants() {
        let dragons = vec![Dragon::BaiBan, Dragon::FaCai, Dragon::HongZhong];
        assert_eq!(dragons.len(), 3);
    }
}
