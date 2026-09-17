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

pub mod accepting_feedback;
pub mod accompany_etiquette;
pub mod african_etiquette;
pub mod airplane_etiquette;
pub mod airplane_manner;
pub mod airport_etiquette;
pub mod alcohol_etiquette;
pub mod american_etiquette;
pub mod apartment_noise_considerate;
pub mod apology_etiquette;
pub mod arbor_day;
pub mod army_day;
pub mod asking_directions;
pub mod awards_ceremony_etiquette;
pub mod baby_shower_etiquette;
pub mod banquet_ceremony;
pub mod banquet_seating;
pub mod banquet_toast_etiquette;
pub mod bar;
pub mod barbecue_etiquette;
pub mod beach_etiquette;
pub mod birthday_celebration;
pub mod birthday_senior_feast;
pub mod boardroom_etiquette;
pub mod brazilian_etiquette;
pub mod british_etiquette;
pub mod buffet_dining;
pub mod bus_rider_etiquette;
pub mod bus_travel_protocol;
pub mod business;
pub mod business_card;
pub mod business_dress_code;
pub mod business_email;
pub mod business_gift;
pub mod business_networking;
pub mod business_phone;
pub mod cafe;
pub mod camping_etiquette;
pub mod carpool_etiquette;
pub mod charity_donation_etiquette;
pub mod childrens_day;
pub mod chinese_ancestor_worship;
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
pub mod chongyang_filial_etiquette;
pub mod cinema;
pub mod cocktail_party;
pub mod coffee_house;
pub mod community_meeting;
pub mod community_volunteer_activity;
pub mod concert_etiquette;
pub mod condolence_mourning;
pub mod condolence_visit;
pub mod confucian_etiquette;
pub mod constructive_criticism;
pub mod contract_signing_etiquette;
pub mod corporate_event_etiquette;
pub mod cross_cultural;
pub mod customer_service_etiquette;
pub mod cycling_etiquette;
pub mod dating_etiquette;
pub mod deaf_accessibility;
pub mod digital_detox;
pub mod dining;
pub mod dining_dress_code;
pub mod dinner_seating_order;
pub mod diplomatic_etiquette;
pub mod doorman_greeting_courtesy;
pub mod dragon_boat_festival;
pub mod elevator;
pub mod elevator_detailed;
pub mod elevator_luggage_etiquette;
pub mod email;
pub mod emoji_sticker_use;
pub mod escalator_etiquette;
pub mod festival;
pub mod festival_family_gathering;
pub mod fishing_etiquette;
pub mod flight;
pub mod food_festival;
pub mod formal_banquet;
pub mod formal_introduction;
pub mod french_etiquette;
pub mod funeral;
pub mod gaming_etiquette;
pub mod german_etiquette;
pub mod gift;
pub mod gift_unwrapping_etiquette;
pub mod golf_etiquette;
pub mod graduation_ceremony;
pub mod greeting;
pub mod grocery_supermarket_etiquette;
pub mod group_chat_netiquette;
pub mod guest_reception;
pub mod gym_etiquette;
pub mod gym_shared_etiquette;
pub mod high_speed_rail_manner;
pub mod hospital_etiquette;
pub mod hosting_dinner_party;
pub mod hotel_stay_etiquette;
pub mod hotel_stay_manner;
pub mod housewarming_etiquette;
pub mod housewarming_gift;
pub mod indian_etiquette;
pub mod international_conference;
pub mod international_gift;
pub mod international_greeting;
pub mod international_travel;
pub mod internship_etiquette;
pub mod interview;
pub mod investor_relations_etiquette;
pub mod japanese_cuisine;
pub mod japanese_etiquette;
pub mod korean_cuisine;
pub mod korean_etiquette;
pub mod labor_day;
pub mod leadership_etiquette;
pub mod library;
pub mod line_queue_etiquette;
pub mod live_streaming;
pub mod mahjong_etiquette;
pub mod market_etiquette;
pub mod media_relations_etiquette;
pub mod meeting_etiquette;
pub mod meeting_speech_politeness;
pub mod metro_etiquette;
pub mod mid_autumn;
pub mod middle_east_etiquette;
pub mod minimalism;
pub mod mountain_hiking_etiquette;
pub mod museum_etiquette;
pub mod national_day;
pub mod negotiation_etiquette;
pub mod neighbor;
pub mod neighbor_relations;
pub mod networking_etiquette;
pub mod new_year_day;
pub mod nursing_home_visit;
pub mod office_cooperation;
pub mod online;
pub mod online_classes_etiquette;
pub mod park_etiquette;
pub mod parking_courtesy;
pub mod partnership_etiquette;
pub mod party;
pub mod party_founding_day;
pub mod pedestrian_street_etiquette;
pub mod pet_etiquette;
pub mod pet_outdoor_etiquette;
pub mod pharmacy_etiquette;
pub mod phone;
pub mod phone_answering_etiquette;
pub mod photography_etiquette;
pub mod picnic_etiquette;
pub mod potluck_etiquette;
pub mod praise_compliment_etiquette;
pub mod presentation_etiquette;
pub mod project_management_etiquette;
pub mod property_management_service;
pub mod public_speaking_etiquette;
pub mod public_washroom_manner;
pub mod qingming;
pub mod qixi_festival;
pub mod queue;
pub mod quiet_ward_manner;
pub mod reading_room_etiquette;
pub mod reception_etiquette;
pub mod red_envelope_etiquette;
pub mod remote_work_etiquette;
pub mod reply_promptness;
pub mod report_to_superior;
pub mod resignation_etiquette;
pub mod restaurant_ordering_etiquette;
pub mod restaurant_tipping;
pub mod retirement_party_etiquette;
pub mod russian_etiquette;
pub mod sales_etiquette;
pub mod salon_etiquette;
pub mod seating;
pub mod shared_office_courtesy;
pub mod shopping_etiquette;
pub mod sick_visit_gift_note;
pub mod smoking;
pub mod social_media_etiquette;
pub mod spring_festival_visit;
pub mod station_hall_manner;
pub mod subway_etiquette;
pub mod supplier_relations_etiquette;
pub mod sustainable_living;
pub mod swimming_pool_etiquette;
pub mod sympathy_message_guide;
pub mod taxi_ride_etiquette;
pub mod tea;
pub mod tea_house;
pub mod tea_serving_etiquette;
pub mod teachers_day;
pub mod thai_etiquette;
pub mod theater_performance_etiquette;
pub mod toasting;
pub mod trade_show_etiquette;
pub mod train;
pub mod travel_buddy_manners;
pub mod travel_companion;
pub mod umbrella_etiquette;
pub mod vegan_etiquette;
pub mod vip_etiquette;
pub mod visit_patient_hospital;
pub mod voice_call_courtesy;
pub mod volunteer_etiquette;
pub mod volunteer_service_etiquette;
pub mod volunteer_service_manner;
pub mod waiting_room_etiquette;
pub mod wechat_chat_manner;
pub mod wedding;
pub mod western_dining;
pub mod wine_tasting;
pub mod wine_toast_etiquette;
pub mod womens_day;
pub mod work_email_protocol;
pub mod work_from_home;
pub mod workplace;
pub mod workplace_respect;
pub mod yuanxiao;

