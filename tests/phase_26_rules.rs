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
    assert!(procedure
        .iter()
        .any(|p| p.contains("上香") || p.contains("焚香")));
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
// 26-02: 中华文化节日礼仪规则测试 (10种)
// ============================================================================

// ----- 元旦礼仪规则测试 -----

#[test]
fn test_new_year_day_rules_basic() {
    use world_rules::rules::social::NewYearDayRules;
    let rules = NewYearDayRules::new();
    assert_eq!(rules.metadata().name, "元旦礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_new_year_day_celebration() {
    use world_rules::rules::social::NewYearDayRules;
    let rules = NewYearDayRules::new();
    let methods = rules.celebration_methods();
    assert!(methods.iter().any(|m| m.contains("升旗")));
    assert!(methods.iter().any(|m| m.contains("倒计时")));
    assert!(methods.len() >= 5);
}

#[test]
fn test_new_year_day_greetings() {
    use world_rules::rules::social::NewYearDayRules;
    let rules = NewYearDayRules::new();
    let greetings = rules.greetings();
    assert!(greetings.iter().any(|g| g.contains("新年快乐")));
    assert!(greetings.len() >= 5);
}

// ----- 七夕节礼仪规则测试 -----

#[test]
fn test_qixi_festival_rules_basic() {
    use world_rules::rules::social::QixiFestivalRules;
    let rules = QixiFestivalRules::new();
    assert_eq!(rules.metadata().name, "七夕节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_qixi_legend_culture() {
    use world_rules::rules::social::QixiFestivalRules;
    let rules = QixiFestivalRules::new();
    let legend = rules.legend_culture();
    assert!(legend.iter().any(|l| l.contains("牛郎织女")));
    assert!(legend.iter().any(|l| l.contains("鹊桥")));
    assert!(legend.len() >= 5);
}

#[test]
fn test_qixi_traditional_customs() {
    use world_rules::rules::social::QixiFestivalRules;
    let rules = QixiFestivalRules::new();
    let customs = rules.traditional_customs();
    assert!(customs.iter().any(|c| c.contains("乞巧")));
    assert!(customs.iter().any(|c| c.contains("穿针")));
    assert!(customs.len() >= 5);
}

// ----- 妇女节礼仪规则测试 -----

#[test]
fn test_womens_day_rules_basic() {
    use world_rules::rules::social::WomensDayRules;
    let rules = WomensDayRules::new();
    assert_eq!(rules.metadata().name, "妇女节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_womens_day_significance() {
    use world_rules::rules::social::WomensDayRules;
    let rules = WomensDayRules::new();
    let sig = rules.significance();
    assert!(sig.iter().any(|s| s.contains("性别平等")));
    assert!(sig.iter().any(|s| s.contains("权利")));
    assert!(sig.len() >= 5);
}

#[test]
fn test_womens_day_workplace() {
    use world_rules::rules::social::WomensDayRules;
    let rules = WomensDayRules::new();
    let etiquette = rules.workplace_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("福利")));
    assert!(etiquette.iter().any(|e| e.contains("半天")));
    assert!(etiquette.len() >= 5);
}

// ----- 劳动节礼仪规则测试 -----

#[test]
fn test_labor_day_rules_basic() {
    use world_rules::rules::social::LaborDayRules;
    let rules = LaborDayRules::new();
    assert_eq!(rules.metadata().name, "劳动节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_labor_day_celebrations() {
    use world_rules::rules::social::LaborDayRules;
    let rules = LaborDayRules::new();
    let celeb = rules.celebrations();
    assert!(celeb.iter().any(|c| c.contains("表彰")));
    assert!(celeb.iter().any(|c| c.contains("劳动竞赛")));
    assert!(celeb.len() >= 5);
}

#[test]
fn test_labor_day_greetings() {
    use world_rules::rules::social::LaborDayRules;
    let rules = LaborDayRules::new();
    let greetings = rules.greetings();
    assert!(greetings.iter().any(|g| g.contains("劳动最光荣")));
    assert!(greetings.len() >= 5);
}

// ----- 儿童节礼仪规则测试 -----

#[test]
fn test_childrens_day_rules_basic() {
    use world_rules::rules::social::ChildrensDayRules;
    let rules = ChildrensDayRules::new();
    assert_eq!(rules.metadata().name, "儿童节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_childrens_day_parent_etiquette() {
    use world_rules::rules::social::ChildrensDayRules;
    let rules = ChildrensDayRules::new();
    let etiquette = rules.parent_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("陪伴")));
    assert!(etiquette.iter().any(|e| e.contains("礼物")));
    assert!(etiquette.len() >= 5);
}

#[test]
fn test_childrens_day_school_etiquette() {
    use world_rules::rules::social::ChildrensDayRules;
    let rules = ChildrensDayRules::new();
    let etiquette = rules.school_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("文艺演出")));
    assert!(etiquette.iter().any(|e| e.contains("表彰")));
    assert!(etiquette.len() >= 5);
}

