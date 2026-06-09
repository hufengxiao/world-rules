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
pub mod administrative_detailed;
pub mod administrative_detailed2;
pub mod ai_regulation;
pub mod anti_money_laundering;
pub mod anti_unfair_competition;
pub mod antimonopoly_detailed;
pub mod autonomous_driving_law;
pub mod aviation_law;
pub mod banking_detailed2;
pub mod bankruptcy_detailed;
pub mod civil_detailed2;
pub mod civil_procedure_detailed;
pub mod civil_procedure_detailed2;
pub mod constitution_detailed;
pub mod consumer_detailed;
pub mod consumer_detailed2;
pub mod contract_detailed;
pub mod contract_detailed2;
pub mod corporate_governance;
pub mod criminal_detailed;
pub mod criminal_detailed2;
pub mod criminal_procedure_detailed;
pub mod criminal_procedure_detailed2;
pub mod cybersecurity_detailed;
pub mod data_protection_intl;
pub mod data_security_detailed;
pub mod drone_law;
pub mod ecommerce_law;
pub mod education_detailed;
pub mod electricity_detailed;
pub mod environmental_detailed;
pub mod environmental_detailed2;
pub mod environmental_impact_law;
pub mod family_violence;
pub mod food_safety_detailed;
pub mod food_safety_detailed2;
pub mod forest;
pub mod forest_detailed;
pub mod gene_editing_law;
pub mod humanitarian_law;
pub mod infectious_disease_law;
pub mod inheritance_detailed;
pub mod insurance_detailed;
pub mod international_human_rights;
pub mod international_investment;
pub mod international_public_law;
pub mod international_trade_law;
pub mod ip_detailed;
pub mod ip_detailed2;
pub mod labor_detailed;
pub mod labor_detailed2;
pub mod land_detailed;
pub mod legal_aid;
pub mod maritime_detailed;
pub mod maritime_law_intl;
pub mod marriage_detailed;
pub mod marriage_detailed2;
pub mod mental_health_law;
pub mod mineral_detailed;
pub mod mineral_resources;
pub mod noise_pollution_law;
pub mod personal_info_protection;
pub mod public_interest_litigation;
pub mod real_estate_detailed;
pub mod securities_detailed;
pub mod securities_detailed2;
pub mod soil_pollution_law;
pub mod space_law;
pub mod tax_detailed;
pub mod tax_detailed2;
pub mod tcm_law;
pub mod telecom_detailed;
pub mod water_detailed;
pub mod wildlife_protection_law;

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
pub use administrative_detailed::AdministrativeDetailedRules;
pub use administrative_detailed2::AdministrativeDetailed2Rules;
pub use ai_regulation::AiRegulationRules;
pub use anti_money_laundering::AntiMoneyLaunderingRules;
pub use anti_unfair_competition::AntiUnfairCompetitionRules;
pub use antimonopoly_detailed::AntimonopolyDetailedRules;
pub use autonomous_driving_law::AutonomousDrivingLawRules;
pub use aviation_law::AviationLawRules;
pub use banking_detailed2::BankingDetailed2Rules;
pub use bankruptcy_detailed::BankruptcyDetailedRules;
pub use civil_detailed2::CivilDetailed2Rules;
pub use civil_procedure_detailed::CivilProcedureDetailedRules;
pub use civil_procedure_detailed2::CivilProcedureDetailed2Rules;
pub use constitution_detailed::ConstitutionDetailedRules;
pub use consumer_detailed::ConsumerDetailedRules;
pub use consumer_detailed2::ConsumerDetailed2Rules;
pub use contract_detailed::ContractDetailedRules;
pub use contract_detailed2::ContractDetailed2Rules;
pub use corporate_governance::CorporateGovernanceRules;
pub use criminal_detailed::CriminalDetailedRules;
pub use criminal_detailed2::CriminalDetailed2Rules;
pub use criminal_procedure_detailed::CriminalProcedureDetailedRules;
pub use criminal_procedure_detailed2::CriminalProcedureDetailed2Rules;
pub use cybersecurity_detailed::CybersecurityDetailedRules;
pub use data_protection_intl::DataProtectionIntlRules;
pub use data_security_detailed::DataSecurityDetailedRules;
pub use drone_law::DroneLawRules;
pub use ecommerce_law::EcommerceLawRules;
pub use education_detailed::EducationDetailedRules;
pub use electricity_detailed::ElectricityDetailedRules;
pub use environmental_detailed::EnvironmentalDetailedRules;
pub use environmental_detailed2::EnvironmentalDetailed2Rules;
pub use environmental_impact_law::EnvironmentalImpactLawRules;
pub use family_violence::FamilyViolenceRules;
pub use food_safety_detailed::FoodSafetyDetailedRules;
pub use food_safety_detailed2::FoodSafetyDetailed2Rules;
pub use forest::ForestLawRules;
pub use forest_detailed::ForestDetailedRules;
pub use gene_editing_law::GeneEditingLawRules;
pub use humanitarian_law::HumanitarianLawRules;
pub use infectious_disease_law::InfectiousDiseaseLawRules;
pub use inheritance_detailed::InheritanceDetailedRules;
pub use insurance_detailed::InsuranceDetailedRules;
pub use international_human_rights::InternationalHumanRightsRules;
pub use international_investment::InternationalInvestmentRules;
pub use international_public_law::InternationalPublicLawRules;
pub use international_trade_law::InternationalTradeLawRules;
pub use ip_detailed::IpDetailedRules;
pub use ip_detailed2::IpDetailed2Rules;
pub use labor_detailed::LaborDetailedRules;
pub use labor_detailed2::LaborDetailed2Rules;
pub use land_detailed::LandDetailedRules;
pub use legal_aid::LegalAidRules;
pub use maritime_detailed::MaritimeDetailedRules;
pub use maritime_law_intl::MaritimeLawIntlRules;
pub use marriage_detailed::MarriageDetailedRules;
pub use marriage_detailed2::MarriageDetailed2Rules;
pub use mental_health_law::MentalHealthLawRules;
pub use mineral_detailed::MineralDetailedRules;
pub use mineral_resources::MineralResourcesLawRules;
pub use noise_pollution_law::NoisePollutionLawRules;
pub use personal_info_protection::PersonalInfoProtectionRules;
pub use public_interest_litigation::PublicInterestLitigationRules;
pub use real_estate_detailed::RealEstateDetailedRules;
pub use securities_detailed::SecuritiesDetailedRules;
pub use securities_detailed2::SecuritiesDetailed2Rules;
pub use soil_pollution_law::SoilPollutionLawRules;
pub use space_law::SpaceLawRules;
pub use tax_detailed::TaxDetailedRules;
pub use tax_detailed2::TaxDetailed2Rules;
pub use tcm_law::TcmLawRules;
pub use telecom_detailed::TelecomDetailedRules;
pub use water_detailed::WaterDetailedRules;
pub use wildlife_protection_law::WildlifeProtectionLawRules;

