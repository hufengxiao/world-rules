//! 法律法规模块

pub mod traffic;
pub mod contract;
pub mod labor;
pub mod consumer;
pub mod intellectual_property;
pub mod road_safety;
pub mod marriage;
pub mod inheritance;
pub mod criminal;
pub mod civil;
pub mod constitution;

// 商法类
pub mod company;
pub mod securities;
pub mod bankruptcy;
pub mod insurance;
pub mod banking;
pub mod maritime;

// 经济法类
pub mod tax;
pub mod antimonopoly;
pub mod foreign_trade;
pub mod customs;
pub mod patent;
pub mod trademark;
pub mod advertising;
pub mod bidding;
pub mod price;

// 行政法类
pub mod administrative;
pub mod environmental;
pub mod construction;
pub mod food_safety;
pub mod cybersecurity;
pub mod data_security;
pub mod safety_production;
pub mod fire_protection;
pub mod land_administration;
pub mod water;
pub mod agriculture;
pub mod meteorology;
pub mod earthquake_prevention;
pub mod archives;
pub mod statistics;

// 社会法类
pub mod labor_extended;
pub mod education;
pub mod medical;
pub mod consumer_extended;
pub mod real_estate;
pub mod social_insurance;
pub mod tourism;
pub mod property_management;

// 程序法类
pub mod arbitration;
pub mod civil_procedure;
pub mod criminal_procedure;
pub mod administrative_procedure;
pub mod lawyer;
pub mod notarization;

// 知识产权类
pub mod copyright;

// 投资法类
pub mod foreign_investment;

// 特殊群体保护法类
pub mod children_protection;
pub mod women_protection;
pub mod elderly_protection;
pub mod disability_protection;

// 慈善法类
pub mod charity;

// 应急管理法类
pub mod emergency_response;

// 医药法类
pub mod drug_management;
pub mod vaccine_management;

// 交通能源法类
pub mod electricity;
pub mod railway;
pub mod civil_aviation;
pub mod post;
pub mod telecommunications;

// 资源法类
pub mod forest;
pub mod mineral_resources;

pub use traffic::{TrafficRules, TrafficRegion};
pub use contract::ContractRules;
pub use labor::LaborLawRules;
pub use consumer::ConsumerLawRules;
pub use intellectual_property::IPRules;
pub use road_safety::RoadSafetyRules;
pub use marriage::MarriageLawRules;
pub use inheritance::InheritanceLawRules;
pub use criminal::CriminalLawRules;
pub use civil::CivilLawRules;
pub use constitution::ConstitutionRules;

// 商法类
pub use company::CompanyLawRules;
pub use securities::SecuritiesLawRules;
pub use bankruptcy::BankruptcyLawRules;
pub use insurance::InsuranceLawRules;
pub use banking::BankingLawRules;
pub use maritime::MaritimeLawRules;

// 经济法类
pub use tax::TaxLawRules;
pub use antimonopoly::AntimonopolyLawRules;
pub use foreign_trade::ForeignTradeLawRules;
pub use customs::CustomsLawRules;
pub use patent::PatentLawRules;
pub use trademark::TrademarkLawRules;
pub use advertising::AdvertisingLawRules;
pub use bidding::BiddingLawRules;
pub use price::PriceLawRules;

// 行政法类
pub use administrative::AdministrativeLawRules;
pub use environmental::EnvironmentalLawRules;
pub use construction::ConstructionLawRules;
pub use food_safety::FoodSafetyLawRules;
pub use cybersecurity::CybersecurityLawRules;
pub use data_security::DataSecurityLawRules;
pub use safety_production::SafetyProductionLawRules;
pub use fire_protection::FireProtectionLawRules;
pub use land_administration::LandAdministrationLawRules;
pub use water::WaterLawRules;
pub use agriculture::AgricultureLawRules;
pub use meteorology::MeteorologyLawRules;
pub use earthquake_prevention::EarthquakePreventionLawRules;
pub use archives::ArchivesLawRules;
pub use statistics::StatisticsLawRules;

// 社会法类
pub use labor_extended::LaborLawExtendedRules;
pub use education::EducationLawRules;
pub use medical::MedicalLawRules;
pub use consumer_extended::ConsumerLawExtendedRules;
pub use real_estate::RealEstateLawRules;
pub use social_insurance::SocialInsuranceLawRules;
pub use tourism::TourismLawRules;
pub use property_management::PropertyManagementLawRules;

// 程序法类
pub use arbitration::ArbitrationLawRules;
pub use civil_procedure::CivilProcedureLawRules;
pub use criminal_procedure::CriminalProcedureLawRules;
pub use administrative_procedure::AdministrativeProcedureLawRules;
pub use lawyer::LawyerLawRules;
pub use notarization::NotarizationLawRules;

// 知识产权类
pub use copyright::CopyrightLawRules;

// 投资法类
pub use foreign_investment::ForeignInvestmentLawRules;

// 特殊群体保护法类
pub use children_protection::ChildrenProtectionLawRules;
pub use women_protection::WomenProtectionLawRules;
pub use elderly_protection::ElderlyProtectionLawRules;
pub use disability_protection::DisabilityProtectionLawRules;

// 慈善法类
pub use charity::CharityLawRules;

// 应急管理法类
pub use emergency_response::EmergencyResponseLawRules;

// 医药法类
pub use drug_management::DrugManagementLawRules;
pub use vaccine_management::VaccineManagementLawRules;

// 交通能源法类
pub use electricity::ElectricityLawRules;
pub use railway::RailwayLawRules;
pub use civil_aviation::CivilAviationLawRules;
pub use post::PostLawRules;
pub use telecommunications::TelecommunicationsLawRules;

// 资源法类
pub use forest::ForestLawRules;
pub use mineral_resources::MineralResourcesLawRules;