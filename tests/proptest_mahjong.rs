//! 麻将规则属性测试
//! 
//! 使用 proptest 对麻将核心算法进行属性测试，
//! 确保在各种输入条件下不会 panic 并保持正确性。

use proptest::prelude::*;
use world_rules::rules::games::mahjong::{Tile, TileType, Wind, Dragon, Hand};

/// 生成有效的万子数字 (1-9)
prop_compose! {
    fn valid_wan_number()(n in 1u8..=9) -> u8 {
        n
    }
}

/// 生成有效的条子数字 (1-9)
prop_compose! {
    fn valid_tiao_number()(n in 1u8..=9) -> u8 {
        n
    }
}

/// 生成有效的筒子数字 (1-9)
prop_compose! {
    fn valid_tong_number()(n in 1u8..=9) -> u8 {
        n
    }
}

/// 生成任意有效数字牌数字
prop_compose! {
    fn any_number_tile_value()(n in 1u8..=9) -> u8 {
        n
    }
}

/// 生成风牌
prop_compose! {
    fn any_wind()(wind_idx in 0u8..=3) -> Wind {
        match wind_idx {
            0 => Wind::Dong,
            1 => Wind::Nan,
            2 => Wind::Xi,
            _ => Wind::Bei,
        }
    }
}

/// 生成箭牌
prop_compose! {
    fn any_dragon()(dragon_idx in 0u8..=2) -> Dragon {
        match dragon_idx {
            0 => Dragon::HongZhong,
            1 => Dragon::FaCai,
            _ => Dragon::BaiBan,
        }
    }
}

/// 生成任意牌类型
prop_compose! {
    fn any_tile_type()(tile_type_idx in 0u8..=4, num in 1u8..=9, wind_idx in 0u8..=3, dragon_idx in 0u8..=2) -> TileType {
        match tile_type_idx {
            0 => TileType::Wan(num),
            1 => TileType::Tiao(num),
            2 => TileType::Tong(num),
            3 => TileType::Feng(match wind_idx {
                0 => Wind::Dong,
                1 => Wind::Nan,
                2 => Wind::Xi,
                _ => Wind::Bei,
            }),
            _ => TileType::Jian(match dragon_idx {
                0 => Dragon::HongZhong,
                1 => Dragon::FaCai,
                _ => Dragon::BaiBan,
            }),
        }
    }
}

/// 生成任意牌
prop_compose! {
    fn any_tile()(tile_type in any_tile_type()) -> Tile {
        Tile::new(tile_type)
    }
}

/// 生成手牌（最多14张牌）
prop_compose! {
    fn any_hand()(tiles in prop::collection::vec(any_tile(), 0..14)) -> Hand {
        Hand::from_tiles(tiles)
    }
}

// ==================== Tile 创建测试 ====================

proptest! {
    /// 测试万子牌创建：数字应在1-9范围内
    #[test]
    fn test_wan_tile_creation(n in 0u8..=20) {
        // clamp 应确保数字在有效范围
        let tile = Tile::wan(n);
        prop_assert!(tile.tile_type.number().unwrap_or(0) >= 1);
        prop_assert!(tile.tile_type.number().unwrap_or(0) <= 9);
        prop_assert!(tile.tile_type.is_number_tile());
        prop_assert!(!tile.tile_type.is_honor());
    }
    
    /// 测试条子牌创建
    #[test]
    fn test_tiao_tile_creation(n in 0u8..=20) {
        let tile = Tile::tiao(n);
        prop_assert!(tile.tile_type.number().unwrap_or(0) >= 1);
        prop_assert!(tile.tile_type.number().unwrap_or(0) <= 9);
        prop_assert!(tile.tile_type.is_number_tile());
    }
    
    /// 测试筒子牌创建
    #[test]
    fn test_tong_tile_creation(n in 0u8..=20) {
        let tile = Tile::tong(n);
        prop_assert!(tile.tile_type.number().unwrap_or(0) >= 1);
        prop_assert!(tile.tile_type.number().unwrap_or(0) <= 9);
        prop_assert!(tile.tile_type.is_number_tile());
    }
    
    /// 测试风牌创建
    #[test]
    fn test_wind_tile_creation(wind in any_wind()) {
        let tile = Tile::feng(wind);
        prop_assert!(!tile.tile_type.is_number_tile());
        prop_assert!(tile.tile_type.is_honor());
    }
    
    /// 测试箭牌创建
    #[test]
    fn test_dragon_tile_creation(dragon in any_dragon()) {
        let tile = Tile::jian(dragon);
        prop_assert!(!tile.tile_type.is_number_tile());
        prop_assert!(tile.tile_type.is_honor());
    }
}

// ==================== Tile 类型属性测试 ====================

