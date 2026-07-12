//! Phase 30-03: 其他生命科学规则测试
//!
//! 测试新增的 10 个生命科学规则模块：
//! - NeurobiologyRules (神经生物学)
//! - RegenerativeBiologyRules (再生生物学)
//! - StemCellBiologyRules (干细胞生物学)
//! - ImmunobiologyRules (免疫生物学)
//! - BiomechanicsRules (生物力学)
//! - BioopticsRules (生物光学)
//! - BioacousticsRules (生物声学)
//! - BioelectricityRules (生物电学)
//! - BiothermodynamicsRules (生物热力学)
//! - ChronobiologyRules (生物节律)

//!
//! 每个模块包含 10 个方法，每个方法返回 6-7 个规则元组
//! 总规则数: 10 × 10 × ~7 = 700 条规则

use world_rules::rules::science::{
    NeurobiologyRules, RegenerativeBiologyRules, StemCellBiologyRules,
    ImmunobiologyRules, BiomechanicsRules, BioopticsRules,
    BioacousticsRules, BioelectricityRules, BiothermodynamicsRules,
    ChronobiologyRules,
};
use world_rules::rules::core::{Rule, RuleCategory};

/// 测试神经生物学规则
#[test]
fn test_neurobiology_rules() {
    let rules = NeurobiologyRules::new();
    
    // 测试基本属性
    assert_eq!(rules.metadata().name, "神经生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("neurobiology"));
    
    // 测试所有方法返回正确的数量
    assert_eq!(rules.neuron_structure_laws().len(), 7);
    assert_eq!(rules.synaptic_transmission_laws().len(), 7);
    assert_eq!(rules.action_potential_laws().len(), 7);
    assert_eq!(rules.neuroplasticity_laws().len(), 7);
    assert_eq!(rules.neural_coding_laws().len(), 7);
    assert_eq!(rules.neurotransmitter_laws().len(), 7);
    assert_eq!(rules.sensory_neural_laws().len(), 7);
    assert_eq!(rules.motor_neural_laws().len(), 7);
    assert_eq!(rules.neural_development_laws().len(), 7);
    assert_eq!(rules.neural_regeneration_laws().len(), 7);
    
    // 测试 explain 方法不为空
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("神经生物学规则"));
}

