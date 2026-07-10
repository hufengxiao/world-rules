//! World Rules CLI - 世界规则库命令行工具
//!
//! 提供规则查询、验证和导出功能的命令行接口。
//!
//! # 命令概览
//!
//! - `list`: 列出所有规则或按条件过滤
//! - `show`: 显示规则详情
//! - `stats`: 显示统计信息
//! - `validate`: 验证游戏状态（麻将、扑克、斗地主、象棋、数独）
//! - `export`: 导出规则到 JSON/HTML/Markdown
//! - `web`: 生成交互式 HTML 页面
//!
//! # 使用示例
//!
//! ```bash
//! # 列出所有规则
//! wr list
//!
//! # 按分类过滤
//! wr list --category games
//!
//! # 搜索规则
//! wr list --search 麻将
//!
//! # 显示规则详情
//! wr show 围棋
//!
//! # 验证麻将胡牌
//! wr validate mahjong "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 2条 3条 4条 4条"
//!
//! # 验证扑克牌型
//! wr validate poker "Ah Kd Qs Jc 10h"
//!
//! # 验证斗地主牌型
//! wr validate doudizhu "3s 3s 3s 4h 4h 4h"
//!
//! # 导出为 JSON
//! wr export json > rules.json
//! ```
//!
//! # 支持的游戏验证
//!
//! ## 麻将
//! - 牌面格式: `1万 2万 3万 ... 东 南 西 北 中 发 白`
//! - 支持胡牌检测和听牌分析
//!
//! ## 扑克
//! - 牌面格式: `Ah Kd Qs Jc 10h`（花色: h红心 d方块 s黑桃 c梅花）
//! - 支持德州扑克牌型评估
//!
//! ## 斗地主
//! - 牌面格式: `3s 4h 10d Jc 2s X D`（X小王 D大王）
//! - 支持牌型识别
//!
//! ## 象棋
//! - 命令: `wr validate chess <棋子> <起点> <终点>`
//! - 支持走法合法性验证
//!
//! ## 数独
//! - 命令: `wr validate sudoku <81位数字或.>`
//! - 支持网格合法性验证

use std::env;
use world_rules::rules::core::{RuleCategory, RuleMetadata};
use world_rules::rules::games::mahjong::Hand;

/// JSON 输出结构
///
/// 用于将规则信息序列化为 JSON 格式输出。
///
/// # Fields
///
/// - `name`: 规则名称
/// - `category`: 规则分类（如 "games/mahjong"）
/// - `origin`: 规则来源/地区
/// - `type_name`: 分类显示名称（如 "🎮 游戏规则"）
/// - `version`: 规则版本
/// - `tags`: 规则标签列表
/// - `description`: 规则简短描述
/// - `explain`: 规则详细说明
///
/// # Examples
///
/// ```rust,no_run
/// use world_rules::rules::core::RuleMetadata;
///
/// let meta = RuleMetadata::new("四川麻将", "血战到底规则");
/// // 创建 RuleJson 实例用于 JSON 输出
/// ```
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

/// 将规则元数据转换为简化 JSON 结构
///
/// # Parameters
///
/// - `cat`: 分类字符串（如 "games"）
/// - `meta`: 规则元数据
/// - `explain`: 规则详细说明
///
/// # Examples
///
/// ```rust,no_run
/// use world_rules::rules::core::RuleMetadata;
///
/// let meta = RuleMetadata::new("围棋", "围棋游戏规则");
/// // let json = to_rule_json("games", &meta, "详细说明...");
/// ```
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

/// 将 RuleCategory 枚举转换为路径字符串
///
/// # Parameters
///
/// - `cat`: 规则分类枚举
///
/// # Returns
///
/// 返回分类路径字符串，如 "games/mahjong"、"sports/football"。
///
/// # Examples
///
/// ```rust,no_run
/// use world_rules::rules::core::RuleCategory;
///
/// let cat = RuleCategory::Games("mahjong".to_string());
/// // let path = rule_cat_str(&cat); // "games/mahjong"
/// ```
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

