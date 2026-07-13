//! Phase 19 新规则集成测试
//!
//! 测试 Phase 19 添加的棋类和桌游规则：
//! - 19-01: 5种象棋变体（迷你象棋、四国象棋、暗棋、查图兰加、盲棋）
//! - 19-02: 5种围棋变体（9路盘、13路盘、盲围棋、一色围棋、联棋）
//! - 19-03: 5种其他棋类（朝鲜将棋、泰国象棋、斗兽棋、播棋、迷你将棋）
//! - 19-04: 5种桌游（Agricola, Carcassonne, Dominion, Power Grid, Puerto Rico）

use world_rules::prelude::*;

// ===== Phase 19-01: 象棋变体规则测试 =====
// 已在之前的提交中完成

// ===== Phase 19-02: 围棋变体规则测试 =====

#[test]
fn go_9x9_rules_basic() {
    use world_rules::rules::games::go_9x9::Go9x9Rules;

    let rules = Go9x9Rules::new();
    assert_eq!(rules.metadata().name, "围棋9路盘规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn go_9x9_board_size() {
    use world_rules::rules::games::go_9x9::Go9x9Rules;

    let rules = Go9x9Rules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("9×9"), "应说明棋盘大小");
    assert!(explanation.contains("81"), "应说明交叉点数");
    assert!(explanation.contains("5.5目"), "应说明贴目值");
}

#[test]
fn go_9x9_komi() {
    use world_rules::rules::games::go_9x9::Go9x9Rules;

    let rules = Go9x9Rules::new();
    assert_eq!(rules.komi(), 5.5);
}

#[test]
fn go_9x9_strategy() {
    use world_rules::rules::games::go_9x9::Go9x9Rules;

    let rules = Go9x9Rules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("入门"), "应说明入门特点");
    assert!(explanation.contains("快"), "应说明快节奏");
}

#[test]
fn go_13x13_rules_basic() {
    use world_rules::rules::games::go_13x13::Go13x13Rules;

    let rules = Go13x13Rules::new();
    assert_eq!(rules.metadata().name, "围棋13路盘规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn go_13x13_board_size() {
    use world_rules::rules::games::go_13x13::Go13x13Rules;

    let rules = Go13x13Rules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("13×13"), "应说明棋盘大小");
    assert!(explanation.contains("169"), "应说明交叉点数");
}

#[test]
fn go_13x13_komi() {
    use world_rules::rules::games::go_13x13::Go13x13Rules;

    let rules = Go13x13Rules::new();
    assert_eq!(rules.komi(), 5.5);
}

#[test]
fn go_13x13_transition() {
    use world_rules::rules::games::go_13x13::Go13x13Rules;

    let rules = Go13x13Rules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("过渡"), "应说明过渡特点");
}

#[test]
fn blind_go_rules_basic() {
    use world_rules::rules::games::blind_go::BlindGoRules;

    let rules = BlindGoRules::new();
    assert_eq!(rules.metadata().name, "盲围棋规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn blind_go_mechanics() {
    use world_rules::rules::games::blind_go::BlindGoRules;

    let rules = BlindGoRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("19×19"), "应说明使用标准棋盘");
    assert!(explanation.contains("记忆"), "应说明记忆挑战");
    assert!(explanation.contains("坐标"), "应说明坐标报点");
}

#[test]
fn blind_go_challenge_levels() {
    use world_rules::rules::games::blind_go::BlindGoRules;

    let rules = BlindGoRules::new();
    let explanation = rules.explain();

    assert!(
        explanation.contains("普通") || explanation.contains("完全"),
        "应说明挑战等级"
    );
    assert!(explanation.contains("大师"), "应说明大师级挑战");
}

#[test]
fn blind_go_special_rules() {
    use world_rules::rules::games::blind_go::BlindGoRules;

    let rules = BlindGoRules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("非法落子"), "应说明非法落子处理");
    assert!(explanation.contains("pass"), "应说明pass规则");
}

