//! 体育规则模块

pub mod football;
pub mod basketball;
pub mod table_tennis;
pub mod tennis;
pub mod volleyball;
pub mod badminton;
pub mod swimming;
pub mod athletics;
pub mod golf;
pub mod skiing;
pub mod cycling;
pub mod weightlifting;
pub mod billiards;
pub mod shooting;
pub mod archery;
pub mod gymnastics;
pub mod diving;
pub mod synchronized_swimming;

pub use football::FootballRules;
pub use basketball::BasketballRules;
pub use table_tennis::TableTennisRules;
pub use tennis::TennisRules;
pub use volleyball::VolleyballRules;
pub use badminton::BadmintonRules;
pub use swimming::{SwimmingRules, SwimmingStyle};
pub use athletics::{AthleticsRules, AthleticsEvent};
pub use golf::GolfRules;
pub use skiing::{SkiingRules, SkiingType};
pub use cycling::{CyclingRules, CyclingType};
pub use weightlifting::WeightliftingRules;
pub use billiards::{BilliardsRules, BilliardsType};
pub use shooting::ShootingRules;
pub use archery::ArcheryRules;
pub use gymnastics::{GymnasticsRules, GymnasticsType};
pub use diving::DivingRules;
pub use synchronized_swimming::SynchronizedSwimmingRules;