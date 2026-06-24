//! 社交礼仪规则模块

pub mod african_etiquette;
pub mod airplane_etiquette;
pub mod alcohol_etiquette;
pub mod american_etiquette;
pub mod apology_etiquette;
pub mod bar;
pub mod brazilian_etiquette;
pub mod british_etiquette;
pub mod business;
pub mod cafe;
pub mod chinese_ancestor_worship;
pub mod chinese_business;
pub mod chinese_calligraphy;
pub mod chinese_chess_etiquette;
pub mod chinese_dining;
pub mod chinese_festival_food;
pub mod chinese_funeral;
pub mod chinese_gift;
pub mod chinese_greeting;
pub mod chinese_housewarming;
pub mod chinese_kowtow;
pub mod chinese_mahjong_etiquette;
pub mod chinese_martial_arts_etiquette;
pub mod chinese_medicine_etiquette;
pub mod chinese_new_year;
pub mod chinese_seating;
pub mod chinese_tea_ceremony;
pub mod chinese_temple;
pub mod chinese_wedding;
pub mod chongyang;
pub mod cinema;
pub mod confucian_etiquette;
pub mod cross_cultural;
pub mod customer_service_etiquette;
pub mod dating_etiquette;
pub mod digital_detox;
pub mod dining;
pub mod dragon_boat_festival;
pub mod elevator;
pub mod elevator_detailed;
pub mod email;
pub mod festival;
pub mod flight;
pub mod french_etiquette;
pub mod funeral;
pub mod german_etiquette;
pub mod gift;
pub mod golf_etiquette;
pub mod greeting;
pub mod hospital_etiquette;
pub mod indian_etiquette;
pub mod internship_etiquette;
pub mod interview;
pub mod japanese_etiquette;
pub mod korean_etiquette;
pub mod leadership_etiquette;
pub mod library;
pub mod live_streaming;
pub mod mahjong_etiquette;
pub mod meeting_etiquette;
pub mod mid_autumn;
pub mod middle_east_etiquette;
pub mod minimalism;
pub mod neighbor;
pub mod networking_etiquette;
pub mod online;
pub mod party;
pub mod pet_etiquette;
pub mod phone;
pub mod presentation_etiquette;
pub mod qingming;
pub mod queue;
pub mod remote_work_etiquette;
pub mod resignation_etiquette;
pub mod russian_etiquette;
pub mod seating;
pub mod shopping_etiquette;
pub mod smoking;
pub mod social_media_etiquette;
pub mod sustainable_living;
pub mod tea;
pub mod thai_etiquette;
pub mod toasting;
pub mod train;
pub mod vegan_etiquette;
pub mod vip_etiquette;
pub mod volunteer_etiquette;
pub mod wedding;
pub mod work_from_home;
pub mod workplace;
pub mod yuanxiao;

