//! 麻将规则模块
//!
//! 支持多种麻将变体规则

mod hands;
mod rules;
mod tiles;
pub mod variants;

pub use hands::{Hand, HandPattern, Meld, WinningHand};
pub use rules::*;
pub use tiles::{Dragon, Tile, TileType, Wind};

// 重新导出各地方麻将变体
pub use variants::{
    AnhuiMahjongRules, BeijingMahjongRules, ChangshaMahjongRules, ChaoshanMahjongRules,
    ChongqingMahjongRules, DongbeiMahjongRules, FuzhouMahjongRules, GuangdongMahjongRules,
    GuangxiMahjongRules, GuiyangMahjongRules, HainanMahjongRules, HangzhouMahjongRules,
    KejiaMahjongRules, KunmingMahjongRules, NanchangMahjongRules, NanjingMahjongRules,
    ShanghaiMahjongRules, SichuanDetailedMahjongRules, SuzhouMahjongRules, TaiwanMahjongRules,
    TianjinMahjongRules, WuhanMahjongRules, XianMahjongRules, XinjiangMahjongRules,
    ZhengzhouMahjongRules,
};
