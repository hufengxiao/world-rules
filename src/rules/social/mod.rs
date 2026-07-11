//! 社交礼仪规则模块 - 涵盖各类社交礼仪和人际交往规则
//!
//! 本模块包含丰富的社交礼仪规则，覆盖：
//! - **中华文化礼仪**: 传统礼仪、节日礼仪、餐饮礼仪等
//! - **国际礼仪**: 商务礼仪、西餐礼仪、各国文化礼仪
//! - **社交场合礼仪**: 婚礼、葬礼、宴会、会议等
//! - **职场礼仪**: 面试、办公、邮件、电话礼仪
//! - **现代礼仪**: 网络礼仪、社交媒体礼仪、远程办公礼仪
//!
//! # 模块结构
//!
//! ```text
//! social/
//! ├── chinese_*       # 中华文化礼仪系列
//! ├── business        # 商务礼仪
//! ├── dining          # 餐饮礼仪
//! ├── wedding         # 婚礼礼仪
//! ├── funeral         # 葬礼礼仪
//! ├── interview       # 面试礼仪
//! ├── email           # 邮件礼仪
//! ├── phone           # 电话礼仪
//! └── online          # 网络礼仪
//! ```
//!
//! # Examples
//!
//! 使用规则示例：
//!
//! ```rust
//! use world_rules::rules::social::{BusinessEtiquette, DiningEtiquette};
//! use world_rules::rules::core::Rule;
//!
//! // 商务礼仪规则
//! let business = BusinessEtiquette::new("中国");
//! println!("规则: {}", business.metadata().name);
//! println!("分类: {:?}", business.category());
//!
//! // 餐饮礼仪规则
//! let dining = DiningEtiquette::new(world_rules::rules::social::dining::DiningCulture::Chinese);
//! let explanation = dining.explain();
//! assert!(!explanation.is_empty());
//! ```
//!
//! # 规则统计
//!
//! 当前包含数十条社交礼仪规则，覆盖：
//! - 20+ 种中华文化礼仪
//! - 15+ 种国际礼仪
//! - 20+ 种社交场合礼仪
//! - 10+ 种职场礼仪
//! - 10+ 种现代礼仪

pub mod accompany_etiquette;
pub mod african_etiquette;
pub mod airplane_etiquette;
pub mod alcohol_etiquette;
pub mod american_etiquette;
pub mod apology_etiquette;
pub mod arbor_day;
pub mod army_day;
pub mod bar;
pub mod boardroom_etiquette;
pub mod brazilian_etiquette;
pub mod british_etiquette;
pub mod buffet_dining;
pub mod business;
pub mod business_card;
pub mod business_dress_code;
pub mod business_email;
pub mod business_gift;
pub mod business_networking;
pub mod business_phone;
pub mod cafe;
pub mod childrens_day;
pub mod chinese_ancestor_worship;
pub mod coffee_house;
pub mod chinese_antique_etiquette;
pub mod chinese_architecture_etiquette;
pub mod chinese_birthday_etiquette;
pub mod chinese_book_collection;
pub mod chinese_business;
pub mod chinese_calligraphy;
pub mod chinese_chess_etiquette;
pub mod chinese_clan_etiquette;
pub mod chinese_coming_of_age;
pub mod chinese_correspondence;
pub mod chinese_dining;
pub mod chinese_etiquette_basics;
pub mod chinese_festival_food;
pub mod chinese_funeral;
pub mod chinese_gift;
pub mod chinese_greeting;
pub mod chinese_housewarming;
pub mod chinese_interpersonal_etiquette;
pub mod chinese_kowtow;
pub mod chinese_mahjong_etiquette;
pub mod chinese_martial_arts_etiquette;
pub mod chinese_medicine_etiquette;
pub mod chinese_new_year;
pub mod chinese_official_etiquette;
pub mod chinese_ritual_sacrifice;
pub mod chinese_seating;
pub mod chinese_taboo;
pub mod chinese_tea_ceremony;
pub mod chinese_temple;
pub mod chinese_traditional_dress;
pub mod chinese_wedding;
pub mod chongyang;
pub mod cinema;
pub mod confucian_etiquette;
pub mod contract_signing_etiquette;
pub mod cocktail_party;
pub mod corporate_event_etiquette;
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
pub mod food_festival;
pub mod formal_banquet;
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
pub mod investor_relations_etiquette;
pub mod japanese_etiquette;
pub mod japanese_cuisine;
pub mod korean_etiquette;
pub mod korean_cuisine;
pub mod labor_day;
pub mod leadership_etiquette;
pub mod library;
pub mod live_streaming;
pub mod mahjong_etiquette;
pub mod media_relations_etiquette;
pub mod meeting_etiquette;
pub mod mid_autumn;
pub mod middle_east_etiquette;
pub mod minimalism;
pub mod national_day;
pub mod negotiation_etiquette;
pub mod neighbor;
pub mod networking_etiquette;
pub mod new_year_day;
pub mod online;
pub mod partnership_etiquette;
pub mod party;
pub mod party_founding_day;
pub mod pet_etiquette;
pub mod phone;
pub mod presentation_etiquette;
pub mod project_management_etiquette;
pub mod qingming;
pub mod qixi_festival;
pub mod queue;
pub mod reception_etiquette;
pub mod remote_work_etiquette;
pub mod resignation_etiquette;
pub mod russian_etiquette;
pub mod sales_etiquette;
pub mod seating;
pub mod shopping_etiquette;
pub mod smoking;
pub mod social_media_etiquette;
pub mod supplier_relations_etiquette;
pub mod sustainable_living;
pub mod tea;
pub mod tea_house;
pub mod teachers_day;
pub mod thai_etiquette;
pub mod toasting;
pub mod trade_show_etiquette;
pub mod train;
pub mod vegan_etiquette;
pub mod vip_etiquette;
pub mod wine_tasting;
pub mod volunteer_etiquette;
pub mod wedding;
pub mod western_dining;
pub mod womens_day;
pub mod work_from_home;
pub mod workplace;
pub mod yuanxiao;

