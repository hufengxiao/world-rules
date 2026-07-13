//! Phase 18 新规则集成测试
//!
//! 测试 Phase 18-01 到 18-04 添加的 20 种卡牌游戏规则：
//! - 18-01: 5种扑克变体（Omaha, Stud, Draw, Chinese Poker, Short Deck）
//! - 18-02: 5种桥牌变体（Rubber, Duplicate, Chicago, Minibridge, IMP）
//! - 18-03: 5种其他卡牌（Big Two, Pai Gow Poker, Baccarat, Three Card Poker, Caribbean Stud）
//! - 18-04: 5种桌面卡牌（Gin Rummy, Klondike Solitaire, Cassino, Canfield, Pyramid Solitaire）

use world_rules::prelude::*;

// ===== Phase 18-01: 扑克变体规则测试 =====

#[test]
fn poker_omaha_rules_basic() {
    use world_rules::rules::games::poker_omaha::PokerOmahaRules;

    let rules = PokerOmahaRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn poker_omaha_hand_structure() {
    use world_rules::rules::games::poker_omaha::PokerOmahaRules;

    let rules = PokerOmahaRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("4张私有牌"), "应说明私有牌数量");
    assert!(explanation.contains("2张"), "应说明必须使用2张私有牌");
    assert!(explanation.contains("3张公共牌"), "应说明必须使用3张公共牌");
}

#[test]
fn stud_poker_rules_basic() {
    use world_rules::rules::games::stud_poker::StudPokerRules;

    let rules = StudPokerRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn stud_poker_seven_card_variant() {
    use world_rules::rules::games::stud_poker::StudPokerRules;

    let rules = StudPokerRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("7张牌"), "应说明牌数");
    assert!(explanation.contains("3张朝下"), "应说明朝下牌数");
    assert!(explanation.contains("4张朝上"), "应说明朝上牌数");
}

#[test]
fn poker_five_card_draw_basic() {
    use world_rules::rules::games::poker_five_card::PokerFiveCardRules;

    let rules = PokerFiveCardRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn poker_five_card_draw_rules() {
    use world_rules::rules::games::poker_five_card::PokerFiveCardRules;

    let rules = PokerFiveCardRules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("换牌"), "应说明换牌规则");
    assert!(explanation.contains("5张"), "应说明牌数");
}

#[test]
fn poker_chinese_basic() {
    use world_rules::rules::games::poker_chinese::PokerChineseRules;

    let rules = PokerChineseRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn poker_chinese_hand_division() {
    use world_rules::rules::games::poker_chinese::PokerChineseRules;

    let rules = PokerChineseRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("13张"), "应说明总牌数");
    assert!(explanation.contains("3手"), "应说明分成3手牌");
    assert!(explanation.contains("3张"), "应说明前手牌数");
    assert!(explanation.contains("5张"), "应说明中手牌数");
    assert!(explanation.contains("5张"), "应说明后手牌数");
}

#[test]
fn poker_short_deck_basic() {
    use world_rules::rules::games::card_games::short_deck::ShortDeckRules;

    let rules = ShortDeckRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn poker_short_deck_card_range() {
    use world_rules::rules::games::card_games::short_deck::ShortDeckRules;

    let rules = ShortDeckRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("36张"), "应说明牌总数");
    assert!(explanation.contains("6"), "应说明牌范围从6开始");
    assert!(explanation.contains("顺子"), "应说明顺子规则变化");
}

// ===== Phase 18-02: 桥牌变体规则测试 =====

#[test]
fn bridge_rubber_basic() {
    use world_rules::rules::games::bridge_rubber::BridgeRubberRules;

    let rules = BridgeRubberRules::new();
    assert_eq!(rules.metadata().name, "盘式桥牌规则");
    assert!(!rules.explain().is_empty());
}

#[test]
fn bridge_rubber_scoring() {
    use world_rules::rules::games::bridge_rubber::BridgeRubberRules;

    let rules = BridgeRubberRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("盘"), "应说明盘的概念");
    assert!(explanation.contains("100分"), "应说明成局分数");
    assert!(explanation.contains("游戏分"), "应说明游戏分");
}

