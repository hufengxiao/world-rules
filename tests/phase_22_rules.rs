//! Phase 22 新规则集成测试
//!
//! 测试 Phase 22 添加的格斗与武术规则：
//! - 22-01: 10种武术规则（咏春拳、八卦掌、形意拳、中国摔跤、忍术、极真会馆空手道、松涛馆空手道、刚柔流空手道、菲律宾短棍术、马来传统武术）
//! - 22-02: 10种拳击规则（奥运会拳击、WBO、业余拳击、昆斯伯里拳击、英国拳击、散打、法国踢腿术、缅甸拳击、高棉拳击、白拳）
//! - 22-03: 5种其他格斗规则（K-1、Luta Livre、ONE Championship MMA、Pancrase、Pankration）

use world_rules::prelude::*;

// ===== Phase 22-01: 武术规则测试 =====

#[test]
fn wing_chun_rules_basic() {
    use world_rules::rules::sports::WingChunRules;

    let rules = WingChunRules::new();
    assert_eq!(rules.metadata().name, "咏春拳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn wing_chun_forms() {
    use world_rules::rules::sports::WingChunRules;

    let rules = WingChunRules::new();
    let forms = rules.forms();
    assert!(forms.iter().any(|f| f.contains("小念头")));
    assert!(forms.iter().any(|f| f.contains("寻桥")));
    assert!(forms.iter().any(|f| f.contains("标指")));
    assert!(forms.len() >= 5);
}

#[test]
fn wing_chun_basic_punches() {
    use world_rules::rules::sports::WingChunRules;

    let rules = WingChunRules::new();
    let punches = rules.basic_punches();
    assert!(punches.iter().any(|p| p.contains("日字拳")));
    assert!(punches.iter().any(|p| p.contains("摊手")));
    assert!(punches.len() >= 5);
}

#[test]
fn wing_chun_centerline_theory() {
    use world_rules::rules::sports::WingChunRules;

    let rules = WingChunRules::new();
    let theory = rules.centerline_theory();
    assert!(theory.iter().any(|t| t.contains("守中")));
    assert!(theory.iter().any(|t| t.contains("用中")));
    assert!(theory.len() >= 5);
}

#[test]
fn wing_chun_training_methods() {
    use world_rules::rules::sports::WingChunRules;

    let rules = WingChunRules::new();
    let methods = rules.training_methods();
    assert!(methods.iter().any(|m| m.contains("黐手")));
    assert!(methods.len() >= 5);
}

#[test]
fn bagua_rules_basic() {
    use world_rules::rules::sports::BaguaRules;

    let rules = BaguaRules::new();
    assert_eq!(rules.metadata().name, "八卦掌规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn bagua_characteristics() {
    use world_rules::rules::sports::BaguaRules;

    let rules = BaguaRules::new();
    let chars = rules.characteristics();
    assert!(chars.iter().any(|c| c.contains("走圈")));
    assert!(chars.iter().any(|c| c.contains("转身")));
    assert!(chars.len() >= 5);
}

#[test]
fn bagua_eight_palms() {
    use world_rules::rules::sports::BaguaRules;

    let rules = BaguaRules::new();
    let palms = rules.eight_palms();
    assert!(palms.iter().any(|p| p.contains("乾卦")));
    assert!(palms.iter().any(|p| p.contains("坤卦")));
    assert!(palms.iter().any(|p| p.contains("坎卦")));
    assert!(palms.iter().any(|p| p.contains("离卦")));
    assert_eq!(palms.len(), 8);
}

#[test]
fn bagua_forms() {
    use world_rules::rules::sports::BaguaRules;

    let rules = BaguaRules::new();
    let forms = rules.forms();
    assert!(forms.iter().any(|f| f.contains("老八掌")));
    assert!(forms.iter().any(|f| f.contains("新八掌")));
    assert!(forms.len() >= 5);
}

#[test]
fn bagua_stepping_methods() {
    use world_rules::rules::sports::BaguaRules;

    let rules = BaguaRules::new();
    let steps = rules.stepping_methods();
    assert!(steps.iter().any(|s| s.contains("趟泥步")));
    assert!(steps.len() >= 5);
}

#[test]
fn xingyi_rules_basic() {
    use world_rules::rules::sports::XingyiRules;

    let rules = XingyiRules::new();
    assert_eq!(rules.metadata().name, "形意拳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn xingyi_five_elements() {
    use world_rules::rules::sports::XingyiRules;

    let rules = XingyiRules::new();
    let elements = rules.five_elements();
    assert!(elements.iter().any(|e| e.contains("崩拳")));
    assert!(elements.iter().any(|e| e.contains("炮拳")));
    assert!(elements.iter().any(|e| e.contains("劈拳")));
    assert!(elements.len() >= 5);
}

#[test]
fn xingyi_twelve_animals() {
    use world_rules::rules::sports::XingyiRules;

    let rules = XingyiRules::new();
    let animals = rules.twelve_animals();
    assert!(animals.len() >= 12);
}

#[test]
fn shuai_jiao_rules_basic() {
    use world_rules::rules::sports::ShuaiJiaoRules;

    let rules = ShuaiJiaoRules::new();
    assert_eq!(rules.metadata().name, "中国摔跤规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn shuai_jiao_techniques() {
    use world_rules::rules::sports::ShuaiJiaoRules;

    let rules = ShuaiJiaoRules::new();
    let techniques = rules.valid_techniques();
    assert!(techniques.len() >= 5);
}

#[test]
fn shuai_jiao_scoring() {
    use world_rules::rules::sports::ShuaiJiaoRules;

    let rules = ShuaiJiaoRules::new();
    let scoring = rules.scoring_criteria();
    assert!(scoring.iter().any(|s| s.contains("一本")));
    assert!(scoring.iter().any(|s| s.contains("有技")));
}

#[test]
fn ninjutsu_rules_basic() {
    use world_rules::rules::sports::NinjutsuRules;

    let rules = NinjutsuRules::new();
    assert_eq!(rules.metadata().name, "忍术规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn ninjutsu_techniques() {
    use world_rules::rules::sports::NinjutsuRules;

    let rules = NinjutsuRules::new();
    let techniques = rules.basic_techniques();
    assert!(techniques.len() >= 5);
}

#[test]
fn kyokushin_rules_basic() {
    use world_rules::rules::sports::KyokushinRules;

    let rules = KyokushinRules::new();
    assert_eq!(rules.metadata().name, "极真会馆空手道规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn kyokushin_techniques() {
    use world_rules::rules::sports::KyokushinRules;

    let rules = KyokushinRules::new();
    let techniques = rules.permitted_techniques();
    assert!(techniques.len() >= 5);
}

#[test]
fn shotokan_rules_basic() {
    use world_rules::rules::sports::ShotokanRules;

    let rules = ShotokanRules::new();
    assert_eq!(rules.metadata().name, "松涛馆空手道规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn shotokan_techniques() {
    use world_rules::rules::sports::ShotokanRules;

    let rules = ShotokanRules::new();
    let kata = rules.kata();
    assert!(kata.len() >= 5);
}

#[test]
fn goju_ryu_rules_basic() {
    use world_rules::rules::sports::GojuRyuRules;

    let rules = GojuRyuRules::new();
    assert_eq!(rules.metadata().name, "刚柔流空手道规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn goju_ryu_techniques() {
    use world_rules::rules::sports::GojuRyuRules;

    let rules = GojuRyuRules::new();
    let characteristics = rules.characteristics();
    assert!(characteristics.len() >= 5);
}

#[test]
fn escrima_rules_basic() {
    use world_rules::rules::sports::EscrimaRules;

    let rules = EscrimaRules::new();
    assert_eq!(rules.metadata().name, "菲律宾短棍术规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn escrima_techniques() {
    use world_rules::rules::sports::EscrimaRules;

    let rules = EscrimaRules::new();
    let techniques = rules.basic_techniques();
    assert!(techniques.len() >= 5);
}

#[test]
fn silat_melayu_rules_basic() {
    use world_rules::rules::sports::SilatMelayuRules;

    let rules = SilatMelayuRules::new();
    assert_eq!(rules.metadata().name, "马来传统武术规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn silat_melayu_techniques() {
    use world_rules::rules::sports::SilatMelayuRules;

    let rules = SilatMelayuRules::new();
    let techniques = rules.basic_techniques();
    assert!(techniques.len() >= 5);
}

// ===== Phase 22-02: 拳击规则测试 =====

#[test]
fn boxing_olympic_rules_basic() {
    use world_rules::rules::sports::BoxingOlympicRules;

    let rules = BoxingOlympicRules::new();
    assert_eq!(rules.metadata().name, "奥运会拳击规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn boxing_olympic_weight_classes() {
    use world_rules::rules::sports::BoxingOlympicRules;

    let rules = BoxingOlympicRules::new();
    let classes = rules.weight_classes();
    assert!(classes.len() >= 8);
}

#[test]
fn boxing_olympic_women_weight_classes() {
    use world_rules::rules::sports::BoxingOlympicRules;

    let rules = BoxingOlympicRules::new();
    let classes = rules.women_weight_classes();
    assert!(classes.len() >= 6);
}

#[test]
fn boxing_olympic_round_system() {
    use world_rules::rules::sports::BoxingOlympicRules;

    let rules = BoxingOlympicRules::new();
    let rounds = rules.round_system();
    assert!(rounds.iter().any(|r| r.contains("回合")));
}

#[test]
fn boxing_olympic_scoring_system() {
    use world_rules::rules::sports::BoxingOlympicRules;

    let rules = BoxingOlympicRules::new();
    let scoring = rules.scoring_system();
    assert!(scoring.iter().any(|s| s.contains("10分")));
}

#[test]
fn boxing_olympic_equipment() {
    use world_rules::rules::sports::BoxingOlympicRules;

    let rules = BoxingOlympicRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("拳套")));
    assert!(equipment.iter().any(|e| e.contains("护齿")));
}

#[test]
fn boxing_olympic_fouls() {
    use world_rules::rules::sports::BoxingOlympicRules;

    let rules = BoxingOlympicRules::new();
    let fouls = rules.fouls();
    assert!(fouls.len() >= 10);
}

#[test]
fn boxing_wbo_rules_basic() {
    use world_rules::rules::sports::BoxingWboRules;

    let rules = BoxingWboRules::new();
    assert_eq!(rules.metadata().name, "世界拳击组织规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn boxing_wbo_weight_classes() {
    use world_rules::rules::sports::BoxingWboRules;

    let rules = BoxingWboRules::new();
    let classes = rules.weight_classes();
    assert!(classes.len() >= 10);
}

#[test]
fn boxing_wbo_championship_rules() {
    use world_rules::rules::sports::BoxingWboRules;

    let rules = BoxingWboRules::new();
    let championship = rules.championship_rules();
    assert!(championship.iter().any(|c| c.contains("12回合")));
}

#[test]
fn boxing_amateur_rules_basic() {
    use world_rules::rules::sports::BoxingAmateurRules;

    let rules = BoxingAmateurRules::new();
    assert_eq!(rules.metadata().name, "业余拳击规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn boxing_amateur_weight_classes() {
    use world_rules::rules::sports::BoxingAmateurRules;

    let rules = BoxingAmateurRules::new();
    let classes = rules.weight_classes();
    assert!(classes.len() >= 10);
}

#[test]
fn boxing_amateur_rounds() {
    use world_rules::rules::sports::BoxingAmateurRules;

    let rules = BoxingAmateurRules::new();
    let rounds = rules.rounds();
    assert!(rounds.iter().any(|r| r.contains("回合")));
}

#[test]
fn boxing_queensberry_rules_basic() {
    use world_rules::rules::sports::BoxingQueensberryRules;

    let rules = BoxingQueensberryRules::new();
    assert_eq!(rules.metadata().name, "昆斯伯里拳击规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn boxing_queensberry_core_rules() {
    use world_rules::rules::sports::BoxingQueensberryRules;

    let rules = BoxingQueensberryRules::new();
    let core = rules.core_rules();
    assert!(core.len() >= 5);
}

#[test]
fn boxing_british_rules_basic() {
    use world_rules::rules::sports::BoxingBritishRules;

    let rules = BoxingBritishRules::new();
    assert_eq!(rules.metadata().name, "英国拳击规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn boxing_british_weight_classes() {
    use world_rules::rules::sports::BoxingBritishRules;

    let rules = BoxingBritishRules::new();
    let classes = rules.weight_classes();
    assert!(classes.len() >= 8);
}

#[test]
fn sanda_rules_basic() {
    use world_rules::rules::sports::SandaRules;

    let rules = SandaRules::new();
    assert_eq!(rules.metadata().name, "散打规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn sanda_weight_classes() {
    use world_rules::rules::sports::SandaRules;

    let rules = SandaRules::new();
    let classes = rules.weight_classes();
    assert!(classes.len() >= 12);
}

#[test]
fn sanda_round_system() {
    use world_rules::rules::sports::SandaRules;

    let rules = SandaRules::new();
    let rounds = rules.round_system();
    assert!(rounds.iter().any(|r| r.contains("3回合")));
}

#[test]
fn sanda_scoring_areas() {
    use world_rules::rules::sports::SandaRules;

    let rules = SandaRules::new();
    let areas = rules.scoring_areas();
    assert!(areas.iter().any(|a| a.contains("头部")));
    assert!(areas.iter().any(|a| a.contains("躯干")));
}

#[test]
fn sanda_permitted_techniques() {
    use world_rules::rules::sports::SandaRules;

    let rules = SandaRules::new();
    let techniques = rules.permitted_techniques();
    assert!(techniques.iter().any(|t| t.contains("拳法")));
    assert!(techniques.iter().any(|t| t.contains("腿法")));
    assert!(techniques.iter().any(|t| t.contains("摔法")));
}

#[test]
fn sanda_prohibited_techniques() {
    use world_rules::rules::sports::SandaRules;

    let rules = SandaRules::new();
    let prohibited = rules.prohibited_techniques();
    assert!(prohibited.iter().any(|p| p.contains("后脑")));
    assert!(prohibited.iter().any(|p| p.contains("裆部")));
}

#[test]
fn sanda_equipment() {
    use world_rules::rules::sports::SandaRules;

    let rules = SandaRules::new();
    let equipment = rules.equipment();
    assert!(equipment.iter().any(|e| e.contains("头盔")));
    assert!(equipment.iter().any(|e| e.contains("拳套")));
}

#[test]
fn savate_rules_basic() {
    use world_rules::rules::sports::SavateRules;

    let rules = SavateRules::new();
    assert_eq!(rules.metadata().name, "法国踢腿术规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn savate_weight_classes() {
    use world_rules::rules::sports::SavateRules;

    let rules = SavateRules::new();
    let classes = rules.weight_classes();
    assert!(classes.len() >= 6);
}

#[test]
fn savate_round_system() {
    use world_rules::rules::sports::SavateRules;

    let rules = SavateRules::new();
    let rounds = rules.round_system();
    assert!(rounds.iter().any(|r| r.contains("回合")));
}

#[test]
fn lethwei_rules_basic() {
    use world_rules::rules::sports::LethweiRules;

    let rules = LethweiRules::new();
    assert_eq!(rules.metadata().name, "缅甸拳击规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn lethwei_unique_characteristics() {
    use world_rules::rules::sports::LethweiRules;

    let rules = LethweiRules::new();
    let characteristics = rules.unique_characteristics();
    assert!(characteristics.iter().any(|c| c.contains("头击")));
    assert!(characteristics.iter().any(|c| c.contains("无拳套")));
}

#[test]
fn lethwei_permitted_techniques() {
    use world_rules::rules::sports::LethweiRules;

    let rules = LethweiRules::new();
    let techniques = rules.permitted_techniques();
    assert!(techniques.iter().any(|t| t.contains("拳法")));
    assert!(techniques.iter().any(|t| t.contains("腿法")));
    assert!(techniques.iter().any(|t| t.contains("肘击")));
    assert!(techniques.iter().any(|t| t.contains("头击")));
}

#[test]
fn lethwei_victory_conditions() {
    use world_rules::rules::sports::LethweiRules;

    let rules = LethweiRules::new();
    let conditions = rules.victory_conditions();
    assert!(conditions.iter().any(|c| c.contains("KO")));
}

#[test]
fn bokator_rules_basic() {
    use world_rules::rules::sports::BokatorRules;

    let rules = BokatorRules::new();
    assert_eq!(rules.metadata().name, "高棉拳击规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn bokator_techniques() {
    use world_rules::rules::sports::BokatorRules;

    let rules = BokatorRules::new();
    let techniques = rules.permitted_techniques();
    assert!(techniques.len() >= 5);
}

#[test]
fn bokator_animal_forms() {
    use world_rules::rules::sports::BokatorRules;

    let rules = BokatorRules::new();
    let forms = rules.animal_forms();
    assert!(forms.iter().any(|f| f.contains("鹰形")));
    assert!(forms.iter().any(|f| f.contains("龙形")));
}

// ===== Phase 22-03: 其他格斗规则测试 =====

#[test]
fn k1_rules_basic() {
    use world_rules::rules::sports::K1Rules;

    let rules = K1Rules::new();
    assert_eq!(rules.metadata().name, "K-1踢拳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn k1_round_system() {
    use world_rules::rules::sports::K1Rules;

    let rules = K1Rules::new();
    let rounds = rules.round_system();
    assert!(rounds.iter().any(|r| r.contains("3回合")));
}

#[test]
fn k1_permitted_techniques() {
    use world_rules::rules::sports::K1Rules;

    let rules = K1Rules::new();
    let techniques = rules.permitted_techniques();
    assert!(techniques.iter().any(|t| t.contains("拳法")));
    assert!(techniques.iter().any(|t| t.contains("腿法")));
    assert!(techniques.iter().any(|t| t.contains("膝击")));
}

#[test]
fn k1_prohibited_techniques() {
    use world_rules::rules::sports::K1Rules;

    let rules = K1Rules::new();
    let prohibited = rules.prohibited_techniques();
    assert!(prohibited.iter().any(|p| p.contains("肘击")));
    assert!(prohibited.iter().any(|p| p.contains("地面")));
}

#[test]
fn k1_clinching_rules() {
    use world_rules::rules::sports::K1Rules;

    let rules = K1Rules::new();
    let clinching = rules.clinching_rules();
    assert!(clinching.iter().any(|c| c.contains("5秒")));
}

#[test]
fn k1_weight_classes() {
    use world_rules::rules::sports::K1Rules;

    let rules = K1Rules::new();
    let classes = rules.weight_classes();
    assert!(classes.len() >= 7);
}

#[test]
fn luta_livre_rules_basic() {
    use world_rules::rules::sports::LutaLivreRules;

    let rules = LutaLivreRules::new();
    assert_eq!(rules.metadata().name, "Luta Livre规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn luta_livre_unique_characteristics() {
    use world_rules::rules::sports::LutaLivreRules;

    let rules = LutaLivreRules::new();
    let characteristics = rules.unique_characteristics();
    assert!(characteristics.iter().any(|c| c.contains("无道服")));
}

#[test]
fn luta_livre_permitted_techniques() {
    use world_rules::rules::sports::LutaLivreRules;

    let rules = LutaLivreRules::new();
    let techniques = rules.permitted_techniques();
    assert!(techniques.iter().any(|t| t.contains("摔法")));
    assert!(techniques.iter().any(|t| t.contains("关节技")));
}

#[test]
fn luta_livre_submission_categories() {
    use world_rules::rules::sports::LutaLivreRules;

    let rules = LutaLivreRules::new();
    let submissions = rules.submission_categories();
    assert!(submissions.iter().any(|s| s.contains("手臂锁")));
    assert!(submissions.iter().any(|s| s.contains("腿部锁")));
}

#[test]
fn one_championship_mma_rules_basic() {
    use world_rules::rules::sports::OneChampionshipMmaRules;

    let rules = OneChampionshipMmaRules::new();
    assert_eq!(rules.metadata().name, "ONE Championship MMA规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn one_championship_round_system() {
    use world_rules::rules::sports::OneChampionshipMmaRules;

    let rules = OneChampionshipMmaRules::new();
    let rounds = rules.round_system();
    assert!(rounds.len() >= 5);
}

#[test]
fn one_championship_weight_classes() {
    use world_rules::rules::sports::OneChampionshipMmaRules;

    let rules = OneChampionshipMmaRules::new();
    let classes = rules.weight_classes();
    assert!(classes.len() >= 6);
}

#[test]
fn pancrase_rules_basic() {
    use world_rules::rules::sports::PancraseRules;

    let rules = PancraseRules::new();
    assert_eq!(rules.metadata().name, "Pancrase规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn pancrase_historical_characteristics() {
    use world_rules::rules::sports::PancraseRules;

    let rules = PancraseRules::new();
    let characteristics = rules.historical_characteristics();
    assert!(characteristics.iter().any(|c| c.contains("日本")));
    assert!(characteristics.iter().any(|c| c.contains("1993")));
}

#[test]
fn pancrase_permitted_techniques() {
    use world_rules::rules::sports::PancraseRules;

    let rules = PancraseRules::new();
    let techniques = rules.permitted_techniques();
    assert!(techniques.len() >= 5);
}

#[test]
fn pankration_rules_basic() {
    use world_rules::rules::sports::PankrationRules;

    let rules = PankrationRules::new();
    assert_eq!(rules.metadata().name, "Pankration规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn pankration_historical_characteristics() {
    use world_rules::rules::sports::PankrationRules;

    let rules = PankrationRules::new();
    let characteristics = rules.historical_characteristics();
    assert!(characteristics.iter().any(|c| c.contains("古代奥林匹克")));
    assert!(characteristics.iter().any(|c| c.contains("公元前")));
}

#[test]
fn pankration_ancient_rules() {
    use world_rules::rules::sports::PankrationRules;

    let rules = PankrationRules::new();
    let ancient = rules.ancient_rules();
    assert!(ancient.iter().any(|a| a.contains("无时间")));
    assert!(ancient.iter().any(|a| a.contains("咬人")));
}

#[test]
fn pankration_modern_revision() {
    use world_rules::rules::sports::PankrationRules;

    let rules = PankrationRules::new();
    let modern = rules.modern_revision();
    assert!(modern.iter().any(|m| m.contains("现代")));
    assert!(modern.iter().any(|m| m.contains("安全")));
}

#[test]
fn pankration_modern_permitted_techniques() {
    use world_rules::rules::sports::PankrationRules;

    let rules = PankrationRules::new();
    let techniques = rules.modern_permitted_techniques();
    assert!(techniques.iter().any(|t| t.contains("拳法")));
    assert!(techniques.iter().any(|t| t.contains("腿法")));
    assert!(techniques.iter().any(|t| t.contains("摔法")));
}

#[test]
fn pankration_modern_prohibited_techniques() {
    use world_rules::rules::sports::PankrationRules;

    let rules = PankrationRules::new();
    let prohibited = rules.modern_prohibited_techniques();
    assert!(prohibited.iter().any(|p| p.contains("肘击")));
    assert!(prohibited.iter().any(|p| p.contains("眼睛")));
}
