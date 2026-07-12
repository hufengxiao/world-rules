//! Phase 31-03: 其他地球科学规则扩充综合测试
//!
//! 本文件测试 Phase 31-03 添加的所有地球科学详细规则模块
//!
//! 包含 5 种地球科学规则：
//! - GeologyDetailedRules (地质学详细)
//! - GeoscienceDetailedRules (地球科学综合详细)
//! - SeismologyDetailedRules (地震学详细)
//! - OceanographyDetailedRules (海洋学详细)
//! - VolcanologyDetailedRules (火山学详细)

use world_rules::rules::core::{Rule, RuleCategory};
use world_rules::rules::science::{
    GeologyDetailedRules, GeoscienceDetailedRules, SeismologyDetailedRules,
    OceanographyDetailedRules, VolcanologyDetailedRules,
};

// ============================================================================
// Phase 31-03: 其他地球科学详细规则测试
// ============================================================================

/// 测试地质学详细规则
#[test]
fn test_geology_detailed_rules_comprehensive() {
    let rules = GeologyDetailedRules::new();
    assert_eq!(rules.metadata().name, "地质学详细规则");
    assert_eq!(rules.category(), RuleCategory::science("geology_detailed"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.rock_types_rules().len(), 8);
    assert_eq!(rules.mineralogy_rules().len(), 8);
    assert_eq!(rules.geological_structure_rules().len(), 8);
    assert_eq!(rules.plate_tectonics_rules().len(), 8);
    assert_eq!(rules.stratigraphy_rules().len(), 8);
    assert_eq!(rules.geological_time_rules().len(), 8);
    assert_eq!(rules.geological_processes_rules().len(), 8);
    assert_eq!(rules.geological_resources_rules().len(), 8);
    
    // 总规则数: 64
    let total = rules.rock_types_rules().len()
        + rules.mineralogy_rules().len()
        + rules.geological_structure_rules().len()
        + rules.plate_tectonics_rules().len()
        + rules.stratigraphy_rules().len()
        + rules.geological_time_rules().len()
        + rules.geological_processes_rules().len()
        + rules.geological_resources_rules().len();
    assert_eq!(total, 64);
    
    assert!(!rules.explain().is_empty());
}

/// 测试地球科学综合详细规则
#[test]
fn test_geoscience_detailed_rules_comprehensive() {
    let rules = GeoscienceDetailedRules::new();
    assert_eq!(rules.metadata().name, "地球科学综合详细规则");
    assert_eq!(rules.category(), RuleCategory::science("geoscience_detailed"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.earth_system_rules().len(), 8);
    assert_eq!(rules.global_change_rules().len(), 8);
    assert_eq!(rules.environmental_change_rules().len(), 8);
    assert_eq!(rules.human_environment_rules().len(), 8);
    assert_eq!(rules.natural_disasters_rules().len(), 8);
    assert_eq!(rules.earth_observation_rules().len(), 8);
    
    // 总规则数: 48
    let total = rules.earth_system_rules().len()
        + rules.global_change_rules().len()
        + rules.environmental_change_rules().len()
        + rules.human_environment_rules().len()
        + rules.natural_disasters_rules().len()
        + rules.earth_observation_rules().len();
    assert_eq!(total, 48);
    
    assert!(!rules.explain().is_empty());
}

/// 测试地震学详细规则
#[test]
fn test_seismology_detailed_rules_comprehensive() {
    let rules = SeismologyDetailedRules::new();
    assert_eq!(rules.metadata().name, "地震学详细规则");
    assert_eq!(rules.category(), RuleCategory::science("seismology_detailed"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.seismic_wave_types_rules().len(), 8);
    assert_eq!(rules.earthquake_magnitude_rules().len(), 8);
    assert_eq!(rules.earthquake_intensity_rules().len(), 8);
    assert_eq!(rules.earthquake_origin_rules().len(), 8);
    assert_eq!(rules.earthquake_monitoring_rules().len(), 8);
    assert_eq!(rules.earthquake_disaster_rules().len(), 8);
    assert_eq!(rules.earthquake_engineering_rules().len(), 8);
    assert_eq!(rules.earthquake_prediction_rules().len(), 8);
    
    // 总规则数: 64
    let total = rules.seismic_wave_types_rules().len()
        + rules.earthquake_magnitude_rules().len()
        + rules.earthquake_intensity_rules().len()
        + rules.earthquake_origin_rules().len()
        + rules.earthquake_monitoring_rules().len()
        + rules.earthquake_disaster_rules().len()
        + rules.earthquake_engineering_rules().len()
        + rules.earthquake_prediction_rules().len();
    assert_eq!(total, 64);
    
    assert!(!rules.explain().is_empty());
}

/// 测试海洋学详细规则
#[test]
fn test_oceanography_detailed_rules_comprehensive() {
    let rules = OceanographyDetailedRules::new();
    assert_eq!(rules.metadata().name, "海洋学详细规则");
    assert_eq!(rules.category(), RuleCategory::science("oceanography_detailed"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.ocean_circulation_rules().len(), 8);
    assert_eq!(rules.ocean_chemistry_rules().len(), 8);
    assert_eq!(rules.ocean_physics_rules().len(), 8);
    assert_eq!(rules.ocean_wave_rules().len(), 8);
    assert_eq!(rules.ocean_tide_rules().len(), 8);
    assert_eq!(rules.marine_ecosystem_rules().len(), 8);
    assert_eq!(rules.ocean_geology_rules().len(), 8);
    assert_eq!(rules.ocean_resources_rules().len(), 8);
    
    // 总规则数: 64
    let total = rules.ocean_circulation_rules().len()
        + rules.ocean_chemistry_rules().len()
        + rules.ocean_physics_rules().len()
        + rules.ocean_wave_rules().len()
        + rules.ocean_tide_rules().len()
        + rules.marine_ecosystem_rules().len()
        + rules.ocean_geology_rules().len()
        + rules.ocean_resources_rules().len();
    assert_eq!(total, 64);
    
    assert!(!rules.explain().is_empty());
}

/// 测试火山学详细规则
#[test]
fn test_volcanology_detailed_rules_comprehensive() {
    let rules = VolcanologyDetailedRules::new();
    assert_eq!(rules.metadata().name, "火山学详细规则");
    assert_eq!(rules.category(), RuleCategory::science("volcanology_detailed"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.volcano_types_rules().len(), 8);
    assert_eq!(rules.eruption_types_rules().len(), 8);
    assert_eq!(rules.volcanic_products_rules().len(), 8);
    assert_eq!(rules.volcano_monitoring_rules().len(), 8);
    assert_eq!(rules.volcanic_disaster_rules().len(), 8);
    assert_eq!(rules.volcano_warning_rules().len(), 8);
    assert_eq!(rules.volcanic_geology_rules().len(), 8);
    assert_eq!(rules.volcanic_hazards_rules().len(), 8);
    
    // 总规则数: 64
    let total = rules.volcano_types_rules().len()
        + rules.eruption_types_rules().len()
        + rules.volcanic_products_rules().len()
        + rules.volcano_monitoring_rules().len()
        + rules.volcanic_disaster_rules().len()
        + rules.volcano_warning_rules().len()
        + rules.volcanic_geology_rules().len()
        + rules.volcanic_hazards_rules().len();
    assert_eq!(total, 64);
    
    assert!(!rules.explain().is_empty());
}

// ============================================================================
// 特定功能测试
// ============================================================================

/// 测试岩石类型规则
#[test]
fn test_rock_types_rules() {
    let rules = GeologyDetailedRules::new();
    let laws = rules.rock_types_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("岩浆岩")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("沉积岩")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("变质岩")));
}

/// 测试板块构造规则
#[test]
fn test_plate_tectonics_rules() {
    let rules = GeologyDetailedRules::new();
    let laws = rules.plate_tectonics_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("板块")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("边界")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("俯冲")));
}

/// 测试地球系统科学规则
#[test]
fn test_earth_system_rules() {
    let rules = GeoscienceDetailedRules::new();
    let laws = rules.earth_system_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("圈层")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("系统")));
}

