//! 法律法规模块 - 涵盖中国法律和国际法律体系的规则
//!
//! 本模块包含各类法律法规的规则实现，覆盖：
//! - **民事法律**: 民法典、合同法、婚姻法、继承法等
//! - **刑事法律**: 刑法、刑事诉讼法等
//! - **商事法律**: 公司法、证券法、保险法、破产法等
//! - **经济法律**: 反垄断法、税法、银行法等
//! - **行政法律**: 行政程序法、环境保护法等
//! - **知识产权**: 专利法、商标法、著作权法等
//! - **国际法律**: 国际公法、国际私法、国际贸易法等
//!
//! # 模块结构
//!
//! ```text
//! law/
//! ├── civil            # 民事法律
//! ├── criminal         # 刑事法律
//! ├── constitution     # 宪法
//! ├── company          # 公司法
//! ├── labor            # 劳动法
//! ├── intellectual_property # 知识产权法
//! ├── environmental    # 环境保护法
//! ├── cybersecurity    # 网络安全法
//! └── international_*  # 国际法律系列
//! ```
//!
//! # Examples
//!
//! 使用规则示例：
//!
//! ```rust
//! use world_rules::rules::law::{CivilRules, CriminalRules};
//! use world_rules::rules::core::Rule;
//!
//! // 民法规则
//! let civil = CivilRules::new();
//! println!("规则: {}", civil.metadata().name);
//! println!("分类: {:?}", civil.category());
//!
//! // 刑法规则
//! let criminal = CriminalRules::new();
//! let explanation = criminal.explain();
//! assert!(!explanation.is_empty());
//! ```
//!
//! # 规则统计
//!
//! 当前包含数百条法律规则，覆盖：
//! - 50+ 条民事法律规则
//! - 30+ 条刑事法律规则
//! - 40+ 条商事法律规则
//! - 30+ 条经济法律规则
//! - 50+ 条行政法律规则
//! - 20+ 条知识产权规则
//! - 40+ 条国际法律规则

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
pub mod admin_license;
pub mod admin_penalty;
pub mod admin_reconsideration;
pub mod administrative_detailed;
pub mod administrative_detailed2;
pub mod advertising_detailed_law;
pub mod ai_regulation;
pub mod anti_corruption;
pub mod anti_money_laundering;
pub mod anti_monopoly_detailed_law;
pub mod anti_unfair_competition;
pub mod antimonopoly_detailed;
pub mod arbitration_law_detailed;
pub mod asset_recovery;
pub mod autonomous_driving_law;
pub mod aviation_law;
pub mod bail_law;
pub mod banking_detailed2;
pub mod banking_law_detailed;
pub mod bankruptcy_detailed;
pub mod biometric_law;
pub mod children_rights;
pub mod civil_code_contract;
pub mod civil_code_contract_deep;
pub mod civil_code_general;
pub mod civil_code_general_deep;
pub mod civil_code_inheritance;
pub mod civil_code_marriage;
pub mod civil_code_marriage_deep;
pub mod civil_code_personality;
pub mod civil_code_personality_deep;
pub mod civil_code_property;
pub mod civil_code_property_deep;
pub mod civil_code_tort;
pub mod civil_code_tort_deep;
pub mod civil_detailed2;
pub mod civil_procedure_deep;
pub mod civil_procedure_detailed;
pub mod civil_procedure_detailed2;
pub mod civil_procedure_detailed_law;
pub mod class_action;
pub mod company_law_detailed;
pub mod constitution_detailed;
pub mod constitution_rights;
pub mod consumer_detailed;
pub mod consumer_detailed2;
pub mod consumer_protection_deep;
pub mod consumer_protection_intl;
pub mod contract_detailed;
pub mod contract_detailed2;
pub mod corporate_governance;
pub mod criminal_detailed;
pub mod criminal_detailed2;
pub mod criminal_evidence_deep;
pub mod criminal_justice_reform_deep;
pub mod criminal_law_economic;
pub mod criminal_law_general;
pub mod criminal_law_general_deep;
pub mod criminal_law_specific;
pub mod criminal_law_specific_deep;
pub mod criminal_procedure_deep;
pub mod criminology_deep;
pub mod economic_crime_deep;
pub mod sentencing_guideline_deep;
pub mod victim_protection_deep;
pub mod criminal_procedure_detailed;
pub mod criminal_procedure_detailed2;
pub mod criminal_procedure_detailed_law;
pub mod customs_law_detailed;
pub mod cybercrime_law;
pub mod cybersecurity_detailed;
pub mod cybersecurity_detailed_law2;
pub mod data_protection_intl;
pub mod data_security_detailed;
pub mod data_security_detailed_law;
pub mod death_penalty;
pub mod deepfake_law;
pub mod digital_evidence;
pub mod disability_rights;
pub mod drone_law;
pub mod drug_mgmt_detailed;
pub mod ecommerce_detailed_law;
pub mod ecommerce_law;
pub mod education_detailed;
pub mod elderly_rights;
pub mod electricity_detailed;
pub mod environmental_detailed;
pub mod environmental_detailed2;
pub mod environmental_detailed_law3;
pub mod environmental_impact_law;
pub mod environmental_litigation;
pub mod eu_gdpr;
pub mod family_violence;
pub mod food_safety_detailed;
pub mod food_safety_detailed2;
pub mod food_safety_detailed_law;
pub mod forensic_evidence;
pub mod forest;
pub mod forest_detailed;
pub mod freedom_of_expression;
pub mod gene_editing_law;
pub mod german_company_law;
pub mod hague_convention;
pub mod housing_fund_law;
pub mod humanitarian_law;
pub mod icc_law;
pub mod icj_law;
pub mod icsid_law;
pub mod indigenous_rights;
pub mod infectious_disease_law;
pub mod inheritance_detailed;
pub mod insurance_detailed;
pub mod insurance_law_detailed;
pub mod international_arbitration;
pub mod international_aviation;
pub mod international_competition;
pub mod international_criminal;
pub mod international_cyber;
pub mod international_env;
pub mod international_financial;
pub mod international_human_rights;
pub mod international_humanitarian;
pub mod international_investment;
pub mod international_investment_treaty;
pub mod international_ip;
pub mod international_maritime;
pub mod international_public_law;
pub mod international_refugee;
pub mod international_sale;
pub mod international_space;
pub mod international_tax;
pub mod international_trade_customs;
pub mod international_trade_law;
pub mod ip_copyright_detailed;
pub mod ip_detailed;
pub mod ip_detailed2;
pub mod ip_patent_detailed;
pub mod ip_trademark_detailed;
pub mod japan_company_law;
pub mod juvenile_justice;
pub mod labor_contract_law;
pub mod labor_detailed;
pub mod labor_detailed2;
pub mod labor_dispute_law;
pub mod labor_international;
pub mod labor_law_deep;
pub mod land_detailed;
pub mod legal_aid;
pub mod maritime_detailed;
pub mod maritime_law_intl;
pub mod marriage_detailed;
pub mod marriage_detailed2;
pub mod mediation_law;
pub mod mental_health_law;
pub mod metaverse_law;
pub mod mineral_detailed;
pub mod mineral_resources;
pub mod new_york_convention;
pub mod noise_pollution_law;
pub mod online_dispute;
pub mod parole_law;
pub mod personal_info_detailed;
pub mod personal_info_protection;
pub mod plea_bargaining;
pub mod privacy_rights;
pub mod probation_law;
pub mod public_interest_litigation;
pub mod real_estate_detailed;
pub mod real_estate_law_detailed;
pub mod right_to_education;
pub mod right_to_health;
pub mod right_to_housing;
pub mod right_to_water;
pub mod securities_detailed;
pub mod securities_detailed2;
pub mod securities_law_detailed;
pub mod smart_contract_law;
pub mod social_insurance_law_detailed;
pub mod social_security_intl;
pub mod soil_pollution_law;
pub mod space_law;
pub mod tax_detailed;
pub mod tax_detailed2;
pub mod tax_law_detailed_law;
pub mod tcm_law;
pub mod telecom_detailed;
pub mod uk_company_law;
pub mod un_charter;
pub mod uncitral_law;
pub mod us_antitrust;
pub mod victim_rights;
pub mod water_detailed;
pub mod whistleblower_protection;
pub mod wildlife_protection_law;
pub mod witness_protection;
pub mod women_rights;
pub mod wto_law;

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
pub use admin_license::AdminLicenseRules;
pub use admin_penalty::AdminPenaltyRules;
pub use admin_reconsideration::AdminReconsiderationRules;
pub use administrative_detailed::AdministrativeDetailedRules;
pub use administrative_detailed2::AdministrativeDetailed2Rules;
pub use advertising_detailed_law::AdvertisingDetailedLawRules;
pub use ai_regulation::AiRegulationRules;
pub use anti_corruption::AntiCorruptionRules;
pub use anti_money_laundering::AntiMoneyLaunderingRules;
pub use anti_monopoly_detailed_law::AntiMonopolyDetailedLawRules;
pub use anti_unfair_competition::AntiUnfairCompetitionRules;
pub use antimonopoly_detailed::AntimonopolyDetailedRules;
pub use arbitration_law_detailed::ArbitrationLawDetailedRules;
pub use asset_recovery::AssetRecoveryRules;
pub use autonomous_driving_law::AutonomousDrivingLawRules;
pub use aviation_law::AviationLawRules;
pub use bail_law::BailLawRules;
pub use banking_detailed2::BankingDetailed2Rules;
pub use banking_law_detailed::BankingLawDetailedRules;
pub use bankruptcy_detailed::BankruptcyDetailedRules;
pub use biometric_law::BiometricLawRules;
pub use children_rights::ChildrenRightsRules;
pub use civil_code_contract::CivilCodeContractRules;
pub use civil_code_general::CivilCodeGeneralRules;
pub use civil_code_inheritance::CivilCodeInheritanceRules;
pub use civil_code_marriage::CivilCodeMarriageRules;
pub use civil_code_personality::CivilCodePersonalityRules;
pub use civil_code_property::CivilCodePropertyRules;
pub use civil_code_tort::CivilCodeTortRules;
pub use civil_detailed2::CivilDetailed2Rules;
pub use civil_procedure_detailed::CivilProcedureDetailedRules;
pub use civil_procedure_detailed2::CivilProcedureDetailed2Rules;
pub use civil_procedure_detailed_law::CivilProcedureDetailedLawRules;
pub use class_action::ClassActionRules;
pub use company_law_detailed::CompanyLawDetailedRules;
pub use constitution_detailed::ConstitutionDetailedRules;
pub use constitution_rights::ConstitutionRightsRules;
pub use consumer_detailed::ConsumerDetailedRules;
pub use consumer_detailed2::ConsumerDetailed2Rules;
pub use consumer_protection_intl::ConsumerProtectionIntlRules;
pub use contract_detailed::ContractDetailedRules;
pub use contract_detailed2::ContractDetailed2Rules;
pub use corporate_governance::CorporateGovernanceRules;
pub use criminal_detailed::CriminalDetailedRules;
pub use criminal_detailed2::CriminalDetailed2Rules;
pub use criminal_law_economic::CriminalLawEconomicRules;
pub use criminal_law_general::CriminalLawGeneralRules;
pub use criminal_law_specific::CriminalLawSpecificRules;
pub use criminal_evidence_deep::CriminalEvidenceDeepRules;
pub use criminal_justice_reform_deep::CriminalJusticeReformDeepRules;
pub use criminal_law_general_deep::CriminalLawGeneralDeepRules;
pub use criminal_law_specific_deep::CriminalLawSpecificDeepRules;
pub use criminal_procedure_deep::CriminalProcedureDeepRules;
pub use criminology_deep::CriminologyDeepRules;
pub use economic_crime_deep::EconomicCrimeDeepRules;
pub use sentencing_guideline_deep::SentencingGuidelineDeepRules;
pub use victim_protection_deep::VictimProtectionDeepRules;
pub use criminal_procedure_detailed::CriminalProcedureDetailedRules;
pub use criminal_procedure_detailed2::CriminalProcedureDetailed2Rules;
pub use criminal_procedure_detailed_law::CriminalProcedureDetailedLawRules;
pub use customs_law_detailed::CustomsLawDetailedRules;
pub use cybercrime_law::CybercrimeLawRules;
pub use cybersecurity_detailed::CybersecurityDetailedRules;
pub use cybersecurity_detailed_law2::CybersecurityDetailedLaw2Rules;
pub use data_protection_intl::DataProtectionIntlRules;
pub use data_security_detailed::DataSecurityDetailedRules;
pub use data_security_detailed_law::DataSecurityDetailedLawRules;
pub use death_penalty::DeathPenaltyRules;
pub use deepfake_law::DeepfakeLawRules;
pub use digital_evidence::DigitalEvidenceRules;
pub use disability_rights::DisabilityRightsRules;
pub use drone_law::DroneLawRules;
pub use drug_mgmt_detailed::DrugMgmtDetailedRules;
pub use ecommerce_detailed_law::EcommerceDetailedLawRules;
pub use ecommerce_law::EcommerceLawRules;
pub use education_detailed::EducationDetailedRules;
pub use elderly_rights::ElderlyRightsRules;
pub use electricity_detailed::ElectricityDetailedRules;
pub use environmental_detailed::EnvironmentalDetailedRules;
pub use environmental_detailed2::EnvironmentalDetailed2Rules;
pub use environmental_detailed_law3::EnvironmentalDetailedLaw3Rules;
pub use environmental_impact_law::EnvironmentalImpactLawRules;
pub use environmental_litigation::EnvironmentalLitigationRules;
pub use eu_gdpr::EuGdprRules;
pub use family_violence::FamilyViolenceRules;
pub use food_safety_detailed::FoodSafetyDetailedRules;
pub use food_safety_detailed2::FoodSafetyDetailed2Rules;
pub use food_safety_detailed_law::FoodSafetyDetailedLawRules;
pub use forensic_evidence::ForensicEvidenceRules;
pub use forest::ForestLawRules;
pub use forest_detailed::ForestDetailedRules;
pub use freedom_of_expression::FreedomOfExpressionRules;
pub use gene_editing_law::GeneEditingLawRules;
pub use german_company_law::GermanCompanyLawRules;
pub use hague_convention::HagueConventionRules;
pub use housing_fund_law::HousingFundLawRules;
pub use humanitarian_law::HumanitarianLawRules;
pub use icc_law::IccLawRules;
pub use icj_law::IcjLawRules;
pub use icsid_law::IcsidLawRules;
pub use indigenous_rights::IndigenousRightsRules;
pub use infectious_disease_law::InfectiousDiseaseLawRules;
pub use inheritance_detailed::InheritanceDetailedRules;
pub use insurance_detailed::InsuranceDetailedRules;
pub use insurance_law_detailed::InsuranceLawDetailedRules;
pub use international_arbitration::InternationalArbitrationRules;
pub use international_aviation::InternationalAviationRules;
pub use international_competition::InternationalCompetitionRules;
pub use international_criminal::InternationalCriminalRules;
pub use international_cyber::InternationalCyberRules;
pub use international_env::InternationalEnvRules;
pub use international_financial::InternationalFinancialRules;
pub use international_human_rights::InternationalHumanRightsRules;
pub use international_humanitarian::InternationalHumanitarianRules;
pub use international_investment::InternationalInvestmentRules;
pub use international_investment_treaty::InternationalInvestmentTreatyRules;
pub use international_ip::InternationalIpRules;
pub use international_maritime::InternationalMaritimeRules;
pub use international_public_law::InternationalPublicLawRules;
pub use international_refugee::InternationalRefugeeRules;
pub use international_sale::InternationalSaleRules;
pub use international_space::InternationalSpaceRules;
pub use international_tax::InternationalTaxRules;
pub use international_trade_customs::InternationalTradeCustomsRules;
pub use international_trade_law::InternationalTradeLawRules;
pub use ip_copyright_detailed::IpCopyrightDetailedRules;
pub use ip_detailed::IpDetailedRules;
pub use ip_detailed2::IpDetailed2Rules;
pub use ip_patent_detailed::IpPatentDetailedRules;
pub use ip_trademark_detailed::IpTrademarkDetailedRules;
pub use japan_company_law::JapanCompanyLawRules;
pub use juvenile_justice::JuvenileJusticeRules;
pub use labor_contract_law::LaborContractLawRules;
pub use labor_detailed::LaborDetailedRules;
pub use labor_detailed2::LaborDetailed2Rules;
pub use labor_dispute_law::LaborDisputeLawRules;
pub use labor_international::LaborInternationalRules;
pub use land_detailed::LandDetailedRules;
pub use legal_aid::LegalAidRules;
pub use maritime_detailed::MaritimeDetailedRules;
pub use maritime_law_intl::MaritimeLawIntlRules;
pub use marriage_detailed::MarriageDetailedRules;
pub use marriage_detailed2::MarriageDetailed2Rules;
pub use mediation_law::MediationLawRules;
pub use mental_health_law::MentalHealthLawRules;
pub use metaverse_law::MetaverseLawRules;
pub use mineral_detailed::MineralDetailedRules;
pub use mineral_resources::MineralResourcesLawRules;
pub use new_york_convention::NewYorkConventionRules;
pub use noise_pollution_law::NoisePollutionLawRules;
pub use online_dispute::OnlineDisputeRules;
pub use parole_law::ParoleLawRules;
pub use personal_info_detailed::PersonalInfoDetailedRules;
pub use personal_info_protection::PersonalInfoProtectionRules;
pub use plea_bargaining::PleaBargainingRules;
pub use privacy_rights::PrivacyRightsRules;
pub use probation_law::ProbationLawRules;
pub use public_interest_litigation::PublicInterestLitigationRules;
pub use real_estate_detailed::RealEstateDetailedRules;
pub use real_estate_law_detailed::RealEstateLawDetailedRules;
pub use right_to_education::RightToEducationRules;
pub use right_to_health::RightToHealthRules;
pub use right_to_housing::RightToHousingRules;
pub use right_to_water::RightToWaterRules;
pub use securities_detailed::SecuritiesDetailedRules;
pub use securities_detailed2::SecuritiesDetailed2Rules;
pub use securities_law_detailed::SecuritiesLawDetailedRules;
pub use smart_contract_law::SmartContractLawRules;
pub use social_insurance_law_detailed::SocialInsuranceLawDetailedRules;
pub use social_security_intl::SocialSecurityIntlRules;
pub use soil_pollution_law::SoilPollutionLawRules;
pub use space_law::SpaceLawRules;
pub use tax_detailed::TaxDetailedRules;
pub use tax_detailed2::TaxDetailed2Rules;
pub use tax_law_detailed_law::TaxLawDetailedLawRules;
pub use tcm_law::TcmLawRules;
pub use telecom_detailed::TelecomDetailedRules;
pub use uk_company_law::UkCompanyLawRules;
pub use un_charter::UnCharterRules;
pub use uncitral_law::UncitralLawRules;
pub use us_antitrust::UsAntitrustRules;
pub use victim_rights::VictimRightsRules;
pub use water_detailed::WaterDetailedRules;
pub use whistleblower_protection::WhistleblowerProtectionRules;
pub use wildlife_protection_law::WildlifeProtectionLawRules;
pub use witness_protection::WitnessProtectionRules;
pub use women_rights::WomenRightsRules;
pub use wto_law::WtoLawRules;

