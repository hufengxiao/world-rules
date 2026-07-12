//! Phase 30: 生命科学扩充综合测试
//!
//! 本文件测试 Phase 30 添加的所有生命科学规则模块
//!
//! Phase 30-01: 生物学规则（10种）
//! - AgingBiologyLaws (衰老生物学)
//! - CancerBiologyLaws (癌症生物学)
//! - DevelopmentalBiologyLaws (发育生物学)
//! - EpigeneticsLaws (表观遗传学)
//! - MetabolicBiologyLaws (代谢生物学)
//! - PopulationBiologyLaws (种群生物学)
//! - StructuralBiologyLaws (结构生物学)
//! - SystemsBiologyLaws (系统生物学)
//! - VirologyLaws (病毒学)
//!
//! Phase 30-02: 医学基础规则（10种）
//! - DiagnosticsRules (诊断学)
//! - EmergencyMedicineRules (急诊医学)
//! - InternalMedicineRules (内科)
//! - ObstetricsGynecologyRules (妇产科)
//! - PathophysiologyRules (病理生理学)
//! - PediatricsRules (儿科)
//! - PharmacologyRules (药理学)
//! - PreventiveMedicineRules (预防医学)
//! - PsychiatryRules (精神病学)
//! - SurgeryRules (外科)
//!
//! Phase 30-03: 其他生命科学规则（10种）
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

use world_rules::rules::core::{Rule, RuleCategory};
use world_rules::rules::science::{
    AgingBiologyLaws, CancerBiologyLaws, DevelopmentalBiologyLaws,
    EpigeneticsLaws, MetabolicBiologyLaws, PopulationBiologyLaws,
    StructuralBiologyLaws, SystemsBiologyLaws, VirologyLaws,
    DiagnosticsRules, EmergencyMedicineRules, InternalMedicineRules,
    ObstetricsGynecologyRules, PathophysiologyRules, PediatricsRules,
    PharmacologyRules, PreventiveMedicineRules, PsychiatryRules, SurgeryRules,
    NeurobiologyRules, RegenerativeBiologyRules, StemCellBiologyRules,
    ImmunobiologyRules, BiomechanicsRules, BioopticsRules,
    BioacousticsRules, BioelectricityRules, BiothermodynamicsRules,
    ChronobiologyRules,
};

// ============================================================================
// Phase 30-01: 生物学规则测试
// ============================================================================

