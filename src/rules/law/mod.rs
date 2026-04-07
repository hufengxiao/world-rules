//! 法律法规模块

pub mod traffic;
pub mod contract;
pub mod labor;

pub use traffic::{TrafficRules, TrafficRegion};
pub use contract::ContractRules;
pub use labor::LaborLawRules;