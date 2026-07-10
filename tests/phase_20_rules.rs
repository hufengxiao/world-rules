//! Phase 20 新规则集成测试
//!
//! 测试 Phase 20 添加的麻将变体规则：
//! - 20-01: 5种中国麻将变体（湖南、河北、山西、宁夏、内蒙古）
//! - 20-02: 5种日本麻将变体（竞技立直、和志、三人、关西、开放立直）
//! - 20-03: 5种其他麻将变体（美国、越南、菲律宾、新加坡、马来西亚）

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

// ===== Phase 20-02: 日本麻将变体规则测试 =====

#[test]
fn riichi_competitive_rules_basic() {
    use world_rules::rules::games::mahjong::variants::riichi_competitive::RiichiCompetitiveRules;

    let rules = RiichiCompetitiveRules::new();
    assert_eq!(rules.metadata().name, "日本立直麻将竞技规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn riichi_competitive_yakuman_rules() {
    use world_rules::rules::games::mahjong::variants::riichi_competitive::RiichiCompetitiveRules;

    let rules = RiichiCompetitiveRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("天和"), "应说明天和役满");
    assert!(explanation.contains("大三元"), "应说明大三元役满");
    assert!(explanation.contains("四暗刻"), "应说明四暗刻役满");
}

#[test]
fn riichi_competitive_yaku_types() {
    use world_rules::rules::games::mahjong::variants::riichi_competitive::RiichiCompetitiveRules;

    let rules = RiichiCompetitiveRules::new();
    let yaku = rules.competitive_yaku();

    assert!(!yaku.is_empty());
    assert!(yaku.iter().any(|(name, _)| name == "立直"));
    assert!(yaku.iter().any(|(name, _)| name == "断幺九"));
}

#[test]
fn washizu_rules_basic() {
    use world_rules::rules::games::mahjong::variants::washizu::WashizuMahjongRules;

    let rules = WashizuMahjongRules::new();
    assert_eq!(rules.metadata().name, "和志麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn washizu_transparent_rules() {
    use world_rules::rules::games::mahjong::variants::washizu::WashizuMahjongRules;

    let rules = WashizuMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("透明牌"), "应说明透明牌规则");
}

#[test]
fn washizu_special_yaku() {
    use world_rules::rules::games::mahjong::variants::washizu::WashizuMahjongRules;

    let rules = WashizuMahjongRules::new();
    let yaku = rules.special_yaku();

    assert!(!yaku.is_empty());
    assert!(yaku.iter().any(|(name, _)| name.contains("透明")));
}

#[test]
fn sanma_rules_basic() {
    use world_rules::rules::games::mahjong::variants::sanma::SanmaMahjongRules;

    let rules = SanmaMahjongRules::new();
    assert_eq!(rules.metadata().name, "三人麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn sanma_tile_adjustments() {
    use world_rules::rules::games::mahjong::variants::sanma::SanmaMahjongRules;

    let rules = SanmaMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("108张"), "应说明108张牌");
    assert!(explanation.contains("2-8万"), "应说明去除2-8万");
}

#[test]
fn sanma_special_yaku() {
    use world_rules::rules::games::mahjong::variants::sanma::SanmaMahjongRules;

    let rules = SanmaMahjongRules::new();
    let yaku = rules.special_yaku();

    assert!(!yaku.is_empty());
    assert!(yaku.iter().any(|(name, _)| name.contains("三色")));
}

#[test]
fn kansai_rules_basic() {
    use world_rules::rules::games::mahjong::variants::kansai::KansaiMahjongRules;

    let rules = KansaiMahjongRules::new();
    assert_eq!(rules.metadata().name, "关西麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn kansai_local_yaku() {
    use world_rules::rules::games::mahjong::variants::kansai::KansaiMahjongRules;

    let rules = KansaiMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("关西"), "应说明关西特色");
}

#[test]
fn kansai_yaku_types() {
    use world_rules::rules::games::mahjong::variants::kansai::KansaiMahjongRules;

    let rules = KansaiMahjongRules::new();
    let yaku = rules.kansai_yaku();

    assert!(!yaku.is_empty());
    assert!(yaku.iter().any(|(name, _)| name.contains("关西")));
}

#[test]
fn open_riichi_rules_basic() {
    use world_rules::rules::games::mahjong::variants::open_riichi::OpenRiichiMahjongRules;

    let rules = OpenRiichiMahjongRules::new();
    assert_eq!(rules.metadata().name, "开放立直麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn open_riichi_open_rules() {
    use world_rules::rules::games::mahjong::variants::open_riichi::OpenRiichiMahjongRules;

    let rules = OpenRiichiMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("明牌"), "应说明明牌规则");
    assert!(explanation.contains("立直"), "应说明立直规则");
}

