//! 科学规则模块

pub mod physics;
pub mod math;
pub mod chemistry;
pub mod biology;
pub mod astronomy;
pub mod economics;
pub mod psychology;
pub mod statistics;

pub use physics::PhysicsLaws;
pub use math::MathRules;
pub use chemistry::ChemistryRules;
pub use biology::BiologyRules;
pub use astronomy::AstronomyRules;
pub use economics::EconomicsRules;
pub use psychology::PsychologyRules;
pub use statistics::StatisticsRules;