#[test]
fn one_color_go_rules_basic() {
    use world_rules::rules::games::one_color_go::OneColorGoRules;

    let rules = OneColorGoRules::new();
    assert_eq!(rules.metadata().name, "一色围棋规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn one_color_go_mechanics() {
    use world_rules::rules::games::one_color_go::OneColorGoRules;

    let rules = OneColorGoRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("同色"), "应说明使用同色棋子");
    assert!(explanation.contains("记忆"), "应说明记忆要求");
}

#[test]
fn one_color_go_distinguishing() {
    use world_rules::rules::games::one_color_go::OneColorGoRules;

    let rules = OneColorGoRules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("落子顺序"), "应说明用落子顺序区分");
    assert!(explanation.contains("记录"), "应说明记录的重要性");
}

#[test]
fn one_color_go_training_value() {
    use world_rules::rules::games::one_color_go::OneColorGoRules;

    let rules = OneColorGoRules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("记忆"), "应说明训练记忆力");
    assert!(explanation.contains("推理"), "应说明训练推理能力");
}

#[test]
fn pair_go_rules_basic() {
    use world_rules::rules::games::pair_go::PairGoRules;

    let rules = PairGoRules::new();
    assert_eq!(rules.metadata().name, "联棋规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn pair_go_team_structure() {
    use world_rules::rules::games::pair_go::PairGoRules;

    let rules = PairGoRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("四人"), "应说明四人参赛");
    assert!(explanation.contains("团队"), "应说明团队对弈");
    assert!(explanation.contains("两队"), "应说明分两队");
}

#[test]
fn pair_go_turn_order() {
    use world_rules::rules::games::pair_go::PairGoRules;

    let rules = PairGoRules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("轮流"), "应说明轮流落子");
    assert!(explanation.contains("顺序"), "应说明顺序固定");
}

#[test]
fn pair_go_communication_rules() {
    use world_rules::rules::games::pair_go::PairGoRules;

    let rules = PairGoRules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("禁止"), "应说明禁止交流");
    assert!(explanation.contains("交流"), "应说明交流规则");
}

#[test]
fn pair_go_competition() {
    use world_rules::rules::games::pair_go::PairGoRules;

    let rules = PairGoRules::new();
    let explanation = rules.explain();

    assert!(explanation.contains("混双"), "应说明混双赛形式");
    assert!(
        explanation.contains("世界") || explanation.contains("国际"),
        "应说明国际赛事"
    );
}

// ===== 综合测试：验证所有规则在模块中注册 =====

#[test]
fn phase_19_go_variants_registered_in_module() {
    use world_rules::rules::core::Rule;
    use world_rules::rules::games::{
        BlindGoRules, Go13x13Rules, Go9x9Rules, OneColorGoRules, PairGoRules,
    };

    // 验证所有规则类型存在并可实例化
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(Go9x9Rules::new()),
        Box::new(Go13x13Rules::new()),
        Box::new(BlindGoRules::new()),
        Box::new(OneColorGoRules::new()),
        Box::new(PairGoRules::new()),
    ];

    for rule in &rules {
        assert!(!rule.metadata().name.is_empty());
        assert!(!rule.explain().is_empty());
        assert!(matches!(rule.category(), RuleCategory::Games(_)));
    }
}

#[test]
fn phase_19_all_go_variants_have_unique_categories() {
    use world_rules::rules::core::Rule;
    use world_rules::rules::games::{
        BlindGoRules, Go13x13Rules, Go9x9Rules, OneColorGoRules, PairGoRules,
    };

    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(Go9x9Rules::new()),
        Box::new(Go13x13Rules::new()),
        Box::new(BlindGoRules::new()),
        Box::new(OneColorGoRules::new()),
        Box::new(PairGoRules::new()),
    ];

    // 确保每个规则有不同的名称
    let names: Vec<_> = rules.iter().map(|r| r.metadata().name.clone()).collect();
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            assert_ne!(names[i], names[j], "规则名称应该唯一");
        }
    }
}

