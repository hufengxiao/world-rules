//! Phase 22 新规则集成测试
//!
//! 测试 Phase 22 添加的武术规则：
//! - 22-01: 10种武术规则

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