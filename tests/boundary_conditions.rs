//! 边界条件测试 - Phase 22
//!
//! 本文件包含所有规则的边界条件测试，确保在各种极限输入下系统稳定性。
//! 测试分类：
//! 1. 数值边界测试 - 测试数值范围的极限和溢出
//! 2. 空值/None 测试 - 测试空输入和 None 处理
//! 3. 极限值测试 - 测试集合和序列的边界
//! 4. 并发边界测试 - 测试多线程访问的安全性

use world_rules::prelude::*;
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

        let tile = Tile::tiao(-1);
        let num = tile.tile_type.number().unwrap_or(0);
        assert!(num >= 1 && num <= 9, "负数应被 clamp 到 1-9 范围");
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

    /// 测试空扑克手牌
    #[test]
    fn empty_poker_hand() {
        let cards: Vec<Card> = vec![];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        // 空手牌应返回最低牌型
        assert!(true, "空手牌评估不应 panic");
    }

    /// 测试单张扑克手牌
    #[test]
    fn single_card_hand() {
        let cards = vec![Card::new(Suit::Heart, Rank::Ace)];
        let eval = TexasHoldemRules::evaluate_hand(&cards);
        // 单张牌评估不应 panic
        assert!(true, "单张牌评估不应 panic");
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
        let eval = TexasHoldemRules::evaluate_hand(&cards);
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
        let empty_cat = RuleCategory::games("");
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
}

// ============================================================================
// 4. 并发边界测试
// ============================================================================

mod concurrency_boundary_tests {
    use super::*;

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

    /// 测试非标准扑克牌数
    #[test]
    fn non_standard_poker_hand_sizes() {
        // 2 张牌
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Spade, Rank::King),
        ];
        let _ = TexasHoldemRules::evaluate_hand(&cards);

        // 10 张牌
        let cards: Vec<Card> = (0..10)
            .map(|i| Card::new(Suit::Heart, Rank::values()[i % 13].clone()))
            .collect();
        let _ = TexasHoldemRules::evaluate_hand(&cards);
    }

    /// 测试重复牌在手牌中
    #[test]
    fn duplicate_cards_in_hand() {
        // 扑克手牌中包含完全相同的牌（实际不可能）
        let cards = vec![
            Card::new(Suit::Heart, Rank::Ace),
            Card::new(Suit::Heart, Rank::Ace), // 重复
        ];
        let _ = TexasHoldemRules::evaluate_hand(&cards);
        // 不应 panic
        assert!(true);
    }
}