#[test]
fn open_riichi_yaku_adjustments() {
    use world_rules::rules::games::mahjong::variants::open_riichi::OpenRiichiMahjongRules;

    let rules = OpenRiichiMahjongRules::new();
    let yaku = rules.yaku_adjustments();

    assert!(!yaku.is_empty());
    assert!(yaku
        .iter()
        .any(|(name, _)| name.contains("明") || name.contains("开放")));
}

// ===== Phase 20-02 总体测试 =====

#[test]
fn phase_20_02_all_japanese_variants_exist() {
    // 验证所有 5 种日本麻将变体都能正确导入和创建
    use world_rules::rules::games::mahjong::variants::{
        KansaiMahjongRules, OpenRiichiMahjongRules, RiichiCompetitiveRules, SanmaMahjongRules,
        WashizuMahjongRules,
    };

    let riichi_competitive = RiichiCompetitiveRules::new();
    let washizu = WashizuMahjongRules::new();
    let sanma = SanmaMahjongRules::new();
    let kansai = KansaiMahjongRules::new();
    let open_riichi = OpenRiichiMahjongRules::new();

    // 所有规则都有有效的元数据
    assert!(!riichi_competitive.metadata().name.is_empty());
    assert!(!washizu.metadata().name.is_empty());
    assert!(!sanma.metadata().name.is_empty());
    assert!(!kansai.metadata().name.is_empty());
    assert!(!open_riichi.metadata().name.is_empty());
}

#[test]
fn phase_20_02_unique_categories() {
    // 验证所有日本麻将变体有唯一的分类标识
    use world_rules::rules::games::mahjong::variants::{
        KansaiMahjongRules, OpenRiichiMahjongRules, RiichiCompetitiveRules, SanmaMahjongRules,
        WashizuMahjongRules,
    };

    let riichi_competitive = RiichiCompetitiveRules::new();
    let washizu = WashizuMahjongRules::new();
    let sanma = SanmaMahjongRules::new();
    let kansai = KansaiMahjongRules::new();
    let open_riichi = OpenRiichiMahjongRules::new();

    assert_eq!(
        riichi_competitive.category().to_string(),
        "Games/mahjong_riichi_competitive"
    );
    assert_eq!(washizu.category().to_string(), "Games/mahjong_washizu");
    assert_eq!(sanma.category().to_string(), "Games/mahjong_sanma");
    assert_eq!(kansai.category().to_string(), "Games/mahjong_kansai");
    assert_eq!(
        open_riichi.category().to_string(),
        "Games/mahjong_open_riichi"
    );
}

#[test]
fn phase_20_02_all_explain_works() {
    // 验证所有日本麻将变体的 explain 方法都能生成有效说明
    use world_rules::rules::games::mahjong::variants::{
        KansaiMahjongRules, OpenRiichiMahjongRules, RiichiCompetitiveRules, SanmaMahjongRules,
        WashizuMahjongRules,
    };

    let rules_list: Vec<Box<dyn world_rules::rules::core::Rule>> = vec![
        Box::new(RiichiCompetitiveRules::new()),
        Box::new(WashizuMahjongRules::new()),
        Box::new(SanmaMahjongRules::new()),
        Box::new(KansaiMahjongRules::new()),
        Box::new(OpenRiichiMahjongRules::new()),
    ];

    for rules in rules_list {
        let explanation = rules.explain();
        assert!(explanation.contains("基本设置"));
    }
}

#[test]
fn phase_20_02_origin_tags() {
    // 验证所有日本麻将变体的来源标签
    use world_rules::rules::games::mahjong::variants::{
        KansaiMahjongRules, OpenRiichiMahjongRules, RiichiCompetitiveRules, SanmaMahjongRules,
        WashizuMahjongRules,
    };

    assert_eq!(
        RiichiCompetitiveRules::new().metadata().origin,
        Some("日本")
    );
    assert_eq!(WashizuMahjongRules::new().metadata().origin, Some("日本"));
    assert_eq!(SanmaMahjongRules::new().metadata().origin, Some("日本"));
    assert_eq!(
        KansaiMahjongRules::new().metadata().origin,
        Some("日本关西")
    );
    assert_eq!(
        OpenRiichiMahjongRules::new().metadata().origin,
        Some("日本")
    );
}

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

// ===== Phase 20-03: 其他麻将变体规则测试 =====

