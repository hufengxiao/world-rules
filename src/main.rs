//! World Rules - 世界规则库示例程序
//!
//! 展示如何使用各种规则

use world_rules::prelude::*;

fn main() {
    println!("=== 世界规则库 ===\n");

    // 麻将规则
    demonstrate_mahjong_rules();

    // 德州扑克规则
    demonstrate_poker_rules();

    // 体育规则
    demonstrate_sports_rules();

    // 社交礼仪
    demonstrate_social_etiquette();

    // 科学定律
    demonstrate_science_rules();

    // 交通规则
    demonstrate_traffic_rules();
}

fn demonstrate_mahjong_rules() {
    println!("--- 麻将规则 ---\n");

    let sichuan = SichuanMahjongRules::new();
    println!("{}\n", sichuan.explain());

    let guobiao = GuobiaoMahjongRules::new();
    println!("{}\n", guobiao.explain());

    let riichi = RiichiMahjongRules::new();
    println!("{}\n", riichi.explain());
}

fn demonstrate_poker_rules() {
    println!("--- 德州扑克规则 ---\n");

    use world_rules::rules::games::card_games::poker::TexasHoldemRules;

    let poker = TexasHoldemRules::new();
    println!("{}\n", poker.explain());
}

fn demonstrate_sports_rules() {
    println!("--- 体育规则 ---\n");

    let football = FootballRules::new();
    println!("{}\n", football.explain());

    let basketball = BasketballRules::new();
    println!("{}\n", basketball.explain());

    let table_tennis = TableTennisRules::new();
    println!("{}\n", table_tennis.explain());
}

fn demonstrate_social_etiquette() {
    println!("--- 社交礼仪 ---\n");

    use world_rules::rules::social::{DiningCulture, GiftEtiquette};

    let chinese_dining = DiningEtiquette::new(DiningCulture::Chinese);
    println!("{}\n", chinese_dining.explain());

    let western_dining = DiningEtiquette::new(DiningCulture::Western);
    println!("{}\n", western_dining.explain());

    let business = BusinessEtiquette::new("中国");
    println!("{}\n", business.explain());

    let gift = GiftEtiquette::new("中国");
    println!("{}\n", gift.explain());
}

fn demonstrate_science_rules() {
    println!("--- 科学定律 ---\n");

    let physics = PhysicsLaws::new();
    println!("{}\n", physics.explain());

    let math = MathRules::new();
    println!("{}\n", math.explain());

    // 数学计算示例
    println!("勾股定理示例: 3² + 4² = 5² → c = {}\n", MathRules::pythagorean(3.0, 4.0));
    println!("圆面积示例: r=5 → S = {}\n", MathRules::circle_area(5.0));
    println!("斐波那契数列前10项: {:?}\n", MathRules::fibonacci(10));
}

fn demonstrate_traffic_rules() {
    println!("--- 交通规则 ---\n");

    use world_rules::rules::law::TrafficRegion;

    let china_traffic = TrafficRules::new(TrafficRegion::China);
    println!("{}\n", china_traffic.explain());

    let japan_traffic = TrafficRules::new(TrafficRegion::Japan);
    println!("{}\n", japan_traffic.explain());
}