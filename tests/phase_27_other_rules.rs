//! Phase 27-03: 5种其他国际礼仪规则测试
//!
//! 测试新增的其他国际礼仪规则

use world_rules::rules::core::{Rule, RuleCategory, ValidateContext};
use world_rules::rules::social::{
    DiplomaticEtiquetteRules, InternationalConferenceRules, InternationalGiftRules,
    InternationalGreetingRules, InternationalTravelRules,
};

/// 测试外交礼仪
#[test]
fn test_diplomatic_etiquette_rules() {
    let rules = DiplomaticEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "外交礼仪");
    assert_eq!(rules.category(), RuleCategory::social("international"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.privileges_and_immunities().is_empty());
    assert!(!rules.credentials_presentation().is_empty());
    assert!(!rules.diplomatic_meetings().is_empty());
    assert!(!rules.state_banquets().is_empty());
    assert!(!rules.diplomatic_gifts().is_empty());
    assert!(!rules.flag_protocol().is_empty());
    assert!(!rules.diplomatic_correspondence().is_empty());
    assert!(!rules.diplomatic_courtesy().is_empty());
}

/// 测试国际会议礼仪
#[test]
fn test_international_conference_rules() {
    let rules = InternationalConferenceRules::new();
    assert_eq!(rules.metadata().name, "国际会议礼仪");
    assert_eq!(rules.category(), RuleCategory::social("international"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.preparation().is_empty());
    assert!(!rules.registration().is_empty());
    assert!(!rules.seating_protocol().is_empty());
    assert!(!rules.speaking().is_empty());
    assert!(!rules.interpretation().is_empty());
    assert!(!rules.voting().is_empty());
    assert!(!rules.networking().is_empty());
    assert!(!rules.dress_code().is_empty());
}

/// 测试国际旅行礼仪
#[test]
fn test_international_travel_rules() {
    let rules = InternationalTravelRules::new();
    assert_eq!(rules.metadata().name, "国际旅行礼仪");
    assert_eq!(rules.category(), RuleCategory::social("international"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.passport_visa().is_empty());
    assert!(!rules.border_control().is_empty());
    assert!(!rules.customs().is_empty());
    assert!(!rules.airport_etiquette().is_empty());
    assert!(!rules.hotel_etiquette().is_empty());
    assert!(!rules.cultural_taboos().is_empty());
    assert!(!rules.emergency().is_empty());
    assert!(!rules.public_places().is_empty());
}

/// 测试国际礼物礼仪
#[test]
fn test_international_gift_rules() {
    let rules = InternationalGiftRules::new();
    assert_eq!(rules.metadata().name, "国际礼物礼仪");
    assert_eq!(rules.category(), RuleCategory::social("international"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.gift_selection().is_empty());
    assert!(!rules.gift_wrapping().is_empty());
    assert!(!rules.giving_timing().is_empty());
    assert!(!rules.giving_manner().is_empty());
    assert!(!rules.receiving().is_empty());
    assert!(!rules.cultural_taboos().is_empty());
    assert!(!rules.business_gifts().is_empty());
    assert!(!rules.reciprocal_gifts().is_empty());
}

/// 测试国际问候礼仪
#[test]
fn test_international_greeting_rules() {
    let rules = InternationalGreetingRules::new();
    assert_eq!(rules.metadata().name, "国际问候礼仪");
    assert_eq!(rules.category(), RuleCategory::social("international"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.handshake().is_empty());
    assert!(!rules.bowing().is_empty());
    assert!(!rules.kissing().is_empty());
    assert!(!rules.business_card_exchange().is_empty());
    assert!(!rules.addressing().is_empty());
    assert!(!rules.country_customs().is_empty());
    assert!(!rules.special_occasions().is_empty());
    assert!(!rules.taboos().is_empty());
}

/// 测试所有其他国际礼仪规则的验证功能
#[test]
fn test_all_international_rules_validation() {
    // 测试所有其他国际礼仪规则的验证功能
    let rules_list: Vec<&dyn Rule> = vec![
        &DiplomaticEtiquetteRules::new(),
        &InternationalConferenceRules::new(),
        &InternationalTravelRules::new(),
        &InternationalGiftRules::new(),
        &InternationalGreetingRules::new(),
    ];

    for rules in rules_list {
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(rules.category(), RuleCategory::social("international"));
    }
}

/// 测试其他国际礼仪规则解释内容包含关键信息
#[test]
fn test_international_rules_explain_content() {
    // 测试解释内容包含关键信息
    let diplomatic = DiplomaticEtiquetteRules::new();
    assert!(diplomatic.explain().contains("外交礼仪"));
    assert!(diplomatic.explain().contains("外交特权与豁免"));

    let conference = InternationalConferenceRules::new();
    assert!(conference.explain().contains("国际会议礼仪"));
    assert!(conference.explain().contains("会议筹备礼仪"));

    let travel = InternationalTravelRules::new();
    assert!(travel.explain().contains("国际旅行礼仪"));
    assert!(travel.explain().contains("护照签证礼仪"));

    let gift = InternationalGiftRules::new();
    assert!(gift.explain().contains("国际礼物礼仪"));
    assert!(gift.explain().contains("礼物选择礼仪"));

    let greeting = InternationalGreetingRules::new();
    assert!(greeting.explain().contains("国际问候礼仪"));
    assert!(greeting.explain().contains("握手礼仪"));
}

/// 测试其他国际礼仪规则要点统计
#[test]
fn test_international_rules_points_count() {
    // 每个规则有 8 个方法返回礼仪要点
    let rules = DiplomaticEtiquetteRules::new();
    let total_points = rules.privileges_and_immunities().len()
        + rules.credentials_presentation().len()
        + rules.diplomatic_meetings().len()
        + rules.state_banquets().len()
        + rules.diplomatic_gifts().len()
        + rules.flag_protocol().len()
        + rules.diplomatic_correspondence().len()
        + rules.diplomatic_courtesy().len();

    // 每个方法应返回 8 个要点，共 64 个要点
    assert_eq!(total_points, 64, "外交礼仪应有 64 个要点");
}
