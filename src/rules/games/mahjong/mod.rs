//! 麻将规则模块
//!
//! 支持多种麻将变体规则

mod rules;
mod tiles;
mod hands;

pub use rules::*;
pub use tiles::{Tile, TileType, Wind, Dragon};
pub use hands::{Hand, HandPattern, WinningHand};