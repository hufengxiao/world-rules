//! Phase 22 新规则集成测试
//!
//! 测试 Phase 22 添加的武术规则：
//! - 22-01: 10种武术规则
//! - 22-02: 10种拳击规则

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
fn wing_chun_basic_punches() {
    use world_rules::rules::sports::WingChunRules;

    let rules = WingChunRules::new();
    assert!(!rules.basic_punches().is_empty());
    assert!(rules.basic_punches().contains(&"日字拳: 直拳攻击中线"));
}

#[test]
fn wing_chun_forms() {
    use world_rules::rules::sports::WingChunRules;

    let rules = WingChunRules::new();
    let forms = rules.forms();
    assert!(forms.contains(&"小念头: 基础拳法套路"));
    assert!(forms.contains(&"寻桥: 进阶攻防套路"));
}

#[test]
fn wing_chun_centerline_theory() {
    use world_rules::rules::sports::WingChunRules;

    let rules = WingChunRules::new();
    assert!(!rules.centerline_theory().is_empty());
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
fn bagua_eight_palms() {
    use world_rules::rules::sports::BaguaRules;

    let rules = BaguaRules::new();
    let palms = rules.eight_palms();
    assert_eq!(palms.len(), 8);
    assert!(palms.contains(&"乾卦狮形掌"));
    assert!(palms.contains(&"坤卦麒麟掌"));
}

#[test]
fn bagua_characteristics() {
    use world_rules::rules::sports::BaguaRules;

    let rules = BaguaRules::new();
    assert!(!rules.characteristics().is_empty());
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
    assert_eq!(elements.len(), 5);
    assert!(elements.iter().any(|e| e.contains("崩拳")));
    assert!(elements.iter().any(|e| e.contains("炮拳")));
}

#[test]
fn xingyi_twelve_animals() {
    use world_rules::rules::sports::XingyiRules;

    let rules = XingyiRules::new();
    let animals = rules.twelve_animals();
    assert_eq!(animals.len(), 12);
    assert!(animals.iter().any(|a| a.contains("龙形")));
    assert!(animals.iter().any(|a| a.contains("虎形")));
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
fn shuai_jiao_scoring() {
    use world_rules::rules::sports::ShuaiJiaoRules;

    let rules = ShuaiJiaoRules::new();
    assert!(!rules.scoring_criteria().is_empty());
    assert!(rules.scoring_criteria().contains(&"一本(4分): 完美摔倒对手"));
}

#[test]
fn shuai_jiao_techniques() {
    use world_rules::rules::sports::ShuaiJiaoRules;

    let rules = ShuaiJiaoRules::new();
    let techniques = rules.valid_techniques();
    assert!(!techniques.is_empty());
    assert!(techniques.iter().any(|t| t.contains("揣跤")));
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
fn ninjutsu_schools() {
    use world_rules::rules::sports::NinjutsuRules;

    let rules = NinjutsuRules::new();
    let schools = rules.schools();
    assert!(!schools.is_empty());
    assert!(schools.contains(&"伊贺流: 伊贺忍者传统"));
}

#[test]
fn ninjutsu_weapons() {
    use world_rules::rules::sports::NinjutsuRules;

    let rules = NinjutsuRules::new();
    assert!(!rules.weapons().is_empty());
    assert!(rules.weapons().contains(&"忍刀: 短刀技术"));
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
fn kyokushin_characteristics() {
    use world_rules::rules::sports::KyokushinRules;

    let rules = KyokushinRules::new();
    assert!(!rules.competition_characteristics().is_empty());
    assert!(rules.competition_characteristics().contains(&"全接触: 实际打击"));
}

#[test]
fn kyokushin_prohibited_techniques() {
    use world_rules::rules::sports::KyokushinRules;

    let rules = KyokushinRules::new();
    assert!(!rules.prohibited_techniques().is_empty());
    assert!(rules.prohibited_techniques().contains(&"头部打击: 手部攻击头部"));
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
fn shotokan_competition_types() {
    use world_rules::rules::sports::ShotokanRules;

    let rules = ShotokanRules::new();
    assert!(!rules.competition_types().is_empty());
    assert!(rules.competition_types().contains(&"套路比赛: 型表演评分"));
}

#[test]
fn shotokan_kata() {
    use world_rules::rules::sports::ShotokanRules;

    let rules = ShotokanRules::new();
    assert!(!rules.kata().is_empty());
    assert!(rules.kata().contains(&"平安初段至五段: 基础套路"));
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
fn goju_ryu_characteristics() {
    use world_rules::rules::sports::GojuRyuRules;

    let rules = GojuRyuRules::new();
    let chars = rules.characteristics();
    assert!(!chars.is_empty());
    assert!(chars.contains(&"刚法: 硬性技术，力量打击"));
    assert!(chars.contains(&"柔法: 柔性技术，流畅动作"));
}

#[test]
fn goju_ryu_kata() {
    use world_rules::rules::sports::GojuRyuRules;

    let rules = GojuRyuRules::new();
    let katas = rules.kata();
    assert!(!katas.is_empty());
    assert!(katas.contains(&"击碎第一: 基础刚法套路"));
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
fn escrima_styles() {
    use world_rules::rules::sports::EscrimaRules;

    let rules = EscrimaRules::new();
    assert!(!rules.styles().is_empty());
    assert!(rules.styles().contains(&"Arnis: 马尼拉风格"));
}

#[test]
fn escrima_weapons() {
    use world_rules::rules::sports::EscrimaRules;

    let rules = EscrimaRules::new();
    assert!(!rules.weapons().is_empty());
    assert!(rules.weapons().contains(&"单棍: 单根短棍"));
    assert!(rules.weapons().contains(&"双棍: 两根短棍"));
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
fn silat_melayu_styles() {
    use world_rules::rules::sports::SilatMelayuRules;

    let rules = SilatMelayuRules::new();
    let styles = rules.styles();
    assert!(!styles.is_empty());
    assert!(styles.contains(&"Silat Gayong: 盖勇流派"));
}

#[test]
fn silat_melayu_animal_forms() {
    use world_rules::rules::sports::SilatMelayuRules;

    let rules = SilatMelayuRules::new();
    let forms = rules.animal_forms();
    assert!(!forms.is_empty());
    assert!(forms.contains(&"虎形: 猛虎扑击动作"));
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
    assert!(!rules.weight_classes().is_empty());
    assert!(rules.weight_classes().iter().any(|w| w.contains("重量级")));
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
fn boxing_wbo_championship() {
    use world_rules::rules::sports::BoxingWboRules;

    let rules = BoxingWboRules::new();
    assert!(!rules.championship_rules().is_empty());
    assert!(rules.championship_rules().contains(&"12回合世界冠军赛"));
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
fn boxing_amateur_equipment() {
    use world_rules::rules::sports::BoxingAmateurRules;

    let rules = BoxingAmateurRules::new();
    let equip = rules.equipment();
    assert!(!equip.is_empty());
    assert!(equip.iter().any(|e| e.contains("头盔")));
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
fn boxing_queensberry_core() {
    use world_rules::rules::sports::BoxingQueensberryRules;

    let rules = BoxingQueensberryRules::new();
    let core = rules.core_rules();
    assert!(!core.is_empty());
    assert!(core.contains(&"禁止摔跤或搂抱"));
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
fn boxing_british_licensing() {
    use world_rules::rules::sports::BoxingBritishRules;

    let rules = BoxingBritishRules::new();
    assert!(!rules.licensing().is_empty());
    assert!(rules.licensing().iter().any(|l| l.contains("BBBoC")));
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
fn sanda_techniques() {
    use world_rules::rules::sports::SandaRules;

    let rules = SandaRules::new();
    let techniques = rules.permitted_techniques();
    assert!(!techniques.is_empty());
    assert!(techniques.iter().any(|t| t.contains("拳法")));
    assert!(techniques.iter().any(|t| t.contains("摔法")));
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
fn savate_levels() {
    use world_rules::rules::sports::SavateRules;

    let rules = SavateRules::new();
    let levels = rules.competition_levels();
    assert!(!levels.is_empty());
    assert!(levels.contains(&"大师级: 最高级别"));
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
fn lethwei_headbutt() {
    use world_rules::rules::sports::LethweiRules;

    let rules = LethweiRules::new();
    let chars = rules.unique_characteristics();
    assert!(!chars.is_empty());
    assert!(chars.iter().any(|c| c.contains("头击")));
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
fn bokator_animal_forms() {
    use world_rules::rules::sports::BokatorRules;

    let rules = BokatorRules::new();
    let forms = rules.animal_forms();
    assert_eq!(forms.len(), 8);
    assert!(forms.contains(&"鹰形: 飞翔攻击和爪法"));
    assert!(forms.contains(&"狮形: 猛烈攻击组合"));
}

#[test]
fn voivovam_rules_basic() {
    use world_rules::rules::sports::VoivovamRules;

    let rules = VoivovamRules::new();
    assert_eq!(rules.metadata().name, "白拳规则");
    assert!(!rules.explain().is_empty());
    assert!(matches!(rules.category(), RuleCategory::Sports(_)));
}

#[test]
fn voivovam_weapons() {
    use world_rules::rules::sports::VoivovamRules;

    let rules = VoivovamRules::new();
    let weapons = rules.traditional_weapons();
    assert!(!weapons.is_empty());
    assert!(weapons.iter().any(|w| w.contains("刀")));
    assert!(weapons.iter().any(|w| w.contains("剑")));
}