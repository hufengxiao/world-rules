//! 法律法规模块

pub mod traffic;
pub mod contract;
pub mod labor;
pub mod consumer;
pub mod intellectual_property;
pub mod road_safety;
pub mod marriage;
pub mod inheritance;

pub use traffic::{TrafficRules, TrafficRegion};
pub use contract::ContractRules;
pub use labor::LaborLawRules;
pub use consumer::ConsumerLawRules;
pub use intellectual_property::IPRules;
pub use road_safety::RoadSafetyRules;
pub use marriage::MarriageLawRules;
pub use inheritance::InheritanceLawRules;