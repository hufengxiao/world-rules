//! Phase 44 测试 - 新兴法律规则深度扩展

use world_rules::rules::core::Rule;
use world_rules::rules::law::{
    AILawDeepRules, BlockchainLawDeepRules, CybersecurityLawDeepRules, DataSecurityLawDeepRules,
    EnvironmentalLawDeepRules,
};

#[test]
fn test_environmental_law_deep_rules() {
    let rules = EnvironmentalLawDeepRules::new();
    assert_eq!(rules.metadata().name, "环境法深度规则");

    // 测试各个方法
    let sections = vec![
        rules.environmental_impact_assessment_detailed(),
        rules.discharge_permit_detailed(),
        rules.pollution_liability_detailed(),
        rules.environmental_public_interest_litigation(),
        rules.ecological_protection_detailed(),
    ];

    for section in &sections {
        assert!(!section.is_empty());
        for rule in section {
            assert!(!rule.is_empty());
        }
    }

    // 测试 explain 方法
    let explanation = rules.explain();
    assert!(explanation.contains("环境影响评价"));
    assert!(explanation.contains("排污许可"));
    assert!(explanation.contains("环境污染责任"));
    assert!(explanation.contains("环境公益诉讼"));
    assert!(explanation.contains("生态保护"));
}

#[test]
fn test_cybersecurity_law_deep_rules() {
    let rules = CybersecurityLawDeepRules::new();
    assert_eq!(rules.metadata().name, "网络安全法深度规则");

    // 测试各个方法
    let sections = vec![
        rules.level_protection_detailed(),
        rules.critical_infrastructure_detailed(),
        rules.personal_info_protection_detailed(),
        rules.security_incident_handling_detailed(),
        rules.security_review_detailed(),
    ];

    for section in &sections {
        assert!(!section.is_empty());
        for rule in section {
            assert!(!rule.is_empty());
        }
    }

    // 测试 explain 方法
    let explanation = rules.explain();
    assert!(explanation.contains("等级保护"));
    assert!(explanation.contains("关键信息基础设施"));
    assert!(explanation.contains("个人信息保护"));
    assert!(explanation.contains("安全事件处置"));
    assert!(explanation.contains("安全审查"));
}

#[test]
fn test_data_security_law_deep_rules() {
    let rules = DataSecurityLawDeepRules::new();
    assert_eq!(rules.metadata().name, "数据安全法深度规则");

    // 测试各个方法
    let sections = vec![
        rules.data_classification_detailed(),
        rules.data_processing_detailed(),
        rules.data_security_obligations_detailed(),
        rules.data_export_detailed(),
        rules.data_security_supervision_detailed(),
    ];

    for section in &sections {
        assert!(!section.is_empty());
        for rule in section {
            assert!(!rule.is_empty());
        }
    }

    // 测试 explain 方法
    let explanation = rules.explain();
    assert!(explanation.contains("数据分类分级"));
    assert!(explanation.contains("数据处理"));
    assert!(explanation.contains("安全保护义务"));
    assert!(explanation.contains("数据跨境"));
    assert!(explanation.contains("安全监管"));
}

#[test]
fn test_ai_law_deep_rules() {
    let rules = AILawDeepRules::new();
    assert_eq!(rules.metadata().name, "人工智能法深度规则");

    // 测试各个方法
    let sections = vec![
        rules.ai_system_classification_detailed(),
        rules.algorithm_transparency_detailed(),
        rules.ai_data_security_detailed(),
        rules.ai_ethics_review_detailed(),
        rules.ai_safety_liability_detailed(),
    ];

    for section in &sections {
        assert!(!section.is_empty());
        for rule in section {
            assert!(!rule.is_empty());
        }
    }

    // 测试 explain 方法
    let explanation = rules.explain();
    assert!(explanation.contains("AI系统分类"));
    assert!(explanation.contains("算法透明度"));
    assert!(explanation.contains("数据安全"));
    assert!(explanation.contains("伦理审查"));
    assert!(explanation.contains("安全责任"));
}