/// 测试再生生物学规则
#[test]
fn test_regenerative_biology_rules() {
    let rules = RegenerativeBiologyRules::new();
    
    assert_eq!(rules.metadata().name, "再生生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("regenerative_biology"));
    
    assert_eq!(rules.regeneration_types().len(), 6);
    assert_eq!(rules.regeneration_capacity().len(), 7);
    assert_eq!(rules.stem_cell_regeneration().len(), 7);
    assert_eq!(rules.regeneration_signaling().len(), 7);
    assert_eq!(rules.regeneration_factors().len(), 7);
    assert_eq!(rules.regeneration_inhibition().len(), 7);
    assert_eq!(rules.organ_regeneration().len(), 7);
    assert_eq!(rules.limb_regeneration().len(), 7);
    assert_eq!(rules.regenerative_medicine().len(), 7);
    assert_eq!(rules.regeneration_methods().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("再生生物学规则"));
}

/// 测试干细胞生物学规则
#[test]
fn test_stem_cell_biology_rules() {
    let rules = StemCellBiologyRules::new();
    
    assert_eq!(rules.metadata().name, "干细胞生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("stem_cell_biology"));
    
    assert_eq!(rules.stem_cell_types().len(), 7);
    assert_eq!(rules.stem_cell_properties().len(), 7);
    assert_eq!(rules.stem_cell_niche().len(), 7);
    assert_eq!(rules.stem_cell_differentiation().len(), 7);
    assert_eq!(rules.stem_cell_regulation().len(), 7);
    assert_eq!(rules.transcription_factor_network().len(), 7);
    assert_eq!(rules.epigenetic_regulation().len(), 7);
    assert_eq!(rules.stem_cell_applications().len(), 7);
    assert_eq!(rules.stem_cell_techniques().len(), 7);
    assert_eq!(rules.stem_cell_ethics().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("干细胞生物学规则"));
}

/// 测试免疫生物学规则
#[test]
fn test_immunobiology_rules() {
    let rules = ImmunobiologyRules::new();
    
    assert_eq!(rules.metadata().name, "免疫生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("immunobiology"));
    
    assert_eq!(rules.immune_cell_types().len(), 7);
    assert_eq!(rules.immune_response_laws().len(), 7);
    assert_eq!(rules.antigen_presentation().len(), 7);
    assert_eq!(rules.t_cell_development().len(), 7);
    assert_eq!(rules.b_cell_development().len(), 7);
    assert_eq!(rules.immune_regulation().len(), 7);
    assert_eq!(rules.immune_pathology().len(), 7);
    assert_eq!(rules.immune_tolerance().len(), 7);
    assert_eq!(rules.immune_memory().len(), 7);
    assert_eq!(rules.immune_evolution().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("免疫生物学规则"));
}

/// 测试生物力学规则
#[test]
fn test_biomechanics_rules() {
    let rules = BiomechanicsRules::new();
    
    assert_eq!(rules.metadata().name, "生物力学规则");
    assert_eq!(rules.category(), RuleCategory::science("biomechanics"));
    
    assert_eq!(rules.bone_mechanics().len(), 7);
    assert_eq!(rules.muscle_mechanics().len(), 7);
    assert_eq!(rules.joint_mechanics().len(), 7);
    assert_eq!(rules.hemodynamics().len(), 7);
    assert_eq!(rules.cardiac_mechanics().len(), 7);
    assert_eq!(rules.respiratory_mechanics().len(), 7);
    assert_eq!(rules.locomotion_mechanics().len(), 7);
    assert_eq!(rules.cell_mechanics().len(), 7);
    assert_eq!(rules.tissue_mechanics().len(), 7);
    assert_eq!(rules.biomechanics_applications().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("生物力学规则"));
}

/// 测试生物光学规则
#[test]
fn test_biooptics_rules() {
    let rules = BioopticsRules::new();
    
    assert_eq!(rules.metadata().name, "生物光学规则");
    assert_eq!(rules.category(), RuleCategory::science("biooptics"));
    
    assert_eq!(rules.visual_system().len(), 7);
    assert_eq!(rules.photoreception().len(), 7);
    assert_eq!(rules.bioluminescence().len(), 7);
    assert_eq!(rules.photosynthesis().len(), 7);
    assert_eq!(rules.photoperiod().len(), 7);
    assert_eq!(rules.phototherapy().len(), 7);
    assert_eq!(rules.photodamage().len(), 7);
    assert_eq!(rules.bioimaging().len(), 7);
    assert_eq!(rules.color_perception().len(), 7);
    assert_eq!(rules.biooptics_applications().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("生物光学规则"));
}

/// 测试生物声学规则
#[test]
fn test_bioacoustics_rules() {
    let rules = BioacousticsRules::new();
    
    assert_eq!(rules.metadata().name, "生物声学规则");
    assert_eq!(rules.category(), RuleCategory::science("bioacoustics"));
    
    assert_eq!(rules.auditory_system().len(), 7);
    assert_eq!(rules.sound_production().len(), 7);
    assert_eq!(rules.animal_sound().len(), 7);
    assert_eq!(rules.acoustic_communication().len(), 7);
    assert_eq!(rules.sonar().len(), 7);
    assert_eq!(rules.acoustic_environment().len(), 7);
    assert_eq!(rules.acoustic_behavior().len(), 7);
    assert_eq!(rules.acoustic_measurement().len(), 7);
    assert_eq!(rules.noise().len(), 7);
    assert_eq!(rules.bioacoustics_applications().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("生物声学规则"));
}

/// 测试生物电学规则
#[test]
fn test_bioelectricity_rules() {
    let rules = BioelectricityRules::new();
    
    assert_eq!(rules.metadata().name, "生物电学规则");
    assert_eq!(rules.category(), RuleCategory::science("bioelectricity"));
    
    assert_eq!(rules.neural_electrical().len(), 7);
    assert_eq!(rules.cardiac_electrical().len(), 7);
    assert_eq!(rules.muscle_electrical().len(), 7);
    assert_eq!(rules.bioelectric_signals().len(), 7);
    assert_eq!(rules.electroreception().len(), 7);
    assert_eq!(rules.bioelectric_generation().len(), 7);
    assert_eq!(rules.electrical_transmission().len(), 7);
    assert_eq!(rules.electrical_measurement().len(), 7);
    assert_eq!(rules.electrical_modulation().len(), 7);
    assert_eq!(rules.bioelectricity_applications().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("生物电学规则"));
}

/// 测试生物热力学规则
#[test]
fn test_biothermodynamics_rules() {
    let rules = BiothermodynamicsRules::new();
    
    assert_eq!(rules.metadata().name, "生物热力学规则");
    assert_eq!(rules.category(), RuleCategory::science("biothermodynamics"));
    
    assert_eq!(rules.temperature_regulation().len(), 7);
    assert_eq!(rules.heat_production().len(), 7);
    assert_eq!(rules.heat_loss().len(), 7);
    assert_eq!(rules.energy_metabolism().len(), 7);
    assert_eq!(rules.temperature_adaptation().len(), 7);
    assert_eq!(rules.thermal_stress().len(), 7);
    assert_eq!(rules.tissue_temperature().len(), 7);
    assert_eq!(rules.heat_transfer().len(), 7);
    assert_eq!(rules.thermal_sensation().len(), 7);
    assert_eq!(rules.biothermodynamics_applications().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("生物热力学规则"));
}

/// 测试生物节律规则
#[test]
fn test_chronobiology_rules() {
    let rules = ChronobiologyRules::new();
    
    assert_eq!(rules.metadata().name, "生物节律规则");
    assert_eq!(rules.category(), RuleCategory::science("chronobiology"));
    
    assert_eq!(rules.circadian_rhythms().len(), 7);
    assert_eq!(rules.biological_clock().len(), 7);
    assert_eq!(rules.clock_genes().len(), 7);
    assert_eq!(rules.sleep_wake_rhythm().len(), 7);
    assert_eq!(rules.seasonal_rhythms().len(), 7);
    assert_eq!(rules.tidal_rhythms().len(), 7);
    assert_eq!(rules.rhythm_regulation().len(), 7);
    assert_eq!(rules.rhythm_disorders().len(), 7);
    assert_eq!(rules.rhythm_measurement().len(), 7);
    assert_eq!(rules.chronobiology_applications().len(), 7);
    
    let explanation = rules.explain();
    assert!(!explanation.is_empty());
    assert!(explanation.contains("生物节律规则"));
}

/// 测试所有规则可以通过 all_rules 获取
#[test]
fn test_phase_30_03_all_rules_registered() {
    use world_rules::rules::science::all_rules;
    
    let rules = all_rules();
    
    // 查找新增的规则
    let neurobiology = rules.iter().find(|r| r.1.name == "神经生物学规则");
    assert!(neurobiology.is_some());
    
    let regenerative = rules.iter().find(|r| r.1.name == "再生生物学规则");
    assert!(regenerative.is_some());
    
    let stem_cell = rules.iter().find(|r| r.1.name == "干细胞生物学规则");
    assert!(stem_cell.is_some());
    
    let immunobiology = rules.iter().find(|r| r.1.name == "免疫生物学规则");
    assert!(immunobiology.is_some());
    
    let biomechanics = rules.iter().find(|r| r.1.name == "生物力学规则");
    assert!(biomechanics.is_some());
    
    let biooptics = rules.iter().find(|r| r.1.name == "生物光学规则");
    assert!(biooptics.is_some());
    
    let bioacoustics = rules.iter().find(|r| r.1.name == "生物声学规则");
    assert!(bioacoustics.is_some());
    
    let bioelectricity = rules.iter().find(|r| r.1.name == "生物电学规则");
    assert!(bioelectricity.is_some());
    
    let biothermodynamics = rules.iter().find(|r| r.1.name == "生物热力学规则");
    assert!(biothermodynamics.is_some());
    
    let chronobiology = rules.iter().find(|r| r.1.name == "生物节律规则");
    assert!(chronobiology.is_some());
}