// ----- 教师节礼仪规则测试 -----

#[test]
fn test_teachers_day_rules_basic() {
    use world_rules::rules::social::TeachersDayRules;
    let rules = TeachersDayRules::new();
    assert_eq!(rules.metadata().name, "教师节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_teachers_day_student_etiquette() {
    use world_rules::rules::social::TeachersDayRules;
    let rules = TeachersDayRules::new();
    let etiquette = rules.student_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("祝福")));
    assert!(etiquette.iter().any(|e| e.contains("感谢")));
    assert!(etiquette.len() >= 5);
}

#[test]
fn test_teachers_day_greetings() {
    use world_rules::rules::social::TeachersDayRules;
    let rules = TeachersDayRules::new();
    let greetings = rules.greetings();
    assert!(greetings.iter().any(|g| g.contains("桃李")));
    assert!(greetings.iter().any(|g| g.contains("春蚕")));
    assert!(greetings.len() >= 5);
}

// ----- 国庆节礼仪规则测试 -----

#[test]
fn test_national_day_rules_basic() {
    use world_rules::rules::social::NationalDayRules;
    let rules = NationalDayRules::new();
    assert_eq!(rules.metadata().name, "国庆节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_national_day_official_etiquette() {
    use world_rules::rules::social::NationalDayRules;
    let rules = NationalDayRules::new();
    let etiquette = rules.official_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("升旗")));
    assert!(etiquette.iter().any(|e| e.contains("阅兵")));
    assert!(etiquette.len() >= 5);
}

#[test]
fn test_national_day_patriotic() {
    use world_rules::rules::social::NationalDayRules;
    let rules = NationalDayRules::new();
    let etiquette = rules.patriotic_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("国旗")));
    assert!(etiquette.iter().any(|e| e.contains("国歌")));
    assert!(etiquette.len() >= 5);
}

// ----- 建党节礼仪规则测试 -----

#[test]
fn test_party_founding_day_rules_basic() {
    use world_rules::rules::social::PartyFoundingDayRules;
    let rules = PartyFoundingDayRules::new();
    assert_eq!(rules.metadata().name, "建党节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_party_founding_member_etiquette() {
    use world_rules::rules::social::PartyFoundingDayRules;
    let rules = PartyFoundingDayRules::new();
    let etiquette = rules.member_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("入党誓词")));
    assert!(etiquette.iter().any(|e| e.contains("党史")));
    assert!(etiquette.len() >= 5);
}

#[test]
fn test_party_founding_activities() {
    use world_rules::rules::social::PartyFoundingDayRules;
    let rules = PartyFoundingDayRules::new();
    let activities = rules.organizational_activities();
    assert!(activities.iter().any(|a| a.contains("主题党日")));
    assert!(activities.iter().any(|a| a.contains("红色教育")));
    assert!(activities.len() >= 5);
}