pub fn all_rules() -> Vec<(
    &'static str,
    crate::rules::core::RuleMetadata,
    crate::rules::core::RuleCategory,
    String,
)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    {
        let r = AdministrativeDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AdministrativeDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AdministrativeLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AdministrativeProcedureLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AdvertisingLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AgricultureLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AiRegulationRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AntiMoneyLaunderingRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AntiUnfairCompetitionRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AntimonopolyDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AntimonopolyLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ArbitrationLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ArchivesLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AutonomousDrivingLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AviationLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BankingDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BankingLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BankruptcyDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BankruptcyLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BiddingLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CharityLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChildrenProtectionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CivilAviationLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CivilDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CivilLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CivilProcedureDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CivilProcedureDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CivilProcedureLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CompanyLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConstitutionDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConstitutionRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConstructionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConsumerDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConsumerDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConsumerLawExtendedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConsumerLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ContractDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ContractDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ContractRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CopyrightLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CorporateGovernanceRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CriminalDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CriminalDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CriminalLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CriminalProcedureDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CriminalProcedureDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CriminalProcedureLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CustomsLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CybersecurityDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CybersecurityLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DataProtectionIntlRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DataSecurityDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DataSecurityLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DisabilityProtectionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DroneLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DrugManagementLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EarthquakePreventionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EcommerceLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EducationDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EducationLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElderlyProtectionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElectricityDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElectricityLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EmergencyResponseLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EnvironmentalDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EnvironmentalDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EnvironmentalImpactLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EnvironmentalLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FamilyViolenceRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FireProtectionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FoodSafetyDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FoodSafetyDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FoodSafetyLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ForeignInvestmentLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ForeignTradeLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ForestDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ForestLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GeneEditingLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HumanitarianLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IPRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InfectiousDiseaseLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InheritanceDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InheritanceLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InsuranceDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InsuranceLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InternationalHumanRightsRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InternationalInvestmentRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InternationalPublicLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InternationalTradeLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IpDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IpDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LaborDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LaborDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LaborLawExtendedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LaborLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LandAdministrationLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LandDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LawyerLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LegalAidRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MaritimeDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MaritimeLawIntlRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MaritimeLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MarriageDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MarriageDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MarriageLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MedicalLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MentalHealthLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MeteorologyLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MineralDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MineralResourcesLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NoisePollutionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NotarizationLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PatentLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PersonalInfoProtectionRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PostLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PriceLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PropertyManagementLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PublicInterestLitigationRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RailwayLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RealEstateDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RealEstateLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RoadSafetyRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SafetyProductionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SecuritiesDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SecuritiesDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SecuritiesLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SocialInsuranceLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SoilPollutionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SpaceLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = StatisticsLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaxDetailed2Rules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaxDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaxLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TcmLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TelecomDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TelecommunicationsLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TourismLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TrademarkLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TrafficRules::new(crate::rules::law::traffic::TrafficRegion::China);
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VaccineManagementLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WaterDetailedRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WaterLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WildlifeProtectionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WomenProtectionLawRules::new();
        rules.push(("law", r.metadata().clone(), r.category(), r.explain()));
    }
    rules
}
