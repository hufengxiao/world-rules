//! 社交礼仪规则模块

pub mod dining;
pub mod business;
pub mod gift;
pub mod tea;
pub mod wedding;
pub mod interview;

pub use dining::{DiningEtiquette, DiningCulture};
pub use business::BusinessEtiquette;
pub use gift::GiftEtiquette;
pub use tea::{TeaEtiquette, TeaCulture};
pub use wedding::{WeddingEtiquette, WeddingCulture};
pub use interview::InterviewEtiquette;