/// 获取规则分类的显示类型名称
///
/// # Parameters
///
/// - `cat`: 规则分类枚举
///
/// # Returns
///
/// 返回带 emoji 的分类显示名称，如 "🎮 游戏规则"、"🏃 体育规则"。
///
/// # Examples
///
/// ```rust,no_run
/// use world_rules::rules::core::RuleCategory;
///
/// let cat = RuleCategory::Games("mahjong".to_string());
/// // let name = rule_cat_type(&cat); // "🎮 游戏规则"
/// ```
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

/// 将规则转换为完整 JSON 结构（包含分类路径）
///
/// # Parameters
///
/// - `cat`: 规则分类枚举
/// - `meta`: 规则元数据
/// - `explain`: 规则详细说明
///
/// # Returns
///
/// 返回包含完整分类信息的 JSON 结构。
///
/// # Examples
///
/// ```rust,no_run
/// use world_rules::rules::core::{RuleCategory, RuleMetadata};
///
/// let cat = RuleCategory::Games("mahjong".to_string());
/// let meta = RuleMetadata::new("四川麻将", "血战到底");
/// // let json = rule_to_json(&cat, &meta, "详细说明...");
/// ```
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
        "count" => cmd_count(),
        "export" => cmd_export(&args[2..]),
        "web" => cmd_web(),
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
  wr validate chess <棋子> <起点> <终点>

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
  wr validate chess 车 0,0 0,5

麻将牌面格式: 1万 2万 3万 ... 东 南 西 北 中 发 白
扑克牌面格式: Ah Kd Qs Jc 10h (花色: h红心 d方块 s黑桃 c梅花)
斗地主牌面格式: 3s 4h 10d Jc 2s X D (s黑桃 h红心 d方块 c梅花, X小王 D大王)"#,
        ver = env!("CARGO_PKG_VERSION"),
    );
}

/// 解析命令行参数中的 flag 值
///
/// # Parameters
///
/// - `args`: 命令行参数切片
/// - `flag`: 要查找的 flag 名称（如 "--category"）
///
/// # Returns
///
/// 如果找到 flag 且其后有参数，返回该参数值；否则返回 `None`。
///
/// # Examples
///
/// ```rust,no_run
/// let args = vec![
///     String::from("--category"),
///     String::from("games"),
/// ];
/// let category = parse_flag(&args, "--category"); // Some("games")
/// let missing = parse_flag(&args, "--search"); // None
/// ```
fn parse_flag(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}

/// 收集所有规则到统一列表
///
/// 从各个分类模块收集所有规则，返回包含元数据、分类和说明的元组列表。
///
/// # Returns
///
/// 返回 `Vec<(&str, RuleMetadata, RuleCategory, String)>`，其中：
/// - 第一个元素是分类字符串（如 "games"）
/// - 第二个元素是规则元数据
/// - 第三个元素是规则分类枚举
/// - 第四个元素是规则详细说明
///
/// # Examples
///
/// ```rust,no_run
/// let all_rules = collect_all_rules();
/// println!("共有 {} 条规则", all_rules.len());
/// ```
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

/// 获取分类的中文显示名称
///
/// # Parameters
///
/// - `cat`: 分类字符串（如 "games"）
///
/// # Returns
///
/// 返回带 emoji 的分类显示名称。
///
/// # Examples
///
/// ```rust,no_run
/// let name = cat_name("games"); // "🎮 游戏规则"
/// let name = cat_name("sports"); // "🏃 体育规则"
/// ```
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

/// 执行 list 命令 - 列出所有规则
///
/// 支持按分类、关键词和标签过滤规则列表。
///
/// # Parameters
///
/// - `args`: 命令行参数，支持以下 flags：
///   - `--category <分类>`: 按分类过滤（如 games、sports）
///   - `--search <关键词>`: 搜索名称或描述
///   - `--tag <标签>`: 按标签过滤
///   - `--json`: 输出 JSON 格式
///
/// # Examples
///
/// ```bash
/// # 列出所有规则
/// wr list
///
/// # 按分类过滤
/// wr list --category games
///
/// # 搜索关键词
/// wr list --search 麻将
///
/// # 按标签过滤
/// wr list --tag 扑克
///
/// # JSON 输出
/// wr list --json
/// ```
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

