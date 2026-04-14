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

// 行政法类
pub mod administrative;
pub mod environmental;
pub mod construction;
pub mod food_safety;
pub mod cybersecurity;
pub mod data_security;

// 社会法类
pub mod labor_extended;
pub mod education;
pub mod medical;
pub mod consumer_extended;
pub mod real_estate;

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

// 行政法类
pub use administrative::AdministrativeLawRules;
pub use environmental::EnvironmentalLawRules;
pub use construction::ConstructionLawRules;
pub use food_safety::FoodSafetyLawRules;
pub use cybersecurity::CybersecurityLawRules;
pub use data_security::DataSecurityLawRules;

// 社会法类
pub use labor_extended::LaborLawExtendedRules;
pub use education::EducationLawRules;
pub use medical::MedicalLawRules;
pub use consumer_extended::ConsumerLawExtendedRules;
pub use real_estate::RealEstateLawRules;