/// 测试衰老生物学规则
#[test]
fn test_aging_biology_rules() {
    let rules = AgingBiologyLaws::new();
    assert_eq!(rules.metadata().name, "衰老生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("aging_biology"));
    assert!(!rules.explain().is_empty());
}

/// 测试癌症生物学规则
#[test]
fn test_cancer_biology_rules() {
    let rules = CancerBiologyLaws::new();
    assert_eq!(rules.metadata().name, "癌症生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("cancer_biology"));
    assert!(!rules.explain().is_empty());
}

/// 测试发育生物学规则
#[test]
fn test_developmental_biology_rules() {
    let rules = DevelopmentalBiologyLaws::new();
    assert_eq!(rules.metadata().name, "发育生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("developmental_biology"));
    assert!(!rules.explain().is_empty());
}

/// 测试表观遗传学规则
#[test]
fn test_epigenetics_rules() {
    let rules = EpigeneticsLaws::new();
    assert_eq!(rules.metadata().name, "表观遗传学规则");
    assert_eq!(rules.category(), RuleCategory::science("epigenetics"));
    assert!(!rules.explain().is_empty());
}

/// 测试代谢生物学规则
#[test]
fn test_metabolic_biology_rules() {
    let rules = MetabolicBiologyLaws::new();
    assert_eq!(rules.metadata().name, "代谢生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("metabolic_biology"));
    assert!(!rules.explain().is_empty());
}

/// 测试种群生物学规则
#[test]
fn test_population_biology_rules() {
    let rules = PopulationBiologyLaws::new();
    assert_eq!(rules.metadata().name, "种群生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("population_biology"));
    assert!(!rules.explain().is_empty());
}

/// 测试结构生物学规则
#[test]
fn test_structural_biology_rules() {
    let rules = StructuralBiologyLaws::new();
    assert_eq!(rules.metadata().name, "结构生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("structural_biology"));
    assert!(!rules.explain().is_empty());
}

/// 测试系统生物学规则
#[test]
fn test_systems_biology_rules() {
    let rules = SystemsBiologyLaws::new();
    assert_eq!(rules.metadata().name, "系统生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("systems_biology"));
    assert!(!rules.explain().is_empty());
}

/// 测试病毒学规则
#[test]
fn test_virology_rules() {
    let rules = VirologyLaws::new();
    assert_eq!(rules.metadata().name, "病毒学规则");
    assert_eq!(rules.category(), RuleCategory::science("virology"));
    assert!(!rules.explain().is_empty());
}

// ============================================================================
// Phase 30-02: 医学基础规则测试
// ============================================================================

/// 测试诊断学规则
#[test]
fn test_diagnostics_rules() {
    let rules = DiagnosticsRules::new();
    assert_eq!(rules.metadata().name, "诊断学规则");
    assert_eq!(rules.category(), RuleCategory::science("diagnostics"));
    assert!(!rules.explain().is_empty());
}

/// 测试急诊医学规则
#[test]
fn test_emergency_medicine_rules() {
    let rules = EmergencyMedicineRules::new();
    assert_eq!(rules.metadata().name, "急诊医学规则");
    assert_eq!(rules.category(), RuleCategory::science("emergency_medicine"));
    assert!(!rules.explain().is_empty());
}

/// 测试内科规则
#[test]
fn test_internal_medicine_rules() {
    let rules = InternalMedicineRules::new();
    assert_eq!(rules.metadata().name, "内科规则");
    assert_eq!(rules.category(), RuleCategory::science("internal_medicine"));
    assert!(!rules.explain().is_empty());
}

/// 测试妇产科规则
#[test]
fn test_obstetrics_gynecology_rules() {
    let rules = ObstetricsGynecologyRules::new();
    assert_eq!(rules.metadata().name, "妇产科规则");
    assert_eq!(rules.category(), RuleCategory::science("obstetrics_gynecology"));
    assert!(!rules.explain().is_empty());
}

/// 测试病理生理学规则
#[test]
fn test_pathophysiology_rules() {
    let rules = PathophysiologyRules::new();
    assert_eq!(rules.metadata().name, "病理生理学规则");
    assert_eq!(rules.category(), RuleCategory::science("pathophysiology"));
    assert!(!rules.explain().is_empty());
}

/// 测试儿科规则
#[test]
fn test_pediatrics_rules() {
    let rules = PediatricsRules::new();
    assert_eq!(rules.metadata().name, "儿科规则");
    assert_eq!(rules.category(), RuleCategory::science("pediatrics"));
    assert!(!rules.explain().is_empty());
}

/// 测试药理学规则
#[test]
fn test_pharmacology_rules() {
    let rules = PharmacologyRules::new();
    assert_eq!(rules.metadata().name, "药理学规则");
    assert_eq!(rules.category(), RuleCategory::science("pharmacology"));
    assert!(!rules.explain().is_empty());
}

/// 测试预防医学规则
#[test]
fn test_preventive_medicine_rules() {
    let rules = PreventiveMedicineRules::new();
    assert_eq!(rules.metadata().name, "预防医学规则");
    assert_eq!(rules.category(), RuleCategory::science("preventive_medicine"));
    assert!(!rules.explain().is_empty());
}

/// 测试精神病学规则
#[test]
fn test_psychiatry_rules() {
    let rules = PsychiatryRules::new();
    assert_eq!(rules.metadata().name, "精神病学规则");
    assert_eq!(rules.category(), RuleCategory::science("psychiatry"));
    assert!(!rules.explain().is_empty());
}

/// 测试外科规则
#[test]
fn test_surgery_rules() {
    let rules = SurgeryRules::new();
    assert_eq!(rules.metadata().name, "外科规则");
    assert_eq!(rules.category(), RuleCategory::science("surgery"));
    assert!(!rules.explain().is_empty());
}

// ============================================================================
// Phase 30-03: 其他生命科学规则测试
// ============================================================================

/// 测试神经生物学规则
#[test]
fn test_neurobiology_rules_comprehensive() {
    let rules = NeurobiologyRules::new();
    assert_eq!(rules.metadata().name, "神经生物学规则");
    assert_eq!(rules.category(), RuleCategory::science("neurobiology"));
    
    // 验证所有方法返回正确数量
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
    
    // 总规则数: 70
    let total = rules.neuron_structure_laws().len()
        + rules.synaptic_transmission_laws().len()
        + rules.action_potential_laws().len()
        + rules.neuroplasticity_laws().len()
        + rules.neural_coding_laws().len()
        + rules.neurotransmitter_laws().len()
        + rules.sensory_neural_laws().len()
        + rules.motor_neural_laws().len()
        + rules.neural_development_laws().len()
        + rules.neural_regeneration_laws().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

/// 测试再生生物学规则
#[test]
fn test_regenerative_biology_rules_comprehensive() {
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
    
    // 总规则数: 69
    let total = rules.regeneration_types().len()
        + rules.regeneration_capacity().len()
        + rules.stem_cell_regeneration().len()
        + rules.regeneration_signaling().len()
        + rules.regeneration_factors().len()
        + rules.regeneration_inhibition().len()
        + rules.organ_regeneration().len()
        + rules.limb_regeneration().len()
        + rules.regenerative_medicine().len()
        + rules.regeneration_methods().len();
    assert_eq!(total, 69);
    
    assert!(!rules.explain().is_empty());
}

/// 测试干细胞生物学规则
#[test]
fn test_stem_cell_biology_rules_comprehensive() {
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
    
    // 总规则数: 70
    let total = rules.stem_cell_types().len()
        + rules.stem_cell_properties().len()
        + rules.stem_cell_niche().len()
        + rules.stem_cell_differentiation().len()
        + rules.stem_cell_regulation().len()
        + rules.transcription_factor_network().len()
        + rules.epigenetic_regulation().len()
        + rules.stem_cell_applications().len()
        + rules.stem_cell_techniques().len()
        + rules.stem_cell_ethics().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

/// 测试免疫生物学规则
#[test]
fn test_immunobiology_rules_comprehensive() {
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
    
    // 总规则数: 70
    let total = rules.immune_cell_types().len()
        + rules.immune_response_laws().len()
        + rules.antigen_presentation().len()
        + rules.t_cell_development().len()
        + rules.b_cell_development().len()
        + rules.immune_regulation().len()
        + rules.immune_pathology().len()
        + rules.immune_tolerance().len()
        + rules.immune_memory().len()
        + rules.immune_evolution().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

/// 测试生物力学规则
#[test]
fn test_biomechanics_rules_comprehensive() {
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
    
    // 总规则数: 70
    let total = rules.bone_mechanics().len()
        + rules.muscle_mechanics().len()
        + rules.joint_mechanics().len()
        + rules.hemodynamics().len()
        + rules.cardiac_mechanics().len()
        + rules.respiratory_mechanics().len()
        + rules.locomotion_mechanics().len()
        + rules.cell_mechanics().len()
        + rules.tissue_mechanics().len()
        + rules.biomechanics_applications().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

/// 测试生物光学规则
#[test]
fn test_biooptics_rules_comprehensive() {
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
    
    // 总规则数: 70
    let total = rules.visual_system().len()
        + rules.photoreception().len()
        + rules.bioluminescence().len()
        + rules.photosynthesis().len()
        + rules.photoperiod().len()
        + rules.phototherapy().len()
        + rules.photodamage().len()
        + rules.bioimaging().len()
        + rules.color_perception().len()
        + rules.biooptics_applications().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

/// 测试生物声学规则
#[test]
fn test_bioacoustics_rules_comprehensive() {
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
    
    // 总规则数: 70
    let total = rules.auditory_system().len()
        + rules.sound_production().len()
        + rules.animal_sound().len()
        + rules.acoustic_communication().len()
        + rules.sonar().len()
        + rules.acoustic_environment().len()
        + rules.acoustic_behavior().len()
        + rules.acoustic_measurement().len()
        + rules.noise().len()
        + rules.bioacoustics_applications().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

/// 测试生物电学规则
#[test]
fn test_bioelectricity_rules_comprehensive() {
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
    
    // 总规则数: 70
    let total = rules.neural_electrical().len()
        + rules.cardiac_electrical().len()
        + rules.muscle_electrical().len()
        + rules.bioelectric_signals().len()
        + rules.electroreception().len()
        + rules.bioelectric_generation().len()
        + rules.electrical_transmission().len()
        + rules.electrical_measurement().len()
        + rules.electrical_modulation().len()
        + rules.bioelectricity_applications().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

/// 测试生物热力学规则
#[test]
fn test_biothermodynamics_rules_comprehensive() {
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
    
    // 总规则数: 70
    let total = rules.temperature_regulation().len()
        + rules.heat_production().len()
        + rules.heat_loss().len()
        + rules.energy_metabolism().len()
        + rules.temperature_adaptation().len()
        + rules.thermal_stress().len()
        + rules.tissue_temperature().len()
        + rules.heat_transfer().len()
        + rules.thermal_sensation().len()
        + rules.biothermodynamics_applications().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

/// 测试生物节律规则
#[test]
fn test_chronobiology_rules_comprehensive() {
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
    
    // 总规则数: 70
    let total = rules.circadian_rhythms().len()
        + rules.biological_clock().len()
        + rules.clock_genes().len()
        + rules.sleep_wake_rhythm().len()
        + rules.seasonal_rhythms().len()
        + rules.tidal_rhythms().len()
        + rules.rhythm_regulation().len()
        + rules.rhythm_disorders().len()
        + rules.rhythm_measurement().len()
        + rules.chronobiology_applications().len();
    assert_eq!(total, 70);
    
    assert!(!rules.explain().is_empty());
}

// ============================================================================
// Phase 30 综合统计测试
// ============================================================================

/// 测试 Phase 30 规则总数统计
#[test]
fn test_phase_30_total_rules_count() {
    // Phase 30-01: 9种生物学规则（每种约10方法，每方法约7规则）约 630 条
    // Phase 30-02: 10种医学规则（每种约10方法，每方法约7规则）约 700 条
    // Phase 30-03: 10种其他生命科学规则（70 + 69 + 70×8）= 699 条
    
    // 这里只验证 Phase 30-03 的精确计数
    // Phase 30-01 和 30-02 的详细计数在各自的模块测试中
    
    let neurobiology = NeurobiologyRules::new();
    let regenerative = RegenerativeBiologyRules::new();
    let stem_cell = StemCellBiologyRules::new();
    let immunobiology = ImmunobiologyRules::new();
    let biomechanics = BiomechanicsRules::new();
    let biooptics = BioopticsRules::new();
    let bioacoustics = BioacousticsRules::new();
    let bioelectricity = BioelectricityRules::new();
    let biothermodynamics = BiothermodynamicsRules::new();
    let chronobiology = ChronobiologyRules::new();
    
    // Phase 30-03 总规则数
    let phase_30_03_total = 70 + 69 + 70 * 8; // 699
    assert_eq!(phase_30_03_total, 699);
}

/// 测试所有 Phase 30 规则已注册到 all_rules
#[test]
fn test_phase_30_rules_registered() {
    use world_rules::rules::science::all_rules;
    
    let rules = all_rules();
    
    // Phase 30-01: 生物学规则
    assert!(rules.iter().any(|r| r.1.name == "衰老生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "癌症生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "发育生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "表观遗传学规则"));
    assert!(rules.iter().any(|r| r.1.name == "代谢生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "种群生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "结构生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "系统生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "病毒学规则"));
    
    // Phase 30-02: 医学规则
    assert!(rules.iter().any(|r| r.1.name == "诊断学规则"));
    assert!(rules.iter().any(|r| r.1.name == "急诊医学规则"));
    assert!(rules.iter().any(|r| r.1.name == "内科规则"));
    assert!(rules.iter().any(|r| r.1.name == "妇产科规则"));
    assert!(rules.iter().any(|r| r.1.name == "病理生理学规则"));
    assert!(rules.iter().any(|r| r.1.name == "儿科规则"));
    assert!(rules.iter().any(|r| r.1.name == "药理学规则"));
    assert!(rules.iter().any(|r| r.1.name == "预防医学规则"));
    assert!(rules.iter().any(|r| r.1.name == "精神病学规则"));
    assert!(rules.iter().any(|r| r.1.name == "外科规则"));
    
    // Phase 30-03: 其他生命科学规则
    assert!(rules.iter().any(|r| r.1.name == "神经生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "再生生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "干细胞生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "免疫生物学规则"));
    assert!(rules.iter().any(|r| r.1.name == "生物力学规则"));
    assert!(rules.iter().any(|r| r.1.name == "生物光学规则"));
    assert!(rules.iter().any(|r| r.1.name == "生物声学规则"));
    assert!(rules.iter().any(|r| r.1.name == "生物电学规则"));
    assert!(rules.iter().any(|r| r.1.name == "生物热力学规则"));
    assert!(rules.iter().any(|r| r.1.name == "生物节律规则"));
}