/// 执行 show 命令 - 显示规则详情
///
/// 显示指定规则的完整信息，包括名称、分类、版本、来源、标签和详细说明。
///
/// # Parameters
///
/// - `args`: 命令行参数
///   - 第一个参数: 规则名称（支持模糊匹配）
///   - `--json`: 输出 JSON 格式
///
/// # Matching Strategy
///
/// 1. 精确匹配（忽略后缀"规则"）
/// 2. 前缀匹配
/// 3. 模糊匹配（名称或描述包含关键词）
///
/// # Examples
///
/// ```bash
/// # 精确查询
/// wr show 围棋
///
/// # 别名查询
/// wr explain 德州扑克
///
/// # JSON 输出
/// wr show 麻将 --json
/// ```
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

/// 执行 export 命令 - 导出规则数据
///
/// 将所有规则导出到指定格式文件。
///
/// # Parameters
///
/// - `args`: 导出格式参数
///   - `json`: 导出为 JSON 格式
///   - `html` / `web`: 导出为交互式 HTML
///   - `md` / `markdown`: 导出为 Markdown 文档
///
/// # Examples
///
/// ```bash
/// # 导出 JSON
/// wr export json > rules.json
///
/// # 导出 HTML
/// wr export html
///
/// # 导出 Markdown
/// wr export md > rules.md
/// ```
fn cmd_export(args: &[String]) {
    let format = if args.is_empty() { "json" } else { &args[0] };
    let all = collect_all_rules();
    match format {
        "json" => {
            let items: Vec<_> = all
                .iter()
                .map(|(_, meta, cat, _)| {
                    serde_json::json!({
                        "name": meta.name,
                        "description": meta.description,
                        "category": format!("{}", cat),
                        "origin": meta.origin,
                        "tags": meta.tags,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&items).unwrap());
        }
        "html" | "web" => generate_web(&all),
        "md" | "markdown" => generate_markdown(&all),
        _ => {
            eprintln!("不支持的格式: {}", format);
            eprintln!("支持: json, html, md");
        }
    }
}

fn generate_web(all: &[(&str, RuleMetadata, RuleCategory, String)]) {
    use std::io::Write;
    let mut f = std::fs::File::create("world-rules.html").unwrap();
    writeln!(
        f,
        r#"<!DOCTYPE html><html lang="zh-CN"><head><meta charset="UTF-8">"#
    )
    .unwrap();
    writeln!(
        f,
        r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#
    )
    .unwrap();
    writeln!(f, r#"<title>World Rules</title><style>"#).unwrap();
    writeln!(f, "*{{margin:0;padding:0;box-sizing:border-box}}").unwrap();
    writeln!(
        f,
        "body{{font-family:system-ui;background:#f5f5f5;color:#333}}"
    )
    .unwrap();
    writeln!(f, ".hdr{{background:linear-gradient(135deg,#667eea,#764ba2);color:#fff;padding:2rem;text-align:center}}").unwrap();
    writeln!(f, ".hdr h1{{font-size:2rem}}.stats{{display:flex;justify-content:center;gap:2rem;margin:1rem 0}}").unwrap();
    writeln!(
        f,
        ".sn b{{font-size:1.5rem;display:block}}.sn span{{font-size:.85rem;opacity:.8}}"
    )
    .unwrap();
    writeln!(
        f,
        ".box{{max-width:1200px;margin:2rem auto;padding:0 1rem}}"
    )
    .unwrap();
    writeln!(f, ".sb{{width:100%;padding:.75rem;font-size:1rem;border:2px solid #ddd;border-radius:8px;margin-bottom:1.5rem}}").unwrap();
    writeln!(f, ".sb:focus{{border-color:#667eea;outline:none}}").unwrap();
    writeln!(f, ".cat{{margin-bottom:2rem}}.cat h2{{font-size:1.2rem;border-bottom:2px solid #667eea;display:inline-block;padding-bottom:.3rem}}").unwrap();
    writeln!(
        f,
        ".g{{display:grid;grid-template-columns:repeat(auto-fill,minmax(300px,1fr));gap:1rem}}"
    )
    .unwrap();
    writeln!(
        f,
        ".c{{background:#fff;border-radius:8px;padding:1rem;box-shadow:0 2px 4px rgba(0,0,0,.1)}}"
    )
    .unwrap();
    writeln!(
        f,
        ".c:hover{{transform:translateY(-2px);box-shadow:0 4px 8px rgba(0,0,0,.15)}}"
    )
    .unwrap();
    writeln!(
        f,
        ".c h3{{font-size:.95rem;margin-bottom:.4rem}}.c p{{font-size:.85rem;color:#666}}"
    )
    .unwrap();
    writeln!(
        f,
        ".t{{display:flex;flex-wrap:wrap;gap:.2rem;margin-top:.4rem}}"
    )
    .unwrap();
    writeln!(f, ".tg{{background:#e8eaf6;color:#3949ab;padding:.1rem .4rem;border-radius:10px;font-size:.7rem}}").unwrap();
    writeln!(f, ".o{{color:#999;font-size:.7rem}}").unwrap();
    writeln!(f, "</style></head><body>").unwrap();

    writeln!(
        f,
        r#"<div class="hdr"><h1>World Rules</h1><p>世界规则库</p><div class="stats">"#
    )
    .unwrap();
    writeln!(
        f,
        r#"<div class="sn"><b>{}</b><span>规则总数</span></div>"#,
        all.len()
    )
    .unwrap();
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (_, _, cat, _) in all {
        *counts.entry(format!("{}", cat)).or_insert(0) += 1;
    }
    for (cat, cnt) in &counts {
        writeln!(
            f,
            r#"<div class="sn"><b>{}</b><span>{}</span></div>"#,
            cnt, cat
        )
        .unwrap();
    }
    writeln!(f, "</div></div><div class=\"box\">").unwrap();
    writeln!(f, r#"<input type="text" class="sb" id="q" placeholder="搜索规则..." oninput="document.querySelectorAll('.c').forEach(c=>{{c.style.display=c.dataset.n.includes(this.value.toLowerCase())?'':'none'}})">"#).unwrap();

    let mut groups: std::collections::HashMap<
        String,
        Vec<&(&str, RuleMetadata, RuleCategory, String)>,
    > = std::collections::HashMap::new();
    for item in all {
        groups.entry(format!("{}", item.2)).or_default().push(item);
    }
    for (cat, items) in &groups {
        writeln!(
            f,
            r#"<div class="cat"><h2>{} ({})</h2><div class="g">"#,
            cat,
            items.len()
        )
        .unwrap();
        for (_, meta, _, _) in items {
            let tags: String = meta
                .tags
                .iter()
                .map(|t| format!(r#"<span class="tg">{}</span>"#, t))
                .collect();
            let o = meta.origin.as_deref().unwrap_or("");
            let ohtml = if o.is_empty() {
                String::new()
            } else {
                format!(r#"<span class="o">{}</span>"#, o)
            };
            writeln!(
                f,
                r#"<div class="c" data-n="{}"><h3>{}</h3><p>{}</p><div class="t">{}{}</div></div>"#,
                meta.name.to_lowercase(),
                meta.name,
                meta.description,
                tags,
                ohtml
            )
            .unwrap();
        }
        writeln!(f, "</div></div>").unwrap();
    }
    writeln!(f, "</div></body></html>").unwrap();
    println!("已生成 world-rules.html ({} 条规则)", all.len());
}

fn generate_markdown(all: &[(&str, RuleMetadata, RuleCategory, String)]) {
    use std::io::Write;
    let mut f = std::fs::File::create("world-rules.md").unwrap();
    writeln!(f, "# World Rules\n\n共 {} 条规则\n", all.len()).unwrap();
    let mut groups: std::collections::HashMap<
        String,
        Vec<&(&str, RuleMetadata, RuleCategory, String)>,
    > = std::collections::HashMap::new();
    for item in all {
        groups.entry(format!("{}", item.2)).or_default().push(item);
    }
    for (cat, items) in &groups {
        writeln!(f, "## {} ({})\n", cat, items.len()).unwrap();
        for (_, meta, _, _) in items {
            let o = meta.origin.as_deref().unwrap_or("");
            let tags = if meta.tags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", meta.tags.join(", "))
            };
            let ostr = if o.is_empty() {
                String::new()
            } else {
                format!(" ({})", o)
            };
            writeln!(
                f,
                "- **{}**{}{}: {}",
                meta.name, ostr, tags, meta.description
            )
            .unwrap();
        }
        writeln!(f).unwrap();
    }
    println!("已生成 world-rules.md ({} 条规则)", all.len());
}

fn cmd_web() {
    let all = collect_all_rules();
    generate_web(&all);
    println!("用浏览器打开 world-rules.html 即可浏览");
}

fn cmd_count() {
    let all = collect_all_rules();
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for (cat, _, _, _) in &all {
        *counts.entry(cat).or_insert(0) += 1;
    }
    let total: usize = counts.values().sum();
    let q = '"';
    println!("{{");
    for cat in &["games", "sports", "social", "science", "law", "health"] {
        let count = counts.get(cat).copied().unwrap_or(0);
        println!("  {q}{cat}{q}: {count},");
    }
    println!("  {q}total{q}: {total}");
    println!("}}");
}

/// 执行 stats 命令 - 显示统计信息
///
/// 显示规则库的总体统计，包括各分类规则数量和版本信息。
///
/// # Examples
///
/// ```bash
/// wr stats
/// ```
///
/// 输出示例：
/// ```
/// ╔═══════════════════════════════════════╗
/// ║        世界规则库 - 统计信息          ║
/// ╠═══════════════════════════════════════╣
/// ║ 🎮 游戏规则  120 ████████████
/// ║ 🏃 体育规则   80 ████████
/// ║ ...
/// ╚═══════════════════════════════════════╝
/// ```
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

/// 执行 validate 命令 - 验证游戏状态
///
/// 根据游戏类型验证牌面或走法的合法性。
///
/// # Supported Games
///
/// - `mahjong` / `mj` / `麻将`: 麻将胡牌验证
/// - `poker` / `德州`: 德州扑克牌型评估
/// - `doudizhu` / `ddz` / `斗地主`: 斗地主牌型识别
/// - `chess` / `象棋`: 中国象棋走法验证
/// - `sudoku` / `数独`: 数独网格验证
///
/// # Examples
///
/// ```bash
/// # 验证麻将胡牌
/// wr validate mahjong "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 2条 3条 4条 4条"
///
/// # 验证扑克牌型
/// wr validate poker "Ah Kd Qs Jc 10h"
///
/// # 验证斗地主
/// wr validate doudizhu "3s 3s 3s 4h 4h 4h"
///
/// # 验证象棋走法
/// wr validate chess 车 0,0 0,5
///
/// # 验证数独
/// wr validate sudoku "53..7....6..195....98....6.8...6..."
/// ```
fn cmd_validate(args: &[String]) {
    if args.len() < 2 {
        eprintln!("用法: wr validate <游戏类型> <牌面>");
        eprintln!("支持: mahjong, poker, doudizhu, chess");
        return;
    }
    match args[0].as_str() {
        "mahjong" | "mj" | "麻将" => cmd_validate_mahjong(&args[1]),
        "poker" | "德州" | "德州扑克" => cmd_validate_poker(&args[1]),
        "doudizhu" | "ddz" | "斗地主" => cmd_validate_doudizhu(&args[1]),
        "chess" | "象棋" | "中国象棋" => cmd_validate_chess(&args[1..]),
        "sudoku" | "数独" => cmd_validate_sudoku(&args[1]),
        other => {
            eprintln!("不支持的游戏类型: {}", other);
            eprintln!("支持: mahjong, poker, doudizhu, chess");
        }
    }
}

/// 验证麻将胡牌
///
/// 解析麻将牌面并判断是否可以胡牌，同时显示听牌信息。
///
/// # Parameters
///
/// - `tiles_str`: 麻将牌面字符串，用空格分隔
///   - 万子: 1万 2万 ... 9万
///   - 条子: 1条 2条 ... 9条
///   - 筒子: 1筒 2筒 ... 9筒（或 1饼 2饼）
///   - 风牌: 东 南 西 北
///   - 番牌: 中 发 白
///
/// # Examples
///
/// ```bash
/// # 胡牌验证（14张）
/// wr validate mahjong "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 2条 3条 4条 4条"
///
/// # 听牌查询（13张）
/// wr validate mahjong "1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 2条 3条 4条"
/// ```
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

/// 解析麻将牌面字符串
///
/// # Parameters
///
/// - `s`: 麻将牌面字符串，用空格分隔每张牌
///
/// # Returns
///
/// 成功返回 `Vec<Tile>`，失败返回错误信息。
///
/// # Supported Formats
///
/// - 数字+花色: `1万`、`2条`、`3筒`（或 `3饼`）
/// - 风牌: `东`、`南`、`西`、`北`
/// - 番牌: `中`、`发`、`白`
///
/// # Errors
///
/// - 无效数字
/// - 无效花色
/// - 格式错误
fn parse_mahjong_tiles(s: &str) -> Result<Vec<world_rules::rules::games::mahjong::Tile>, String> {
    let mut tiles = Vec::new();
    for part in s.split_whitespace() {
        let tile = parse_single_tile(part)?;
        tiles.push(tile);
    }
    Ok(tiles)
}

/// 解析单张麻将牌
///
/// # Parameters
///
/// - `s`: 单张牌的字符串表示
///
/// # Returns
///
/// 成功返回 `Tile`，失败返回错误信息。
///
/// # Examples
///
/// ```rust,no_run
/// // let tile = parse_single_tile("1万"); // Tile::Wan(1)
/// // let tile = parse_single_tile("东"); // Tile::Feng(Wind::Dong)
/// ```
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

/// 验证扑克牌型
///
/// 解析扑克牌面并评估牌型等级。
///
/// # Parameters
///
/// - `cards_str`: 扑克牌面字符串，用空格分隔
///   - 点数: A、K、Q、J、10、9、8、7、6、5、4、3、2
///   - 花色: h（红心♥）、d（方块♦）、s（黑桃♠）、c（梅花♣）
///
/// # Examples
///
/// ```bash
/// # 同花顺
/// wr validate poker "Ah Kh Qh Jh 10h"
///
/// # 皇家同花顺
/// wr validate poker "As Ks Qs Js 10s"
///
/// # 两对
/// wr validate poker "Ah Ad Ks Kh 2c"
/// ```
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

/// 验证斗地主牌型
///
/// 解析斗地主牌面并识别牌型。
///
/// # Parameters
///
/// - `cards_str`: 斗地主牌面字符串，用空格分隔
///   - 点数: 3、4、5、6、7、8、9、10、J、Q、K、A、2
///   - 花色: s（黑桃）、h（红心）、d（方块）、c（梅花）
///   - 王牌: X（小王）、D（大王）
///
/// # Examples
///
/// ```bash
/// # 三张带一对（炸弹）
/// wr validate doudizhu "3s 3s 3s 4h 4h 4h"
///
/// # 火箭（双王）
/// wr validate doudizhu "X D"
///
/// # 顺子
/// wr validate doudizhu "3s 4h 5d 6c 7s"
/// ```
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

/// 验证数独网格
///
/// 验证数独网格是否符合规则（无冲突）。
///
/// # Parameters
///
/// - `grid_str`: 数独网格字符串（81个字符）
///   - 数字 1-9 表示已填
///   - `.` 表示空格
///
/// # Examples
///
/// ```bash
/// # 验证部分填写的数独
/// wr validate sudoku "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28."
/// ```
fn cmd_validate_sudoku(grid_str: &str) {
    use world_rules::rules::core::ValidateContext;
    use world_rules::rules::games::sudoku::SudokuRules;
    use world_rules::rules::Rule;

    let grid: String = grid_str
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    if grid.len() != 81 {
        eprintln!(
            "数独网格需要81个字符(数字1-9或.表示空格)，当前{}个",
            grid.len()
        );
        return;
    }
    let rules = SudokuRules::new();
    let ctx = ValidateContext::Generic(grid);
    match rules.validate(&ctx) {
        Ok(true) => println!("数独网格合法"),
        Ok(false) => println!("数独网格不合法(存在冲突)"),
        Err(e) => eprintln!("验证错误: {}", e),
    }
}

/// 验证象棋走法
///
/// 验证中国象棋走法的合法性。
///
/// # Parameters
///
/// - `args`: 包含棋子、起点和终点的参数数组
///   - 棋子: 车、马、象（相）、士（仕）、将（帅）、炮、兵（卒）
///   - 起点/终点: 坐标格式（如 0,0）
///
/// # Examples
///
/// ```bash
/// # 验证车从 (0,0) 走到 (0,5)
/// wr validate chess 车 0,0 0,5
///
/// # 验证马从 (2,1) 走到 (4,2)
/// wr validate chess 马 2,1 4,2
///
/// # 使用英文名称
/// wr validate chess Rook 0,0 0,5
/// ```
fn cmd_validate_chess(args: &[String]) {
    use world_rules::prelude::Rule;
    use world_rules::rules::core::ValidateContext;
    use world_rules::rules::games::board_games::chinese_chess::ChineseChessRules;

    if args.len() < 3 {
        eprintln!("用法: wr validate chess <棋子> <起点> <终点>");
        eprintln!("示例: wr validate chess 车 0,0 0,5");
        eprintln!("棋子: 车 马 象/相 士/仕 将/帅 炮 兵/卒");
        eprintln!("      Rook Horse Elephant Advisor King Cannon Pawn");
        return;
    }

    let ctx = ValidateContext::chess_move(&args[0], &args[1], &args[2]);
    let rules = ChineseChessRules::new();

    match rules.validate(&ctx) {
        Ok(true) => {
            println!("✅ 合法走法: {} {} → {}", args[0], args[1], args[2]);
        }
        Ok(false) => {
            println!("❌ 非法走法: {} {} → {}", args[0], args[1], args[2]);
        }
        Err(e) => {
            eprintln!("验证失败: {}", e);
        }
    }
}

/// 解析扑克牌面字符串
///
/// # Parameters
///
/// - `s`: 扑克牌面字符串，用空格分隔每张牌
///
/// # Returns
///
/// 成功返回 `Vec<Card>`，失败返回错误信息。
///
/// # Supported Formats
///
/// - 点数+花色: `Ah`、`Kd`、`Qs`、`Jc`、`10h`
/// - 支持 Unicode 花色符号: `A♥`、`K♦`
///
/// # Errors
///
/// - 无效点数
/// - 无效花色
/// - 格式错误
fn parse_poker_cards(s: &str) -> Result<Vec<world_rules::rules::games::card_games::Card>, String> {
    let mut cards = Vec::new();
    for part in s.split_whitespace() {
        let card = parse_single_card(part)?;
        cards.push(card);
    }
    Ok(cards)
}

/// 解析单张扑克牌
///
/// # Parameters
///
/// - `s`: 单张牌的字符串表示（如 `Ah`、`10h`）
///
/// # Returns
///
/// 成功返回 `Card`，失败返回错误信息。
///
/// # Examples
///
/// ```rust,no_run
/// // let card = parse_single_card("Ah"); // Card { suit: Heart, rank: Ace }
/// // let card = parse_single_card("10s"); // Card { suit: Spade, rank: Ten }
/// ```
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
