//! Phase 20 新规则集成测试
//!
//! 测试 Phase 20 添加的麻将变体规则：
//! - 20-01: 5种中国麻将变体（湖南、河北、山西、宁夏、内蒙古）
//! - 20-02: 5种日本麻将变体（待添加）
//! - 20-03: 5种其他麻将变体（待添加）

use world_rules::prelude::*;

// ===== Phase 20-01: 中国麻将变体规则测试 =====

#[test]
fn hunan_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::hunan::HunanMahjongRules;

    let rules = HunanMahjongRules::new();
    assert_eq!(rules.metadata().name, "湖南麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn hunan_mahjong_seven_pairs_rules() {
    use world_rules::rules::games::mahjong::variants::hunan::HunanMahjongRules;

    let rules = HunanMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("七小对"), "应说明七小对规则");
    assert!(explanation.contains("龙七对"), "应说明龙七对");
    assert!(explanation.contains("双龙七对"), "应说明双龙七对");
}

#[test]
fn hunan_mahjong_fan_types() {
    use world_rules::rules::games::mahjong::variants::hunan::HunanMahjongRules;

    let rules = HunanMahjongRules::new();
    let fan_types = rules.fan_types();

    assert!(!fan_types.is_empty());
    assert!(fan_types.iter().any(|(name, _)| name == "七小对"));
    assert!(fan_types.iter().any(|(name, _)| name == "龙七对"));
}

#[test]
fn hunan_mahjong_basic_settings() {
    use world_rules::rules::games::mahjong::variants::hunan::HunanMahjongRules;

    let rules = HunanMahjongRules::new();
    let settings = rules.basic_settings();

    assert!(settings.iter().any(|s| s.contains("136张")));
    assert!(settings.iter().any(|s| s.contains("七小对")));
}

#[test]
fn hebei_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::hebei::HebeiMahjongRules;

    let rules = HebeiMahjongRules::new();
    assert_eq!(rules.metadata().name, "河北麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn hebei_mahjong_tuidao_hu_rules() {
    use world_rules::rules::games::mahjong::variants::hebei::HebeiMahjongRules;

    let rules = HebeiMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("推倒胡"), "应说明推倒胡规则");
}

#[test]
fn hebei_mahjong_fan_types() {
    use world_rules::rules::games::mahjong::variants::hebei::HebeiMahjongRules;

    let rules = HebeiMahjongRules::new();
    let fan_types = rules.fan_types();

    assert!(!fan_types.is_empty());
    assert!(fan_types.iter().any(|(name, _)| name == "屁胡"));
}

#[test]
fn hebei_mahjong_basic_settings() {
    use world_rules::rules::games::mahjong::variants::hebei::HebeiMahjongRules;

    let rules = HebeiMahjongRules::new();
    let settings = rules.basic_settings();

    assert!(settings.iter().any(|s| s.contains("136张")));
    assert!(settings.iter().any(|s| s.contains("推倒胡")));
}

#[test]
fn shanxi_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::shanxi::ShanxiMahjongRules;

    let rules = ShanxiMahjongRules::new();
    assert_eq!(rules.metadata().name, "山西麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn shanxi_mahjong_quemen_rules() {
    use world_rules::rules::games::mahjong::variants::shanxi::ShanxiMahjongRules;

    let rules = ShanxiMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("缺一门"), "应说明缺一门规则");
}

#[test]
fn shanxi_mahjong_fan_types() {
    use world_rules::rules::games::mahjong::variants::shanxi::ShanxiMahjongRules;

    let rules = ShanxiMahjongRules::new();
    let fan_types = rules.fan_types();

    assert!(!fan_types.is_empty());
    assert!(fan_types.iter().any(|(name, _)| name == "缺一门"));
}

#[test]
fn shanxi_mahjong_basic_settings() {
    use world_rules::rules::games::mahjong::variants::shanxi::ShanxiMahjongRules;

    let rules = ShanxiMahjongRules::new();
    let settings = rules.basic_settings();

    assert!(settings.iter().any(|s| s.contains("136张")));
    assert!(settings.iter().any(|s| s.contains("缺一门")));
}

#[test]
fn ningxia_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::ningxia::NingxiaMahjongRules;

    let rules = NingxiaMahjongRules::new();
    assert_eq!(rules.metadata().name, "宁夏麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn ningxia_mahjong_pengpeng_hu_rules() {
    use world_rules::rules::games::mahjong::variants::ningxia::NingxiaMahjongRules;

    let rules = NingxiaMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("碰碰胡"), "应说明碰碰胡规则");
}

#[test]
fn ningxia_mahjong_fan_types() {
    use world_rules::rules::games::mahjong::variants::ningxia::NingxiaMahjongRules;

    let rules = NingxiaMahjongRules::new();
    let fan_types = rules.fan_types();

    assert!(!fan_types.is_empty());
    assert!(fan_types.iter().any(|(name, _)| name == "碰碰胡"));
}

#[test]
fn ningxia_mahjong_basic_settings() {
    use world_rules::rules::games::mahjong::variants::ningxia::NingxiaMahjongRules;

    let rules = NingxiaMahjongRules::new();
    let settings = rules.basic_settings();

    assert!(settings.iter().any(|s| s.contains("136张")));
    assert!(settings.iter().any(|s| s.contains("碰碰胡")));
}