pub use african_etiquette::AfricanEtiquetteRules;
pub use airplane_etiquette::AirplaneEtiquetteRules;
pub use alcohol_etiquette::AlcoholEtiquetteRules;
pub use american_etiquette::AmericanEtiquetteRules;
pub use apology_etiquette::ApologyEtiquetteRules;
pub use bar::BarRules;
pub use brazilian_etiquette::BrazilianEtiquetteRules;
pub use british_etiquette::BritishEtiquetteRules;
pub use business::BusinessEtiquette;
pub use cafe::CafeRules;
pub use chinese_ancestor_worship::ChineseAncestorWorshipRules;
pub use chinese_business::ChineseBusinessRules;
pub use chinese_calligraphy::ChineseCalligraphyRules;
pub use chinese_chess_etiquette::ChineseChessEtiquetteRules;
pub use chinese_dining::ChineseDiningRules;
pub use chinese_festival_food::ChineseFestivalFoodRules;
pub use chinese_funeral::ChineseFuneralRules;
pub use chinese_gift::ChineseGiftRules;
pub use chinese_greeting::ChineseGreetingRules;
pub use chinese_housewarming::ChineseHousewarmingRules;
pub use chinese_kowtow::ChineseKowtowRules;
pub use chinese_mahjong_etiquette::ChineseMahjongEtiquetteRules;
pub use chinese_martial_arts_etiquette::ChineseMartialArtsEtiquetteRules;
pub use chinese_medicine_etiquette::ChineseMedicineEtiquetteRules;
pub use chinese_new_year::ChineseNewYearRules;
pub use chinese_seating::ChineseSeatingRules;
pub use chinese_tea_ceremony::ChineseTeaCeremonyRules;
pub use chinese_temple::ChineseTempleRules;
pub use chinese_wedding::ChineseWeddingRules;
pub use chongyang::ChongyangRules;
pub use cinema::CinemaEtiquette;
pub use confucian_etiquette::ConfucianEtiquetteRules;
pub use cross_cultural::CrossCulturalRules;
pub use customer_service_etiquette::CustomerServiceEtiquetteRules;
pub use dating_etiquette::DatingEtiquetteRules;
pub use digital_detox::DigitalDetoxRules;
pub use dining::{DiningCulture, DiningEtiquette};
pub use dragon_boat_festival::DragonBoatFestivalRules;
pub use elevator::ElevatorEtiquette;
pub use elevator_detailed::ElevatorDetailedRules;
pub use email::EmailEtiquette;
pub use festival::{ChineseFestival, FestivalEtiquette};
pub use flight::FlightEtiquette;
pub use french_etiquette::FrenchEtiquetteRules;
pub use funeral::{FuneralCulture, FuneralEtiquette};
pub use german_etiquette::GermanEtiquetteRules;
pub use gift::GiftEtiquette;
pub use golf_etiquette::GolfEtiquetteRules;
pub use greeting::GreetingEtiquette;
pub use hospital_etiquette::HospitalEtiquetteRules;
pub use indian_etiquette::IndianEtiquetteRules;
pub use internship_etiquette::InternshipEtiquetteRules;
pub use interview::InterviewEtiquette;
pub use japanese_etiquette::JapaneseEtiquetteRules;
pub use korean_etiquette::KoreanEtiquetteRules;
pub use leadership_etiquette::LeadershipEtiquetteRules;
pub use library::LibraryEtiquette;
pub use live_streaming::LiveStreamingRules;
pub use mahjong_etiquette::MahjongEtiquetteRules;
pub use meeting_etiquette::MeetingEtiquetteRules;
pub use mid_autumn::MidAutumnRules;
pub use middle_east_etiquette::MiddleEastEtiquetteRules;
pub use minimalism::MinimalismRules;
pub use neighbor::NeighborRules;
pub use networking_etiquette::NetworkingEtiquetteRules;
pub use online::OnlineRules;
pub use party::PartyRules;
pub use pet_etiquette::PetEtiquetteRules;
pub use phone::PhoneEtiquette;
pub use presentation_etiquette::PresentationEtiquetteRules;
pub use qingming::QingmingRules;
pub use queue::QueueRules;
pub use remote_work_etiquette::RemoteWorkEtiquetteRules;
pub use resignation_etiquette::ResignationEtiquetteRules;
pub use russian_etiquette::RussianEtiquetteRules;
pub use seating::SeatingEtiquette;
pub use shopping_etiquette::ShoppingEtiquetteRules;
pub use smoking::SmokingRules;
pub use social_media_etiquette::SocialMediaEtiquetteRules;
pub use sustainable_living::SustainableLivingRules;
pub use tea::{TeaCulture, TeaEtiquette};
pub use thai_etiquette::ThaiEtiquetteRules;
pub use toasting::ToastingEtiquette;
pub use train::TrainEtiquette;
pub use vegan_etiquette::VeganEtiquetteRules;
pub use vip_etiquette::VipEtiquetteRules;
pub use volunteer_etiquette::VolunteerEtiquetteRules;
pub use wedding::{WeddingCulture, WeddingEtiquette};
pub use work_from_home::WorkFromHomeRules;
pub use workplace::WorkplaceRules;
pub use yuanxiao::YuanxiaoRules;

pub fn all_rules() -> Vec<(
    &'static str,
    crate::rules::core::RuleMetadata,
    crate::rules::core::RuleCategory,
    String,
)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    {
        let r = AirplaneEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BarRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BusinessEtiquette::new("中国");
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CafeRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CinemaEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DatingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DiningEtiquette::new(crate::rules::social::dining::DiningCulture::Chinese);
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElevatorDetailedRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElevatorEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EmailEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r =
            FestivalEtiquette::new(crate::rules::social::festival::ChineseFestival::SpringFestival);
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FlightEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FuneralEtiquette::new(crate::rules::social::funeral::FuneralCulture::Chinese);
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GiftEtiquette::new("中国");
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GolfEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GreetingEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HospitalEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InterviewEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LibraryEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LiveStreamingRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MahjongEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NeighborRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OnlineRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PartyRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PetEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PhoneEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = QueueRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SeatingEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ShoppingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SmokingRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SocialMediaEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TeaEtiquette::new(crate::rules::social::tea::TeaCulture::Chinese);
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ToastingEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TrainEtiquette::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WeddingEtiquette::new(crate::rules::social::wedding::WeddingCulture::Chinese);
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WorkplaceRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    rules
}