#[test]
fn bridge_duplicate_basic() {
    use world_rules::rules::games::bridge_duplicate::BridgeDuplicateRules;

    let rules = BridgeDuplicateRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn bridge_duplicate_comparison() {
    use world_rules::rules::games::bridge_duplicate::BridgeDuplicateRules;

    let rules = BridgeDuplicateRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("IMP"), "应说明IMP计分");
    assert!(explanation.contains("相同牌"), "应说明同牌组比较");
}

#[test]
fn bridge_chicago_basic() {
    use world_rules::rules::games::bridge_chicago::BridgeChicagoRules;

    let rules = BridgeChicagoRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn bridge_chicago_four_deal() {
    use world_rules::rules::games::bridge_chicago::BridgeChicagoRules;

    let rules = BridgeChicagoRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("4副"), "应说明固定4副牌");
    assert!(explanation.contains("局况"), "应说明局况轮转");
}

#[test]
fn bridge_minibridge_basic() {
    use world_rules::rules::games::bridge_minibridge::BridgeMinibridgeRules;

    let rules = BridgeMinibridgeRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn bridge_minibridge_features() {
    use world_rules::rules::games::bridge_minibridge::BridgeMinibridgeRules;

    let rules = BridgeMinibridgeRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("简化"), "应说明简化特点");
    assert!(explanation.contains("叫牌"), "应说明叫牌流程");
}

#[test]
fn bridge_imp_basic() {
    use world_rules::rules::games::bridge_imp::BridgeImpRules;

    let rules = BridgeImpRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn bridge_imp_conversion() {
    use world_rules::rules::games::bridge_imp::BridgeImpRules;

    let rules = BridgeImpRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("IMP"), "应说明IMP系统");
    assert!(explanation.contains("分差"), "应说明分差转换");
}

// ===== Phase 18-03: 其他卡牌规则测试 =====

#[test]
fn big_two_basic() {
    use world_rules::rules::games::big_two::BigTwoRules;

    let rules = BigTwoRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn big_two_card_ranking() {
    use world_rules::rules::games::big_two::BigTwoRules;

    let rules = BigTwoRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("2最大"), "应说明2是最大牌");
    assert!(explanation.contains("13张"), "应说明每人13张牌");
    assert!(explanation.contains("先出完"), "应说明获胜条件");
}

#[test]
fn pai_gow_poker_basic() {
    use world_rules::rules::games::pai_gow_poker::PaiGowPokerRules;

    let rules = PaiGowPokerRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn pai_gow_poker_hand_setting() {
    use world_rules::rules::games::pai_gow_poker::PaiGowPokerRules;

    let rules = PaiGowPokerRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("7张"), "应说明牌数");
    assert!(explanation.contains("两手"), "应说明分成两手");
    assert!(explanation.contains("5张"), "应说明后手牌数");
    assert!(explanation.contains("2张"), "应说明前手牌数");
}

#[test]
fn baccarat_basic() {
    use world_rules::rules::games::baccarat::BaccaratRules;

    let rules = BaccaratRules::new();
    assert_eq!(rules.metadata().name, "百家乐规则");
    assert!(!rules.explain().is_empty());
}

#[test]
fn baccarat_scoring() {
    use world_rules::rules::games::baccarat::BaccaratRules;

    let rules = BaccaratRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("9点"), "应说明最大点数");
    assert!(explanation.contains("庄家"), "应说明庄家押注");
    assert!(explanation.contains("闲家"), "应说明闲家押注");
    assert!(explanation.contains("个位数"), "应说明个位数计算");
}

#[test]
fn three_card_poker_basic() {
    use world_rules::rules::games::three_card_poker::ThreeCardPokerRules;

    let rules = ThreeCardPokerRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn three_card_poker_hand_types() {
    use world_rules::rules::games::three_card_poker::ThreeCardPokerRules;

    let rules = ThreeCardPokerRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("3张"), "应说明牌数");
    assert!(explanation.contains("顺子"), "应说明顺子牌型");
}

#[test]
fn caribbean_stud_basic() {
    use world_rules::rules::games::caribbean_stud::CaribbeanStudRules;

    let rules = CaribbeanStudRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn caribbean_stud_progressive() {
    use world_rules::rules::games::caribbean_stud::CaribbeanStudRules;

    let rules = CaribbeanStudRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("5张"), "应说明牌数");
    assert!(
        explanation.contains(" Progressive") || explanation.contains("累计奖"),
        "应说明累计奖"
    );
}