#[test]
fn american_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::american::AmericanMahjongRules;

    let rules = AmericanMahjongRules::new();
    assert_eq!(rules.metadata().name, "美国麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn american_mahjong_joker_rules() {
    use world_rules::rules::games::mahjong::variants::american::AmericanMahjongRules;

    let rules = AmericanMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("Joker"), "应说明Joker牌规则");
    assert!(explanation.contains("152张"), "应说明152张牌");
    assert!(explanation.contains("Charleston"), "应说明Charleston规则");
}

#[test]
fn american_mahjong_scoring_rules() {
    use world_rules::rules::games::mahjong::variants::american::AmericanMahjongRules;

    let rules = AmericanMahjongRules::new();
    let scoring = rules.scoring_rules();

    assert!(!scoring.is_empty());
    assert!(scoring.iter().any(|(name, _)| name == "清一色"));
    assert!(scoring.iter().any(|(name, _)| name.contains("Joker")));
}

#[test]
fn american_mahjong_basic_settings() {
    use world_rules::rules::games::mahjong::variants::american::AmericanMahjongRules;

    let rules = AmericanMahjongRules::new();
    let settings = rules.basic_settings();

    assert!(settings.iter().any(|s| s.contains("152张")));
    assert!(settings.iter().any(|s| s.contains("Joker")));
}

#[test]
fn vietnamese_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::vietnamese::VietnameseMahjongRules;

    let rules = VietnameseMahjongRules::new();
    assert_eq!(rules.metadata().name, "越南麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn vietnamese_mahjong_16_cards() {
    use world_rules::rules::games::mahjong::variants::vietnamese::VietnameseMahjongRules;

    let rules = VietnameseMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("16张"), "应说明16张牌");
    assert!(explanation.contains("17张"), "应说明17张胡牌");
}

#[test]
fn vietnamese_mahjong_scoring() {
    use world_rules::rules::games::mahjong::variants::vietnamese::VietnameseMahjongRules;

    let rules = VietnameseMahjongRules::new();
    let scoring = rules.scoring_rules();

    assert!(!scoring.is_empty());
    assert!(scoring.iter().any(|(name, _)| name == "清一色"));
    assert!(scoring.iter().any(|(name, _)| name == "十三幺"));
}

#[test]
fn filipino_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::filipino::FilipinoMahjongRules;

    let rules = FilipinoMahjongRules::new();
    assert_eq!(rules.metadata().name, "菲律宾麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn filipino_mahjong_simplified_scoring() {
    use world_rules::rules::games::mahjong::variants::filipino::FilipinoMahjongRules;

    let rules = FilipinoMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("简化"), "应说明简化计分");
    assert!(explanation.contains("144张"), "应说明144张牌");
}

#[test]
fn filipino_mahjong_scoring_rules() {
    use world_rules::rules::games::mahjong::variants::filipino::FilipinoMahjongRules;

    let rules = FilipinoMahjongRules::new();
    let scoring = rules.simplified_scoring();

    assert!(!scoring.is_empty());
    assert!(scoring.iter().any(|(name, _)| name == "平胡"));
    assert!(scoring.iter().any(|(name, _)| name == "清一色"));
}

#[test]
fn singapore_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::singapore::SingaporeMahjongRules;

    let rules = SingaporeMahjongRules::new();
    assert_eq!(rules.metadata().name, "新加坡麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn singapore_mahjong_animal_cards() {
    use world_rules::rules::games::mahjong::variants::singapore::SingaporeMahjongRules;

    let rules = SingaporeMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("动物牌"), "应说明动物牌规则");
    assert!(explanation.contains("148张"), "应说明148张牌");
}

#[test]
fn singapore_mahjong_animal_scoring() {
    use world_rules::rules::games::mahjong::variants::singapore::SingaporeMahjongRules;

    let rules = SingaporeMahjongRules::new();
    let scoring = rules.scoring_rules();

    assert!(!scoring.is_empty());
    assert!(scoring.iter().any(|(name, _)| name.contains("动物")));
}

#[test]
fn malaysian_mahjong_rules_basic() {
    use world_rules::rules::games::mahjong::variants::malaysian::MalaysianMahjongRules;

    let rules = MalaysianMahjongRules::new();
    assert_eq!(rules.metadata().name, "马来西亚麻将规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn malaysian_mahjong_animal_cards() {
    use world_rules::rules::games::mahjong::variants::malaysian::MalaysianMahjongRules;

    let rules = MalaysianMahjongRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("动物牌"), "应说明动物牌规则");
    assert!(explanation.contains("148张"), "应说明148张牌");
}

