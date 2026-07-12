//! Phase 31-02: 气象规则扩充综合测试
//!
//! 本文件测试 Phase 31-02 添加的所有气象规则模块
//!
//! 包含 10 种气象规则：
//! - MeteorologyDetailedRules (气象学详细)
//! - SynopticMeteorologyRules (天气学)
//! - TropicalMeteorologyRules (热带气象)
//! - MesoscaleMeteorologyRules (中尺度气象)
//! - DynamicMeteorologyRules (动力气象)
//! - PhysicalMeteorologyRules (物理气象)
//! - MarineMeteorologyRules (海洋气象)
//! - AviationMeteorologyRules (航空气象)
//! - RadarMeteorologyRules (雷达气象)
//! - SatelliteMeteorologyRules (卫星气象)

use world_rules::rules::core::{Rule, RuleCategory};
use world_rules::rules::science::{
    MeteorologyDetailedRules, SynopticMeteorologyRules, TropicalMeteorologyRules,
    MesoscaleMeteorologyRules, DynamicMeteorologyRules, PhysicalMeteorologyRules,
    MarineMeteorologyRules, AviationMeteorologyRules, RadarMeteorologyRules,
    SatelliteMeteorologyRules,
};

// ============================================================================
// Phase 31-02: 气象规则测试
// ============================================================================

