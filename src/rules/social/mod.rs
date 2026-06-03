//! 社交礼仪规则模块

pub mod business;
pub mod cinema;
pub mod dining;
pub mod elevator;
pub mod email;
pub mod festival;
pub mod flight;
pub mod funeral;
pub mod gift;
pub mod greeting;
pub mod interview;
pub mod library;
pub mod phone;
pub mod seating;
pub mod tea;
pub mod toasting;
pub mod train;
pub mod wedding;

pub use business::BusinessEtiquette;
pub use cinema::CinemaEtiquette;
pub use dining::{DiningCulture, DiningEtiquette};
pub use elevator::ElevatorEtiquette;
pub use email::EmailEtiquette;
pub use festival::{ChineseFestival, FestivalEtiquette};
pub use flight::FlightEtiquette;
pub use funeral::{FuneralCulture, FuneralEtiquette};
pub use gift::GiftEtiquette;
pub use greeting::GreetingEtiquette;
pub use interview::InterviewEtiquette;
pub use library::LibraryEtiquette;
pub use phone::PhoneEtiquette;
pub use seating::SeatingEtiquette;
pub use tea::{TeaCulture, TeaEtiquette};
pub use toasting::ToastingEtiquette;
pub use train::TrainEtiquette;
pub use wedding::{WeddingCulture, WeddingEtiquette};
