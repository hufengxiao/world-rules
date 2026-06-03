//! 法律法规模块

pub mod civil;
pub mod constitution;
pub mod consumer;
pub mod contract;
pub mod criminal;
pub mod inheritance;
pub mod intellectual_property;
pub mod labor;
pub mod marriage;
pub mod road_safety;
pub mod traffic;

// 商法类
pub mod banking;
pub mod bankruptcy;
pub mod company;
pub mod insurance;
pub mod maritime;
pub mod securities;

// 经济法类
pub mod advertising;
pub mod antimonopoly;
pub mod bidding;
pub mod customs;
pub mod foreign_trade;
pub mod patent;
pub mod price;
pub mod tax;
pub mod trademark;

// 行政法类
pub mod administrative;
pub mod agriculture;
pub mod archives;
pub mod construction;
pub mod cybersecurity;
pub mod data_security;
pub mod earthquake_prevention;
pub mod environmental;
pub mod fire_protection;
pub mod food_safety;
pub mod land_administration;
pub mod meteorology;
pub mod safety_production;
pub mod statistics;
pub mod water;

// 社会法类
pub mod consumer_extended;
pub mod education;
pub mod labor_extended;
pub mod medical;
pub mod property_management;
pub mod real_estate;
pub mod social_insurance;
pub mod tourism;

// 程序法类
pub mod administrative_procedure;
pub mod arbitration;
pub mod civil_procedure;
pub mod criminal_procedure;
pub mod lawyer;
pub mod notarization;

// 知识产权类
pub mod copyright;

// 投资法类
pub mod foreign_investment;

// 特殊群体保护法类
pub mod children_protection;
pub mod disability_protection;
pub mod elderly_protection;
pub mod women_protection;

// 慈善法类
pub mod charity;

// 应急管理法类
pub mod emergency_response;

// 医药法类
pub mod drug_management;
pub mod vaccine_management;

// 交通能源法类
pub mod civil_aviation;
pub mod electricity;
pub mod post;
pub mod railway;
pub mod telecommunications;

// 资源法类
pub mod forest;
pub mod mineral_resources;

pub use civil::CivilLawRules;
pub use constitution::ConstitutionRules;
pub use consumer::ConsumerLawRules;
pub use contract::ContractRules;
pub use criminal::CriminalLawRules;
pub use inheritance::InheritanceLawRules;
pub use intellectual_property::IPRules;
pub use labor::LaborLawRules;
pub use marriage::MarriageLawRules;
pub use road_safety::RoadSafetyRules;
pub use traffic::{TrafficRegion, TrafficRules};

// 商法类
pub use banking::BankingLawRules;
pub use bankruptcy::BankruptcyLawRules;
pub use company::CompanyLawRules;
pub use insurance::InsuranceLawRules;
pub use maritime::MaritimeLawRules;
pub use securities::SecuritiesLawRules;

// 经济法类
pub use advertising::AdvertisingLawRules;
pub use antimonopoly::AntimonopolyLawRules;
pub use bidding::BiddingLawRules;
pub use customs::CustomsLawRules;
pub use foreign_trade::ForeignTradeLawRules;
pub use patent::PatentLawRules;
pub use price::PriceLawRules;
pub use tax::TaxLawRules;
pub use trademark::TrademarkLawRules;

// 行政法类
pub use administrative::AdministrativeLawRules;
pub use agriculture::AgricultureLawRules;
pub use archives::ArchivesLawRules;
pub use construction::ConstructionLawRules;
pub use cybersecurity::CybersecurityLawRules;
pub use data_security::DataSecurityLawRules;
pub use earthquake_prevention::EarthquakePreventionLawRules;
pub use environmental::EnvironmentalLawRules;
pub use fire_protection::FireProtectionLawRules;
pub use food_safety::FoodSafetyLawRules;
pub use land_administration::LandAdministrationLawRules;
pub use meteorology::MeteorologyLawRules;
pub use safety_production::SafetyProductionLawRules;
pub use statistics::StatisticsLawRules;
pub use water::WaterLawRules;

// 社会法类
pub use consumer_extended::ConsumerLawExtendedRules;
pub use education::EducationLawRules;
pub use labor_extended::LaborLawExtendedRules;
pub use medical::MedicalLawRules;
pub use property_management::PropertyManagementLawRules;
pub use real_estate::RealEstateLawRules;
pub use social_insurance::SocialInsuranceLawRules;
pub use tourism::TourismLawRules;

// 程序法类
pub use administrative_procedure::AdministrativeProcedureLawRules;
pub use arbitration::ArbitrationLawRules;
pub use civil_procedure::CivilProcedureLawRules;
pub use criminal_procedure::CriminalProcedureLawRules;
pub use lawyer::LawyerLawRules;
pub use notarization::NotarizationLawRules;

// 知识产权类
pub use copyright::CopyrightLawRules;

// 投资法类
pub use foreign_investment::ForeignInvestmentLawRules;

// 特殊群体保护法类
pub use children_protection::ChildrenProtectionLawRules;
pub use disability_protection::DisabilityProtectionLawRules;
pub use elderly_protection::ElderlyProtectionLawRules;
pub use women_protection::WomenProtectionLawRules;

// 慈善法类
pub use charity::CharityLawRules;

// 应急管理法类
pub use emergency_response::EmergencyResponseLawRules;

// 医药法类
pub use drug_management::DrugManagementLawRules;
pub use vaccine_management::VaccineManagementLawRules;

// 交通能源法类
pub use civil_aviation::CivilAviationLawRules;
pub use electricity::ElectricityLawRules;
pub use post::PostLawRules;
pub use railway::RailwayLawRules;
pub use telecommunications::TelecommunicationsLawRules;

// 资源法类
pub use forest::ForestLawRules;
pub use mineral_resources::MineralResourcesLawRules;
