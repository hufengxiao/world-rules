//! World Rules CLI - 世界规则库命令行工具
//!
//! 用法:
//!   wr list [--category <cat>] [--search <query>]
//!   wr show <name>
//!   wr stats
//!   wr validate mahjong <tiles>
//!   wr validate poker <cards>

use std::env;
use world_rules::rules::core::{RuleCategory, RuleMetadata};
use world_rules::rules::games::mahjong::Hand;

/// JSON 输出结构
#[derive(serde::Serialize)]
struct RuleJson {
    name: String,
    category: String,
    origin: String,
    #[serde(rename = "type")]
    type_name: String,
    version: String,
    tags: Vec<String>,
    description: String,
    explain: String,
}

fn to_rule_json(cat: &str, meta: &RuleMetadata, explain: &str) -> RuleJson {
    RuleJson {
        name: meta.name.clone(),
        category: cat.to_string(),
        origin: meta.origin.clone().unwrap_or_default(),
        type_name: cat_name(cat).to_string(),
        version: meta.version.clone(),
        tags: meta.tags.clone(),
        description: meta.description.clone(),
        explain: explain.to_string(),
    }
}

fn rule_cat_str(cat: &RuleCategory) -> String {
    match cat {
        RuleCategory::Games(n) => format!("games/{}", n),
        RuleCategory::Sports(n) => format!("sports/{}", n),
        RuleCategory::Social(n) => format!("social/{}", n),
        RuleCategory::Science(n) => format!("science/{}", n),
        RuleCategory::Law(n) => format!("law/{}", n),
        RuleCategory::Health(n) => format!("health/{}", n),
        RuleCategory::Custom(n) => n.clone(),
    }
}

fn rule_cat_type(cat: &RuleCategory) -> &'static str {
    match cat {
        RuleCategory::Games(_) => "🎮 游戏规则",
        RuleCategory::Sports(_) => "🏃 体育规则",
        RuleCategory::Social(_) => "🤝 社交礼仪",
        RuleCategory::Science(_) => "🔬 科学定律",
        RuleCategory::Law(_) => "⚖️ 法律法规",
        RuleCategory::Health(_) => "🏥 健康规则",
        RuleCategory::Custom(_) => "其他",
    }
}

fn rule_to_json(cat: &RuleCategory, meta: &RuleMetadata, explain: &str) -> RuleJson {
    RuleJson {
        name: meta.name.clone(),
        category: rule_cat_str(cat),
        origin: meta.origin.clone().unwrap_or_default(),
        type_name: rule_cat_type(cat).to_string(),
        version: meta.version.clone(),
        tags: meta.tags.clone(),
        description: meta.description.clone(),
        explain: explain.to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "list" => cmd_list(&args[2..]),
        "show" | "explain" | "info" => cmd_show(&args[2..]),
        "stats" => cmd_stats(),
        "validate" | "val" => cmd_validate(&args[2..]),
        "help" | "--help" | "-h" => print_usage(),
        "version" | "--version" | "-V" => println!("wr {}", env!("CARGO_PKG_VERSION")),
        other => {
            eprintln!("未知命令: {}", other);
            print_usage();
        }
    }
}

fn print_usage() {
    println!(
        r#"世界规则库 (wr) v{ver}

用法:
  wr list [--category <分类>] [--search <关键词>] [--tag <标签>]
  wr show <规则名称>
  wr stats
  wr validate mahjong <牌面>
  wr validate poker <牌面>
  wr validate doudizhu <牌面>

示例:
  wr list                        列出所有规则
  wr list --category sports      列出体育规则
  wr list --search 麻将          搜索包含"麻将"的规则
  wr list --tag 扑克             按标签过滤
  wr show 围棋                   显示围棋规则详解
  wr explain 德州扑克             同 show，显示规则详解
  wr stats                       显示统计信息
  wr validate mahjong "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 2条 3条 4条 4条"
  wr validate poker "Ah Kd Qs Jc 10h 9d 8s"
  wr validate doudizhu "3s 3s 3s 4h 4h 4h"

麻将牌面格式: 1万 2万 3万 ... 东 南 西 北 中 发 白
扑克牌面格式: Ah Kd Qs Jc 10h (花色: h红心 d方块 s黑桃 c梅花)
斗地主牌面格式: 3s 4h 10d Jc 2s X D (s黑桃 h红心 d方块 c梅花, X小王 D大王)"#,
        ver = env!("CARGO_PKG_VERSION"),
    );
}

fn parse_flag(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}

fn collect_all_rules() -> Vec<(&'static str, RuleMetadata, RuleCategory, String)> {
    let mut rules = Vec::new();
    rules.extend(world_rules::rules::games::all_rules());
    rules.extend(world_rules::rules::sports::all_rules());
    rules.extend(world_rules::rules::social::all_rules());
    rules.extend(world_rules::rules::science::all_rules());
    rules.extend(world_rules::rules::law::all_rules());
    rules.extend(world_rules::rules::health::all_rules());
    rules
}