#[test]
fn test_blockchain_law_deep_rules() {
    let rules = BlockchainLawDeepRules::new();
    assert_eq!(rules.metadata().name, "区块链法深度规则");

    // 测试各个方法
    let sections = vec![
        rules.blockchain_supervision_detailed(),
        rules.smart_contract_legal_detailed(),
        rules.digital_asset_rights_detailed(),
        rules.blockchain_data_governance_detailed(),
        rules.blockchain_safety_liability_detailed(),
    ];

    for section in &sections {
        assert!(!section.is_empty());
        for rule in section {
            assert!(!rule.is_empty());
        }
    }

    // 测试 explain 方法
    let explanation = rules.explain();
    assert!(explanation.contains("应用监管"));
    assert!(explanation.contains("智能合约"));
    assert!(explanation.contains("数字资产确权"));
    assert!(explanation.contains("数据治理"));
    assert!(explanation.contains("安全责任"));
}

#[test]
fn test_all_deep_rules_validation() {
    // 测试所有规则的 validate 方法
    use world_rules::rules::core::ValidateContext;

    let ctx = ValidateContext::Generic("test".to_string());

    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(EnvironmentalLawDeepRules::new()),
        Box::new(CybersecurityLawDeepRules::new()),
        Box::new(DataSecurityLawDeepRules::new()),
        Box::new(AILawDeepRules::new()),
        Box::new(BlockchainLawDeepRules::new()),
    ];

    for rule in &rules {
        let result = rule.validate(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}

#[test]
fn test_rule_count() {
    // 验证每个规则至少有 50 条具体规则（5个方法 * 10条规则）
    let env_rules = EnvironmentalLawDeepRules::new();
    assert!(env_rules.environmental_impact_assessment_detailed().len() >= 10);
    assert!(env_rules.discharge_permit_detailed().len() >= 10);
    assert!(env_rules.pollution_liability_detailed().len() >= 10);
    assert!(env_rules.environmental_public_interest_litigation().len() >= 10);
    assert!(env_rules.ecological_protection_detailed().len() >= 10);

    let cyber_rules = CybersecurityLawDeepRules::new();
    assert!(cyber_rules.level_protection_detailed().len() >= 10);
    assert!(cyber_rules.critical_infrastructure_detailed().len() >= 10);
    assert!(cyber_rules.personal_info_protection_detailed().len() >= 10);
    assert!(cyber_rules.security_incident_handling_detailed().len() >= 10);
    assert!(cyber_rules.security_review_detailed().len() >= 10);

    let data_rules = DataSecurityLawDeepRules::new();
    assert!(data_rules.data_classification_detailed().len() >= 10);
    assert!(data_rules.data_processing_detailed().len() >= 10);
    assert!(data_rules.data_security_obligations_detailed().len() >= 10);
    assert!(data_rules.data_export_detailed().len() >= 10);
    assert!(data_rules.data_security_supervision_detailed().len() >= 10);

    let ai_rules = AILawDeepRules::new();
    assert!(ai_rules.ai_system_classification_detailed().len() >= 10);
    assert!(ai_rules.algorithm_transparency_detailed().len() >= 10);
    assert!(ai_rules.ai_data_security_detailed().len() >= 10);
    assert!(ai_rules.ai_ethics_review_detailed().len() >= 10);
    assert!(ai_rules.ai_safety_liability_detailed().len() >= 10);

    let blockchain_rules = BlockchainLawDeepRules::new();
    assert!(blockchain_rules.blockchain_supervision_detailed().len() >= 10);
    assert!(blockchain_rules.smart_contract_legal_detailed().len() >= 10);
    assert!(blockchain_rules.digital_asset_rights_detailed().len() >= 10);
    assert!(blockchain_rules.blockchain_data_governance_detailed().len() >= 10);
    assert!(
        blockchain_rules
            .blockchain_safety_liability_detailed()
            .len()
            >= 10
    );
}
