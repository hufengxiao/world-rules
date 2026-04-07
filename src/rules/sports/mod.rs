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

pub use football::FootballRules;
pub use basketball::BasketballRules;
pub use table_tennis::TableTennisRules;
pub use tennis::TennisRules;
pub use volleyball::VolleyballRules;
pub use badminton::BadmintonRules;
pub use swimming::{SwimmingRules, SwimmingStyle};
pub use athletics::{AthleticsRules, AthleticsEvent};
pub use golf::GolfRules;