fn cat_name(cat: &str) -> &'static str {
    match cat {
        "games" => "🎮 游戏规则",
        "sports" => "🏃 体育规则",
        "social" => "🤝 社交礼仪",
        "science" => "🔬 科学定律",
        "law" => "⚖️ 法律法规",
        "health" => "🏥 健康规则",
        _ => "其他",
    }
}

fn cmd_list(args: &[String]) {
    let category = parse_flag(args, "--category");
    let search = parse_flag(args, "--search");
    let tag = parse_flag(args, "--tag");
    let json = args.iter().any(|a| a == "--json");

    let all = collect_all_rules();
    let mut filtered: Vec<_> = all
        .iter()
        .filter(|(cat, _, _, _)| {
            if let Some(ref c) = category {
                cat.contains(&c.to_lowercase())
            } else {
                true
            }
        })
        .filter(|(_, meta, _, _)| {
            if let Some(ref q) = search {
                let q_lower = q.to_lowercase();
                meta.name.to_lowercase().contains(&q_lower)
                    || meta.description.to_lowercase().contains(&q_lower)
            } else {
                true
            }
        })
        .filter(|(_, meta, _, _)| {
            if let Some(ref t) = tag {
                meta.tags.iter().any(|tg| tg.contains(t.as_str()))
            } else {
                true
            }
        })
        .collect();

    filtered.sort_by(|a, b| a.0.cmp(b.0).then_with(|| a.1.name.cmp(&b.1.name)));

    if json {
        let items: Vec<_> = filtered
            .iter()
            .map(|(cat, meta, _, explain)| to_rule_json(cat, meta, explain))
            .collect();
        println!("{}", serde_json::to_string_pretty(&items).unwrap());
        return;
    }

    let mut current_cat = "";
    for (cat, meta, _, _) in &filtered {
        if *cat != current_cat {
            current_cat = cat;
            println!("\n=== {} ===", cat_name(cat));
        }
        let origin = meta.origin.as_deref().unwrap_or("");
        let tags = if meta.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", meta.tags.join(", "))
        };
        let origin_str = if origin.is_empty() {
            String::new()
        } else {
            format!(" ({})", origin)
        };
        println!("  • {}{}{}", meta.name, origin_str, tags);
    }
    println!("\n共 {} 条规则", filtered.len());
}

fn cmd_show(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let args: Vec<&String> = args.iter().filter(|a| a.as_str() != "--json").collect();
    if args.is_empty() {
        eprintln!("用法: wr show <规则名称>");
        return;
    }
    let name = args[0];
    let all = collect_all_rules();
    let name_lower = name.to_lowercase();

    // 1. 精确匹配 name（忽略后缀"规则"）
    let strip = |s: &str| -> String { s.strip_suffix("规则").unwrap_or(s).to_string() };
    let exact: Vec<_> = all
        .iter()
        .filter(|(_, meta, _, _)| strip(&meta.name) == *name || meta.name == *name)
        .collect();

    // 2. 前缀匹配
    let prefix: Vec<_> = if exact.is_empty() {
        all.iter()
            .filter(|(_, meta, _, _)| {
                strip(&meta.name).starts_with(&name_lower) || meta.name.starts_with(name)
            })
            .collect()
    } else {
        vec![]
    };

    // 3. 模糊匹配（fallback）
    let matches = if !exact.is_empty() {
        exact
    } else if !prefix.is_empty() {
        prefix
    } else {
        all.iter()
            .filter(|(_, meta, _, _)| {
                meta.name.to_lowercase().contains(&name_lower)
                    || meta.description.to_lowercase().contains(&name_lower)
            })
            .collect()
    };

    if matches.is_empty() {
        eprintln!("未找到匹配 '{}' 的规则", name);
        eprintln!("提示: 使用 'wr list' 查看所有规则");
        return;
    }

    if matches.len() > 1 {
        if json {
            let items: Vec<_> = matches
                .iter()
                .map(|(_, meta, cat, explain)| rule_to_json(cat, meta, explain))
                .collect();
            println!("{}", serde_json::to_string_pretty(&items).unwrap());
        } else {
            println!("找到 {} 条匹配规则:\n", matches.len());
            for (_, meta, cat, _) in &matches {
                let origin = meta.origin.as_deref().unwrap_or("");
                let origin_str = if origin.is_empty() {
                    String::new()
                } else {
                    format!(" · {}", origin)
                };
                println!("  • {} ({}){}", meta.name, cat, origin_str);
            }
            println!(
                "\n提示: 使用更精确的名称，或 'wr list --search {}' 查看更多。",
                name
            );
        }
        return;
    }

    let (_, meta, cat, explain) = matches[0];

    if json {
        let item = rule_to_json(cat, meta, explain);
        println!("{}", serde_json::to_string_pretty(&item).unwrap());
        return;
    }

    let width = 42;
    println!("┌{}┐", "─".repeat(width));
    println!("│ 📋 {:<width$}", meta.name, width = width - 4);
    println!("├{}┤", "─".repeat(width));
    println!("│ 分类: {:<width$}", cat, width = width - 6);
    println!("│ 版本: {:<width$}", meta.version, width = width - 6);
    if let Some(origin) = &meta.origin {
        println!("│ 来源: {:<width$}", origin, width = width - 6);
    }
    if !meta.tags.is_empty() {
        println!(
            "│ 标签: {:<width$}",
            meta.tags.join(", "),
            width = width - 6
        );
    }
    println!("├{}┤", "─".repeat(width));
    for line in explain.lines() {
        println!("│ {:<width$}", line, width = width - 2);
    }
    println!("└{}┘", "─".repeat(width));
}

