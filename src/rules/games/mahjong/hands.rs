//! 麻将手牌与牌型定义

use super::tiles::{Tile, TileType};
use std::collections::HashMap;

/// 牌组类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Meld {
    /// 顺子
    Shunzi(Tile, Tile, Tile),
    /// 刻子
    Kezi(Tile),
    /// 杠子
    Gangzi(Tile),
    /// 对子
    Duizi(Tile),
}

impl Meld {
    pub fn tiles(&self) -> Vec<Tile> {
        match self {
            Meld::Shunzi(a, b, c) => vec![*a, *b, *c],
            Meld::Kezi(t) | Meld::Duizi(t) => vec![*t],
            Meld::Gangzi(t) => vec![*t],
        }
    }

    pub fn is_shunzi(&self) -> bool {
        matches!(self, Meld::Shunzi(_, _, _))
    }

    pub fn is_kezi(&self) -> bool {
        matches!(self, Meld::Kezi(_))
    }
}

/// 牌型模式
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandPattern {
    /// 标准胡牌 (4组+1对)
    Standard { melds: Vec<Meld>, pair: Tile },
    /// 七对子
    SevenPairs { pairs: Vec<Tile> },
    /// 十三幺
    ThirteenOrphans,
    /// 全不靠
    AllSingles,
}

/// 胡牌类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WinningType {
    /// 自摸
    Zimo,
    /// 点炮
    Dianpao,
    /// 抢杠
    Qianggang,
    /// 杠开
    Gangkai,
}

/// 胡牌结果
#[derive(Debug, Clone)]
pub struct WinningHand {
    pub pattern: HandPattern,
    pub winning_type: WinningType,
    pub winning_tile: Tile,
    pub fan: u8,
}

/// 手牌
#[derive(Debug, Clone)]
pub struct Hand {
    /// 手中的牌
    tiles: Vec<Tile>,
    /// 明牌 (吃碰杠)
    melds: Vec<Meld>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            melds: Vec::new(),
        }
    }

    pub fn from_tiles(tiles: Vec<Tile>) -> Self {
        Self {
            tiles,
            melds: Vec::new(),
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn remove_tile(&mut self, tile: &Tile) -> bool {
        if let Some(pos) = self.tiles.iter().position(|t| t == tile) {
            self.tiles.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }

    pub fn melds(&self) -> &[Meld] {
        &self.melds
    }

    pub fn add_meld(&mut self, meld: Meld) {
        self.melds.push(meld);
    }

    /// 统计各牌数量
    pub fn tile_counts(&self) -> HashMap<Tile, u8> {
        let mut counts = HashMap::new();
        for tile in &self.tiles {
            *counts.entry(*tile).or_insert(0) += 1;
        }
        counts
    }

    /// 检查是否听牌
    pub fn is_ready(&self) -> bool {
        // 简化实现：检查是否差一张牌胡
        self.find_waiting_tiles().len() > 0
    }

    /// 找出听哪些牌
    pub fn find_waiting_tiles(&self) -> Vec<Tile> {
        let mut waiting = Vec::new();
        let mut test_hand = self.clone();

        // 遍历所有可能的牌
        for tile in super::tiles::standard_tiles() {
            test_hand.tiles.push(tile);
            if test_hand.can_win() {
                waiting.push(tile);
            }
            test_hand.tiles.pop();
        }

        waiting.sort();
        waiting.dedup();
        waiting
    }

    /// 检查是否能胡牌
    pub fn can_win(&self) -> bool {
        let n = self.tiles.len();
        // 标准胡牌需要14张 (包含胡的那张)
        // 七对子需要14张
        // 十三幺需要14张
        if n != 14 {
            return false;
        }

        // 尝试匹配标准胡牌
        if self.can_win_standard() {
            return true;
        }

        // 尝试七对子
        if self.can_win_seven_pairs() {
            return true;
        }

        // 尝试十三幺
        if self.can_win_thirteen_orphans() {
            return true;
        }

        false
    }

    fn can_win_standard(&self) -> bool {
        let counts = self.tile_counts();
        Self::check_standard_recursive(&counts, 0, false)
    }

    fn check_standard_recursive(
        counts: &HashMap<Tile, u8>,
        melds: u8,
        has_pair: bool,
    ) -> bool {
        if melds == 4 && has_pair {
            // 检查是否所有牌都用完了
            return counts.values().all(|&c| c == 0);
        }

        // 找第一张还有数量的牌
        let first_tile = counts.iter().find(|(_, &c)| c > 0);
        let Some((&tile, &count)) = first_tile else {
            return melds == 4 && has_pair;
        };

        // 尝试做对子
        if !has_pair && count >= 2 {
            let mut new_counts = counts.clone();
            *new_counts.get_mut(&tile).unwrap() -= 2;
            if Self::check_standard_recursive(&new_counts, melds, true) {
                return true;
            }
        }

        // 尝试做刻子
        if count >= 3 {
            let mut new_counts = counts.clone();
            *new_counts.get_mut(&tile).unwrap() -= 3;
            if Self::check_standard_recursive(&new_counts, melds + 1, has_pair) {
                return true;
            }
        }

        // 尝试做顺子 (只有数牌可以)
        if let Some(num) = tile.tile_type.number() {
            if num <= 7 {
                let suit = tile.tile_type.suit().unwrap();
                // 找同花色的 n, n+1, n+2
                let t2 = Tile::new(match suit {
                    "万" => TileType::Wan(num + 1),
                    "条" => TileType::Tiao(num + 1),
                    "筒" => TileType::Tong(num + 1),
                    _ => return false,
                });
                let t3 = Tile::new(match suit {
                    "万" => TileType::Wan(num + 2),
                    "条" => TileType::Tiao(num + 2),
                    "筒" => TileType::Tong(num + 2),
                    _ => return false,
                });

                if counts.get(&t2).copied().unwrap_or(0) > 0
                    && counts.get(&t3).copied().unwrap_or(0) > 0
                {
                    let mut new_counts = counts.clone();
                    *new_counts.get_mut(&tile).unwrap() -= 1;
                    *new_counts.get_mut(&t2).unwrap() -= 1;
                    *new_counts.get_mut(&t3).unwrap() -= 1;
                    if Self::check_standard_recursive(&new_counts, melds + 1, has_pair) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn can_win_seven_pairs(&self) -> bool {
        let counts = self.tile_counts();
        if counts.len() != 7 {
            return false;
        }
        counts.values().all(|&c| c == 2)
    }

    fn can_win_thirteen_orphans(&self) -> bool {
        use super::tiles::{Dragon, Wind};

        // 十三幺需要: 1,9万 + 1,9条 + 1,9筒 + 东南西北 + 中发白 + 其中任意一张成对
        let required: Vec<Tile> = vec![
            Tile::wan(1), Tile::wan(9),
            Tile::tiao(1), Tile::tiao(9),
            Tile::tong(1), Tile::tong(9),
            Tile::feng(Wind::Dong), Tile::feng(Wind::Nan),
            Tile::feng(Wind::Xi), Tile::feng(Wind::Bei),
            Tile::jian(Dragon::HongZhong), Tile::jian(Dragon::FaCai),
            Tile::jian(Dragon::BaiBan),
        ];

        let counts = self.tile_counts();

        // 检查是否包含所有幺九牌
        for tile in &required {
            if counts.get(tile).copied().unwrap_or(0) == 0 {
                return false;
            }
        }

        // 检查是否有一张成对
        required.iter().any(|t| counts.get(t).copied().unwrap_or(0) == 2)
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_creation() {
        let hand = Hand::new();
        assert_eq!(hand.tiles().len(), 0);
    }
}