//! Phase 32-01: 民法规则扩充综合测试
//!
//! 本文件测试 Phase 32-01 添加的所有民法规则模块
//!
//! 包含 10 种民法深度规则：
//! - CivilCodeGeneralDeepRules (民法典总则编深度规则)
//! - CivilCodeContractDeepRules (民法典合同编深度规则)
//! - CivilCodePropertyDeepRules (民法典物权编深度规则)
//! - CivilCodePersonalityDeepRules (民法典人格权编深度规则)
//! - CivilCodeMarriageDeepRules (民法典婚姻家庭编深度规则)
//! - CivilCodeTortDeepRules (民法典侵权责任编深度规则)
//! - CivilProcedureDeepRules (民事诉讼法深度规则)
//! - ConsumerProtectionDeepRules (消费者权益保护法深度规则)
//! - LaborLawRules (劳动法规则)

use world_rules::rules::core::{Rule, RuleCategory};
use world_rules::rules::law::{
    CivilCodeContractDeepRules, CivilCodeGeneralDeepRules, CivilCodeMarriageDeepRules,
    CivilCodePersonalityDeepRules, CivilCodePropertyDeepRules, CivilCodeTortDeepRules,
    CivilProcedureDeepRules, ConsumerProtectionDeepRules, LaborLawRules,
};

// ============================================================================
// Phase 32-01: 民法规则测试
// ============================================================================

/// 测试民法典总则编深度规则
#[test]
fn test_civil_code_general_deep_rules() {
    let rules = CivilCodeGeneralDeepRules::new();
    assert_eq!(rules.metadata().name, "民法典总则编深度规则");
    assert_eq!(
        rules.category(),
        RuleCategory::law("civil_code_general_deep")
    );
    assert_eq!(rules.basic_principles_detailed().len(), 8);
    assert_eq!(rules.natural_person_detailed().len(), 10);
    assert_eq!(rules.legal_person_detailed().len(), 10);
    assert_eq!(rules.civil_act_detailed().len(), 10);
    assert_eq!(rules.agency_detailed().len(), 10);
    assert_eq!(rules.civil_liability_detailed().len(), 10);
    assert_eq!(rules.limitation_period_detailed().len(), 10);
    assert_eq!(rules.period_calculation_detailed().len(), 8);
    assert!(!rules.explain().is_empty());
}