#[test]
fn malaysian_mahjong_special_fans() {
    use world_rules::rules::games::mahjong::variants::malaysian::MalaysianMahjongRules;

    let rules = MalaysianMahjongRules::new();
    let fans = rules.special_fan_types();

    assert!(!fans.is_empty());
    assert!(fans.iter().any(|(name, _)| name.contains("动物")));
    assert!(fans.iter().any(|(name, _)| name == "清一色"));
}

#[test]
fn malaysian_mahjong_animal_scoring() {
    use world_rules::rules::games::mahjong::variants::malaysian::MalaysianMahjongRules;

    let rules = MalaysianMahjongRules::new();
    let fans = rules.special_fan_types();

    // 验证动物牌番种计分
    let full_animal = fans.iter().find(|(name, _)| name == "全动物");
    assert!(full_animal.is_some());
    assert_eq!(full_animal.unwrap().1, 8);
}

// ===== Phase 20-03 总体测试 =====

#[test]
fn phase_20_03_all_other_variants_exist() {
    // 验证所有 5 种其他麻将变体都能正确导入和创建
    use world_rules::rules::games::mahjong::variants::{
        AmericanMahjongRules, FilipinoMahjongRules, MalaysianMahjongRules, SingaporeMahjongRules,
        VietnameseMahjongRules,
    };

    let american = AmericanMahjongRules::new();
    let vietnamese = VietnameseMahjongRules::new();
    let filipino = FilipinoMahjongRules::new();
    let singapore = SingaporeMahjongRules::new();
    let malaysian = MalaysianMahjongRules::new();

    // 所有规则都有有效的元数据
    assert!(!american.metadata().name.is_empty());
    assert!(!vietnamese.metadata().name.is_empty());
    assert!(!filipino.metadata().name.is_empty());
    assert!(!singapore.metadata().name.is_empty());
    assert!(!malaysian.metadata().name.is_empty());
}

#[test]
fn phase_20_03_unique_categories() {
    // 验证所有其他麻将变体有唯一的分类标识
    use world_rules::rules::games::mahjong::variants::{
        AmericanMahjongRules, FilipinoMahjongRules, MalaysianMahjongRules, SingaporeMahjongRules,
        VietnameseMahjongRules,
    };

    let american = AmericanMahjongRules::new();
    let vietnamese = VietnameseMahjongRules::new();
    let filipino = FilipinoMahjongRules::new();
    let singapore = SingaporeMahjongRules::new();
    let malaysian = MalaysianMahjongRules::new();

    assert_eq!(american.category().to_string(), "Games/mahjong_american");
    assert_eq!(
        vietnamese.category().to_string(),
        "Games/mahjong_vietnamese"
    );
    assert_eq!(filipino.category().to_string(), "Games/mahjong_filipino");
    assert_eq!(singapore.category().to_string(), "Games/mahjong_singapore");
    assert_eq!(malaysian.category().to_string(), "Games/mahjong_malaysian");
}

#[test]
fn phase_20_03_all_explain_works() {
    // 验证所有其他麻将变体的 explain 方法都能生成有效说明
    use world_rules::rules::games::mahjong::variants::{
        AmericanMahjongRules, FilipinoMahjongRules, MalaysianMahjongRules, SingaporeMahjongRules,
        VietnameseMahjongRules,
    };

    let rules_list: Vec<Box<dyn world_rules::rules::core::Rule>> = vec![
        Box::new(AmericanMahjongRules::new()),
        Box::new(VietnameseMahjongRules::new()),
        Box::new(FilipinoMahjongRules::new()),
        Box::new(SingaporeMahjongRules::new()),
        Box::new(MalaysianMahjongRules::new()),
    ];

    for rules in rules_list {
        let explanation = rules.explain();
        assert!(explanation.contains("基本设置"));
    }
}

#[test]
fn phase_20_03_origin_tags() {
    // 验证所有其他麻将变体的来源标签
    use world_rules::rules::games::mahjong::variants::{
        AmericanMahjongRules, FilipinoMahjongRules, MalaysianMahjongRules, SingaporeMahjongRules,
        VietnameseMahjongRules,
    };

    assert_eq!(AmericanMahjongRules::new().metadata().origin, Some("美国"));
    assert_eq!(
        VietnameseMahjongRules::new().metadata().origin,
        Some("越南")
    );
    assert_eq!(
        FilipinoMahjongRules::new().metadata().origin,
        Some("菲律宾")
    );
    assert_eq!(
        SingaporeMahjongRules::new().metadata().origin,
        Some("新加坡")
    );
    assert_eq!(
        MalaysianMahjongRules::new().metadata().origin,
        Some("马来西亚")
    );
}
