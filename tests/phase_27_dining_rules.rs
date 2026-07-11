//! Phase 27-02: 10种餐饮礼仪规则测试
//!
//! 测试新增的餐饮礼仪规则

use world_rules::rules::core::{Rule, RuleCategory, ValidateContext};
use world_rules::rules::social::{
    BuffetDiningRules, CocktailPartyRules, CoffeeHouseRules, FoodFestivalRules,
    FormalBanquetRules, JapaneseCuisineRules, KoreanCuisineRules, TeaHouseRules,
    WesternDiningRules, WineTastingRules,
};

#[test]
fn test_western_dining_rules() {
    let rules = WesternDiningRules::new();
    assert_eq!(rules.metadata().name, "西餐礼仪");
    assert!(!rules.table_setting().is_empty());
    assert!(!rules.course_order().is_empty());
    assert!(!rules.utensil_etiquette().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_japanese_cuisine_rules() {
    let rules = JapaneseCuisineRules::new();
    assert_eq!(rules.metadata().name, "日式料理礼仪");
    assert!(!rules.basic_manners().is_empty());
    assert!(!rules.chopstick_etiquette().is_empty());
    assert!(!rules.sushi_etiquette().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_korean_cuisine_rules() {
    let rules = KoreanCuisineRules::new();
    assert_eq!(rules.metadata().name, "韩餐礼仪");
    assert!(!rules.basic_manners().is_empty());
    assert!(!rules.utensil_etiquette().is_empty());
    assert!(!rules.banchan_etiquette().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_formal_banquet_rules() {
    let rules = FormalBanquetRules::new();
    assert_eq!(rules.metadata().name, "正式宴会礼仪");
    assert!(!rules.entrance().is_empty());
    assert!(!rules.seating().is_empty());
    assert!(!rules.toast().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_cocktail_party_rules() {
    let rules = CocktailPartyRules::new();
    assert_eq!(rules.metadata().name, "鸡尾酒会礼仪");
    assert!(!rules.entrance().is_empty());
    assert!(!rules.drinking().is_empty());
    assert!(!rules.networking().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_buffet_dining_rules() {
    let rules = BuffetDiningRules::new();
    assert_eq!(rules.metadata().name, "自助餐礼仪");
    assert!(!rules.serving().is_empty());
    assert!(!rules.dining().is_empty());
    assert!(!rules.hygiene().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_coffee_house_rules() {
    let rules = CoffeeHouseRules::new();
    assert_eq!(rules.metadata().name, "咖啡厅礼仪");
    assert!(!rules.seating().is_empty());
    assert!(!rules.drinking().is_empty());
    assert!(!rules.working().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_tea_house_rules() {
    let rules = TeaHouseRules::new();
    assert_eq!(rules.metadata().name, "茶馆礼仪");
    assert!(!rules.seating().is_empty());
    assert!(!rules.tea_tasting().is_empty());
    assert!(!rules.tea_art().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_wine_tasting_rules() {
    let rules = WineTastingRules::new();
    assert_eq!(rules.metadata().name, "红酒礼仪");
    assert!(!rules.wine_selection().is_empty());
    assert!(!rules.tasting().is_empty());
    assert!(!rules.pairing().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_food_festival_rules() {
    let rules = FoodFestivalRules::new();
    assert_eq!(rules.metadata().name, "美食节礼仪");
    assert!(!rules.entrance().is_empty());
    assert!(!rules.tasting().is_empty());
    assert!(!rules.environment().is_empty());
    assert_eq!(rules.category(), RuleCategory::social("dining"));
}

#[test]
fn test_all_dining_rules_validation() {
    // 测试所有餐饮礼仪规则的验证功能
    let rules_list: Vec<&dyn Rule> = vec![
        &WesternDiningRules::new(),
        &JapaneseCuisineRules::new(),
        &KoreanCuisineRules::new(),
        &FormalBanquetRules::new(),
        &CocktailPartyRules::new(),
        &BuffetDiningRules::new(),
        &CoffeeHouseRules::new(),
        &TeaHouseRules::new(),
        &WineTastingRules::new(),
        &FoodFestivalRules::new(),
    ];

    for rules in rules_list {
        assert!(rules.validate(&ValidateContext::Generic("test".to_string())).is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }
}

#[test]
fn test_dining_rules_explain_content() {
    // 测试解释内容包含关键信息
    let western = WesternDiningRules::new();
    assert!(western.explain().contains("西餐礼仪"));
    assert!(western.explain().contains("餐具摆放"));

    let japanese = JapaneseCuisineRules::new();
    assert!(japanese.explain().contains("日式料理礼仪"));
    assert!(japanese.explain().contains("筷子礼仪"));

    let korean = KoreanCuisineRules::new();
    assert!(korean.explain().contains("韩餐礼仪"));
    assert!(korean.explain().contains("用餐基本礼仪"));
}