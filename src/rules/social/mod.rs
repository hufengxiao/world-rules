//! 社交礼仪规则模块

pub mod dining;
pub mod business;
pub mod gift;

pub use dining::{DiningEtiquette, DiningCulture};
pub use business::BusinessEtiquette;
pub use gift::GiftEtiquette;