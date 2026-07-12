//! Phase 31-01: 地理规则扩充综合测试
//!
//! 本文件测试 Phase 31-01 添加的所有地理规则模块
//!
//! 包含 10 种地理规则：
//! - GeomorphologyDetailedRules (地貌学详细)
//! - ClimatologyDetailedRules (气候学详细)
//! - HydrologyDetailedRules (水文学详细)
//! - SoilGeographyRules (土壤地理)
//! - BiogeographyDetailedRules (生物地理详细)
//! - UrbanGeographyDetailedRules (城市地理详细)
//! - EconomicGeographyDetailedRules (经济地理详细)
//! - CulturalGeographyDetailedRules (文化地理详细)
//! - PoliticalGeographyDetailedRules (政治地理详细)
//! - RemoteSensingDetailedRules (遥感地理详细)
//! - GISDetailedRules (GIS地理详细)

use world_rules::rules::core::{Rule, RuleCategory};
use world_rules::rules::science::{
    GeomorphologyDetailedRules, ClimatologyDetailedRules, HydrologyDetailedRules,
    SoilGeographyRules, BiogeographyDetailedRules, UrbanGeographyDetailedRules,
    EconomicGeographyDetailedRules, CulturalGeographyDetailedRules,
    PoliticalGeographyDetailedRules, RemoteSensingDetailedRules, GISDetailedRules,
};

// ============================================================================
// Phase 31-01: 地理规则测试
// ============================================================================

/// 测试地貌学详细规则
#[test]
fn test_geomorphology_detailed_rules() {
    let rules = GeomorphologyDetailedRules::new();
    assert_eq!(rules.metadata().name, "地貌学详细规则");
    assert_eq!(rules.category(), RuleCategory::science("geomorphology_detailed"));
    assert_eq!(rules.erosion_landform_rules().len(), 8);
    assert_eq!(rules.deposition_landform_rules().len(), 8);
    assert_eq!(rules.fluvial_landform_rules().len(), 8);
    assert_eq!(rules.glacial_landform_rules().len(), 8);
    assert_eq!(rules.coastal_landform_rules().len(), 8);
    assert_eq!(rules.aeolian_landform_rules().len(), 8);
    assert_eq!(rules.karst_landform_rules().len(), 8);
    assert_eq!(rules.tectonic_landform_rules().len(), 8);
    assert!(!rules.explain().is_empty());
}

/// 测试气候学详细规则
#[test]
fn test_climatology_detailed_rules() {
    let rules = ClimatologyDetailedRules::new();
    assert_eq!(rules.metadata().name, "气候学详细规则");
    assert_eq!(rules.category(), RuleCategory::science("climatology_detailed"));
    assert_eq!(rules.climate_classification_rules().len(), 8);
    assert_eq!(rules.climate_elements_rules().len(), 8);
    assert_eq!(rules.climate_change_rules().len(), 8);
    assert_eq!(rules.climate_system_rules().len(), 8);
    assert_eq!(rules.climate_regions_rules().len(), 8);
    assert_eq!(rules.extreme_weather_rules().len(), 8);
    assert_eq!(rules.climate_prediction_rules().len(), 8);
    assert_eq!(rules.climate_impact_rules().len(), 8);
    assert!(!rules.explain().is_empty());
}

/// 测试水文学详细规则
#[test]
fn test_hydrology_detailed_rules() {
    let rules = HydrologyDetailedRules::new();
    assert_eq!(rules.metadata().name, "水文学详细规则");
    assert_eq!(rules.category(), RuleCategory::science("hydrology_detailed"));
    assert_eq!(rules.hydrological_cycle_rules().len(), 8);
    assert_eq!(rules.river_hydrology_rules().len(), 8);
    assert_eq!(rules.groundwater_rules().len(), 8);
    assert_eq!(rules.lake_hydrology_rules().len(), 8);
    assert_eq!(rules.hydrological_forecast_rules().len(), 8);
    assert_eq!(rules.hydrological_measurement_rules().len(), 8);
    assert_eq!(rules.water_resources_rules().len(), 8);
    assert_eq!(rules.hydrological_geography_rules().len(), 8);
    assert!(!rules.explain().is_empty());
}

