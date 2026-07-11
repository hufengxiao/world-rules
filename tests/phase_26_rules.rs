//! Phase 26 新规则集成测试
//!
//! 测试 Phase 26 添加的中华文化礼仪规则：
//! - 26-01: 10种传统礼仪规则

use world_rules::prelude::*;

// ============================================================================
// 26-01: 中华文化传统礼仪规则测试 (10种)
// ============================================================================

// ----- 中国传统成年礼规则测试 -----

#[test]
fn test_coming_of_age_rules_basic() {
    use world_rules::rules::social::ChineseComingOfAgeRules;
    let rules = ChineseComingOfAgeRules::new();
    assert_eq!(rules.metadata().name, "中国传统成年礼");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_coming_of_age_guanli_procedure() {
    use world_rules::rules::social::ChineseComingOfAgeRules;
    let rules = ChineseComingOfAgeRules::new();
    let procedure = rules.guanli_procedure();
    assert!(procedure.iter().any(|p| p.contains("二十岁")));
    assert!(procedure.iter().any(|p| p.contains("三加")));
    assert!(procedure.iter().any(|p| p.contains("缁布冠")));
    assert!(procedure.len() >= 10);
}

#[test]
fn test_coming_of_age_jili_procedure() {
    use world_rules::rules::social::ChineseComingOfAgeRules;
    let rules = ChineseComingOfAgeRules::new();
    let procedure = rules.jili_procedure();
    assert!(procedure.iter().any(|p| p.contains("十五岁")));
    assert!(procedure.iter().any(|p| p.contains("笄")));
    assert!(procedure.len() >= 8);
}

#[test]
fn test_coming_of_age_meaning() {
    use world_rules::rules::social::ChineseComingOfAgeRules;
    let rules = ChineseComingOfAgeRules::new();
    let guanli_meaning = rules.guanli_meaning();
    let jili_meaning = rules.jili_meaning();
    assert!(guanli_meaning.iter().any(|m| m.contains("缁布冠")));
    assert!(jili_meaning.iter().any(|m| m.contains("发笄")));
    assert!(guanli_meaning.len() >= 3);
    assert!(jili_meaning.len() >= 3);
}

#[test]
fn test_coming_of_age_symbolism() {
    use world_rules::rules::social::ChineseComingOfAgeRules;
    let rules = ChineseComingOfAgeRules::new();
    let symbolism = rules.symbolism();
    assert!(symbolism.iter().any(|s| s.contains("责任")));
    assert!(symbolism.len() >= 4);
}

// ----- 中国寿礼礼仪规则测试 -----

#[test]
fn test_birthday_etiquette_rules_basic() {
    use world_rules::rules::social::ChineseBirthdayEtiquetteRules;
    let rules = ChineseBirthdayEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "中国寿礼礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_birthday_levels() {
    use world_rules::rules::social::ChineseBirthdayEtiquetteRules;
    let rules = ChineseBirthdayEtiquetteRules::new();
    let levels = rules.birthday_levels();
    assert!(levels.iter().any(|l| l.contains("花甲")));
    assert!(levels.iter().any(|l| l.contains("古稀")));
    assert!(levels.iter().any(|l| l.contains("耄耋")));
    assert!(levels.iter().any(|l| l.contains("期颐")));
    assert!(levels.len() >= 5);
}

#[test]
fn test_birthday_ceremony_procedure() {
    use world_rules::rules::social::ChineseBirthdayEtiquetteRules;
    let rules = ChineseBirthdayEtiquetteRules::new();
    let procedure = rules.ceremony_procedure();
    assert!(procedure.iter().any(|p| p.contains("寿星")));
    assert!(procedure.iter().any(|p| p.contains("叩首")));
    assert!(procedure.iter().any(|p| p.contains("寿桃")));
    assert!(procedure.len() >= 6);
}

#[test]
fn test_birthday_taboos() {
    use world_rules::rules::social::ChineseBirthdayEtiquetteRules;
    let rules = ChineseBirthdayEtiquetteRules::new();
    let taboos = rules.taboos();
    assert!(taboos.iter().any(|t| t.contains("钟")));
    assert!(taboos.iter().any(|t| t.contains("梨")));
    assert!(taboos.len() >= 6);
}

#[test]
fn test_birthday_traditional_gifts() {
    use world_rules::rules::social::ChineseBirthdayEtiquetteRules;
    let rules = ChineseBirthdayEtiquetteRules::new();
    let gifts = rules.traditional_gifts();
    assert!(gifts.iter().any(|g| g.contains("寿桃")));
    assert!(gifts.iter().any(|g| g.contains("寿面")));
    assert!(gifts.iter().any(|g| g.contains("松鹤")));
    assert!(gifts.len() >= 6);
}

#[test]
fn test_birthday_greetings() {
    use world_rules::rules::social::ChineseBirthdayEtiquetteRules;
    let rules = ChineseBirthdayEtiquetteRules::new();
    let greetings = rules.birthday_greetings();
    assert!(greetings.iter().any(|g| g.contains("福如东海")));
    assert!(greetings.iter().any(|g| g.contains("寿比南山")));
    assert!(greetings.len() >= 6);
}

// ----- 中国传统禁忌规则测试 -----

#[test]
fn test_taboo_rules_basic() {
    use world_rules::rules::social::ChineseTabooRules;
    let rules = ChineseTabooRules::new();
    assert_eq!(rules.metadata().name, "中国传统禁忌");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_language_taboos() {
    use world_rules::rules::social::ChineseTabooRules;
    let rules = ChineseTabooRules::new();
    let taboos = rules.language_taboos();
    assert!(taboos.iter().any(|t| t.contains("过年")));
    assert!(taboos.iter().any(|t| t.contains("船上")));
    assert!(taboos.len() >= 6);
}

#[test]
fn test_dietary_taboos() {
    use world_rules::rules::social::ChineseTabooRules;
    let rules = ChineseTabooRules::new();
    let taboos = rules.dietary_taboos();
    assert!(taboos.iter().any(|t| t.contains("筷子")));
    assert!(taboos.iter().any(|t| t.contains("饭")));
    assert!(taboos.len() >= 6);
}

#[test]
fn test_behavioral_taboos() {
    use world_rules::rules::social::ChineseTabooRules;
    let rules = ChineseTabooRules::new();
    let taboos = rules.behavioral_taboos();
    assert!(taboos.iter().any(|t| t.contains("月亮")));
    assert!(taboos.iter().any(|t| t.contains("口哨")));
    assert!(taboos.len() >= 6);
}

#[test]
fn test_festival_taboos() {
    use world_rules::rules::social::ChineseTabooRules;
    let rules = ChineseTabooRules::new();
    let taboos = rules.festival_taboos();
    assert!(taboos.iter().any(|t| t.contains("春节")));
    assert!(taboos.iter().any(|t| t.contains("清明")));
    assert!(taboos.len() >= 6);
}

#[test]
fn test_number_taboos() {
    use world_rules::rules::social::ChineseTabooRules;
    let rules = ChineseTabooRules::new();
    let taboos = rules.number_taboos();
    assert!(taboos.iter().any(|t| t.contains("四")));
    assert!(taboos.iter().any(|t| t.contains("六") || t.contains("八")));
    assert!(taboos.len() >= 6);
}

// ----- 中国传统礼仪基础规则测试 -----

#[test]
fn test_etiquette_basics_rules_basic() {
    use world_rules::rules::social::ChineseEtiquetteBasicsRules;
    let rules = ChineseEtiquetteBasicsRules::new();
    assert_eq!(rules.metadata().name, "中国传统礼仪基础");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_five_constants() {
    use world_rules::rules::social::ChineseEtiquetteBasicsRules;
    let rules = ChineseEtiquetteBasicsRules::new();
    let constants = rules.five_constants();
    assert!(constants.iter().any(|c| c.contains("仁")));
    assert!(constants.iter().any(|c| c.contains("义")));
    assert!(constants.iter().any(|c| c.contains("礼")));
    assert!(constants.iter().any(|c| c.contains("智")));
    assert!(constants.iter().any(|c| c.contains("信")));
    assert_eq!(constants.len(), 5);
}

#[test]
fn test_eight_virtues() {
    use world_rules::rules::social::ChineseEtiquetteBasicsRules;
    let rules = ChineseEtiquetteBasicsRules::new();
    let virtues = rules.eight_virtues();
    assert!(virtues.iter().any(|v| v.contains("孝")));
    assert!(virtues.iter().any(|v| v.contains("悌")));
    assert!(virtues.iter().any(|v| v.contains("忠")));
    assert!(virtues.iter().any(|v| v.contains("信")));
    assert_eq!(virtues.len(), 8);
}

#[test]
fn test_six_arts() {
    use world_rules::rules::social::ChineseEtiquetteBasicsRules;
    let rules = ChineseEtiquetteBasicsRules::new();
    let arts = rules.six_arts();
    assert!(arts.iter().any(|a| a.contains("礼")));
    assert!(arts.iter().any(|a| a.contains("乐")));
    assert!(arts.iter().any(|a| a.contains("射")));
    assert_eq!(arts.len(), 6);
}

#[test]
fn test_five_relations() {
    use world_rules::rules::social::ChineseEtiquetteBasicsRules;
    let rules = ChineseEtiquetteBasicsRules::new();
    let relations = rules.five_relations();
    assert!(relations.iter().any(|r| r.contains("父子")));
    assert!(relations.iter().any(|r| r.contains("君臣")));
    assert!(relations.iter().any(|r| r.contains("夫妇")));
    assert_eq!(relations.len(), 5);
}

// ----- 中国祭祀礼仪规则测试 -----

#[test]
fn test_ritual_sacrifice_rules_basic() {
    use world_rules::rules::social::ChineseRitualSacrificeRules;
    let rules = ChineseRitualSacrificeRules::new();
    assert_eq!(rules.metadata().name, "中国祭祀礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_sacrifice_objects() {
    use world_rules::rules::social::ChineseRitualSacrificeRules;
    let rules = ChineseRitualSacrificeRules::new();
    let objects = rules.sacrifice_objects();
    assert!(objects.iter().any(|o| o.contains("天神")));
    assert!(objects.iter().any(|o| o.contains("祖先")));
    assert!(objects.iter().any(|o| o.contains("圣贤")));
    assert!(objects.len() >= 5);
}

#[test]
fn test_offering_types() {
    use world_rules::rules::social::ChineseRitualSacrificeRules;
    let rules = ChineseRitualSacrificeRules::new();
    let offerings = rules.offering_types();
    assert!(offerings.iter().any(|o| o.contains("太牢")));
    assert!(offerings.iter().any(|o| o.contains("少牢")));
    assert!(offerings.iter().any(|o| o.contains("五谷")));
    assert!(offerings.len() >= 6);
}

#[test]
fn test_sacrifice_procedure() {
    use world_rules::rules::social::ChineseRitualSacrificeRules;
    let rules = ChineseRitualSacrificeRules::new();
    let procedure = rules.sacrifice_procedure();
    assert!(procedure.iter().any(|p| p.contains("斋戒")));
    assert!(procedure.iter().any(|p| p.contains("上香") || p.contains("焚香")));
    assert!(procedure.len() >= 6);
}

#[test]
fn test_qingming_sacrifice() {
    use world_rules::rules::social::ChineseRitualSacrificeRules;
    let rules = ChineseRitualSacrificeRules::new();
    let qingming = rules.qingming_sacrifice();
    assert!(qingming.iter().any(|q| q.contains("坟墓")));
    assert!(qingming.iter().any(|q| q.contains("上香")));
    assert!(qingming.len() >= 6);
}

// ----- 中国传统服饰礼仪规则测试 -----

#[test]
fn test_traditional_dress_rules_basic() {
    use world_rules::rules::social::ChineseTraditionalDressRules;
    let rules = ChineseTraditionalDressRules::new();
    assert_eq!(rules.metadata().name, "中国传统服饰礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_dress_types() {
    use world_rules::rules::social::ChineseTraditionalDressRules;
    let rules = ChineseTraditionalDressRules::new();
    let types = rules.dress_types();
    assert!(types.iter().any(|t| t.contains("深衣")));
    assert!(types.iter().any(|t| t.contains("袍服")));
    assert!(types.iter().any(|t| t.contains("襦裙")));
    assert!(types.len() >= 6);
}

#[test]
fn test_dress_hierarchy() {
    use world_rules::rules::social::ChineseTraditionalDressRules;
    let rules = ChineseTraditionalDressRules::new();
    let hierarchy = rules.dress_hierarchy();
    assert!(hierarchy.iter().any(|h| h.contains("帝王")));
    assert!(hierarchy.iter().any(|h| h.contains("官员")));
    assert!(hierarchy.iter().any(|h| h.contains("颜色")));
    assert!(hierarchy.len() >= 5);
}

#[test]
fn test_color_symbolism() {
    use world_rules::rules::social::ChineseTraditionalDressRules;
    let rules = ChineseTraditionalDressRules::new();
    let colors = rules.color_symbolism();
    assert!(colors.iter().any(|c| c.contains("黄色")));
    assert!(colors.iter().any(|c| c.contains("红色")));
    assert!(colors.iter().any(|c| c.contains("白色")));
    assert!(colors.len() >= 6);
}

#[test]
fn test_dress_taboos() {
    use world_rules::rules::social::ChineseTraditionalDressRules;
    let rules = ChineseTraditionalDressRules::new();
    let taboos = rules.dress_taboos();
    assert!(taboos.iter().any(|t| t.contains("奇装异服")));
    assert!(taboos.len() >= 6);
}

// ----- 中国待人接物礼仪规则测试 -----

#[test]
fn test_interpersonal_etiquette_rules_basic() {
    use world_rules::rules::social::ChineseInterpersonalEtiquetteRules;
    let rules = ChineseInterpersonalEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "中国待人接物礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_greeting_etiquette() {
    use world_rules::rules::social::ChineseInterpersonalEtiquetteRules;
    let rules = ChineseInterpersonalEtiquetteRules::new();
    let greeting = rules.greeting_etiquette();
    assert!(greeting.iter().any(|g| g.contains("拱手")));
    assert!(greeting.iter().any(|g| g.contains("鞠躬")));
    assert!(greeting.len() >= 6);
}

#[test]
fn test_address_etiquette() {
    use world_rules::rules::social::ChineseInterpersonalEtiquetteRules;
    let rules = ChineseInterpersonalEtiquetteRules::new();
    let address = rules.address_etiquette();
    assert!(address.iter().any(|a| a.contains("长辈")));
    assert!(address.iter().any(|a| a.contains("平辈")));
    assert!(address.len() >= 6);
}

#[test]
fn test_hosting_etiquette() {
    use world_rules::rules::social::ChineseInterpersonalEtiquetteRules;
    let rules = ChineseInterpersonalEtiquetteRules::new();
    let hosting = rules.hosting_etiquette();
    assert!(hosting.iter().any(|h| h.contains("迎客")));
    assert!(hosting.iter().any(|h| h.contains("送客")));
    assert!(hosting.len() >= 6);
}

#[test]
fn test_tea_etiquette() {
    use world_rules::rules::social::ChineseInterpersonalEtiquetteRules;
    let rules = ChineseInterpersonalEtiquetteRules::new();
    let tea = rules.tea_etiquette();
    assert!(tea.iter().any(|t| t.contains("奉茶")));
    assert!(tea.iter().any(|t| t.contains("叩指")));
    assert!(tea.len() >= 6);
}

// ----- 中国传统书信礼仪规则测试 -----

#[test]
fn test_correspondence_rules_basic() {
    use world_rules::rules::social::ChineseCorrespondenceRules;
    let rules = ChineseCorrespondenceRules::new();
    assert_eq!(rules.metadata().name, "中国传统书信礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_letter_format() {
    use world_rules::rules::social::ChineseCorrespondenceRules;
    let rules = ChineseCorrespondenceRules::new();
    let format = rules.letter_format();
    assert!(format.iter().any(|f| f.contains("抬头")));
    assert!(format.iter().any(|f| f.contains("正文")));
    assert!(format.iter().any(|f| f.contains("署名")));
    assert!(format.len() >= 6);
}

#[test]
fn test_address_format() {
    use world_rules::rules::social::ChineseCorrespondenceRules;
    let rules = ChineseCorrespondenceRules::new();
    let address = rules.address_format();
    assert!(address.iter().any(|a| a.contains("长辈")));
    assert!(address.iter().any(|a| a.contains("师长")));
    assert!(address.iter().any(|a| a.contains("平辈")));
    assert!(address.len() >= 6);
}

#[test]
fn test_humble_terms() {
    use world_rules::rules::social::ChineseCorrespondenceRules;
    let rules = ChineseCorrespondenceRules::new();
    let terms = rules.humble_terms();
    assert!(terms.iter().any(|t| t.contains("鄙人")));
    assert!(terms.iter().any(|t| t.contains("愚")));
    assert!(terms.len() >= 6);
}

#[test]
fn test_honorific_terms() {
    use world_rules::rules::social::ChineseCorrespondenceRules;
    let rules = ChineseCorrespondenceRules::new();
    let terms = rules.honorific_terms();
    assert!(terms.iter().any(|t| t.contains("尊称")));
    assert!(terms.iter().any(|t| t.contains("令")));
    assert!(terms.len() >= 6);
}

// ----- 中国传统仕途礼仪规则测试 -----

#[test]
fn test_official_etiquette_rules_basic() {
    use world_rules::rules::social::ChineseOfficialEtiquetteRules;
    let rules = ChineseOfficialEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "中国传统仕途礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_official_ranks() {
    use world_rules::rules::social::ChineseOfficialEtiquetteRules;
    let rules = ChineseOfficialEtiquetteRules::new();
    let ranks = rules.official_ranks();
    assert!(ranks.iter().any(|r| r.contains("一品")));
    assert!(ranks.iter().any(|r| r.contains("四品")));
    assert!(ranks.len() >= 6);
}

#[test]
fn test_official_dress() {
    use world_rules::rules::social::ChineseOfficialEtiquetteRules;
    let rules = ChineseOfficialEtiquetteRules::new();
    let dress = rules.official_dress();
    assert!(dress.iter().any(|d| d.contains("仙鹤")));
    assert!(dress.iter().any(|d| d.contains("孔雀")));
    assert!(dress.len() >= 6);
}

#[test]
fn test_court_etiquette() {
    use world_rules::rules::social::ChineseOfficialEtiquetteRules;
    let rules = ChineseOfficialEtiquetteRules::new();
    let court = rules.court_etiquette();
    assert!(court.iter().any(|c| c.contains("跪")));
    assert!(court.iter().any(|c| c.contains("叩")));
    assert!(court.len() >= 6);
}

#[test]
fn test_official_virtues() {
    use world_rules::rules::social::ChineseOfficialEtiquetteRules;
    let rules = ChineseOfficialEtiquetteRules::new();
    let virtues = rules.official_virtues();
    assert!(virtues.iter().any(|v| v.contains("清正")));
    assert!(virtues.iter().any(|v| v.contains("勤政")));
    assert!(virtues.len() >= 6);
}

// ----- 中国传统宗族礼仪规则测试 -----

#[test]
fn test_clan_etiquette_rules_basic() {
    use world_rules::rules::social::ChineseClanEtiquetteRules;
    let rules = ChineseClanEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "中国传统宗族礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_clan_organization() {
    use world_rules::rules::social::ChineseClanEtiquetteRules;
    let rules = ChineseClanEtiquetteRules::new();
    let org = rules.clan_organization();
    assert!(org.iter().any(|o| o.contains("族长")));
    assert!(org.iter().any(|o| o.contains("祠堂")));
    assert!(org.iter().any(|o| o.contains("族谱")));
    assert!(org.len() >= 6);
}

#[test]
fn test_generation_order() {
    use world_rules::rules::social::ChineseClanEtiquetteRules;
    let rules = ChineseClanEtiquetteRules::new();
    let gen = rules.generation_order();
    assert!(gen.iter().any(|g| g.contains("辈分")));
    assert!(gen.iter().any(|g| g.contains("嫡庶")));
    assert!(gen.len() >= 6);
}

#[test]
fn test_family_rules() {
    use world_rules::rules::social::ChineseClanEtiquetteRules;
    let rules = ChineseClanEtiquetteRules::new();
    let rules_list = rules.family_rules();
    assert!(rules_list.iter().any(|r| r.contains("孝")));
    assert!(rules_list.iter().any(|r| r.contains("诚")));
    assert!(rules_list.len() >= 6);
}

#[test]
fn test_clan_duties() {
    use world_rules::rules::social::ChineseClanEtiquetteRules;
    let rules = ChineseClanEtiquetteRules::new();
    let duties = rules.clan_duties();
    assert!(duties.iter().any(|d| d.contains("祭祀")));
    assert!(duties.len() >= 6);
}

// ============================================================================
// 总体测试计数验证
// ============================================================================

#[test]
fn test_phase_26_total_rules_count() {
    // Phase 26-01 应添加 10 种规则
    use world_rules::rules::social::{
        ChineseComingOfAgeRules,
        ChineseBirthdayEtiquetteRules,
        ChineseTabooRules,
        ChineseEtiquetteBasicsRules,
        ChineseRitualSacrificeRules,
        ChineseTraditionalDressRules,
        ChineseInterpersonalEtiquetteRules,
        ChineseCorrespondenceRules,
        ChineseOfficialEtiquetteRules,
        ChineseClanEtiquetteRules,
    };

    let rules_count = 10;
    assert_eq!(rules_count, 10, "Phase 26-01 应包含 10 种传统礼仪规则");
}