pub use accompany_etiquette::AccompanyEtiquetteRules;
pub use african_etiquette::AfricanEtiquetteRules;
pub use airplane_etiquette::AirplaneEtiquetteRules;
pub use alcohol_etiquette::AlcoholEtiquetteRules;
pub use american_etiquette::AmericanEtiquetteRules;
pub use apology_etiquette::ApologyEtiquetteRules;
pub use arbor_day::ArborDayRules;
pub use army_day::ArmyDayRules;
pub use bar::BarRules;
pub use boardroom_etiquette::BoardroomEtiquetteRules;
pub use brazilian_etiquette::BrazilianEtiquetteRules;
pub use british_etiquette::BritishEtiquetteRules;
pub use buffet_dining::BuffetDiningRules;
pub use business::BusinessEtiquette;
pub use business_card::BusinessCardRules;
pub use business_dress_code::BusinessDressCodeRules;
pub use business_email::BusinessEmailRules;
pub use business_gift::BusinessGiftRules;
pub use business_networking::BusinessNetworkingRules;
pub use business_phone::BusinessPhoneRules;
pub use cafe::CafeRules;
pub use childrens_day::ChildrensDayRules;
pub use chinese_ancestor_worship::ChineseAncestorWorshipRules;
pub use coffee_house::CoffeeHouseRules;
pub use chinese_antique_etiquette::ChineseAntiqueEtiquetteRules;
pub use chinese_architecture_etiquette::ChineseArchitectureEtiquetteRules;
pub use chinese_birthday_etiquette::ChineseBirthdayEtiquetteRules;
pub use chinese_book_collection::ChineseBookCollectionRules;
pub use chinese_business::ChineseBusinessRules;
pub use chinese_calligraphy::ChineseCalligraphyRules;
pub use chinese_chess_etiquette::ChineseChessEtiquetteRules;
pub use chinese_clan_etiquette::ChineseClanEtiquetteRules;
pub use chinese_coming_of_age::ChineseComingOfAgeRules;
pub use chinese_correspondence::ChineseCorrespondenceRules;
pub use chinese_dining::ChineseDiningRules;
pub use chinese_etiquette_basics::ChineseEtiquetteBasicsRules;
pub use chinese_festival_food::ChineseFestivalFoodRules;
pub use chinese_funeral::ChineseFuneralRules;
pub use chinese_gift::ChineseGiftRules;
pub use chinese_greeting::ChineseGreetingRules;
pub use chinese_housewarming::ChineseHousewarmingRules;
pub use chinese_interpersonal_etiquette::ChineseInterpersonalEtiquetteRules;
pub use chinese_kowtow::ChineseKowtowRules;
pub use chinese_mahjong_etiquette::ChineseMahjongEtiquetteRules;
pub use chinese_martial_arts_etiquette::ChineseMartialArtsEtiquetteRules;
pub use chinese_medicine_etiquette::ChineseMedicineEtiquetteRules;
pub use chinese_new_year::ChineseNewYearRules;
pub use chinese_official_etiquette::ChineseOfficialEtiquetteRules;
pub use chinese_ritual_sacrifice::ChineseRitualSacrificeRules;
pub use chinese_seating::ChineseSeatingRules;
pub use chinese_taboo::ChineseTabooRules;
pub use chinese_tea_ceremony::ChineseTeaCeremonyRules;
pub use chinese_temple::ChineseTempleRules;
pub use chinese_traditional_dress::ChineseTraditionalDressRules;
pub use chinese_wedding::ChineseWeddingRules;
pub use chongyang::ChongyangRules;
pub use cinema::CinemaEtiquette;
pub use confucian_etiquette::ConfucianEtiquetteRules;
pub use contract_signing_etiquette::ContractSigningEtiquetteRules;
pub use cocktail_party::CocktailPartyRules;
pub use corporate_event_etiquette::CorporateEventEtiquetteRules;
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
pub use food_festival::FoodFestivalRules;
pub use formal_banquet::FormalBanquetRules;
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
pub use investor_relations_etiquette::InvestorRelationsEtiquetteRules;
pub use japanese_etiquette::JapaneseEtiquetteRules;
pub use japanese_cuisine::JapaneseCuisineRules;
pub use korean_etiquette::KoreanEtiquetteRules;
pub use korean_cuisine::KoreanCuisineRules;
pub use labor_day::LaborDayRules;
pub use leadership_etiquette::LeadershipEtiquetteRules;
pub use library::LibraryEtiquette;
pub use live_streaming::LiveStreamingRules;
pub use mahjong_etiquette::MahjongEtiquetteRules;
pub use media_relations_etiquette::MediaRelationsEtiquetteRules;
pub use meeting_etiquette::MeetingEtiquetteRules;
pub use mid_autumn::MidAutumnRules;
pub use middle_east_etiquette::MiddleEastEtiquetteRules;
pub use minimalism::MinimalismRules;
pub use national_day::NationalDayRules;
pub use negotiation_etiquette::NegotiationEtiquetteRules;
pub use neighbor::NeighborRules;
pub use networking_etiquette::NetworkingEtiquetteRules;
pub use new_year_day::NewYearDayRules;
pub use online::OnlineRules;
pub use partnership_etiquette::PartnershipEtiquetteRules;
pub use party::PartyRules;
pub use party_founding_day::PartyFoundingDayRules;
pub use pet_etiquette::PetEtiquetteRules;
pub use phone::PhoneEtiquette;
pub use presentation_etiquette::PresentationEtiquetteRules;
pub use project_management_etiquette::ProjectManagementEtiquetteRules;
pub use qingming::QingmingRules;
pub use qixi_festival::QixiFestivalRules;
pub use queue::QueueRules;
pub use reception_etiquette::ReceptionEtiquetteRules;
pub use remote_work_etiquette::RemoteWorkEtiquetteRules;
pub use resignation_etiquette::ResignationEtiquetteRules;
pub use russian_etiquette::RussianEtiquetteRules;
pub use sales_etiquette::SalesEtiquetteRules;
pub use seating::SeatingEtiquette;
pub use shopping_etiquette::ShoppingEtiquetteRules;
pub use smoking::SmokingRules;
pub use social_media_etiquette::SocialMediaEtiquetteRules;
pub use supplier_relations_etiquette::SupplierRelationsEtiquetteRules;
pub use sustainable_living::SustainableLivingRules;
pub use tea::{TeaCulture, TeaEtiquette};
pub use tea_house::TeaHouseRules;
pub use teachers_day::TeachersDayRules;
pub use thai_etiquette::ThaiEtiquetteRules;
pub use toasting::ToastingEtiquette;
pub use trade_show_etiquette::TradeShowEtiquetteRules;
pub use train::TrainEtiquette;
pub use vegan_etiquette::VeganEtiquetteRules;
pub use vip_etiquette::VipEtiquetteRules;
pub use wine_tasting::WineTastingRules;
pub use volunteer_etiquette::VolunteerEtiquetteRules;
pub use wedding::{WeddingCulture, WeddingEtiquette};
pub use western_dining::WesternDiningRules;
pub use womens_day::WomensDayRules;
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