/// 测试民法典合同编深度规则
#[test]
fn test_civil_code_contract_deep_rules() {
    let rules = CivilCodeContractDeepRules::new();
    assert_eq!(rules.metadata().name, "民法典合同编深度规则");
    assert_eq!(
        rules.category(),
        RuleCategory::law("civil_code_contract_deep")
    );
    assert_eq!(rules.contract_form_detailed().len(), 10);
    assert_eq!(rules.contract_effect_detailed().len(), 10);
    assert_eq!(rules.contract_performance_detailed().len(), 10);
    assert_eq!(rules.contract_preservation_detailed().len(), 10);
    assert_eq!(rules.contract_modification_detailed().len(), 10);
    assert_eq!(rules.contract_termination_detailed().len(), 10);
    assert_eq!(rules.breach_liability_detailed().len(), 10);
    assert_eq!(rules.typical_contract_detailed().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试民法典物权编深度规则
#[test]
fn test_civil_code_property_deep_rules() {
    let rules = CivilCodePropertyDeepRules::new();
    assert_eq!(rules.metadata().name, "民法典物权编深度规则");
    assert_eq!(
        rules.category(),
        RuleCategory::law("civil_code_property_deep")
    );
    assert_eq!(rules.property_principles_detailed().len(), 8);
    assert_eq!(rules.ownership_detailed().len(), 10);
    assert_eq!(rules.usufruct_detailed().len(), 10);
    assert_eq!(rules.security_rights_detailed().len(), 10);
    assert_eq!(rules.possession_detailed().len(), 10);
    assert_eq!(rules.property_protection_detailed().len(), 10);
    assert_eq!(rules.registration_detailed().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试民法典人格权编深度规则
#[test]
fn test_civil_code_personality_deep_rules() {
    let rules = CivilCodePersonalityDeepRules::new();
    assert_eq!(rules.metadata().name, "民法典人格权编深度规则");
    assert_eq!(
        rules.category(),
        RuleCategory::law("civil_code_personality_deep")
    );
    assert_eq!(rules.personality_general_detailed().len(), 8);
    assert_eq!(rules.life_body_health_detailed().len(), 10);
    assert_eq!(rules.name_rights_detailed().len(), 10);
    assert_eq!(rules.reputation_honor_detailed().len(), 10);
    assert_eq!(rules.privacy_personal_info_detailed().len(), 10);
    assert_eq!(rules.portrait_rights_detailed().len(), 10);
    assert_eq!(rules.personality_protection_detailed().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试民法典婚姻家庭编深度规则
#[test]
fn test_civil_code_marriage_deep_rules() {
    let rules = CivilCodeMarriageDeepRules::new();
    assert_eq!(rules.metadata().name, "民法典婚姻家庭编深度规则");
    assert_eq!(
        rules.category(),
        RuleCategory::law("civil_code_marriage_deep")
    );
    assert_eq!(rules.marriage_general_detailed().len(), 10);
    assert_eq!(rules.husband_wife_rights_detailed().len(), 10);
    assert_eq!(rules.divorce_rules_detailed().len(), 10);
    assert_eq!(rules.child_parent_rules_detailed().len(), 10);
    assert_eq!(rules.adoption_rules_detailed().len(), 10);
    assert_eq!(rules.inheritance_general_detailed().len(), 10);
    assert_eq!(rules.intestate_succession_detailed().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试民法典侵权责任编深度规则
#[test]
fn test_civil_code_tort_deep_rules() {
    let rules = CivilCodeTortDeepRules::new();
    assert_eq!(rules.metadata().name, "民法典侵权责任编深度规则");
    assert_eq!(rules.category(), RuleCategory::law("civil_code_tort_deep"));
    assert_eq!(rules.tort_general_detailed().len(), 10);
    assert_eq!(rules.special_tort_rules_detailed().len(), 10);
    assert_eq!(rules.liability_methods_detailed().len(), 9);
    assert_eq!(rules.damage_calculation_detailed().len(), 10);
    assert_eq!(rules.product_liability_detailed().len(), 10);
    assert_eq!(rules.medical_liability_detailed().len(), 10);
    assert_eq!(rules.environmental_tort_detailed().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试民事诉讼法深度规则
#[test]
fn test_civil_procedure_deep_rules() {
    let rules = CivilProcedureDeepRules::new();
    assert_eq!(rules.metadata().name, "民事诉讼法深度规则");
    assert_eq!(rules.category(), RuleCategory::law("civil_procedure_deep"));
    assert_eq!(rules.procedure_general_detailed().len(), 10);
    assert_eq!(rules.parties_rules_detailed().len(), 10);
    assert_eq!(rules.evidence_rules_detailed().len(), 10);
    assert_eq!(rules.trial_procedure_detailed().len(), 10);
    assert_eq!(rules.appeal_rules_detailed().len(), 10);
    assert_eq!(rules.execution_rules_detailed().len(), 10);
    assert_eq!(rules.special_procedure_detailed().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试消费者权益保护法深度规则
#[test]
fn test_consumer_protection_deep_rules() {
    let rules = ConsumerProtectionDeepRules::new();
    assert_eq!(rules.metadata().name, "消费者权益保护法深度规则");
    assert_eq!(
        rules.category(),
        RuleCategory::law("consumer_protection_deep")
    );
    assert_eq!(rules.consumer_rights_detailed().len(), 10);
    assert_eq!(rules.operator_obligations_detailed().len(), 10);
    assert_eq!(rules.product_quality_detailed().len(), 10);
    assert_eq!(rules.unfair_practices_detailed().len(), 10);
    assert_eq!(rules.e_commerce_rules_detailed().len(), 10);
    assert_eq!(rules.dispute_resolution_detailed().len(), 10);
    assert_eq!(rules.special_protection_detailed().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试劳动法深度规则
#[test]
fn test_labor_law_deep_rules() {
    let rules = LaborLawRules::new();
    assert_eq!(rules.metadata().name, "劳动法深度规则");
    assert_eq!(rules.category(), RuleCategory::law("labor_law_deep"));
    assert_eq!(rules.labor_relations_detailed().len(), 10);
    assert_eq!(rules.worker_rights_detailed().len(), 10);
    assert_eq!(rules.employer_obligations_detailed().len(), 10);
    assert_eq!(rules.contract_termination_detailed().len(), 10);
    assert_eq!(rules.working_time_rules_detailed().len(), 10);
    assert_eq!(rules.wage_rules_detailed().len(), 10);
    assert_eq!(rules.social_insurance_rules_detailed().len(), 10);
    assert!(!rules.explain().is_empty());
}

/// 测试所有规则的 Rule trait 实现
#[test]
fn test_all_rules_trait_impl() {
    // 总则编深度规则
    let general = CivilCodeGeneralDeepRules::new();
    assert!(general
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());

    // 合同编深度规则
    let contract = CivilCodeContractDeepRules::new();
    assert!(contract
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());

    // 物权编深度规则
    let property = CivilCodePropertyDeepRules::new();
    assert!(property
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());

    // 人格权编深度规则
    let personality = CivilCodePersonalityDeepRules::new();
    assert!(personality
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());

    // 婚姻家庭编深度规则
    let marriage = CivilCodeMarriageDeepRules::new();
    assert!(marriage
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());

    // 侵权责任编深度规则
    let tort = CivilCodeTortDeepRules::new();
    assert!(tort
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());

    // 民事诉讼法深度规则
    let procedure = CivilProcedureDeepRules::new();
    assert!(procedure
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());

    // 消费者权益保护法深度规则
    let consumer = ConsumerProtectionDeepRules::new();
    assert!(consumer
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());

    // 劳动法深度规则
    let labor = LaborLawDeepRules::new();
    assert!(labor
        .validate(&world_rules::rules::core::ValidateContext::Generic(
            "test".into()
        ))
        .is_ok());
}