/// 测试全球变化规则
#[test]
fn test_global_change_rules() {
    let rules = GeoscienceDetailedRules::new();
    let laws = rules.global_change_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("气候")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("温室")));
}

/// 测试地震波类型规则
#[test]
fn test_seismic_wave_types_rules() {
    let rules = SeismologyDetailedRules::new();
    let laws = rules.seismic_wave_types_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("P波")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("S波")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("瑞利")));
}

/// 测试地震震级规则
#[test]
fn test_earthquake_magnitude_rules() {
    let rules = SeismologyDetailedRules::new();
    let laws = rules.earthquake_magnitude_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("里氏")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("矩震级")));
}

/// 测试海洋环流规则
#[test]
fn test_ocean_circulation_rules() {
    let rules = OceanographyDetailedRules::new();
    let laws = rules.ocean_circulation_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("环流")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("暖流")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("寒流")));
}

/// 测试海洋化学规则
#[test]
fn test_ocean_chemistry_rules() {
    let rules = OceanographyDetailedRules::new();
    let laws = rules.ocean_chemistry_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("盐度")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("溶解氧")));
}

/// 测试火山类型规则
#[test]
fn test_volcano_types_rules() {
    let rules = VolcanologyDetailedRules::new();
    let laws = rules.volcano_types_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("盾状")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("层状")));
}