fn cmd_stats() {
    let all = collect_all_rules();
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for (cat, _, _, _) in &all {
        *counts.entry(cat).or_insert(0) += 1;
    }

    println!("╔═══════════════════════════════════════╗");
    println!("║        世界规则库 - 统计信息          ║");
    println!("╠═══════════════════════════════════════╣");

    let order = ["games", "sports", "social", "science", "law", "health"];
    let mut total = 0;
    for cat in &order {
        let count = counts.get(cat).copied().unwrap_or(0);
        total += count;
        let bar: String = "█".repeat(count.min(30));
        println!("║ {} {:>4} {}", cat_name(cat), count, bar);
    }
    println!("╠═══════════════════════════════════════╣");
    println!("║ 📊 CLI 可用规则:   {:>4}             ║", total);
    println!(
        "║ 📦 版本:           {}             ║",
        env!("CARGO_PKG_VERSION")
    );
    println!("╚═══════════════════════════════════════╝");
}

fn cmd_validate(args: &[String]) {
    if args.len() < 2 {
        eprintln!("用法: wr validate <游戏类型> <牌面>");
        eprintln!("支持: mahjong, poker, doudizhu");
        return;
    }
    match args[0].as_str() {
        "mahjong" | "mj" | "麻将" => cmd_validate_mahjong(&args[1]),
        "poker" | "德州" | "德州扑克" => cmd_validate_poker(&args[1]),
        "doudizhu" | "ddz" | "斗地主" => cmd_validate_doudizhu(&args[1]),
        other => {
            eprintln!("不支持的游戏类型: {}", other);
            eprintln!("支持: mahjong, poker, doudizhu");
        }
    }
}

fn cmd_validate_mahjong(tiles_str: &str) {
    let tiles = match parse_mahjong_tiles(tiles_str) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("解析牌面失败: {}", e);
            eprintln!("格式: 用空格分隔每张牌，如 \"1万 2万 3万 东 南 西\"");
            return;
        }
    };

    let mut hand = Hand::new();
    for tile in &tiles {
        hand.add_tile(*tile);
    }

    let hand_tiles = hand.tiles();
    println!("手牌 ({}张):", hand_tiles.len());
    let mut sorted: Vec<_> = hand_tiles.to_vec();
    sorted.sort();
    for tile in &sorted {
        print!(" {} ", tile);
    }
    println!("\n");

    if hand_tiles.len() != 14 {
        println!("⚠️  当前 {} 张牌，胡牌需要 14 张", hand_tiles.len());
        let waiting = hand.find_waiting_tiles();
        if !waiting.is_empty() {
            print!("听牌: ");
            for (i, tile) in waiting.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", tile);
            }
            println!();
        }
        return;
    }

    if hand.can_win() {
        println!("✅ 可以胡牌！");
        let counts = hand.tile_counts();
        let all_pairs = counts.len() == 7 && counts.values().all(|&c| c == 2);
        if all_pairs {
            println!("   类型: 七对子");
        }
    } else {
        println!("❌ 不能胡牌");
        let waiting = hand.find_waiting_tiles();
        if !waiting.is_empty() {
            print!("听牌: ");
            for (i, tile) in waiting.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", tile);
            }
            println!();
        } else {
            println!("未听牌");
        }
    }
}

fn parse_mahjong_tiles(s: &str) -> Result<Vec<world_rules::rules::games::mahjong::Tile>, String> {
    let mut tiles = Vec::new();
    for part in s.split_whitespace() {
        let tile = parse_single_tile(part)?;
        tiles.push(tile);
    }
    Ok(tiles)
}

