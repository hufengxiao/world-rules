//! Phase 28 物理规则测试
//!
//! 测试新增的 20 种物理规则（10 种力学 + 10 种电磁学）

use world_rules::rules::core::{Rule, RuleCategory, ValidateContext};
use world_rules::rules::science::{
    // 力学规则 (Phase 28-01)
    AnalyticalMechanicsRules,
    AntennaTheoryRules,
    CelestialMechanicsRules,
    CircuitTheoryRules,
    ComputationalMechanicsRules,
    DynamicsRules,
    ElectromagneticCompatibilityRules,
    ElectromagneticInductionRules,
    ElectromagneticWavePropagationRules,
    // 电磁学规则 (Phase 28-02)
    ElectrostaticsRules,
    FluidDynamicsRules,
    KinematicsRules,
    MagnetostaticsRules,
    MaterialsMechanicsRules,
    MaxwellEquationsRules,
    MicrowaveTechnologyRules,
    OpticalBasicsRules,
    RigidBodyDynamicsRules,
    StaticsRules,
    VibrationWaveRules,
};

/// 测试静力学规则
#[test]
fn test_statics_rules() {
    let rules = StaticsRules::new();
    assert_eq!(rules.metadata().name, "静力学规则");
    assert_eq!(rules.category(), RuleCategory::science("statics"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.equilibrium_conditions().is_empty());
    assert!(!rules.force_analysis().is_empty());
    assert!(!rules.moment_calculation().is_empty());
    assert!(!rules.truss_analysis().is_empty());
    assert!(!rules.beam_analysis().is_empty());
    assert!(!rules.friction_analysis().is_empty());
    assert!(!rules.center_of_gravity().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试动力学规则
#[test]
fn test_dynamics_rules() {
    let rules = DynamicsRules::new();
    assert_eq!(rules.metadata().name, "动力学规则");
    assert_eq!(rules.category(), RuleCategory::science("dynamics"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.newton_laws().is_empty());
    assert!(!rules.force_types().is_empty());
    assert!(!rules.motion_equations().is_empty());
    assert!(!rules.circular_motion().is_empty());
    assert!(!rules.work_power().is_empty());
    assert!(!rules.energy().is_empty());
    assert!(!rules.momentum_impulse().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试运动学规则
#[test]
fn test_kinematics_rules() {
    let rules = KinematicsRules::new();
    assert_eq!(rules.metadata().name, "运动学规则");
    assert_eq!(rules.category(), RuleCategory::science("kinematics"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.basic_concepts().is_empty());
    assert!(!rules.linear_motion().is_empty());
    assert!(!rules.curvilinear_motion().is_empty());
    assert!(!rules.circular_motion().is_empty());
    assert!(!rules.relative_motion().is_empty());
    assert!(!rules.motion_graphs().is_empty());
    assert!(!rules.motion_constraints().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试材料力学规则
#[test]
fn test_materials_mechanics_rules() {
    let rules = MaterialsMechanicsRules::new();
    assert_eq!(rules.metadata().name, "材料力学规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("materials_mechanics")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.stress_strain().is_empty());
    assert!(!rules.elastic_deformation().is_empty());
    assert!(!rules.tension_compression().is_empty());
    assert!(!rules.shear_torsion().is_empty());
    assert!(!rules.bending().is_empty());
    assert!(!rules.combined_deformation().is_empty());
    assert!(!rules.fatigue_fracture().is_empty());
    assert!(!rules.material_properties().is_empty());
}

/// 测试流体动力学规则
#[test]
fn test_fluid_dynamics_rules() {
    let rules = FluidDynamicsRules::new();
    assert_eq!(rules.metadata().name, "流体动力学规则");
    assert_eq!(rules.category(), RuleCategory::science("fluid_dynamics"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.fluid_properties().is_empty());
    assert!(!rules.fluid_statics().is_empty());
    assert!(!rules.fluid_kinematics().is_empty());
    assert!(!rules.continuity_equation().is_empty());
    assert!(!rules.bernoulli_equation().is_empty());
    assert!(!rules.reynolds_number().is_empty());
    assert!(!rules.fluid_resistance().is_empty());
    assert!(!rules.pipe_flow().is_empty());
}

/// 测试振动与波规则
#[test]
fn test_vibration_wave_rules() {
    let rules = VibrationWaveRules::new();
    assert_eq!(rules.metadata().name, "振动与波规则");
    assert_eq!(rules.category(), RuleCategory::science("vibration_wave"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.simple_harmonic_motion().is_empty());
    assert!(!rules.vibration_energy().is_empty());
    assert!(!rules.damped_vibration().is_empty());
    assert!(!rules.forced_vibration().is_empty());
    assert!(!rules.wave_basics().is_empty());
    assert!(!rules.wave_interference().is_empty());
    assert!(!rules.wave_diffraction().is_empty());
    assert!(!rules.sound_waves().is_empty());
}

/// 测试刚体动力学规则
#[test]
fn test_rigid_body_dynamics_rules() {
    let rules = RigidBodyDynamicsRules::new();
    assert_eq!(rules.metadata().name, "刚体动力学规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("rigid_body_dynamics")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.translation().is_empty());
    assert!(!rules.rotation().is_empty());
    assert!(!rules.moment_of_inertia().is_empty());
    assert!(!rules.rotational_dynamics().is_empty());
    assert!(!rules.translation_rotation_relation().is_empty());
    assert!(!rules.rigid_body_equilibrium().is_empty());
    assert!(!rules.rigid_body_collision().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试分析力学规则
#[test]
fn test_analytical_mechanics_rules() {
    let rules = AnalyticalMechanicsRules::new();
    assert_eq!(rules.metadata().name, "分析力学规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("analytical_mechanics")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.generalized_coordinates().is_empty());
    assert!(!rules.virtual_work().is_empty());
    assert!(!rules.lagrangian_mechanics().is_empty());
    assert!(!rules.lagrangian_applications().is_empty());
    assert!(!rules.hamiltonian_mechanics().is_empty());
    assert!(!rules.hamiltonian_applications().is_empty());
    assert!(!rules.conservation_laws().is_empty());
    assert!(!rules.variational_principle().is_empty());
}

/// 测试天体力学规则
#[test]
fn test_celestial_mechanics_rules() {
    let rules = CelestialMechanicsRules::new();
    assert_eq!(rules.metadata().name, "天体力学规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("celestial_mechanics")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.gravitational_laws().is_empty());
    assert!(!rules.orbital_motion().is_empty());
    assert!(!rules.circular_orbit().is_empty());
    assert!(!rules.orbital_energy().is_empty());
    assert!(!rules.orbital_transfer().is_empty());
    assert!(!rules.multi_body_problem().is_empty());
    assert!(!rules.orbital_perturbation().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试计算力学规则
#[test]
fn test_computational_mechanics_rules() {
    let rules = ComputationalMechanicsRules::new();
    assert_eq!(rules.metadata().name, "计算力学规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("computational_mechanics")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.numerical_methods().is_empty());
    assert!(!rules.finite_element_method().is_empty());
    assert!(!rules.boundary_element_method().is_empty());
    assert!(!rules.meshless_methods().is_empty());
    assert!(!rules.multibody_dynamics().is_empty());
    assert!(!rules.computational_fluid_dynamics().is_empty());
    assert!(!rules.structural_optimization().is_empty());
    assert!(!rules.software_tools().is_empty());
}

// ============= Phase 28-02: 电磁学规则测试 =============

/// 测试静电学规则
#[test]
fn test_electrostatics_rules() {
    let rules = ElectrostaticsRules::new();
    assert_eq!(rules.metadata().name, "静电学规则");
    assert_eq!(rules.category(), RuleCategory::science("electrostatics"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.coulomb_law().is_empty());
    assert!(!rules.electric_field().is_empty());
    assert!(!rules.gauss_law().is_empty());
    assert!(!rules.electric_potential().is_empty());
    assert!(!rules.capacitance().is_empty());
    assert!(!rules.conductors().is_empty());
    assert!(!rules.dielectrics().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试静磁学规则
#[test]
fn test_magnetostatics_rules() {
    let rules = MagnetostaticsRules::new();
    assert_eq!(rules.metadata().name, "静磁学规则");
    assert_eq!(rules.category(), RuleCategory::science("magnetostatics"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.magnetic_field_basics().is_empty());
    assert!(!rules.biot_savart_law().is_empty());
    assert!(!rules.ampere_law().is_empty());
    assert!(!rules.magnetic_dipole().is_empty());
    assert!(!rules.magnetic_force().is_empty());
    assert!(!rules.magnetic_materials().is_empty());
    assert!(!rules.magnetic_circuit().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试电路理论规则
#[test]
fn test_circuit_theory_rules() {
    let rules = CircuitTheoryRules::new();
    assert_eq!(rules.metadata().name, "电路理论规则");
    assert_eq!(rules.category(), RuleCategory::science("circuit_theory"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.basic_laws().is_empty());
    assert!(!rules.kirchhoff_laws().is_empty());
    assert!(!rules.analysis_methods().is_empty());
    assert!(!rules.capacitor_circuits().is_empty());
    assert!(!rules.inductor_circuits().is_empty());
    assert!(!rules.rlc_circuits().is_empty());
    assert!(!rules.ac_circuits().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试电磁感应规则
#[test]
fn test_electromagnetic_induction_rules() {
    let rules = ElectromagneticInductionRules::new();
    assert_eq!(rules.metadata().name, "电磁感应规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("electromagnetic_induction")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.faraday_law().is_empty());
    assert!(!rules.lenz_law().is_empty());
    assert!(!rules.self_mutual_induction().is_empty());
    assert!(!rules.induced_current().is_empty());
    assert!(!rules.eddy_current().is_empty());
    assert!(!rules.transformer_principle().is_empty());
    assert!(!rules.electromagnetic_waves().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试麦克斯韦方程组规则
#[test]
fn test_maxwell_equations_rules() {
    let rules = MaxwellEquationsRules::new();
    assert_eq!(rules.metadata().name, "麦克斯韦方程组规则");
    assert_eq!(rules.category(), RuleCategory::science("maxwell_equations"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.gauss_electric_law().is_empty());
    assert!(!rules.gauss_magnetic_law().is_empty());
    assert!(!rules.faraday_induction_law().is_empty());
    assert!(!rules.ampere_maxwell_law().is_empty());
    assert!(!rules.auxiliary_equations().is_empty());
    assert!(!rules.electromagnetic_wave_equation().is_empty());
    assert!(!rules.boundary_conditions().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试电磁波传播规则
#[test]
fn test_electromagnetic_wave_propagation_rules() {
    let rules = ElectromagneticWavePropagationRules::new();
    assert_eq!(rules.metadata().name, "电磁波传播规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("electromagnetic_wave_propagation")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.wave_generation().is_empty());
    assert!(!rules.propagation_properties().is_empty());
    assert!(!rules.reflection_refraction().is_empty());
    assert!(!rules.polarization().is_empty());
    assert!(!rules.propagation_in_media().is_empty());
    assert!(!rules.propagation_in_conductors().is_empty());
    assert!(!rules.electromagnetic_spectrum().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试电磁兼容规则
#[test]
fn test_electromagnetic_compatibility_rules() {
    let rules = ElectromagneticCompatibilityRules::new();
    assert_eq!(rules.metadata().name, "电磁兼容规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("electromagnetic_compatibility")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.emi_basics().is_empty());
    assert!(!rules.conducted_interference().is_empty());
    assert!(!rules.radiated_interference().is_empty());
    assert!(!rules.shielding_techniques().is_empty());
    assert!(!rules.filtering_techniques().is_empty());
    assert!(!rules.grounding_techniques().is_empty());
    assert!(!rules.emc_standards().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试微波技术规则
#[test]
fn test_microwave_technology_rules() {
    let rules = MicrowaveTechnologyRules::new();
    assert_eq!(rules.metadata().name, "微波技术规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("microwave_technology")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.microwave_properties().is_empty());
    assert!(!rules.transmission_lines().is_empty());
    assert!(!rules.waveguide_theory().is_empty());
    assert!(!rules.microwave_devices().is_empty());
    assert!(!rules.microwave_antennas().is_empty());
    assert!(!rules.microwave_measurements().is_empty());
    assert!(!rules.microwave_applications().is_empty());
    assert!(!rules.microwave_safety().is_empty());
}

/// 测试光学基础规则
#[test]
fn test_optical_basics_rules() {
    let rules = OpticalBasicsRules::new();
    assert_eq!(rules.metadata().name, "光学基础规则");
    assert_eq!(rules.category(), RuleCategory::science("optical_basics"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.nature_of_light().is_empty());
    assert!(!rules.reflection().is_empty());
    assert!(!rules.refraction().is_empty());
    assert!(!rules.interference().is_empty());
    assert!(!rules.diffraction().is_empty());
    assert!(!rules.polarization().is_empty());
    assert!(!rules.optical_devices().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试天线理论规则
#[test]
fn test_antenna_theory_rules() {
    let rules = AntennaTheoryRules::new();
    assert_eq!(rules.metadata().name, "天线理论规则");
    assert_eq!(rules.category(), RuleCategory::science("antenna_theory"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.antenna_basics().is_empty());
    assert!(!rules.antenna_parameters().is_empty());
    assert!(!rules.basic_antenna_types().is_empty());
    assert!(!rules.antenna_arrays().is_empty());
    assert!(!rules.antenna_feeding().is_empty());
    assert!(!rules.antenna_radiation().is_empty());
    assert!(!rules.antenna_measurements().is_empty());
    assert!(!rules.applications().is_empty());
}