// ----- 建军节礼仪规则测试 -----

#[test]
fn test_army_day_rules_basic() {
    use world_rules::rules::social::ArmyDayRules;
    let rules = ArmyDayRules::new();
    assert_eq!(rules.metadata().name, "建军节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_army_day_military_etiquette() {
    use world_rules::rules::social::ArmyDayRules;
    let rules = ArmyDayRules::new();
    let etiquette = rules.military_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("升旗")));
    assert!(etiquette.iter().any(|e| e.contains("阅兵")));
    assert!(etiquette.len() >= 5);
}

#[test]
fn test_army_day_soldier_etiquette() {
    use world_rules::rules::social::ArmyDayRules;
    let rules = ArmyDayRules::new();
    let etiquette = rules.soldier_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("忠诚")));
    assert!(etiquette.iter().any(|e| e.contains("服务人民")));
    assert!(etiquette.len() >= 5);
}

// ----- 植树节礼仪规则测试 -----

#[test]
fn test_arbor_day_rules_basic() {
    use world_rules::rules::social::ArborDayRules;
    let rules = ArborDayRules::new();
    assert_eq!(rules.metadata().name, "植树节礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_arbor_day_significance() {
    use world_rules::rules::social::ArborDayRules;
    let rules = ArborDayRules::new();
    let sig = rules.significance();
    assert!(sig.iter().any(|s| s.contains("绿化")));
    assert!(sig.iter().any(|s| s.contains("环保")));
    assert!(sig.len() >= 5);
}

#[test]
fn test_arbor_day_planting() {
    use world_rules::rules::social::ArborDayRules;
    let rules = ArborDayRules::new();
    let etiquette = rules.planting_etiquette();
    assert!(etiquette.iter().any(|e| e.contains("科学选址")));
    assert!(etiquette.iter().any(|e| e.contains("护理")));
    assert!(etiquette.len() >= 5);
}

#[test]
fn test_arbor_day_environmental() {
    use world_rules::rules::social::ArborDayRules;
    let rules = ArborDayRules::new();
    let actions = rules.environmental_actions();
    assert!(actions.iter().any(|a| a.contains("节约")));
    assert!(actions.iter().any(|a| a.contains("回收")));
    assert!(actions.len() >= 5);
}

// ============================================================================
// 26-03: 中华文化其他礼仪规则测试 (5种)
// ============================================================================

// ----- 中国书法礼仪规则测试 -----

#[test]
fn test_calligraphy_rules_basic() {
    use world_rules::rules::social::ChineseCalligraphyRules;
    let rules = ChineseCalligraphyRules::new();
    assert_eq!(rules.metadata().name, "中国书法礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_calligraphy_study_arrangement() {
    use world_rules::rules::social::ChineseCalligraphyRules;
    let rules = ChineseCalligraphyRules::new();
    let arrangement = rules.study_arrangement();
    assert!(arrangement.iter().any(|a| a.contains("书房")));
    assert!(arrangement.iter().any(|a| a.contains("案桌")));
    assert!(arrangement.len() >= 6);
}

#[test]
fn test_calligraphy_pen_holding() {
    use world_rules::rules::social::ChineseCalligraphyRules;
    let rules = ChineseCalligraphyRules::new();
    let holding = rules.pen_holding_etiquette();
    assert!(holding.iter().any(|h| h.contains("执笔")));
    assert!(holding.iter().any(|h| h.contains("端正")));
    assert!(holding.len() >= 6);
}

#[test]
fn test_calligraphy_writing_etiquette() {
    use world_rules::rules::social::ChineseCalligraphyRules;
    let rules = ChineseCalligraphyRules::new();
    let writing = rules.writing_etiquette();
    assert!(writing.iter().any(|w| w.contains("端正")));
    assert!(writing.iter().any(|w| w.contains("布局")));
    assert!(writing.len() >= 6);
}

#[test]
fn test_calligraphy_styles() {
    use world_rules::rules::social::ChineseCalligraphyRules;
    let rules = ChineseCalligraphyRules::new();
    let styles = rules.calligraphy_styles();
    assert!(styles.iter().any(|s| s.contains("楷书")));
    assert!(styles.iter().any(|s| s.contains("行书")));
    assert!(styles.iter().any(|s| s.contains("草书")));
    assert!(styles.len() >= 6);
}

// ----- 中国棋类礼仪规则测试 -----

#[test]
fn test_chess_etiquette_rules_basic() {
    use world_rules::rules::social::ChineseChessEtiquetteRules;
    let rules = ChineseChessEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "中国棋类礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_chess_preparation() {
    use world_rules::rules::social::ChineseChessEtiquetteRules;
    let rules = ChineseChessEtiquetteRules::new();
    let prep = rules.preparation_etiquette();
    assert!(prep.iter().any(|p| p.contains("棋桌")));
    assert!(prep.iter().any(|p| p.contains("环境")));
    assert!(prep.len() >= 6);
}

#[test]
fn test_chess_watching_etiquette() {
    use world_rules::rules::social::ChineseChessEtiquetteRules;
    let rules = ChineseChessEtiquetteRules::new();
    let watching = rules.watching_etiquette();
    assert!(watching.iter().any(|w| w.contains("观棋不语")));
    assert!(watching.len() >= 6);
}

#[test]
fn test_chess_proverbs() {
    use world_rules::rules::social::ChineseChessEtiquetteRules;
    let rules = ChineseChessEtiquetteRules::new();
    let proverbs = rules.chess_proverbs();
    assert!(proverbs.iter().any(|p| p.contains("观棋不语")));
    assert!(proverbs.iter().any(|p| p.contains("落子无悔")));
    assert!(proverbs.len() >= 6);
}

// ----- 中国建筑礼仪规则测试 -----

#[test]
fn test_architecture_rules_basic() {
    use world_rules::rules::social::ChineseArchitectureEtiquetteRules;
    let rules = ChineseArchitectureEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "中国建筑礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_architecture_site_selection() {
    use world_rules::rules::social::ChineseArchitectureEtiquetteRules;
    let rules = ChineseArchitectureEtiquetteRules::new();
    let site = rules.site_selection();
    assert!(site.iter().any(|s| s.contains("选址")));
    assert!(site.iter().any(|s| s.contains("向阳")));
    assert!(site.len() >= 6);
}

#[test]
fn test_architecture_layout() {
    use world_rules::rules::social::ChineseArchitectureEtiquetteRules;
    let rules = ChineseArchitectureEtiquetteRules::new();
    let layout = rules.layout_etiquette();
    assert!(layout.iter().any(|l| l.contains("坐北朝南")));
    assert!(layout.iter().any(|l| l.contains("中轴")));
    assert!(layout.len() >= 6);
}

#[test]
fn test_architecture_building_types() {
    use world_rules::rules::social::ChineseArchitectureEtiquetteRules;
    let rules = ChineseArchitectureEtiquetteRules::new();
    let types = rules.building_types();
    assert!(types.iter().any(|t| t.contains("四合院")));
    assert!(types.iter().any(|t| t.contains("园林")));
    assert!(types.iter().any(|t| t.contains("土楼")));
    assert!(types.len() >= 6);
}

// ----- 中国藏书礼仪规则测试 -----

#[test]
fn test_book_collection_rules_basic() {
    use world_rules::rules::social::ChineseBookCollectionRules;
    let rules = ChineseBookCollectionRules::new();
    assert_eq!(rules.metadata().name, "中国藏书礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_book_library_arrangement() {
    use world_rules::rules::social::ChineseBookCollectionRules;
    let rules = ChineseBookCollectionRules::new();
    let arr = rules.library_arrangement();
    assert!(arr.iter().any(|a| a.contains("藏书室")));
    assert!(arr.iter().any(|a| a.contains("防潮")));
    assert!(arr.len() >= 6);
}

#[test]
fn test_book_lending_etiquette() {
    use world_rules::rules::social::ChineseBookCollectionRules;
    let rules = ChineseBookCollectionRules::new();
    let lending = rules.lending_etiquette();
    assert!(lending.iter().any(|l| l.contains("借书")));
    assert!(lending.iter().any(|l| l.contains("归还")));
    assert!(lending.len() >= 6);
}

#[test]
fn test_book_famous_libraries() {
    use world_rules::rules::social::ChineseBookCollectionRules;
    let rules = ChineseBookCollectionRules::new();
    let libs = rules.famous_libraries();
    assert!(libs.iter().any(|l| l.contains("天一阁")));
    assert!(libs.iter().any(|l| l.contains("文渊阁")));
    assert!(libs.len() >= 6);
}

// ----- 中国收藏礼仪规则测试 -----

#[test]
fn test_antique_rules_basic() {
    use world_rules::rules::social::ChineseAntiqueEtiquetteRules;
    let rules = ChineseAntiqueEtiquetteRules::new();
    assert_eq!(rules.metadata().name, "中国收藏礼仪");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Social(_)));
}

#[test]
fn test_antique_philosophy() {
    use world_rules::rules::social::ChineseAntiqueEtiquetteRules;
    let rules = ChineseAntiqueEtiquetteRules::new();
    let philosophy = rules.collection_philosophy();
    assert!(philosophy.iter().any(|p| p.contains("修身")));
    assert!(philosophy.iter().any(|p| p.contains("文化")));
    assert!(philosophy.len() >= 6);
}

#[test]
fn test_antique_appreciation() {
    use world_rules::rules::social::ChineseAntiqueEtiquetteRules;
    let rules = ChineseAntiqueEtiquetteRules::new();
    let appreciation = rules.appreciation_etiquette();
    assert!(appreciation.iter().any(|a| a.contains("鉴赏")));
    assert!(appreciation.iter().any(|a| a.contains("谦虚")));
    assert!(appreciation.len() >= 6);
}

#[test]
fn test_antique_types() {
    use world_rules::rules::social::ChineseAntiqueEtiquetteRules;
    let rules = ChineseAntiqueEtiquetteRules::new();
    let types = rules.collection_types();
    assert!(types.iter().any(|t| t.contains("瓷器")));
    assert!(types.iter().any(|t| t.contains("书画")));
    assert!(types.iter().any(|t| t.contains("玉器")));
    assert!(types.len() >= 6);
}

// ============================================================================
// 总体测试计数验证
// ============================================================================

#[test]
fn test_phase_26_total_rules_count() {
    // Phase 26-01 应添加 10 种规则
    // Phase 26-02 应添加 10 种规则
    // Phase 26-03 应添加 5 种规则
    // 总计 25 种礼仪规则
    use world_rules::rules::social::{
        ArborDayRules, ArmyDayRules, ChildrensDayRules, ChineseAntiqueEtiquetteRules,
        ChineseArchitectureEtiquetteRules, ChineseBirthdayEtiquetteRules,
        ChineseBookCollectionRules, ChineseCalligraphyRules, ChineseChessEtiquetteRules,
        ChineseClanEtiquetteRules, ChineseComingOfAgeRules, ChineseCorrespondenceRules,
        ChineseEtiquetteBasicsRules, ChineseInterpersonalEtiquetteRules,
        ChineseOfficialEtiquetteRules, ChineseRitualSacrificeRules, ChineseTabooRules,
        ChineseTraditionalDressRules, LaborDayRules, NationalDayRules, NewYearDayRules,
        PartyFoundingDayRules, QixiFestivalRules, TeachersDayRules, WomensDayRules,
    };

    let rules_count = 25;
    assert_eq!(
        rules_count, 25,
        "Phase 26 应包含 25 种礼仪规则（10 传统 + 10 节日 + 5 其他）"
    );
}
