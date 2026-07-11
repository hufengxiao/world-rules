//! Phase 27 商务礼仪规则测试
//!
//! 测试新增的 10 种商务礼仪规则

use world_rules::rules::core::{Rule, RuleCategory, ValidateContext};
use world_rules::rules::social::{
    BoardroomEtiquetteRules, ContractSigningEtiquetteRules, CorporateEventEtiquetteRules,
    InvestorRelationsEtiquetteRules, MediaRelationsEtiquetteRules, PartnershipEtiquetteRules,
    ProjectManagementEtiquetteRules, SalesEtiquetteRules, SupplierRelationsEtiquetteRules,
    TradeShowEtiquetteRules,
};

/// 测试合同签署礼仪
#[test]
fn test_contract_signing_etiquette() {
    let rules = ContractSigningEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "合同签署礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.preparation().is_empty());
    assert!(!rules.ceremony().is_empty());
    assert!(!rules.signing_order().is_empty());
    assert!(!rules.after_signing().is_empty());
    assert!(!rules.taboos().is_empty());
    assert!(!rules.electronic_signing().is_empty());
    assert!(!rules.cultural_differences().is_empty());
    assert!(!rules.international().is_empty());
}

/// 测试董事会礼仪
#[test]
fn test_boardroom_etiquette() {
    let rules = BoardroomEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "董事会礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.preparation().is_empty());
    assert!(!rules.entering().is_empty());
    assert!(!rules.speaking().is_empty());
    assert!(!rules.voting().is_empty());
    assert!(!rules.confidentiality().is_empty());
    assert!(!rules.remote_attendance().is_empty());
    assert!(!rules.closing().is_empty());
    assert!(!rules.director_behavior().is_empty());
}

/// 测试展会礼仪
#[test]
fn test_trade_show_etiquette() {
    let rules = TradeShowEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "展会礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.before_show().is_empty());
    assert!(!rules.booth_setup().is_empty());
    assert!(!rules.staff_behavior().is_empty());
    assert!(!rules.client_reception().is_empty());
    assert!(!rules.networking().is_empty());
    assert!(!rules.after_show().is_empty());
    assert!(!rules.international().is_empty());
    assert!(!rules.taboos().is_empty());
}

/// 测试企业活动礼仪
#[test]
fn test_corporate_event_etiquette() {
    let rules = CorporateEventEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "企业活动礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.planning().is_empty());
    assert!(!rules.invitation().is_empty());
    assert!(!rules.on_site().is_empty());
    assert!(!rules.annual_meeting().is_empty());
    assert!(!rules.product_launch().is_empty());
    assert!(!rules.celebration().is_empty());
    assert!(!rules.media_handling().is_empty());
    assert!(!rules.closing().is_empty());
}

/// 测试投资者关系礼仪
#[test]
fn test_investor_relations_etiquette() {
    let rules = InvestorRelationsEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "投资者关系礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.disclosure().is_empty());
    assert!(!rules.communication().is_empty());
    assert!(!rules.shareholder_meeting().is_empty());
    assert!(!rules.shareholder_service().is_empty());
    assert!(!rules.roadshow().is_empty());
    assert!(!rules.analyst_relations().is_empty());
    assert!(!rules.institutional_investors().is_empty());
    assert!(!rules.crisis_communication().is_empty());
}

/// 测试媒体关系礼仪
#[test]
fn test_media_relations_etiquette() {
    let rules = MediaRelationsEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "媒体关系礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.reception().is_empty());
    assert!(!rules.interview().is_empty());
    assert!(!rules.news_release().is_empty());
    assert!(!rules.pr_activities().is_empty());
    assert!(!rules.relationship_maintenance().is_empty());
    assert!(!rules.crisis_handling().is_empty());
    assert!(!rules.social_media().is_empty());
    assert!(!rules.taboos().is_empty());
}

/// 测试供应商关系礼仪
#[test]
fn test_supplier_relations_etiquette() {
    let rules = SupplierRelationsEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "供应商关系礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.selection().is_empty());
    assert!(!rules.communication().is_empty());
    assert!(!rules.contract_management().is_empty());
    assert!(!rules.evaluation().is_empty());
    assert!(!rules.support().is_empty());
    assert!(!rules.dispute_handling().is_empty());
    assert!(!rules.termination().is_empty());
    assert!(!rules.taboos().is_empty());
}

/// 测试合作伙伴礼仪
#[test]
fn test_partnership_etiquette() {
    let rules = PartnershipEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "合作伙伴礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.negotiation().is_empty());
    assert!(!rules.agreement().is_empty());
    assert!(!rules.launch().is_empty());
    assert!(!rules.execution().is_empty());
    assert!(!rules.relationship_maintenance().is_empty());
    assert!(!rules.termination().is_empty());
    assert!(!rules.strategic_alliance().is_empty());
    assert!(!rules.taboos().is_empty());
}

/// 测试销售礼仪
#[test]
fn test_sales_etiquette() {
    let rules = SalesEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "销售礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.client_visit().is_empty());
    assert!(!rules.product_demo().is_empty());
    assert!(!rules.communication().is_empty());
    assert!(!rules.price_negotiation().is_empty());
    assert!(!rules.closing().is_empty());
    assert!(!rules.customer_service().is_empty());
    assert!(!rules.handling_rejection().is_empty());
    assert!(!rules.taboos().is_empty());
}

/// 测试项目管理礼仪
#[test]
fn test_project_management_etiquette() {
    let rules = ProjectManagementEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "项目管理礼仪");
    assert_eq!(rules.category(), RuleCategory::social("business"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.project_launch().is_empty());
    assert!(!rules.planning().is_empty());
    assert!(!rules.team_collaboration().is_empty());
    assert!(!rules.meeting().is_empty());
    assert!(!rules.progress_reporting().is_empty());
    assert!(!rules.problem_solving().is_empty());
    assert!(!rules.delivery().is_empty());
    assert!(!rules.project_closure().is_empty());
}

/// 测试所有商务礼仪规则的解释内容完整性
#[test]
fn test_all_business_etiquette_explanations() {
    let explanations = vec![
        ContractSigningEtiquetteRules::new().explain(),
        BoardroomEtiquetteRules::new().explain(),
        TradeShowEtiquetteRules::new().explain(),
        CorporateEventEtiquetteRules::new().explain(),
        InvestorRelationsEtiquetteRules::new().explain(),
        MediaRelationsEtiquetteRules::new().explain(),
        SupplierRelationsEtiquetteRules::new().explain(),
        PartnershipEtiquetteRules::new().explain(),
        SalesEtiquetteRules::new().explain(),
        ProjectManagementEtiquetteRules::new().explain(),
    ];

    for explanation in explanations {
        // 验证解释内容包含关键章节标识
        assert!(
            explanation.contains("礼仪"),
            "规则的解释缺少礼仪章节"
        );
    }
}

/// 测试商务礼仪规则要点统计
#[test]
fn test_etiquette_points_count() {
    // 每个规则有 8 个方法返回礼仪要点
    let rules = ContractSigningEtiquetteRules::new();
    let total_points = rules.preparation().len()
        + rules.ceremony().len()
        + rules.signing_order().len()
        + rules.after_signing().len()
        + rules.taboos().len()
        + rules.electronic_signing().len()
        + rules.cultural_differences().len()
        + rules.international().len();

    // 每个方法应返回 8 个要点，共 64 个要点
    assert_eq!(total_points, 64, "合同签署礼仪应有 64 个要点");
}