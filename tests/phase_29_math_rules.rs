//! Phase 29 数学规则测试 - Phase 29-03 其他数学规则
//!
//! 测试新增的数学规则（Phase 29-03）

use world_rules::rules::core::{Rule, RuleCategory, ValidateContext};
use world_rules::rules::science::{
    CalculusRules, DiscreteMathRules, GraphTheoryRules, NumberTheoryRules, NumericalAnalysisRules,
    OptimizationRules, ProbabilityTheoryRules, RealAnalysisRules, StatisticsRules, TopologyRules,
};

/// 测试微积分规则
#[test]
fn test_calculus_rules() {
    let rules = CalculusRules::new();
    assert_eq!(rules.metadata().name, "微积分规则");
    assert_eq!(rules.category(), RuleCategory::science("calculus"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.differential_basics().is_empty());
    assert!(!rules.differentiation_rules().is_empty());
    assert!(!rules.integral_basics().is_empty());
    assert!(!rules.integration_techniques().is_empty());
    assert!(!rules.applications().is_empty());
    assert!(!rules.multivariable_calculus().is_empty());
    assert!(!rules.differential_equations().is_empty());
    assert!(!rules.series_theory().is_empty());
    assert!(!rules.numerical_methods().is_empty());
    assert!(!rules.history_and_applications().is_empty());
}

/// 测试概率论规则
#[test]
fn test_probability_theory_rules() {
    let rules = ProbabilityTheoryRules::new();
    assert_eq!(rules.metadata().name, "概率论规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("probability_theory")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.probability_basics().is_empty());
    assert!(!rules.probability_formulas().is_empty());
    assert!(!rules.random_variables().is_empty());
    assert!(!rules.important_distributions().is_empty());
    assert!(!rules.laws_of_large_numbers().is_empty());
    assert!(!rules.central_limit_theorem().is_empty());
    assert!(!rules.multivariate_probability().is_empty());
    assert!(!rules.stochastic_processes().is_empty());
    assert!(!rules.statistical_inference_basics().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试拓扑学规则
#[test]
fn test_topology_rules() {
    let rules = TopologyRules::new();
    assert_eq!(rules.metadata().name, "拓扑学规则");
    assert_eq!(rules.category(), RuleCategory::science("topology"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.topological_space_basics().is_empty());
    assert!(!rules.continuity_and_homeomorphism().is_empty());
    assert!(!rules.bases_and_subbases().is_empty());
    assert!(!rules.compactness().is_empty());
    assert!(!rules.connectedness().is_empty());
    assert!(!rules.separation_axioms().is_empty());
    assert!(!rules.important_theorems().is_empty());
    assert!(!rules.homotopy_and_homology().is_empty());
    assert!(!rules.bundles_and_manifolds().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试统计学规则
#[test]
fn test_statistics_rules() {
    let rules = StatisticsRules::new();
    assert_eq!(rules.metadata().name, "统计学规则");
    assert_eq!(rules.category(), RuleCategory::science("statistics"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.descriptive_statistics().is_empty());
    assert!(!rules.sampling_methods().is_empty());
    assert!(!rules.distribution_types().is_empty());
    assert!(!rules.parameter_estimation().is_empty());
    assert!(!rules.hypothesis_testing().is_empty());
    assert!(!rules.test_types().is_empty());
    assert!(!rules.correlation_and_regression().is_empty());
    assert!(!rules.multivariate_analysis().is_empty());
    assert!(!rules.time_series().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试数论规则
#[test]
fn test_number_theory_rules() {
    let rules = NumberTheoryRules::new();
    assert_eq!(rules.metadata().name, "数论规则");
    assert_eq!(rules.category(), RuleCategory::science("number_theory"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.integer_basics().is_empty());
    assert!(!rules.prime_number_theory().is_empty());
    assert!(!rules.unique_factorization().is_empty());
    assert!(!rules.congruence_theory().is_empty());
    assert!(!rules.linear_congruence().is_empty());
    assert!(!rules.quadratic_residues().is_empty());
    assert!(!rules.special_sequences().is_empty());
    assert!(!rules.algebraic_number_theory().is_empty());
    assert!(!rules.analytic_number_theory().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试图论规则
#[test]
fn test_graph_theory_rules() {
    let rules = GraphTheoryRules::new();
    assert_eq!(rules.metadata().name, "图论规则");
    assert_eq!(rules.category(), RuleCategory::science("graph_theory"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.graph_basics().is_empty());
    assert!(!rules.special_graphs().is_empty());
    assert!(!rules.matrix_representation().is_empty());
    assert!(!rules.connectivity().is_empty());
    assert!(!rules.matching_and_covering().is_empty());
    assert!(!rules.coloring().is_empty());
    assert!(!rules.famous_problems().is_empty());
    assert!(!rules.tree_properties().is_empty());
    assert!(!rules.graph_algorithms().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试优化理论规则
#[test]
fn test_optimization_rules() {
    let rules = OptimizationRules::new();
    assert_eq!(rules.metadata().name, "优化理论规则");
    assert_eq!(rules.category(), RuleCategory::science("optimization"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.optimization_basics().is_empty());
    assert!(!rules.unconstrained_optimization().is_empty());
    assert!(!rules.constrained_optimization().is_empty());
    assert!(!rules.linear_programming().is_empty());
    assert!(!rules.integer_programming().is_empty());
    assert!(!rules.nonlinear_programming().is_empty());
    assert!(!rules.dynamic_programming().is_empty());
    assert!(!rules.combinatorial_optimization().is_empty());
    assert!(!rules.stochastic_optimization().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试离散数学规则
#[test]
fn test_discrete_math_rules() {
    let rules = DiscreteMathRules::new();
    assert_eq!(rules.metadata().name, "离散数学规则");
    assert_eq!(rules.category(), RuleCategory::science("discrete_math"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.mathematical_logic().is_empty());
    assert!(!rules.set_theory().is_empty());
    assert!(!rules.relations_and_functions().is_empty());
    assert!(!rules.combinatorics().is_empty());
    assert!(!rules.discrete_probability().is_empty());
    assert!(!rules.recursion_and_induction().is_empty());
    assert!(!rules.discrete_structures().is_empty());
    assert!(!rules.algorithm_basics().is_empty());
    assert!(!rules.computation_theory().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试数值分析规则
#[test]
fn test_numerical_analysis_rules() {
    let rules = NumericalAnalysisRules::new();
    assert_eq!(rules.metadata().name, "数值分析规则");
    assert_eq!(
        rules.category(),
        RuleCategory::science("numerical_analysis")
    );
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.error_analysis().is_empty());
    assert!(!rules.numerical_linear_algebra().is_empty());
    assert!(!rules.numerical_integration().is_empty());
    assert!(!rules.numerical_differentiation().is_empty());
    assert!(!rules.root_finding().is_empty());
    assert!(!rules.solving_odes().is_empty());
    assert!(!rules.interpolation_and_fitting().is_empty());
    assert!(!rules.optimization_algorithms().is_empty());
    assert!(!rules.special_functions().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试实分析规则
#[test]
fn test_real_analysis_rules() {
    let rules = RealAnalysisRules::new();
    assert_eq!(rules.metadata().name, "实分析规则");
    assert_eq!(rules.category(), RuleCategory::science("real_analysis"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.real_number_system().is_empty());
    assert!(!rules.sequences_and_limits().is_empty());
    assert!(!rules.continuity().is_empty());
    assert!(!rules.differentiability().is_empty());
    assert!(!rules.riemann_integration().is_empty());
    assert!(!rules.lebesgue_integration().is_empty());
    assert!(!rules.function_spaces().is_empty());
    assert!(!rules.fourier_analysis().is_empty());
    assert!(!rules.measure_theory().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试所有规则的方法数量
#[test]
fn test_all_math_rules_method_count() {
    // 微积分规则
    let calculus = CalculusRules::new();
    assert!(calculus.differential_basics().len() >= 8);
    assert!(calculus.history_and_applications().len() >= 8);

    // 概率论规则
    let probability = ProbabilityTheoryRules::new();
    assert!(probability.probability_basics().len() >= 8);
    assert!(probability.applications().len() >= 8);

    // 拓扑学规则
    let topology = TopologyRules::new();
    assert!(topology.topological_space_basics().len() >= 8);
    assert!(topology.applications().len() >= 8);

    // 统计学规则
    let statistics = StatisticsRules::new();
    assert!(statistics.descriptive_statistics().len() >= 8);
    assert!(statistics.applications().len() >= 8);

    // 数论规则
    let number_theory = NumberTheoryRules::new();
    assert!(number_theory.integer_basics().len() >= 8);
    assert!(number_theory.applications().len() >= 8);

    // 图论规则
    let graph_theory = GraphTheoryRules::new();
    assert!(graph_theory.graph_basics().len() >= 8);
    assert!(graph_theory.applications().len() >= 8);

    // 优化理论规则
    let optimization = OptimizationRules::new();
    assert!(optimization.optimization_basics().len() >= 8);
    assert!(optimization.applications().len() >= 8);

    // 离散数学规则
    let discrete_math = DiscreteMathRules::new();
    assert!(discrete_math.mathematical_logic().len() >= 8);
    assert!(discrete_math.applications().len() >= 8);

    // 数值分析规则
    let numerical_analysis = NumericalAnalysisRules::new();
    assert!(numerical_analysis.error_analysis().len() >= 8);
    assert!(numerical_analysis.applications().len() >= 8);

    // 实分析规则
    let real_analysis = RealAnalysisRules::new();
    assert!(real_analysis.real_number_system().len() >= 8);
    assert!(real_analysis.applications().len() >= 8);
}