fn parse_single_tile(s: &str) -> Result<world_rules::rules::games::mahjong::Tile, String> {
    use world_rules::rules::games::mahjong::{Dragon, Tile, Wind};

    match s {
        "东" => Ok(Tile::feng(Wind::Dong)),
        "南" => Ok(Tile::feng(Wind::Nan)),
        "西" => Ok(Tile::feng(Wind::Xi)),
        "北" => Ok(Tile::feng(Wind::Bei)),
        "中" => Ok(Tile::jian(Dragon::HongZhong)),
        "发" => Ok(Tile::jian(Dragon::FaCai)),
        "白" => Ok(Tile::jian(Dragon::BaiBan)),
        _ => {
            let chars: Vec<char> = s.chars().collect();
            if chars.len() < 2 {
                return Err(format!("无法解析: {}", s));
            }
            let num = chars[0]
                .to_digit(10)
                .ok_or_else(|| format!("无效数字: {}", chars[0]))? as u8;
            let suit: String = chars[1..].iter().collect();
            match suit.as_str() {
                "万" => Ok(Tile::wan(num)),
                "条" => Ok(Tile::tiao(num)),
                "筒" | "饼" => Ok(Tile::tong(num)),
                _ => Err(format!("无效花色: {}", suit)),
            }
        }
    }
}

fn cmd_validate_poker(cards_str: &str) {
    use world_rules::rules::games::card_games::poker::TexasHoldemRules;

    let cards = match parse_poker_cards(cards_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("解析牌面失败: {}", e);
            eprintln!("格式: 用空格分隔，如 \"Ah Kd Qs Jc 10h\"");
            return;
        }
    };

    println!("手牌 ({}张):", cards.len());
    for card in &cards {
        print!(" {} ", card);
    }
    println!("\n");

    if cards.len() < 5 {
        println!("⚠️  需要至少 5 张牌来评估牌型");
        return;
    }

    let eval = TexasHoldemRules::evaluate_hand(&cards);
    println!("🃏 牌型: {}", eval.rank.name());
    println!("   英文: {}", eval.rank.english_name());
    if !eval.tiebreaker.is_empty() {
        println!("   附带: {:?}", eval.tiebreaker);
    }
}

fn cmd_validate_doudizhu(cards_str: &str) {
    use world_rules::rules::games::doudizhu::{recognize_pattern, DdzCard};

    let cards = match DdzCard::parse_many(cards_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("解析牌面失败: {}", e);
            eprintln!("格式: 用空格分隔，如 \"3s 3s 3s 4h 4h\"");
            eprintln!("点数: 3-10, J, Q, K, A, 2");
            eprintln!("花色: s黑桃 h红心 d方块 c梅花");
            eprintln!("王牌: X小王 D大王");
            return;
        }
    };

    println!("手牌 ({}张):", cards.len());
    for card in &cards {
        print!(" {} ", card);
    }
    println!("\n");

    match recognize_pattern(&cards) {
        Some((pat, rank)) => {
            println!("✅ 识别牌型: {}", pat.name());
            println!("   优先级: {}", pat.priority());
            println!("   关键牌: {}", rank);
        }
        None => {
            println!("❌ 无法识别牌型");
            println!("   可能原因: 牌数不匹配、不连续、含非法组合");
        }
    }
}

fn parse_poker_cards(s: &str) -> Result<Vec<world_rules::rules::games::card_games::Card>, String> {
    let mut cards = Vec::new();
    for part in s.split_whitespace() {
        let card = parse_single_card(part)?;
        cards.push(card);
    }
    Ok(cards)
}

fn parse_single_card(s: &str) -> Result<world_rules::rules::games::card_games::Card, String> {
    use world_rules::rules::games::card_games::{Card, Rank, Suit};

    let s = s.trim();
    if s.len() < 2 {
        return Err(format!("无法解析: {}", s));
    }

    let (rank_str, suit_char) = if let Some(rest) = s.strip_prefix("10") {
        ("10", rest)
    } else {
        (&s[..s.len() - 1], &s[s.len() - 1..])
    };

    let rank = match rank_str.to_uppercase().as_str() {
        "A" => Rank::Ace,
        "K" => Rank::King,
        "Q" => Rank::Queen,
        "J" => Rank::Jack,
        "10" => Rank::Ten,
        "9" => Rank::Nine,
        "8" => Rank::Eight,
        "7" => Rank::Seven,
        "6" => Rank::Six,
        "5" => Rank::Five,
        "4" => Rank::Four,
        "3" => Rank::Three,
        "2" => Rank::Two,
        _ => return Err(format!("无效点数: {}", rank_str)),
    };

    let suit = match suit_char.to_lowercase().as_str() {
        "h" | "♥" => Suit::Heart,
        "d" | "♦" => Suit::Diamond,
        "s" | "♠" => Suit::Spade,
        "c" | "♣" => Suit::Club,
        _ => return Err(format!("无效花色: {}", suit_char)),
    };

    Ok(Card::new(suit, rank))
}
