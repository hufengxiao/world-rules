//! 麻将规则模块
//!
//! 支持多种麻将变体规则

mod rules;
mod tiles;
mod hands;
pub mod variants;

pub use rules::*;
pub use tiles::{Tile, TileType, Wind, Dragon};
pub use hands::{Hand, HandPattern, WinningHand};

// 重新导出各地方麻将变体
pub use variants::{
    GuangdongMahjongRules,
    WuhanMahjongRules,
    ShanghaiMahjongRules,
    TaiwanMahjongRules,
    BeijingMahjongRules,
    DongbeiMahjongRules,
    ChangshaMahjongRules,
    HangzhouMahjongRules,
    NanjingMahjongRules,
    ChaoshanMahjongRules,
    TianjinMahjongRules,
    ChongqingMahjongRules,
    KunmingMahjongRules,
    GuiyangMahjongRules,
    FuzhouMahjongRules,
    NanchangMahjongRules,
    GuangxiMahjongRules,
    XinjiangMahjongRules,
    SichuanDetailedMahjongRules,
};