/// 测试气象学详细规则
#[test]
fn test_meteorology_detailed_rules_comprehensive() {
    let rules = MeteorologyDetailedRules::new();
    assert_eq!(rules.metadata().name, "气象学详细规则");
    assert_eq!(rules.category(), RuleCategory::science("meteorology_detailed"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.weather_forecast_rules().len(), 8);
    assert_eq!(rules.atmospheric_physics_rules().len(), 8);
    assert_eq!(rules.atmospheric_circulation_rules().len(), 8);
    assert_eq!(rules.cloud_precipitation_rules().len(), 8);
    assert_eq!(rules.meteorological_observation_rules().len(), 8);
    assert_eq!(rules.numerical_weather_prediction_rules().len(), 8);
    assert_eq!(rules.meteorological_disaster_rules().len(), 8);
    assert_eq!(rules.atmospheric_boundary_layer_rules().len(), 8);
    assert_eq!(rules.weather_radar_rules().len(), 8);
    assert_eq!(rules.weather_satellite_rules().len(), 8);
    
    // 总规则数: 80
    let total = rules.weather_forecast_rules().len()
        + rules.atmospheric_physics_rules().len()
        + rules.atmospheric_circulation_rules().len()
        + rules.cloud_precipitation_rules().len()
        + rules.meteorological_observation_rules().len()
        + rules.numerical_weather_prediction_rules().len()
        + rules.meteorological_disaster_rules().len()
        + rules.atmospheric_boundary_layer_rules().len()
        + rules.weather_radar_rules().len()
        + rules.weather_satellite_rules().len();
    assert_eq!(total, 80);
    
    assert!(!rules.explain().is_empty());
}

/// 测试天气学规则
#[test]
fn test_synoptic_meteorology_rules_comprehensive() {
    let rules = SynopticMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "天气学规则");
    assert_eq!(rules.category(), RuleCategory::science("synoptic_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.frontal_analysis_rules().len(), 8);
    assert_eq!(rules.cyclone_analysis_rules().len(), 8);
    assert_eq!(rules.anticyclone_analysis_rules().len(), 8);
    assert_eq!(rules.weather_map_analysis_rules().len(), 8);
    assert_eq!(rules.air_mass_analysis_rules().len(), 8);
    assert_eq!(rules.jet_stream_analysis_rules().len(), 8);
    assert_eq!(rules.vorticity_analysis_rules().len(), 8);
    assert_eq!(rules.weather_process_rules().len(), 8);
    assert_eq!(rules.weather_pattern_rules().len(), 8);
    
    // 总规则数: 72
    let total = rules.frontal_analysis_rules().len()
        + rules.cyclone_analysis_rules().len()
        + rules.anticyclone_analysis_rules().len()
        + rules.weather_map_analysis_rules().len()
        + rules.air_mass_analysis_rules().len()
        + rules.jet_stream_analysis_rules().len()
        + rules.vorticity_analysis_rules().len()
        + rules.weather_process_rules().len()
        + rules.weather_pattern_rules().len();
    assert_eq!(total, 72);
    
    assert!(!rules.explain().is_empty());
}

/// 测试热带气象学规则
#[test]
fn test_tropical_meteorology_rules_comprehensive() {
    let rules = TropicalMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "热带气象学规则");
    assert_eq!(rules.category(), RuleCategory::science("tropical_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.typhoon_rules().len(), 8);
    assert_eq!(rules.tropical_convection_rules().len(), 8);
    assert_eq!(rules.monsoon_rules().len(), 8);
    assert_eq!(rules.tropical_waves_rules().len(), 8);
    assert_eq!(rules.tropical_circulation_rules().len(), 8);
    assert_eq!(rules.tropical_cyclone_classification_rules().len(), 8);
    assert_eq!(rules.tropical_precipitation_rules().len(), 8);
    assert_eq!(rules.tropical_cyclone_motion_rules().len(), 8);
    assert_eq!(rules.tropical_cyclone_disaster_rules().len(), 8);
    
    // 总规则数: 72
    let total = rules.typhoon_rules().len()
        + rules.tropical_convection_rules().len()
        + rules.monsoon_rules().len()
        + rules.tropical_waves_rules().len()
        + rules.tropical_circulation_rules().len()
        + rules.tropical_cyclone_classification_rules().len()
        + rules.tropical_precipitation_rules().len()
        + rules.tropical_cyclone_motion_rules().len()
        + rules.tropical_cyclone_disaster_rules().len();
    assert_eq!(total, 72);
    
    assert!(!rules.explain().is_empty());
}

/// 测试中尺度气象学规则
#[test]
fn test_mesoscale_meteorology_rules_comprehensive() {
    let rules = MesoscaleMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "中尺度气象学规则");
    assert_eq!(rules.category(), RuleCategory::science("mesoscale_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.severe_convection_rules().len(), 8);
    assert_eq!(rules.thunderstorm_rules().len(), 8);
    assert_eq!(rules.tornado_rules().len(), 8);
    assert_eq!(rules.heavy_rain_rules().len(), 8);
    assert_eq!(rules.hail_rules().len(), 8);
    assert_eq!(rules.downburst_rules().len(), 8);
    
    // 总规则数: 48
    let total = rules.severe_convection_rules().len()
        + rules.thunderstorm_rules().len()
        + rules.tornado_rules().len()
        + rules.heavy_rain_rules().len()
        + rules.hail_rules().len()
        + rules.downburst_rules().len();
    assert_eq!(total, 48);
    
    assert!(!rules.explain().is_empty());
}

/// 测试动力气象学规则
#[test]
fn test_dynamic_meteorology_rules_comprehensive() {
    let rules = DynamicMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "动力气象学规则");
    assert_eq!(rules.category(), RuleCategory::science("dynamic_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.atmospheric_motion_equations_rules().len(), 8);
    assert_eq!(rules.atmospheric_waves_rules().len(), 8);
    assert_eq!(rules.atmospheric_instability_rules().len(), 8);
    assert_eq!(rules.atmospheric_energy_rules().len(), 8);
    assert_eq!(rules.general_circulation_rules().len(), 8);
    assert_eq!(rules.atmospheric_vortices_rules().len(), 8);
    
    // 总规则数: 48
    let total = rules.atmospheric_motion_equations_rules().len()
        + rules.atmospheric_waves_rules().len()
        + rules.atmospheric_instability_rules().len()
        + rules.atmospheric_energy_rules().len()
        + rules.general_circulation_rules().len()
        + rules.atmospheric_vortices_rules().len();
    assert_eq!(total, 48);
    
    assert!(!rules.explain().is_empty());
}

/// 测试物理气象学规则
#[test]
fn test_physical_meteorology_rules_comprehensive() {
    let rules = PhysicalMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "物理气象学规则");
    assert_eq!(rules.category(), RuleCategory::science("physical_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.atmospheric_radiation_rules().len(), 8);
    assert_eq!(rules.cloud_physics_rules().len(), 8);
    assert_eq!(rules.atmospheric_optics_rules().len(), 8);
    assert_eq!(rules.atmospheric_electrical_rules().len(), 8);
    assert_eq!(rules.atmospheric_acoustics_rules().len(), 8);
    assert_eq!(rules.atmospheric_chemistry_rules().len(), 8);
    
    // 总规则数: 48
    let total = rules.atmospheric_radiation_rules().len()
        + rules.cloud_physics_rules().len()
        + rules.atmospheric_optics_rules().len()
        + rules.atmospheric_electrical_rules().len()
        + rules.atmospheric_acoustics_rules().len()
        + rules.atmospheric_chemistry_rules().len();
    assert_eq!(total, 48);
    
    assert!(!rules.explain().is_empty());
}

/// 测试海洋气象学规则
#[test]
fn test_marine_meteorology_rules_comprehensive() {
    let rules = MarineMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "海洋气象学规则");
    assert_eq!(rules.category(), RuleCategory::science("marine_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.air_sea_interaction_rules().len(), 8);
    assert_eq!(rules.marine_weather_systems_rules().len(), 8);
    assert_eq!(rules.marine_observation_rules().len(), 8);
    assert_eq!(rules.marine_forecast_rules().len(), 8);
    assert_eq!(rules.marine_disaster_rules().len(), 8);
    assert_eq!(rules.shipping_meteorology_rules().len(), 8);
    
    // 总规则数: 48
    let total = rules.air_sea_interaction_rules().len()
        + rules.marine_weather_systems_rules().len()
        + rules.marine_observation_rules().len()
        + rules.marine_forecast_rules().len()
        + rules.marine_disaster_rules().len()
        + rules.shipping_meteorology_rules().len();
    assert_eq!(total, 48);
    
    assert!(!rules.explain().is_empty());
}

/// 测试航空气象学规则
#[test]
fn test_aviation_meteorology_rules_comprehensive() {
    let rules = AviationMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "航空气象学规则");
    assert_eq!(rules.category(), RuleCategory::science("aviation_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.flight_meteorology_rules().len(), 8);
    assert_eq!(rules.airport_meteorology_rules().len(), 8);
    assert_eq!(rules.aviation_hazard_weather_rules().len(), 8);
    assert_eq!(rules.turbulence_rules().len(), 8);
    assert_eq!(rules.aircraft_icing_rules().len(), 8);
    assert_eq!(rules.wind_shear_rules().len(), 8);
    
    // 总规则数: 48
    let total = rules.flight_meteorology_rules().len()
        + rules.airport_meteorology_rules().len()
        + rules.aviation_hazard_weather_rules().len()
        + rules.turbulence_rules().len()
        + rules.aircraft_icing_rules().len()
        + rules.wind_shear_rules().len();
    assert_eq!(total, 48);
    
    assert!(!rules.explain().is_empty());
}

/// 测试气象雷达学规则
#[test]
fn test_radar_meteorology_rules_comprehensive() {
    let rules = RadarMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "气象雷达学规则");
    assert_eq!(rules.category(), RuleCategory::science("radar_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.radar_principle_rules().len(), 8);
    assert_eq!(rules.reflectivity_factor_rules().len(), 8);
    assert_eq!(rules.doppler_radar_rules().len(), 8);
    assert_eq!(rules.dual_polarization_radar_rules().len(), 8);
    assert_eq!(rules.radar_qpe_rules().len(), 8);
    assert_eq!(rules.echo_identification_rules().len(), 8);
    
    // 总规则数: 48
    let total = rules.radar_principle_rules().len()
        + rules.reflectivity_factor_rules().len()
        + rules.doppler_radar_rules().len()
        + rules.dual_polarization_radar_rules().len()
        + rules.radar_qpe_rules().len()
        + rules.echo_identification_rules().len();
    assert_eq!(total, 48);
    
    assert!(!rules.explain().is_empty());
}

/// 测试卫星气象学规则
#[test]
fn test_satellite_meteorology_rules_comprehensive() {
    let rules = SatelliteMeteorologyRules::new();
    assert_eq!(rules.metadata().name, "卫星气象学规则");
    assert_eq!(rules.category(), RuleCategory::science("satellite_meteorology"));
    
    // 验证所有方法返回正确数量
    assert_eq!(rules.satellite_remote_sensing_rules().len(), 8);
    assert_eq!(rules.satellite_channel_rules().len(), 8);
    assert_eq!(rules.satellite_cloud_image_rules().len(), 8);
    assert_eq!(rules.satellite_wind_rules().len(), 8);
    assert_eq!(rules.satellite_retrieval_rules().len(), 8);
    assert_eq!(rules.satellite_monitoring_rules().len(), 8);
    
    // 总规则数: 48
    let total = rules.satellite_remote_sensing_rules().len()
        + rules.satellite_channel_rules().len()
        + rules.satellite_cloud_image_rules().len()
        + rules.satellite_wind_rules().len()
        + rules.satellite_retrieval_rules().len()
        + rules.satellite_monitoring_rules().len();
    assert_eq!(total, 48);
    
    assert!(!rules.explain().is_empty());
}

// ============================================================================
// 特定功能测试
// ============================================================================

/// 测试天气预报规则
#[test]
fn test_weather_forecast_rules() {
    let rules = MeteorologyDetailedRules::new();
    let laws = rules.weather_forecast_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("预报")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("短期")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("中期")));
}

/// 测试大气环流规则
#[test]
fn test_atmospheric_circulation_rules() {
    let rules = MeteorologyDetailedRules::new();
    let laws = rules.atmospheric_circulation_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("环流")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("哈德莱")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("季风")));
}

/// 测试锋面分析规则
#[test]
fn test_frontal_analysis_rules() {
    let rules = SynopticMeteorologyRules::new();
    let laws = rules.frontal_analysis_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("锋面")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("冷锋")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("暖锋")));
}

/// 测试台风规则
#[test]
fn test_typhoon_rules() {
    let rules = TropicalMeteorologyRules::new();
    let laws = rules.typhoon_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("台风")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("路径")));
}

/// 测试强对流规则
#[test]
fn test_severe_convection_rules() {
    let rules = MesoscaleMeteorologyRules::new();
    let laws = rules.severe_convection_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("对流")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("触发")));
}

