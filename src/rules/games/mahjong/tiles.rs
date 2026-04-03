//! 麻将牌定义

use std::fmt;

/// 风牌
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Wind {
    /// 东风
    Dong,
    /// 南风
    Nan,
    /// 西风
    Xi,
    /// 北风
    Bei,
}

impl fmt::Display for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Wind::Dong => write!(f, "东"),
            Wind::Nan => write!(f, "南"),
            Wind::Xi => write!(f, "西"),
            Wind::Bei => write!(f, "北"),
        }
    }
}

/// 箭牌
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Dragon {
    /// 红中
    HongZhong,
    /// 发财
    FaCai,
    /// 白板
    BaiBan,
}

impl fmt::Display for Dragon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dragon::HongZhong => write!(f, "红中"),
            Dragon::FaCai => write!(f, "发财"),
            Dragon::BaiBan => write!(f, "白板"),
        }
    }
}

/// 花色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TileType {
    /// 万子 (1-9万)
    Wan(u8),
    /// 条子 (1-9条)
    Tiao(u8),
    /// 筒子 (1-9筒)
    Tong(u8),
    /// 风牌
    Feng(Wind),
    /// 箭牌
    Jian(Dragon),
    /// 花牌 (春夏秋冬梅兰竹菊)
    Hua(u8),
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileType::Wan(n) => write!(f, "{}万", n),
            TileType::Tiao(n) => write!(f, "{}条", n),
            TileType::Tong(n) => write!(f, "{}筒", n),
            TileType::Feng(w) => write!(f, "{}风", w),
            TileType::Jian(d) => write!(f, "{}", d),
            TileType::Hua(n) => write!(f, "花{}", n),
        }
    }
}

impl TileType {
    pub fn number(&self) -> Option<u8> {
        match self {
            TileType::Wan(n) | TileType::Tiao(n) | TileType::Tong(n) => Some(*n),
            _ => None,
        }
    }

    pub fn is_number_tile(&self) -> bool {
        matches!(self, TileType::Wan(_) | TileType::Tiao(_) | TileType::Tong(_))
    }

    pub fn is_honor(&self) -> bool {
        matches!(self, TileType::Feng(_) | TileType::Jian(_))
    }

    pub fn suit(&self) -> Option<&'static str> {
        match self {
            TileType::Wan(_) => Some("万"),
            TileType::Tiao(_) => Some("条"),
            TileType::Tong(_) => Some("筒"),
            _ => None,
        }
    }
}

/// 麻将牌
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tile {
    pub tile_type: TileType,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type }
    }

    pub fn wan(n: u8) -> Self {
        Self::new(TileType::Wan(n.clamp(1, 9)))
    }

    pub fn tiao(n: u8) -> Self {
        Self::new(TileType::Tiao(n.clamp(1, 9)))
    }

    pub fn tong(n: u8) -> Self {
        Self::new(TileType::Tong(n.clamp(1, 9)))
    }

    pub fn feng(wind: Wind) -> Self {
        Self::new(TileType::Feng(wind))
    }

    pub fn jian(dragon: Dragon) -> Self {
        Self::new(TileType::Jian(dragon))
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tile_type)
    }
}

/// 标准麻将牌堆 (不含花牌)
pub fn standard_tiles() -> Vec<Tile> {
    let mut tiles = Vec::with_capacity(136);

    // 万条筒各4张
    for n in 1..=9 {
        for _ in 0..4 {
            tiles.push(Tile::wan(n));
            tiles.push(Tile::tiao(n));
            tiles.push(Tile::tong(n));
        }
    }

    // 风牌各4张
    for wind in [Wind::Dong, Wind::Nan, Wind::Xi, Wind::Bei] {
        for _ in 0..4 {
            tiles.push(Tile::feng(wind));
        }
    }

    // 箭牌各4张
    for dragon in [Dragon::HongZhong, Dragon::FaCai, Dragon::BaiBan] {
        for _ in 0..4 {
            tiles.push(Tile::jian(dragon));
        }
    }

    tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_display() {
        assert_eq!(Tile::wan(1).to_string(), "1万");
        assert_eq!(Tile::tiao(9).to_string(), "9条");
        assert_eq!(Tile::tong(5).to_string(), "5筒");
        assert_eq!(Tile::feng(Wind::Dong).to_string(), "东风");
    }

    #[test]
    fn test_standard_tiles_count() {
        assert_eq!(standard_tiles().len(), 136);
    }
}