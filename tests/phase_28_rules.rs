//! Phase 28 力学规则测试
//!
//! 测试新增的 10 种力学规则

use world_rules::rules::core::{Rule, RuleCategory, ValidateContext};
use world_rules::rules::science::{
    AnalyticalMechanicsRules, CelestialMechanicsRules, ComputationalMechanicsRules,
    DynamicsRules, FluidDynamicsRules, KinematicsRules, MaterialsMechanicsRules,
    RigidBodyDynamicsRules, StaticsRules, VibrationWaveRules,
};

/// 测试静力学规则
#[test]
fn test_statics_rules() {
    let rules = StaticsRules::new();
    assert_eq!(rules.metadata().name, "静力学规则");
    assert_eq!(rules.category(), RuleCategory::science("statics"));
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert_eq!(rules.category(), RuleCategory::science("materials_mechanics"));
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert_eq!(rules.category(), RuleCategory::science("rigid_body_dynamics"));
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert_eq!(rules.category(), RuleCategory::science("analytical_mechanics"));
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert_eq!(rules.category(), RuleCategory::science("celestial_mechanics"));
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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
    assert_eq!(rules.category(), RuleCategory::science("computational_mechanics"));
    assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
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