/// 测试火山喷发规则
#[test]
fn test_eruption_types_rules() {
    let rules = VolcanologyDetailedRules::new();
    let laws = rules.eruption_types_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("喷发")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("夏威夷")));
}

/// 测试火山监测规则
#[test]
fn test_volcano_monitoring_rules() {
    let rules = VolcanologyDetailedRules::new();
    let laws = rules.volcano_monitoring_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("监测")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("地震")));
}

// ============================================================================
// 统计测试
// ============================================================================

/// 统计 Phase 31-03 所有规则总数
#[test]
fn test_phase_31_03_total_rules() {
    // 地质学详细: 64
    let geology = GeologyDetailedRules::new();
    let geology_total = geology.rock_types_rules().len()
        + geology.mineralogy_rules().len()
        + geology.geological_structure_rules().len()
        + geology.plate_tectonics_rules().len()
        + geology.stratigraphy_rules().len()
        + geology.geological_time_rules().len()
        + geology.geological_processes_rules().len()
        + geology.geological_resources_rules().len();
    
    // 地球科学综合详细: 48
    let geoscience = GeoscienceDetailedRules::new();
    let geoscience_total = geoscience.earth_system_rules().len()
        + geoscience.global_change_rules().len()
        + geoscience.environmental_change_rules().len()
        + geoscience.human_environment_rules().len()
        + geoscience.natural_disasters_rules().len()
        + geoscience.earth_observation_rules().len();
    
    // 地震学详细: 64
    let seismology = SeismologyDetailedRules::new();
    let seismology_total = seismology.seismic_wave_types_rules().len()
        + seismology.earthquake_magnitude_rules().len()
        + seismology.earthquake_intensity_rules().len()
        + seismology.earthquake_origin_rules().len()
        + seismology.earthquake_monitoring_rules().len()
        + seismology.earthquake_disaster_rules().len()
        + seismology.earthquake_engineering_rules().len()
        + seismology.earthquake_prediction_rules().len();
    
    // 海洋学详细: 64
    let oceanography = OceanographyDetailedRules::new();
    let oceanography_total = oceanography.ocean_circulation_rules().len()
        + oceanography.ocean_chemistry_rules().len()
        + oceanography.ocean_physics_rules().len()
        + oceanography.ocean_wave_rules().len()
        + oceanography.ocean_tide_rules().len()
        + oceanography.marine_ecosystem_rules().len()
        + oceanography.ocean_geology_rules().len()
        + oceanography.ocean_resources_rules().len();
    
    // 火山学详细: 64
    let volcanology = VolcanologyDetailedRules::new();
    let volcanology_total = volcanology.volcano_types_rules().len()
        + volcanology.eruption_types_rules().len()
        + volcanology.volcanic_products_rules().len()
        + volcanology.volcano_monitoring_rules().len()
        + volcanology.volcanic_disaster_rules().len()
        + volcanology.volcano_warning_rules().len()
        + volcanology.volcanic_geology_rules().len()
        + volcanology.volcanic_hazards_rules().len();
    
    // 总规则数: 64 + 48 + 64 + 64 + 64 = 288
    let total = geology_total + geoscience_total + seismology_total + oceanography_total + volcanology_total;
    assert_eq!(total, 288);
}