pub use accepting_feedback::AcceptingFeedbackRules;
pub use accompany_etiquette::AccompanyEtiquetteRules;
pub use african_etiquette::AfricanEtiquetteRules;
pub use airplane_etiquette::AirplaneEtiquetteRules;
pub use airplane_manner::AirplaneMannerRules;
pub use airport_etiquette::AirportEtiquetteRules;
pub use alcohol_etiquette::AlcoholEtiquetteRules;
pub use american_etiquette::AmericanEtiquetteRules;
pub use apartment_noise_considerate::ApartmentNoiseConsiderateRules;
pub use apology_etiquette::ApologyEtiquetteRules;
pub use arbor_day::ArborDayRules;
pub use army_day::ArmyDayRules;
pub use asking_directions::AskingDirectionsRules;
pub use awards_ceremony_etiquette::AwardsCeremonyEtiquetteRules;
pub use baby_shower_etiquette::BabyShowerEtiquetteRules;
pub use banquet_ceremony::BanquetCeremonyRules;
pub use banquet_seating::BanquetSeatingRules;
pub use banquet_toast_etiquette::BanquetToastEtiquetteRules;
pub use bar::BarRules;
pub use barbecue_etiquette::BarbecueEtiquetteRules;
pub use beach_etiquette::BeachEtiquetteRules;
pub use birthday_celebration::BirthdayCelebrationRules;
pub use birthday_senior_feast::BirthdaySeniorFeastRules;
pub use boardroom_etiquette::BoardroomEtiquetteRules;
pub use brazilian_etiquette::BrazilianEtiquetteRules;
pub use british_etiquette::BritishEtiquetteRules;
pub use buffet_dining::BuffetDiningRules;
pub use bus_rider_etiquette::BusRiderEtiquetteRules;
pub use bus_travel_protocol::BusTravelProtocolRules;
pub use business::BusinessEtiquette;
pub use business_card::BusinessCardRules;
pub use business_dress_code::BusinessDressCodeRules;
pub use business_email::BusinessEmailRules;
pub use business_gift::BusinessGiftRules;
pub use business_networking::BusinessNetworkingRules;
pub use business_phone::BusinessPhoneRules;
pub use cafe::CafeRules;
pub use camping_etiquette::CampingEtiquetteRules;
pub use carpool_etiquette::CarpoolEtiquetteRules;
pub use charity_donation_etiquette::CharityDonationEtiquetteRules;
pub use childrens_day::ChildrensDayRules;
pub use chinese_ancestor_worship::ChineseAncestorWorshipRules;
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
pub use chongyang_filial_etiquette::ChongyangFilialEtiquetteRules;
pub use cinema::CinemaEtiquette;
pub use cocktail_party::CocktailPartyRules;
pub use coffee_house::CoffeeHouseRules;
pub use community_meeting::CommunityMeetingRules;
pub use community_volunteer_activity::CommunityVolunteerActivityRules;
pub use concert_etiquette::ConcertEtiquetteRules;
pub use condolence_mourning::CondolenceMourningRules;
pub use condolence_visit::CondolenceVisitRules;
pub use confucian_etiquette::ConfucianEtiquetteRules;
pub use constructive_criticism::ConstructiveCriticismRules;
pub use contract_signing_etiquette::ContractSigningEtiquetteRules;
pub use corporate_event_etiquette::CorporateEventEtiquetteRules;
pub use cross_cultural::CrossCulturalRules;
pub use customer_service_etiquette::CustomerServiceEtiquetteRules;
pub use cycling_etiquette::CyclingEtiquetteRules;
pub use dating_etiquette::DatingEtiquetteRules;
pub use deaf_accessibility::DeafAccessibilityEtiquetteRules;
pub use digital_detox::DigitalDetoxRules;
pub use dining::{DiningCulture, DiningEtiquette};
pub use dining_dress_code::DiningDressCodeRules;
pub use dinner_seating_order::DinnerSeatingOrderRules;
pub use diplomatic_etiquette::DiplomaticEtiquetteRules;
pub use doorman_greeting_courtesy::DoormanGreetingCourtesyRules;
pub use dragon_boat_festival::DragonBoatFestivalRules;
pub use elevator::ElevatorEtiquette;
pub use elevator_detailed::ElevatorDetailedRules;
pub use elevator_luggage_etiquette::ElevatorLuggageEtiquetteRules;
pub use email::EmailEtiquette;
pub use emoji_sticker_use::EmojiStickerUseRules;
pub use escalator_etiquette::EscalatorEtiquetteRules;
pub use festival::{ChineseFestival, FestivalEtiquette};
pub use festival_family_gathering::FestivalFamilyGatheringRules;
pub use fishing_etiquette::FishingEtiquetteRules;
pub use flight::FlightEtiquette;
pub use food_festival::FoodFestivalRules;
pub use formal_banquet::FormalBanquetRules;
pub use formal_introduction::FormalIntroductionRules;
pub use french_etiquette::FrenchEtiquetteRules;
pub use funeral::{FuneralCulture, FuneralEtiquette};
pub use gaming_etiquette::GamingEtiquetteRules;
pub use german_etiquette::GermanEtiquetteRules;
pub use gift::GiftEtiquette;
pub use gift_unwrapping_etiquette::GiftUnwrappingEtiquetteRules;
pub use golf_etiquette::GolfEtiquetteRules;
pub use graduation_ceremony::GraduationCeremonyRules;
pub use greeting::GreetingEtiquette;
pub use grocery_supermarket_etiquette::GrocerySupermarketEtiquetteRules;
pub use group_chat_netiquette::GroupChatNetiquetteRules;
pub use guest_reception::GuestReceptionRules;
pub use gym_etiquette::GymEtiquetteRules;
pub use gym_shared_etiquette::GymSharedEtiquetteRules;
pub use high_speed_rail_manner::HighSpeedRailMannerRules;
pub use hospital_etiquette::HospitalEtiquetteRules;
pub use hosting_dinner_party::HostingDinnerPartyRules;
pub use hotel_stay_etiquette::HotelStayEtiquetteRules;
pub use hotel_stay_manner::HotelStayMannerRules;
pub use housewarming_etiquette::HousewarmingEtiquetteRules;
pub use housewarming_gift::HousewarmingGiftRules;
pub use indian_etiquette::IndianEtiquetteRules;
pub use international_conference::InternationalConferenceRules;
pub use international_gift::InternationalGiftRules;
pub use international_greeting::InternationalGreetingRules;
pub use international_travel::InternationalTravelRules;
pub use internship_etiquette::InternshipEtiquetteRules;
pub use interview::InterviewEtiquette;
pub use investor_relations_etiquette::InvestorRelationsEtiquetteRules;
pub use japanese_cuisine::JapaneseCuisineRules;
pub use japanese_etiquette::JapaneseEtiquetteRules;
pub use korean_cuisine::KoreanCuisineRules;
pub use korean_etiquette::KoreanEtiquetteRules;
pub use labor_day::LaborDayRules;
pub use leadership_etiquette::LeadershipEtiquetteRules;
pub use library::LibraryEtiquette;
pub use line_queue_etiquette::LineQueueEtiquetteRules;
pub use live_streaming::LiveStreamingRules;
pub use mahjong_etiquette::MahjongEtiquetteRules;
pub use market_etiquette::MarketEtiquetteRules;
pub use media_relations_etiquette::MediaRelationsEtiquetteRules;
pub use meeting_etiquette::MeetingEtiquetteRules;
pub use meeting_speech_politeness::MeetingSpeechPolitenessRules;
pub use metro_etiquette::MetroEtiquetteRules;
pub use mid_autumn::MidAutumnRules;
pub use middle_east_etiquette::MiddleEastEtiquetteRules;
pub use minimalism::MinimalismRules;
pub use mountain_hiking_etiquette::MountainHikingEtiquetteRules;
pub use museum_etiquette::MuseumEtiquetteRules;
pub use national_day::NationalDayRules;
pub use negotiation_etiquette::NegotiationEtiquetteRules;
pub use neighbor::NeighborRules;
pub use neighbor_relations::NeighborRelationsRules;
pub use networking_etiquette::NetworkingEtiquetteRules;
pub use new_year_day::NewYearDayRules;
pub use nursing_home_visit::NursingHomeVisitRules;
pub use office_cooperation::OfficeCooperationRules;
pub use online::OnlineRules;
pub use online_classes_etiquette::OnlineClassesEtiquetteRules;
pub use park_etiquette::ParkEtiquetteRules;
pub use parking_courtesy::ParkingCourtesyRules;
pub use partnership_etiquette::PartnershipEtiquetteRules;
pub use party::PartyRules;
pub use party_founding_day::PartyFoundingDayRules;
pub use pedestrian_street_etiquette::PedestrianStreetEtiquetteRules;
pub use pet_etiquette::PetEtiquetteRules;
pub use pet_outdoor_etiquette::PetOutdoorEtiquetteRules;
pub use pharmacy_etiquette::PharmacyEtiquetteRules;
pub use phone::PhoneEtiquette;
pub use phone_answering_etiquette::PhoneAnsweringEtiquetteRules;
pub use photography_etiquette::PhotographyEtiquetteRules;
pub use picnic_etiquette::PicnicEtiquetteRules;
pub use potluck_etiquette::PotluckEtiquetteRules;
pub use praise_compliment_etiquette::PraiseComplimentEtiquetteRules;
pub use presentation_etiquette::PresentationEtiquetteRules;
pub use project_management_etiquette::ProjectManagementEtiquetteRules;
pub use property_management_service::PropertyManagementServiceRules;
pub use public_speaking_etiquette::PublicSpeakingEtiquetteRules;
pub use public_washroom_manner::PublicWashroomMannerRules;
pub use qingming::QingmingRules;
pub use qixi_festival::QixiFestivalRules;
pub use queue::QueueRules;
pub use quiet_ward_manner::QuietWardMannerRules;
pub use reading_room_etiquette::ReadingRoomEtiquetteRules;
pub use reception_etiquette::ReceptionEtiquetteRules;
pub use red_envelope_etiquette::RedEnvelopeEtiquetteRules;
pub use remote_work_etiquette::RemoteWorkEtiquetteRules;
pub use reply_promptness::ReplyPromptnessRules;
pub use report_to_superior::ReportToSuperiorRules;
pub use resignation_etiquette::ResignationEtiquetteRules;
pub use restaurant_ordering_etiquette::RestaurantOrderingEtiquetteRules;
pub use restaurant_tipping::RestaurantTippingRules;
pub use retirement_party_etiquette::RetirementPartyEtiquetteRules;
pub use russian_etiquette::RussianEtiquetteRules;
pub use sales_etiquette::SalesEtiquetteRules;
pub use salon_etiquette::SalonEtiquetteRules;
pub use seating::SeatingEtiquette;
pub use shared_office_courtesy::SharedOfficeCourtesyRules;
pub use shopping_etiquette::ShoppingEtiquetteRules;
pub use sick_visit_gift_note::SickVisitGiftNoteRules;
pub use smoking::SmokingRules;
pub use social_media_etiquette::SocialMediaEtiquetteRules;
pub use spring_festival_visit::SpringFestivalVisitRules;
pub use station_hall_manner::StationHallMannerRules;
pub use subway_etiquette::SubwayEtiquetteRules;
pub use supplier_relations_etiquette::SupplierRelationsEtiquetteRules;
pub use sustainable_living::SustainableLivingRules;
pub use swimming_pool_etiquette::SwimmingPoolEtiquetteRules;
pub use sympathy_message_guide::SympathyMessageGuideRules;
pub use taxi_ride_etiquette::TaxiRideEtiquetteRules;
pub use tea::{TeaCulture, TeaEtiquette};
pub use tea_house::TeaHouseRules;
pub use tea_serving_etiquette::TeaServingEtiquetteRules;
pub use teachers_day::TeachersDayRules;
pub use thai_etiquette::ThaiEtiquetteRules;
pub use theater_performance_etiquette::TheaterPerformanceEtiquetteRules;
pub use toasting::ToastingEtiquette;
pub use trade_show_etiquette::TradeShowEtiquetteRules;
pub use train::TrainEtiquette;
pub use travel_buddy_manners::TravelBuddyMannersRules;
pub use travel_companion::TravelCompanionRules;
pub use umbrella_etiquette::UmbrellaEtiquetteRules;
pub use vegan_etiquette::VeganEtiquetteRules;
pub use vip_etiquette::VipEtiquetteRules;
pub use visit_patient_hospital::VisitPatientHospitalRules;
pub use voice_call_courtesy::VoiceCallCourtesyRules;
pub use volunteer_etiquette::VolunteerEtiquetteRules;
pub use volunteer_service_etiquette::VolunteerServiceEtiquetteRules;
pub use volunteer_service_manner::VolunteerServiceMannerRules;
pub use waiting_room_etiquette::WaitingRoomEtiquetteRules;
pub use wechat_chat_manner::WechatChatMannerRules;
pub use wedding::{WeddingCulture, WeddingEtiquette};
pub use western_dining::WesternDiningRules;
pub use wine_tasting::WineTastingRules;
pub use wine_toast_etiquette::WineToastEtiquetteRules;
pub use womens_day::WomensDayRules;
pub use work_email_protocol::WorkEmailProtocolRules;
pub use work_from_home::WorkFromHomeRules;
pub use workplace::WorkplaceRules;
pub use workplace_respect::WorkplaceRespectRules;
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
        let r = DoormanGreetingCourtesyRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ApartmentNoiseConsiderateRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CommunityVolunteerActivityRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PropertyManagementServiceRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NeighborRelationsRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = QuietWardMannerRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SickVisitGiftNoteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SympathyMessageGuideRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CondolenceMourningRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VisitPatientHospitalRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AskingDirectionsRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TravelCompanionRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BusTravelProtocolRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HighSpeedRailMannerRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MetroEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WorkplaceRespectRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WorkEmailProtocolRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ReportToSuperiorRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MeetingSpeechPolitenessRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OfficeCooperationRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DinnerSeatingOrderRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HousewarmingGiftRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BirthdaySeniorFeastRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WineToastEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BanquetCeremonyRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ReplyPromptnessRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EmojiStickerUseRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VoiceCallCourtesyRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GroupChatNetiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WechatChatMannerRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VolunteerServiceMannerRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SharedOfficeCourtesyRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PublicWashroomMannerRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TravelBuddyMannersRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HotelStayMannerRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GymSharedEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AirplaneMannerRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ParkingCourtesyRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = StationHallMannerRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LineQueueEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FestivalFamilyGatheringRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChongyangFilialEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CondolenceVisitRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RedEnvelopeEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SpringFestivalVisitRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RestaurantOrderingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BanquetSeatingRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GiftUnwrappingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DiningDressCodeRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PotluckEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConstructiveCriticismRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElevatorLuggageEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AcceptingFeedbackRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PraiseComplimentEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TheaterPerformanceEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PedestrianStreetEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EscalatorEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaxiRideEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AirportEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BusRiderEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GrocerySupermarketEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ReadingRoomEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SalonEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PharmacyEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BirthdayCelebrationRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RetirementPartyEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BabyShowerEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HousewarmingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HostingDinnerPartyRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VolunteerServiceEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CharityDonationEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NursingHomeVisitRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = UmbrellaEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CommunityMeetingRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PublicSpeakingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PhoneAnsweringEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RestaurantTippingRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DeafAccessibilityEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AwardsCeremonyEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MountainHikingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FishingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GraduationCeremonyRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MarketEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PetOutdoorEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FormalIntroductionRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BanquetToastEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TeaServingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GuestReceptionRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WaitingRoomEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HotelStayEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CarpoolEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PicnicEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BarbecueEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ParkEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SubwayEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OnlineClassesEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PhotographyEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CampingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CyclingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GamingEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SwimmingPoolEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BeachEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConcertEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MuseumEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GymEtiquetteRules::new();
        rules.push(("social", r.metadata().clone(), r.category(), r.explain()));
    }
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