#[test]
fn phase_19_all_go_variants_explain_non_empty() {
    use world_rules::rules::core::Rule;
    use world_rules::rules::games::{
        BlindGoRules, Go13x13Rules, Go9x9Rules, OneColorGoRules, PairGoRules,
    };

    // 每个规则的说明内容应该至少200字符
    for rule in [
        &Go9x9Rules::new(),
        &Go13x13Rules::new() as &dyn Rule,
        &BlindGoRules::new(),
        &OneColorGoRules::new(),
        &PairGoRules::new(),
    ] {
        let explanation = rule.explain();
        assert!(explanation.len() >= 200, "规则说明应该详细（至少200字符）");
    }
}

// ===== Phase 19-03: 其他棋类规则测试 =====

#[test]
fn janggi_rules_basic() {
    use world_rules::rules::games::janggi::JanggiRules;

    let rules = JanggiRules::new();
    assert_eq!(rules.metadata().name, "朝鲜将棋规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn janggi_board_and_pieces() {
    use world_rules::rules::games::janggi::JanggiRules;

    let rules = JanggiRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("朝鲜"), "应说明起源朝鲜");
    assert!(explanation.contains("9x10"), "应说明棋盘大小");
    assert!(explanation.contains("宫"), "应说明宫殿区域");
}

#[test]
fn janggi_special_rules() {
    use world_rules::rules::games::janggi::JanggiRules;

    let rules = JanggiRules::new();
    let explanation = rules.explain();

    // 验证特殊规则被说明
    assert!(explanation.contains("炮"), "应说明炮的规则");
    assert!(explanation.contains("象"), "应说明象的规则");
    assert!(
        explanation.contains("兵") || explanation.contains("卒"),
        "应说明兵卒规则"
    );
}

#[test]
fn makruk_rules_basic() {
    use world_rules::rules::games::makruk::MakrukRules;

    let rules = MakrukRules::new();
    assert_eq!(rules.metadata().name, "泰国象棋规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn makruk_board_and_pieces() {
    use world_rules::rules::games::makruk::MakrukRules;

    let rules = MakrukRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("泰国"), "应说明起源泰国");
    assert!(explanation.contains("8x8"), "应说明棋盘大小");
    assert!(
        explanation.contains("后") || explanation.contains("Met"),
        "应说明后的走法"
    );
}

#[test]
fn makruk_special_rules() {
    use world_rules::rules::games::makruk::MakrukRules;

    let rules = MakrukRules::new();
    let explanation = rules.explain();

    // 验证特殊规则被说明（无王车易位等）
    assert!(explanation.contains("升变"), "应说明兵升变规则");
    assert!(
        explanation.contains("计时") || explanation.contains("计数"),
        "应说明计时规则"
    );
}

