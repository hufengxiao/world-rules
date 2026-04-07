//! 社交礼仪规则模块

pub mod dining;
pub mod business;
pub mod gift;
pub mod tea;
pub mod wedding;
pub mod interview;
pub mod funeral;
pub mod festival;
pub mod elevator;
pub mod cinema;
pub mod library;
pub mod flight;
pub mod train;

pub use dining::{DiningEtiquette, DiningCulture};
pub use business::BusinessEtiquette;
pub use gift::GiftEtiquette;
pub use tea::{TeaEtiquette, TeaCulture};
pub use wedding::{WeddingEtiquette, WeddingCulture};
pub use interview::InterviewEtiquette;
pub use funeral::{FuneralEtiquette, FuneralCulture};
pub use festival::{FestivalEtiquette, ChineseFestival};
pub use elevator::ElevatorEtiquette;
pub use cinema::CinemaEtiquette;
pub use library::LibraryEtiquette;
pub use flight::FlightEtiquette;
pub use train::TrainEtiquette;