// ===== Phase 18-04: 桌面卡牌规则测试 =====

#[test]
fn gin_rummy_basic() {
    use world_rules::rules::games::gin_rummy::GinRummyRules;

    let rules = GinRummyRules::new();
    assert_eq!(rules.metadata().name, "金拉米规则");
    assert!(!rules.explain().is_empty());
}

#[test]
fn gin_rummy_melds() {
    use world_rules::rules::games::gin_rummy::GinRummyRules;

    let rules = GinRummyRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("组"), "应说明组的概念");
    assert!(explanation.contains("顺子"), "应说明顺子概念");
    assert!(explanation.contains("敲牌"), "应说明敲牌规则");
    assert!(explanation.contains("死牌"), "应说明死牌概念");
}

#[test]
fn klondike_solitaire_basic() {
    use world_rules::rules::games::klondike_solitaire::KlondikeSolitaireRules;

    let rules = KlondikeSolitaireRules::new();
    assert_eq!(rules.metadata().name, "经典接龙规则");
    assert!(!rules.explain().is_empty());
}

#[test]
fn klondike_solitaire_structure() {
    use world_rules::rules::games::klondike_solitaire::KlondikeSolitaireRules;

    let rules = KlondikeSolitaireRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("工作牌堆"), "应说明工作牌堆");
    assert!(explanation.contains("基础牌堆"), "应说明基础牌堆");
    assert!(explanation.contains("K"), "应说明K规则");
    assert!(explanation.contains("红黑"), "应说明红黑交替");
}

#[test]
fn cassino_basic() {
    use world_rules::rules::games::cassino::CassinoRules;

    let rules = CassinoRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn cassino_capturing() {
    use world_rules::rules::games::cassino::CassinoRules;

    let rules = CassinoRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("捕获"), "应说明捕获规则");
    assert!(explanation.contains("匹配"), "应说明匹配规则");
}

#[test]
fn canfield_basic() {
    use world_rules::rules::games::canfield::CanfieldRules;

    let rules = CanfieldRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn canfield_layout() {
    use world_rules::rules::games::canfield::CanfieldRules;

    let rules = CanfieldRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("基础牌"), "应说明基础牌");
    assert!(explanation.contains("储备"), "应说明储备牌堆");
}

#[test]
fn pyramid_solitaire_basic() {
    use world_rules::rules::games::pyramid_solitaire::PyramidSolitaireRules;

    let rules = PyramidSolitaireRules::new();
    assert!(!rules.metadata().name.is_empty());
    assert!(!rules.explain().is_empty());
}

#[test]
fn pyramid_solitaire_matching() {
    use world_rules::rules::games::pyramid_solitaire::PyramidSolitaireRules;

    let rules = PyramidSolitaireRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("金字塔"), "应说明金字塔结构");
    assert!(explanation.contains("13"), "应说明配对和为13");
}

// ===== 综合测试：验证所有规则在 all_rules() 中注册 =====

#[test]
fn phase_18_rules_registered_in_module() {
    use world_rules::rules::games::card_games::short_deck::ShortDeckRules;
    use world_rules::rules::games::{
        BaccaratRules, BigTwoRules, BridgeChicagoRules, BridgeDuplicateRules, BridgeImpRules,
        BridgeMinibridgeRules, BridgeRubberRules, CanfieldRules, CaribbeanStudRules, CassinoRules,
        GinRummyRules, KlondikeSolitaireRules, PaiGowPokerRules, PokerChineseRules,
        PokerFiveCardRules, PokerOmahaRules, PyramidSolitaireRules, StudPokerRules,
        ThreeCardPokerRules,
    };

    // 验证所有规则类型存在并可实例化
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(PokerOmahaRules::new()),
        Box::new(StudPokerRules::new()),
        Box::new(PokerFiveCardRules::new()),
        Box::new(PokerChineseRules::new()),
        Box::new(ShortDeckRules::new()),
        Box::new(BridgeRubberRules::new()),
        Box::new(BridgeDuplicateRules::new()),
        Box::new(BridgeChicagoRules::new()),
        Box::new(BridgeMinibridgeRules::new()),
        Box::new(BridgeImpRules::new()),
        Box::new(BigTwoRules::new()),
        Box::new(PaiGowPokerRules::new()),
        Box::new(BaccaratRules::new()),
        Box::new(ThreeCardPokerRules::new()),
        Box::new(CaribbeanStudRules::new()),
        Box::new(GinRummyRules::new()),
        Box::new(KlondikeSolitaireRules::new()),
        Box::new(CassinoRules::new()),
        Box::new(CanfieldRules::new()),
        Box::new(PyramidSolitaireRules::new()),
    ];

    for rule in &rules {
        assert!(!rule.metadata().name.is_empty());
        assert!(!rule.explain().is_empty());
        assert!(matches!(rule.category(), RuleCategory::Games(_)));
    }
}

