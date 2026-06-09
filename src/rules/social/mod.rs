//! 社交礼仪规则模块

pub mod airplane_etiquette;
pub mod bar;
pub mod business;
pub mod cafe;
pub mod cinema;
pub mod dating_etiquette;
pub mod dining;
pub mod elevator;
pub mod elevator_detailed;
pub mod email;
pub mod festival;
pub mod flight;
pub mod funeral;
pub mod gift;
pub mod golf_etiquette;
pub mod greeting;
pub mod hospital_etiquette;
pub mod interview;
pub mod library;
pub mod live_streaming;
pub mod mahjong_etiquette;
pub mod neighbor;
pub mod online;
pub mod party;
pub mod pet_etiquette;
pub mod phone;
pub mod queue;
pub mod seating;
pub mod shopping_etiquette;
pub mod smoking;
pub mod social_media_etiquette;
pub mod tea;
pub mod toasting;
pub mod train;
pub mod wedding;
pub mod workplace;

pub use airplane_etiquette::AirplaneEtiquetteRules;
pub use bar::BarRules;
pub use business::BusinessEtiquette;
pub use cafe::CafeRules;
pub use cinema::CinemaEtiquette;
pub use dating_etiquette::DatingEtiquetteRules;
pub use dining::{DiningCulture, DiningEtiquette};
pub use elevator::ElevatorEtiquette;
pub use elevator_detailed::ElevatorDetailedRules;
pub use email::EmailEtiquette;
pub use festival::{ChineseFestival, FestivalEtiquette};
pub use flight::FlightEtiquette;
pub use funeral::{FuneralCulture, FuneralEtiquette};
pub use gift::GiftEtiquette;
pub use golf_etiquette::GolfEtiquetteRules;
pub use greeting::GreetingEtiquette;
pub use hospital_etiquette::HospitalEtiquetteRules;
pub use interview::InterviewEtiquette;
pub use library::LibraryEtiquette;
pub use live_streaming::LiveStreamingRules;
pub use mahjong_etiquette::MahjongEtiquetteRules;
pub use neighbor::NeighborRules;
pub use online::OnlineRules;
pub use party::PartyRules;
pub use pet_etiquette::PetEtiquetteRules;
pub use phone::PhoneEtiquette;
pub use queue::QueueRules;
pub use seating::SeatingEtiquette;
pub use shopping_etiquette::ShoppingEtiquetteRules;
pub use smoking::SmokingRules;
pub use social_media_etiquette::SocialMediaEtiquetteRules;
pub use tea::{TeaCulture, TeaEtiquette};
pub use toasting::ToastingEtiquette;
pub use train::TrainEtiquette;
pub use wedding::{WeddingCulture, WeddingEtiquette};
pub use workplace::WorkplaceRules;

pub fn all_rules() -> Vec<(&'static str, crate::rules::core::RuleMetadata, crate::rules::core::RuleCategory)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    { let r = AirplaneEtiquetteRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = BarRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = BusinessEtiquette::new("中国"); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = CafeRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = CinemaEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = DatingEtiquetteRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = DiningEtiquette::new(crate::rules::social::dining::DiningCulture::Chinese); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = ElevatorDetailedRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = ElevatorEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = EmailEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = FestivalEtiquette::new(crate::rules::social::festival::ChineseFestival::SpringFestival); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = FlightEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = FuneralEtiquette::new(crate::rules::social::funeral::FuneralCulture::Chinese); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = GiftEtiquette::new("中国"); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = GolfEtiquetteRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = GreetingEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = HospitalEtiquetteRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = InterviewEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = LibraryEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = LiveStreamingRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = MahjongEtiquetteRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = NeighborRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = OnlineRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = PartyRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = PetEtiquetteRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = PhoneEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = QueueRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = SeatingEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = ShoppingEtiquetteRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = SmokingRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = SocialMediaEtiquetteRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = TeaEtiquette::new(crate::rules::social::tea::TeaCulture::Chinese); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = ToastingEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = TrainEtiquette::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = WeddingEtiquette::new(crate::rules::social::wedding::WeddingCulture::Chinese); rules.push(("social", r.metadata().clone(), r.category())); }
    { let r = WorkplaceRules::new(); rules.push(("social", r.metadata().clone(), r.category())); }
    rules
}