/// 测试土壤地理规则
#[test]
fn test_soil_geography_rules() {
    let rules = SoilGeographyRules::new();
    assert_eq!(rules.metadata().name, "土壤地理规则");
    assert_eq!(rules.category(), RuleCategory::science("soil_geography"));
    assert_eq!(rules.soilformation_rules().len(), 8);
    assert_eq!(rules.soil_classification_rules().len(), 8);
    assert_eq!(rules.soil_properties_rules().len(), 8);
    assert_eq!(rules.soil_chemical_rules().len(), 8);
    assert_eq!(rules.soil_distribution_rules().len(), 8);
    assert_eq!(rules.soil_utilization_rules().len(), 8);
    assert_eq!(rules.major_soil_types().len(), 10);
    assert_eq!(rules.research_methods().len(), 8);
    assert_eq!(rules.soil_problems().len(), 8);
    assert!(!rules.explain().is_empty());
}

/// 测试生物地理详细规则
#[test]
fn test_biogeography_detailed_rules() {
    let rules = BiogeographyDetailedRules::new();
    assert_eq!(rules.metadata().name, "生物地理规则");
    assert_eq!(rules.category(), RuleCategory::science("biogeography_detailed"));
    assert_eq!(rules.species_distribution_rules().len(), 8);
    assert_eq!(rules.community_distribution_rules().len(), 8);
    assert_eq!(rules.ecosystem_distribution_rules().len(), 8);
    assert_eq!(rules.biodiversity_rules().len(), 8);
    assert_eq!(rules.plant_geography_rules().len(), 8);
    assert_eq!(rules.animal_geography_rules().len(), 8);
    assert_eq!(rules.historical_biogeography_rules().len(), 8);
    assert_eq!(rules.conservation_biogeography_rules().len(), 8);
    assert_eq!(rules.major_biomes().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试城市地理详细规则
#[test]
fn test_urban_geography_detailed_rules() {
    let rules = UrbanGeographyDetailedRules::new();
    assert_eq!(rules.metadata().name, "城市地理规则");
    assert_eq!(rules.category(), RuleCategory::science("urban_geography_detailed"));
    assert_eq!(rules.urbanization_rules().len(), 8);
    assert_eq!(rules.urban_structure_rules().len(), 8);
    assert_eq!(rules.urban_system_rules().len(), 8);
    assert_eq!(rules.urban_function_rules().len(), 8);
    assert_eq!(rules.urban_social_rules().len(), 8);
    assert_eq!(rules.urban_economic_rules().len(), 8);
    assert_eq!(rules.urban_environment_rules().len(), 8);
    assert_eq!(rules.urban_planning_rules().len(), 8);
    assert_eq!(rules.major_city_types().len(), 10);
    assert_eq!(rules.urban_problems().len(), 8);
    assert!(!rules.explain().is_empty());
}

/// 测试经济地理详细规则
#[test]
fn test_economic_geography_detailed_rules() {
    let rules = EconomicGeographyDetailedRules::new();
    assert_eq!(rules.metadata().name, "经济地理规则");
    assert_eq!(rules.category(), RuleCategory::science("economic_geography_detailed"));
    assert_eq!(rules.industrial_layout_rules().len(), 8);
    assert_eq!(rules.agricultural_geography_rules().len(), 8);
    assert_eq!(rules.industrial_geography_rules().len(), 8);
    assert_eq!(rules.commercial_geography_rules().len(), 8);
    assert_eq!(rules.transportation_geography_rules().len(), 8);
    assert_eq!(rules.regional_development_rules().len(), 8);
    assert_eq!(rules.globalization_rules().len(), 8);
    assert_eq!(rules.spatial_structure_rules().len(), 8);
    assert_eq!(rules.major_economic_regions().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试文化地理详细规则
#[test]
fn test_cultural_geography_detailed_rules() {
    let rules = CulturalGeographyDetailedRules::new();
    assert_eq!(rules.metadata().name, "文化地理规则");
    assert_eq!(rules.category(), RuleCategory::science("cultural_geography_detailed"));
    assert_eq!(rules.cultural_region_rules().len(), 8);
    assert_eq!(rules.cultural_diffusion_rules().len(), 8);
    assert_eq!(rules.cultural_ecology_rules().len(), 8);
    assert_eq!(rules.language_geography_rules().len(), 8);
    assert_eq!(rules.religion_geography_rules().len(), 8);
    assert_eq!(rules.ethnic_geography_rules().len(), 8);
    assert_eq!(rules.population_geography_rules().len(), 8);
    assert_eq!(rules.settlement_geography_rules().len(), 8);
    assert_eq!(rules.major_cultural_regions().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试政治地理详细规则
#[test]
fn test_political_geography_detailed_rules() {
    let rules = PoliticalGeographyDetailedRules::new();
    assert_eq!(rules.metadata().name, "政治地理规则");
    assert_eq!(rules.category(), RuleCategory::science("political_geography_detailed"));
    assert_eq!(rules.territory_rules().len(), 8);
    assert_eq!(rules.boundary_rules().len(), 8);
    assert_eq!(rules.geopolitics_rules().len(), 8);
    assert_eq!(rules.administrative_division_rules().len(), 8);
    assert_eq!(rules.international_organization_rules().len(), 8);
    assert_eq!(rules.military_geography_rules().len(), 8);
    assert_eq!(rules.electoral_geography_rules().len(), 8);
    assert_eq!(rules.political_pattern_rules().len(), 8);
    assert_eq!(rules.major_political_regions().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试遥感地理详细规则
#[test]
fn test_remote_sensing_detailed_rules() {
    let rules = RemoteSensingDetailedRules::new();
    assert_eq!(rules.metadata().name, "遥感地理详细规则");
    assert_eq!(rules.category(), RuleCategory::science("remote_sensing_detailed"));
    assert_eq!(rules.remote_sensing_principles().len(), 8);
    assert_eq!(rules.classification_rules().len(), 8);
    assert_eq!(rules.interpretation_rules().len(), 8);
    assert_eq!(rules.data_rules().len(), 8);
    assert_eq!(rules.application_rules().len(), 8);
    assert_eq!(rules.monitoring_rules().len(), 8);
    assert_eq!(rules.analysis_rules().len(), 8);
    assert_eq!(rules.product_rules().len(), 8);
    assert_eq!(rules.major_rs_types().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试GIS地理详细规则
#[test]
fn test_gis_detailed_rules() {
    let rules = GISDetailedRules::new();
    assert_eq!(rules.metadata().name, "GIS地理详细规则");
    assert_eq!(rules.category(), RuleCategory::science("gis_detailed"));
    assert_eq!(rules.data_rules().len(), 8);
    assert_eq!(rules.analysis_rules().len(), 8);
    assert_eq!(rules.mapping_rules().len(), 8);
    assert_eq!(rules.query_rules().len(), 8);
    assert_eq!(rules.visualization_rules().len(), 8);
    assert_eq!(rules.modeling_rules().len(), 8);
    assert_eq!(rules.application_rules().len(), 8);
    assert_eq!(rules.service_rules().len(), 8);
    assert_eq!(rules.major_gis_types().len(), 10);
    assert_eq!(rules.development_trends().len(), 8);
    assert!(!rules.explain().is_empty());
}

// ============================================================================
// 特定功能测试
// ============================================================================

/// 测试地貌侵蚀规则
#[test]
fn test_erosion_landform_rules() {
    let rules = GeomorphologyDetailedRules::new();
    let laws = rules.erosion_landform_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("侵蚀")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("河流")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("冰川")));
}

/// 测试气候变化规则
#[test]
fn test_climate_change_rules() {
    let rules = ClimatologyDetailedRules::new();
    let laws = rules.climate_change_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("变暖")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("温室")));
}

/// 测试水文循环规则
#[test]
fn test_hydrological_cycle_rules() {
    let rules = HydrologyDetailedRules::new();
    let laws = rules.hydrological_cycle_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("蒸发")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("降水")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("径流")));
}

/// 测试土壤类型
#[test]
fn test_soil_types() {
    let rules = SoilGeographyRules::new();
    let types = rules.major_soil_types();
    assert!(types.iter().any(|t| t.contains("红壤")));
    assert!(types.iter().any(|t| t.contains("黑土")));
}

/// 测试生物群落
#[test]
fn test_major_biomes() {
    let rules = BiogeographyDetailedRules::new();
    let biomes = rules.major_biomes();
    assert!(biomes.iter().any(|b| b.contains("热带雨林")));
    assert!(biomes.iter().any(|b| b.contains("草原")));
}

/// 测试城市类型
#[test]
fn test_city_types() {
    let rules = UrbanGeographyDetailedRules::new();
    let types = rules.major_city_types();
    assert!(types.iter().any(|t| t.contains("特大城市")));
    assert!(types.iter().any(|t| t.contains("港口")));
}

/// 测试遥感类型
#[test]
fn test_rs_types() {
    let rules = RemoteSensingDetailedRules::new();
    let types = rules.major_rs_types();
    assert!(types.iter().any(|t| t.contains("光学")));
    assert!(types.iter().any(|t| t.contains("雷达")));
}

/// 测试GIS类型
#[test]
fn test_gis_types() {
    let rules = GISDetailedRules::new();
    let types = rules.major_gis_types();
    assert!(types.iter().any(|t| t.contains("WebGIS")));
    assert!(types.iter().any(|t| t.contains("云GIS")));
}