#[test]
fn jungle_rules_basic() {
    use world_rules::rules::games::jungle::JungleRules;

    let rules = JungleRules::new();
    assert_eq!(rules.metadata().name, "斗兽棋规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn jungle_pieces_hierarchy() {
    use world_rules::rules::games::jungle::JungleRules;

    let rules = JungleRules::new();
    let explanation = rules.explain();

    // 验证动物等级被说明
    assert!(explanation.contains("象"), "应说明象是最强");
    assert!(explanation.contains("鼠"), "应说明鼠最弱但可吃象");
    assert!(
        explanation.contains("狮") || explanation.contains("虎"),
        "应说明狮虎"
    );
}

#[test]
fn jungle_special_terrain() {
    use world_rules::rules::games::jungle::JungleRules;

    let rules = JungleRules::new();
    let explanation = rules.explain();

    // 验证特殊地形被说明
    assert!(
        explanation.contains("河") || explanation.contains("水"),
        "应说明河流规则"
    );
    assert!(explanation.contains("陷阱"), "应说明陷阱规则");
    assert!(
        explanation.contains("穴") || explanation.contains("兽穴"),
        "应说明兽穴规则"
    );
}

#[test]
fn mancala_rules_basic() {
    use world_rules::rules::games::mancala::MancalaRules;

    let rules = MancalaRules::new();
    assert_eq!(rules.metadata().name, "播棋规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn mancala_mechanics() {
    use world_rules::rules::games::mancala::MancalaRules;

    let rules = MancalaRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("非洲"), "应说明起源非洲");
    assert!(explanation.contains("种子"), "应说明种子");
    assert!(explanation.contains("坑"), "应说明坑");
}

#[test]
fn mancala_capture_rules() {
    use world_rules::rules::games::mancala::MancalaRules;

    let rules = MancalaRules::new();
    let explanation = rules.explain();

    // 验证捕获规则被说明
    assert!(
        explanation.contains("播种") || explanation.contains("播"),
        "应说明播种规则"
    );
    assert!(
        explanation.contains("捕获") || explanation.contains("捕获"),
        "应说明捕获规则"
    );
}

#[test]
fn mini_shogi_rules_basic() {
    use world_rules::rules::games::mini_shogi::MiniShogiRules;

    let rules = MiniShogiRules::new();
    assert_eq!(rules.metadata().name, "迷你将棋规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn mini_shogi_board_and_pieces() {
    use world_rules::rules::games::mini_shogi::MiniShogiRules;

    let rules = MiniShogiRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(explanation.contains("日本"), "应说明起源日本");
    assert!(explanation.contains("5x5"), "应说明棋盘大小");
    assert!(explanation.contains("王"), "应说明王将");
}

#[test]
fn mini_shogi_drop_rules() {
    use world_rules::rules::games::mini_shogi::MiniShogiRules;

    let rules = MiniShogiRules::new();
    let explanation = rules.explain();

    // 验证打入规则被说明
    assert!(
        explanation.contains("持驹") || explanation.contains("打入"),
        "应说明持驹打入"
    );
    assert!(explanation.contains("升变"), "应说明升变规则");
}

// ===== Phase 19-04: 桌游规则测试 =====

#[test]
fn agricola_rules_basic() {
    use world_rules::rules::games::agricola::AgricolaRules;

    let rules = AgricolaRules::new();
    assert_eq!(rules.metadata().name, "农场乐规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn agricola_components() {
    use world_rules::rules::games::agricola::AgricolaRules;

    let rules = AgricolaRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(
        explanation.contains("农场") || explanation.contains("Agricola"),
        "应说明农场主题"
    );
    assert!(
        explanation.contains("资源") || explanation.contains("木材"),
        "应说明资源系统"
    );
    assert!(
        explanation.contains("动物") || explanation.contains("畜牧"),
        "应说明动物系统"
    );
}

#[test]
fn agricola_game_structure() {
    use world_rules::rules::games::agricola::AgricolaRules;

    let rules = AgricolaRules::new();
    let explanation = rules.explain();

    // 验证游戏结构被说明
    assert!(
        explanation.contains("回合") || explanation.contains("阶段"),
        "应说明回合结构"
    );
    assert!(
        explanation.contains("行动") || explanation.contains("格"),
        "应说明行动格"
    );
}

#[test]
fn carcassonne_rules_basic() {
    use world_rules::rules::games::carcassonne::CarcassonneRules;

    let rules = CarcassonneRules::new();
    assert_eq!(rules.metadata().name, "卡卡颂规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn carcassonne_components() {
    use world_rules::rules::games::carcassonne::CarcassonneRules;

    let rules = CarcassonneRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(
        explanation.contains("板块") || explanation.contains("板块放置"),
        "应说明板块"
    );
    assert!(
        explanation.contains("追随者") || explanation.contains("棋子"),
        "应说明追随者"
    );
}

#[test]
fn carcassonne_scoring() {
    use world_rules::rules::games::carcassonne::CarcassonneRules;

    let rules = CarcassonneRules::new();
    let explanation = rules.explain();

    // 验证计分规则被说明
    assert!(
        explanation.contains("道路") || explanation.contains("城"),
        "应说明地形"
    );
    assert!(
        explanation.contains("得分") || explanation.contains("分"),
        "应说明计分"
    );
}

#[test]
fn dominion_rules_basic() {
    use world_rules::rules::games::dominion::DominionRules;

    let rules = DominionRules::new();
    assert_eq!(rules.metadata().name, "皇权争夺规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn dominion_card_types() {
    use world_rules::rules::games::dominion::DominionRules;

    let rules = DominionRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(
        explanation.contains("牌库") || explanation.contains("牌库构建"),
        "应说明牌库构建"
    );
    assert!(
        explanation.contains("财宝") || explanation.contains("金币"),
        "应说明财宝牌"
    );
    assert!(
        explanation.contains("胜利") || explanation.contains("胜利点"),
        "应说明胜利牌"
    );
}

#[test]
fn dominion_game_flow() {
    use world_rules::rules::games::dominion::DominionRules;

    let rules = DominionRules::new();
    let explanation = rules.explain();

    // 验证游戏流程被说明
    assert!(
        explanation.contains("行动") || explanation.contains("购买"),
        "应说明行动/购买阶段"
    );
    assert!(explanation.contains("回合"), "应说明回合结构");
}

#[test]
fn power_grid_rules_basic() {
    use world_rules::rules::games::power_grid::PowerGridRules;

    let rules = PowerGridRules::new();
    assert_eq!(rules.metadata().name, "电力公司规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn power_grid_components() {
    use world_rules::rules::games::power_grid::PowerGridRules;

    let rules = PowerGridRules::new();
    let explanation = rules.explain();

    // 验证关键规则被说明
    assert!(
        explanation.contains("电厂") || explanation.contains("电力"),
        "应说明电厂"
    );
    assert!(
        explanation.contains("资源") || explanation.contains("煤"),
        "应说明资源"
    );
    assert!(
        explanation.contains("城市") || explanation.contains("网络"),
        "应说明城市网络"
    );
}

#[test]
fn power_grid_game_flow() {
    use world_rules::rules::games::power_grid::PowerGridRules;

    let rules = PowerGridRules::new();
    let explanation = rules.explain();

    // 验证游戏流程被说明
    assert!(
        explanation.contains("拍卖") || explanation.contains("竞价"),
        "应说明拍卖"
    );
    assert!(
        explanation.contains("建设") || explanation.contains("连接"),
        "应说明建设"
    );
}

#[test]
fn puerto_rico_rules_basic() {
    use world_rules::rules::games::puerto_rico::PuertoRicoRules;

    let rules = PuertoRicoRules::new();
    assert_eq!(rules.metadata().name, "波多黎各规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Games(_)));
}

#[test]
fn puerto_rico_roles() {
    use world_rules::rules::games::puerto_rico::PuertoRicoRules;

    let rules = PuertoRicoRules::new();
    let explanation = rules.explain();

    // 验证角色系统被说明
    assert!(
        explanation.contains("角色") || explanation.contains("角牌"),
        "应说明角色牌系统"
    );
    assert!(
        explanation.contains("建筑") || explanation.contains("建设"),
        "应说明建筑"
    );
}

#[test]
fn puerto_rico_production() {
    use world_rules::rules::games::puerto_rico::PuertoRicoRules;

    let rules = PuertoRicoRules::new();
    let explanation = rules.explain();

    // 验证生产运输被说明
    assert!(
        explanation.contains("货物") || explanation.contains("作物"),
        "应说明货物"
    );
    assert!(
        explanation.contains("运输") || explanation.contains("船"),
        "应说明运输"
    );
}

// ===== 综合测试：验证所有 Phase 19 规则 =====

#[test]
fn phase_19_all_rules_registered() {
    use world_rules::rules::core::Rule;
    use world_rules::rules::games::{
        AgricolaRules, BlindGoRules, CarcassonneRules, DominionRules, Go13x13Rules, Go9x9Rules,
        JanggiRules, JungleRules, MakrukRules, MancalaRules, MiniShogiRules, OneColorGoRules,
        PairGoRules, PowerGridRules, PuertoRicoRules,
    };

    // 验证所有 20 个规则类型存在并可实例化
    let rules: Vec<Box<dyn Rule>> = vec![
        // Phase 19-01: 象棋变体（已在之前提交中完成）
        Box::new(world_rules::rules::games::mini_chess::MiniChessRules::new()),
        Box::new(world_rules::rules::games::four_player_chess::FourPlayerChessRules::new()),
        Box::new(world_rules::rules::games::dark_chess::DarkChessRules::new()),
        Box::new(world_rules::rules::games::chaturanga::ChaturangaRules::new()),
        Box::new(world_rules::rules::games::blind_chess::BlindChessRules::new()),
        // Phase 19-02: 围棋变体
        Box::new(Go9x9Rules::new()),
        Box::new(Go13x13Rules::new()),
        Box::new(BlindGoRules::new()),
        Box::new(OneColorGoRules::new()),
        Box::new(PairGoRules::new()),
        // Phase 19-03: 其他棋类
        Box::new(JanggiRules::new()),
        Box::new(MakrukRules::new()),
        Box::new(JungleRules::new()),
        Box::new(MancalaRules::new()),
        Box::new(MiniShogiRules::new()),
        // Phase 19-04: 桌游
        Box::new(AgricolaRules::new()),
        Box::new(CarcassonneRules::new()),
        Box::new(DominionRules::new()),
        Box::new(PowerGridRules::new()),
        Box::new(PuertoRicoRules::new()),
    ];

    for rule in &rules {
        assert!(!rule.metadata().name.is_empty());
        assert!(!rule.explain().is_empty());
        assert!(matches!(rule.category(), RuleCategory::Games(_)));
    }
}

#[test]
fn phase_19_all_rules_have_unique_names() {
    use world_rules::rules::core::Rule;
    use world_rules::rules::games::{
        AgricolaRules, BlindGoRules, CarcassonneRules, DominionRules, Go13x13Rules, Go9x9Rules,
        JanggiRules, JungleRules, MakrukRules, MancalaRules, MiniShogiRules, OneColorGoRules,
        PairGoRules, PowerGridRules, PuertoRicoRules,
    };

    let rules: Vec<Box<dyn Rule>> = vec![
        // Phase 19-02: 围棋变体
        Box::new(Go9x9Rules::new()),
        Box::new(Go13x13Rules::new()),
        Box::new(BlindGoRules::new()),
        Box::new(OneColorGoRules::new()),
        Box::new(PairGoRules::new()),
        // Phase 19-03: 其他棋类
        Box::new(JanggiRules::new()),
        Box::new(MakrukRules::new()),
        Box::new(JungleRules::new()),
        Box::new(MancalaRules::new()),
        Box::new(MiniShogiRules::new()),
        // Phase 19-04: 桌游
        Box::new(AgricolaRules::new()),
        Box::new(CarcassonneRules::new()),
        Box::new(DominionRules::new()),
        Box::new(PowerGridRules::new()),
        Box::new(PuertoRicoRules::new()),
    ];

    // 确保每个规则有不同的名称
    let names: Vec<_> = rules.iter().map(|r| r.metadata().name.clone()).collect();
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            assert_ne!(names[i], names[j], "规则名称应该唯一");
        }
    }
}

#[test]
fn phase_19_all_rules_explain_detailed() {
    use world_rules::rules::core::Rule;
    use world_rules::rules::games::{
        AgricolaRules, BlindGoRules, CarcassonneRules, DominionRules, Go13x13Rules, Go9x9Rules,
        JanggiRules, JungleRules, MakrukRules, MancalaRules, MiniShogiRules, OneColorGoRules,
        PairGoRules, PowerGridRules, PuertoRicoRules,
    };

    // 每个规则的说明内容应该至少 200 字符
    let rules: Vec<Box<dyn Rule>> = vec![
        // Phase 19-02: 围棋变体
        Box::new(Go9x9Rules::new()),
        Box::new(Go13x13Rules::new()),
        Box::new(BlindGoRules::new()),
        Box::new(OneColorGoRules::new()),
        Box::new(PairGoRules::new()),
        // Phase 19-03: 其他棋类
        Box::new(JanggiRules::new()),
        Box::new(MakrukRules::new()),
        Box::new(JungleRules::new()),
        Box::new(MancalaRules::new()),
        Box::new(MiniShogiRules::new()),
        // Phase 19-04: 桌游
        Box::new(AgricolaRules::new()),
        Box::new(CarcassonneRules::new()),
        Box::new(DominionRules::new()),
        Box::new(PowerGridRules::new()),
        Box::new(PuertoRicoRules::new()),
    ];

    for rule in &rules {
        let explanation = rule.explain();
        assert!(
            explanation.len() >= 200,
            "规则 {} 说明应该详细（至少200字符），实际 {} 字符",
            rule.metadata().name,
            explanation.len()
        );
    }
}