/// 测试龙卷风规则
#[test]
fn test_tornado_rules() {
    let rules = MesoscaleMeteorologyRules::new();
    let laws = rules.tornado_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("龙卷")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("强度")));
}

/// 测试大气波动规则
#[test]
fn test_atmospheric_waves_rules() {
    let rules = DynamicMeteorologyRules::new();
    let laws = rules.atmospheric_waves_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("Rossby")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("重力波")));
}

/// 测试大气辐射规则
#[test]
fn test_atmospheric_radiation_rules() {
    let rules = PhysicalMeteorologyRules::new();
    let laws = rules.atmospheric_radiation_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("辐射")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("太阳")));
}

/// 测试海气相互作用规则
#[test]
fn test_air_sea_interaction_rules() {
    let rules = MarineMeteorologyRules::new();
    let laws = rules.air_sea_interaction_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("交换")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("热量")));
}

/// 测试飞行颠簸规则
#[test]
fn test_turbulence_rules() {
    let rules = AviationMeteorologyRules::new();
    let laws = rules.turbulence_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("颠簸")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("预报")));
}

/// 测试多普勒雷达规则
#[test]
fn test_doppler_radar_rules() {
    let rules = RadarMeteorologyRules::new();
    let laws = rules.doppler_radar_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("多普勒")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("速度")));
}

/// 测试卫星云图规则
#[test]
fn test_satellite_cloud_image_rules() {
    let rules = SatelliteMeteorologyRules::new();
    let laws = rules.satellite_cloud_image_rules();
    assert!(laws.iter().any(|(n, _, _)| n.contains("云图")));
    assert!(laws.iter().any(|(n, _, _)| n.contains("红外")));
}