pub fn all_rules() -> Vec<(&'static str, crate::rules::core::RuleMetadata, crate::rules::core::RuleCategory)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    { let r = AdministrativeDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AdministrativeDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AdministrativeLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AdministrativeProcedureLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AdvertisingLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AgricultureLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AiRegulationRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AntiMoneyLaunderingRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AntiUnfairCompetitionRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AntimonopolyDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AntimonopolyLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ArbitrationLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ArchivesLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AutonomousDrivingLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = AviationLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = BankingDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = BankingLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = BankruptcyDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = BankruptcyLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = BiddingLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CharityLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ChildrenProtectionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CivilAviationLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CivilDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CivilLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CivilProcedureDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CivilProcedureDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CivilProcedureLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CompanyLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ConstitutionDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ConstitutionRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ConstructionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ConsumerDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ConsumerDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ConsumerLawExtendedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ConsumerLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ContractDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ContractDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ContractRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CopyrightLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CorporateGovernanceRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CriminalDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CriminalDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CriminalLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CriminalProcedureDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CriminalProcedureDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CriminalProcedureLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CustomsLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CybersecurityDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = CybersecurityLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = DataProtectionIntlRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = DataSecurityDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = DataSecurityLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = DisabilityProtectionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = DroneLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = DrugManagementLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EarthquakePreventionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EcommerceLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EducationDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EducationLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ElderlyProtectionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ElectricityDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ElectricityLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EmergencyResponseLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EnvironmentalDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EnvironmentalDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EnvironmentalImpactLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = EnvironmentalLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = FamilyViolenceRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = FireProtectionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = FoodSafetyDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = FoodSafetyDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = FoodSafetyLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ForeignInvestmentLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ForeignTradeLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ForestDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = ForestLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = GeneEditingLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = HumanitarianLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = IPRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InfectiousDiseaseLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InheritanceDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InheritanceLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InsuranceDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InsuranceLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InternationalHumanRightsRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InternationalInvestmentRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InternationalPublicLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = InternationalTradeLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = IpDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = IpDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = LaborDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = LaborDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = LaborLawExtendedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = LaborLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = LandAdministrationLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = LandDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = LawyerLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = LegalAidRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MaritimeDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MaritimeLawIntlRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MaritimeLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MarriageDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MarriageDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MarriageLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MedicalLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MentalHealthLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MeteorologyLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MineralDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = MineralResourcesLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = NoisePollutionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = NotarizationLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = PatentLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = PersonalInfoProtectionRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = PostLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = PriceLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = PropertyManagementLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = PublicInterestLitigationRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = RailwayLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = RealEstateDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = RealEstateLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = RoadSafetyRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = SafetyProductionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = SecuritiesDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = SecuritiesDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = SecuritiesLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = SocialInsuranceLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = SoilPollutionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = SpaceLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = StatisticsLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TaxDetailed2Rules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TaxDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TaxLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TcmLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TelecomDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TelecommunicationsLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TourismLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TrademarkLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = TrafficRules::new(crate::rules::law::traffic::TrafficRegion::China); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = VaccineManagementLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = WaterDetailedRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = WaterLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = WildlifeProtectionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    { let r = WomenProtectionLawRules::new(); rules.push(("law", r.metadata().clone(), r.category())); }
    rules
}