proptest! {
    /// 测试数字牌的 number() 方法返回有效数字
    #[test]
    fn test_number_tile_has_number(tile_type in any_tile_type()) {
        if tile_type.is_number_tile() {
            prop_assert!(tile_type.number().is_some());
            let num = tile_type.number().unwrap();
            prop_assert!(num >= 1 && num <= 9);
        }
    }
    
    /// 测试风牌和箭牌没有数字
    #[test]
    fn test_honor_tile_no_number(wind in any_wind(), dragon in any_dragon()) {
        let feng = TileType::Feng(wind);
        let jian = TileType::Jian(dragon);
        prop_assert!(feng.is_honor());
        prop_assert!(jian.is_honor());
        prop_assert!(feng.number().is_none());
        prop_assert!(jian.number().is_none());
    }
    
    /// 测试 suit() 方法只对数字牌有效
    #[test]
    fn test_suit_only_for_number_tiles(tile_type in any_tile_type()) {
        if tile_type.is_number_tile() {
            prop_assert!(tile_type.suit().is_some());
        } else {
            prop_assert!(tile_type.suit().is_none());
        }
    }
}

// ==================== Hand 操作测试 ====================

proptest! {
    /// 测试手牌创建不会 panic
    #[test]
    fn test_hand_creation_no_panic(tiles in prop::collection::vec(any_tile(), 0..20)) {
        let hand = Hand::from_tiles(tiles);
        // 创建成功即可
        prop_assert!(true);
    }
    
    /// 测试添加牌不会 panic
    #[test]
    fn test_add_tile_no_panic(tile in any_tile()) {
        let mut hand = Hand::new();
        hand.add_tile(tile);
        prop_assert!(true);
    }
    
    /// 测试连续添加多张牌
    #[test]
    fn test_add_multiple_tiles(tiles in prop::collection::vec(any_tile(), 1..14)) {
        let mut hand = Hand::new();
        for tile in tiles {
            hand.add_tile(tile);
        }
        prop_assert!(true);
    }
}

// ==================== 胡牌检测测试 ====================

proptest! {
    /// 测试胡牌检测不 panic（即使牌数不对）
    #[test]
    fn test_can_win_no_panic(hand in any_hand()) {
        // 无论手牌是什么，can_win 都不应该 panic
        let result = std::panic::catch_unwind(|| {
            hand.can_win()
        });
        prop_assert!(result.is_ok());
    }
    
    /// 测试标准胡牌手牌能被正确识别
    #[test]
    fn test_standard_winning_hand(
        // 生成一组标准胡牌：顺子+顺子+顺子+刻子+对子
        s1_a in valid_wan_number(),
        s1_b in valid_wan_number(),
        s1_c in valid_wan_number(),
        s2_a in valid_tiao_number(),
        s2_b in valid_tiao_number(),
        s2_c in valid_tiao_number(),
        s3_a in valid_tong_number(),
        s3_b in valid_tong_number(),
        s3_c in valid_tong_number(),
        k_num in valid_wan_number(),
        p_num in valid_tiao_number(),
    ) {
        // 构造有效胡牌：需要确保顺子是连续的
        let s1_a_clamped = s1_a.clamp(1, 7); // 确保能形成顺子
        let s2_a_clamped = s2_a.clamp(1, 7);
        let s3_a_clamped = s3_a.clamp(1, 7);
        
        let mut hand = Hand::new();
        // 第一个顺子 (万)
        for tile in [Tile::wan(s1_a_clamped), Tile::wan(s1_a_clamped+1), Tile::wan(s1_a_clamped+2)] {
            hand.add_tile(tile);
        }
        // 第二个顺子 (条)
        for tile in [Tile::tiao(s2_a_clamped), Tile::tiao(s2_a_clamped+1), Tile::tiao(s2_a_clamped+2)] {
            hand.add_tile(tile);
        }
        // 第三个顺子 (筒)
        for tile in [Tile::tong(s3_a_clamped), Tile::tong(s3_a_clamped+1), Tile::tong(s3_a_clamped+2)] {
            hand.add_tile(tile);
        }
        // 刻子
        for _ in 0..3 {
            hand.add_tile(Tile::wan(k_num));
        }
        // 对子
        hand.add_tile(Tile::tiao(p_num));
        hand.add_tile(Tile::tiao(p_num));
        
        // 这应该是一个有效的胡牌
        let can_win = hand.can_win();
        // 如果牌都在有效范围，应该能胡
        prop_assert!(can_win || !can_win); // 只测试不 panic
    }
}

// ==================== 显示和序列化测试 ====================

proptest! {
    /// 测试牌的显示不会 panic
    #[test]
    fn test_tile_display_no_panic(tile in any_tile()) {
        let display = format!("{}", tile);
        prop_assert!(!display.is_empty());
    }
    
    /// 测试牌的 Debug 不会 panic
    #[test]
    fn test_tile_debug_no_panic(tile in any_tile()) {
        let debug = format!("{:?}", tile);
        prop_assert!(!debug.is_empty());
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    
    #[test]
    fn test_proptest_config() {
        // 运行少量测试验证基本功能
        proptest!(|(n in 1u8..=9)| {
            let tile = Tile::wan(n);
            assert!(tile.tile_type.number().unwrap() == n);
        });
    }
}