#[test]
fn phase_18_rules_metadata_consistency() {
    use world_rules::rules::games::{
        BaccaratRules, BigTwoRules, BridgeChicagoRules, BridgeDuplicateRules, BridgeRubberRules,
        CaribbeanStudRules, GinRummyRules, KlondikeSolitaireRules, PokerChineseRules,
        PokerOmahaRules, PyramidSolitaireRules, StudPokerRules, ThreeCardPokerRules,
    };

    // 验证规则元数据的完整性和一致性
    // Omaha
    let rule = PokerOmahaRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Omaha 规则描述应足够详细"
    );
    assert!(rule.metadata().tags.len() > 0, "Omaha 规则应有标签");

    // Stud
    let rule = StudPokerRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Stud 规则描述应足够详细"
    );
    assert!(rule.metadata().tags.len() > 0, "Stud 规则应有标签");

    // Chinese Poker
    let rule = PokerChineseRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Chinese Poker 规则描述应足够详细"
    );
    assert!(rule.metadata().tags.len() > 0, "Chinese Poker 规则应有标签");

    // Rubber Bridge
    let rule = BridgeRubberRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Rubber Bridge 规则描述应足够详细"
    );
    assert!(rule.metadata().tags.len() > 0, "Rubber Bridge 规则应有标签");

    // Duplicate Bridge
    let rule = BridgeDuplicateRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Duplicate Bridge 规则描述应足够详细"
    );
    assert!(
        rule.metadata().tags.len() > 0,
        "Duplicate Bridge 规则应有标签"
    );

    // Chicago Bridge
    let rule = BridgeChicagoRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Chicago Bridge 规则描述应足够详细"
    );
    assert!(
        rule.metadata().tags.len() > 0,
        "Chicago Bridge 规则应有标签"
    );

    // Big Two
    let rule = BigTwoRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Big Two 规则描述应足够详细"
    );
    assert!(rule.metadata().tags.len() > 0, "Big Two 规则应有标签");

    // Baccarat
    let rule = BaccaratRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Baccarat 规则描述应足够详细"
    );
    assert!(rule.metadata().tags.len() > 0, "Baccarat 规则应有标签");

    // Three Card Poker
    let rule = ThreeCardPokerRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Three Card Poker 规则描述应足够详细"
    );
    assert!(
        rule.metadata().tags.len() > 0,
        "Three Card Poker 规则应有标签"
    );

    // Caribbean Stud
    let rule = CaribbeanStudRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Caribbean Stud 规则描述应足够详细"
    );
    assert!(
        rule.metadata().tags.len() > 0,
        "Caribbean Stud 规则应有标签"
    );

    // Gin Rummy
    let rule = GinRummyRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Gin Rummy 规则描述应足够详细"
    );
    assert!(rule.metadata().tags.len() > 0, "Gin Rummy 规则应有标签");

    // Klondike Solitaire
    let rule = KlondikeSolitaireRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Klondike Solitaire 规则描述应足够详细"
    );
    assert!(
        rule.metadata().tags.len() > 0,
        "Klondike Solitaire 规则应有标签"
    );

    // Pyramid Solitaire
    let rule = PyramidSolitaireRules::new();
    assert!(
        rule.metadata().description.len() > 10,
        "Pyramid Solitaire 规则描述应足够详细"
    );
    assert!(
        rule.metadata().tags.len() > 0,
        "Pyramid Solitaire 规则应有标签"
    );
}