#[test]
fn inner_mongolia_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::inner_mongolia::InnerMongoliaMahjongRules;

    let rules = InnerMongoliaMahjongRules::new();
    assert_eq!(rules.metadata().name, "内蒙古麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn inner_mongolia_mahjong_dahu_rules() {
    use world_rules::rules::games::mahjong::variants::inner_mongolia::InnerMongoliaMahjongRules;

    let rules = InnerMongoliaMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("大胡"), "应说明大胡规则");
}

#[test]
fn inner_mongolia_mahjong_fan_types() {
    use world_rules::rules::games::mahjong::variants::inner_mongolia::InnerMongoliaMahjongRules;

    let rules = InnerMongoliaMahjongRules::new();
    let fan_types = rules.fan_types();

    assert!(!fan_types.is_empty());
    assert!(fan_types.iter().any(|(name, _)| name == "清一色"));
    assert!(fan_types.iter().any(|(name, _)| name == "清对"));
    assert!(fan_types.iter().any(|(name, _)| name == "清七对"));
}

#[test]
fn inner_mongolia_mahjong_basic_settings() {
    use world_rules::rules::games::mahjong::variants::inner_mongolia::InnerMongoliaMahjongRules;

    let rules = InnerMongoliaMahjongRules::new();
    let settings = rules.basic_settings();

    assert!(settings.iter().any(|s| s.contains("136张")));
    assert!(settings.iter().any(|s| s.contains("大胡")));
}

// ===== Phase 20-01 总体测试 =====

#[test]
fn phase_20_01_all_variants_exist() {
    // 验证所有 5 种变体都能正确导入和创建
    use world_rules::rules::games::mahjong::variants::{
        HebeiMahjongRules, HunanMahjongRules, InnerMongoliaMahjongRules, NingxiaMahjongRules,
        ShanxiMahjongRules,
    };

    let hunan = HunanMahjongRules::new();
    let hebei = HebeiMahjongRules::new();
    let shanxi = ShanxiMahjongRules::new();
    let ningxia = NingxiaMahjongRules::new();
    let inner_mongolia = InnerMongoliaMahjongRules::new();

    // 所有规则都有有效的元数据
    assert!(!hunan.metadata().name.is_empty());
    assert!(!hebei.metadata().name.is_empty());
    assert!(!shanxi.metadata().name.is_empty());
    assert!(!ningxia.metadata().name.is_empty());
    assert!(!inner_mongolia.metadata().name.is_empty());
}

#[test]
fn phase_20_01_unique_categories() {
    // 验证所有变体有唯一的分类标识
    use world_rules::rules::games::mahjong::variants::{
        HebeiMahjongRules, HunanMahjongRules, InnerMongoliaMahjongRules, NingxiaMahjongRules,
        ShanxiMahjongRules,
    };

    let hunan = HunanMahjongRules::new();
    let hebei = HebeiMahjongRules::new();
    let shanxi = ShanxiMahjongRules::new();
    let ningxia = NingxiaMahjongRules::new();
    let inner_mongolia = InnerMongoliaMahjongRules::new();

    assert_eq!(hunan.category().to_string(), "Games/mahjong_hunan");
    assert_eq!(hebei.category().to_string(), "Games/mahjong_hebei");
    assert_eq!(shanxi.category().to_string(), "Games/mahjong_shanxi");
    assert_eq!(ningxia.category().to_string(), "Games/mahjong_ningxia");
    assert_eq!(
        inner_mongolia.category().to_string(),
        "Games/mahjong_inner_mongolia"
    );
}

#[test]
fn phase_20_01_all_explain_works() {
    // 验证所有变体的 explain 方法都能生成有效说明
    use world_rules::rules::games::mahjong::variants::{
        HebeiMahjongRules, HunanMahjongRules, InnerMongoliaMahjongRules, NingxiaMahjongRules,
        ShanxiMahjongRules,
    };

    let rules_list: Vec<Box<dyn world_rules::rules::core::Rule>> = vec![
        Box::new(HunanMahjongRules::new()),
        Box::new(HebeiMahjongRules::new()),
        Box::new(ShanxiMahjongRules::new()),
        Box::new(NingxiaMahjongRules::new()),
        Box::new(InnerMongoliaMahjongRules::new()),
    ];

    for rules in rules_list {
        let explanation = rules.explain();
        assert!(explanation.contains("基本设置"));
        assert!(explanation.contains("番型规则"));
        assert!(explanation.contains("计分规则"));
    }
}

#[test]
fn phase_20_01_origin_tags() {
    // 验证所有变体的来源标签
    use world_rules::rules::games::mahjong::variants::{
        HebeiMahjongRules, HunanMahjongRules, InnerMongoliaMahjongRules, NingxiaMahjongRules,
        ShanxiMahjongRules,
    };

    assert_eq!(HunanMahjongRules::new().metadata().origin, Some("湖南"));
    assert_eq!(HebeiMahjongRules::new().metadata().origin, Some("河北"));
    assert_eq!(ShanxiMahjongRules::new().metadata().origin, Some("山西"));
    assert_eq!(NingxiaMahjongRules::new().metadata().origin, Some("宁夏"));
    assert_eq!(
        InnerMongoliaMahjongRules::new().metadata().origin,
        Some("内蒙古")
    );
}
