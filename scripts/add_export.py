import sys

with open('src/bin/wr.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Add export and web commands
content = content.replace(
    '"count" => cmd_count(),',
    '"count" => cmd_count(),\n        "export" => cmd_export(&args[2..]),\n        "web" => cmd_web(),'
)

# Add cmd_export function before cmd_count
export_fn = r'''
fn cmd_export(args: &[String]) {
    let format = if args.is_empty() { "json" } else { &args[0] };
    let all = collect_all_rules();
    match format {
        "json" => export_json(&all),
        "html" => export_html(&all),
        "md" | "markdown" => export_markdown(&all),
        _ => {
            eprintln!("不支持的格式: {}", format);
            eprintln!("支持: json, html, md");
        }
    }
}

fn export_json(all: &[(String, RuleMetadata, RuleCategory, String)]) {
    let items: Vec<_> = all.iter().map(|(_, meta, cat, _)| {
        serde_json::json!({
            "name": meta.name,
            "description": meta.description,
            "category": format!("{}", cat),
            "origin": meta.origin,
            "tags": meta.tags,
            "version": meta.version,
        })
    }).collect();
    println!("{}", serde_json::to_string_pretty(&items).unwrap());
}

fn export_html(all: &[(String, RuleMetadata, RuleCategory, String)]) {
    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="zh-CN"><head><meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>World Rules</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
body{font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif;background:#f5f5f5;color:#333}
.hdr{background:linear-gradient(135deg,#667eea 0%,#764ba2 100%);color:#fff;padding:2rem;text-align:center}
.hdr h1{font-size:2rem;margin-bottom:.5rem}
.stats{display:flex;justify-content:center;gap:2rem;margin:1.5rem 0}
.sn{text-align:center}.sn b{font-size:1.5rem}.sn span{font-size:.85rem;opacity:.8}
.box{max-width:1200px;margin:2rem auto;padding:0 1rem}
.sbox{width:100%;padding:.75rem 1rem;font-size:1rem;border:2px solid #ddd;border-radius:8px;margin-bottom:1.5rem}
.sbox:focus{border-color:#667eea;outline:none}
.cat{margin-bottom:2rem}.cat h2{font-size:1.3rem;margin-bottom:1rem;padding-bottom:.5rem;border-bottom:2px solid #667eea;display:inline-block}
.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(300px,1fr));gap:1rem}
.card{background:#fff;border-radius:8px;padding:1.25rem;box-shadow:0 2px 4px rgba(0,0,0,.1);transition:transform .2s}
.card:hover{transform:translateY(-2px);box-shadow:0 4px 8px rgba(0,0,0,.15)}
.card h3{font-size:1rem;margin-bottom:.5rem}
.card p{font-size:.85rem;color:#666;margin-bottom:.75rem}
.tags{display:flex;flex-wrap:wrap;gap:.25rem}
.tag{background:#e8eaf6;color:#3949ab;padding:.15rem .5rem;border-radius:12px;font-size:.75rem}
.ori{color:#999;font-size:.75rem}
.ft{text-align:center;padding:2rem;color:#999;font-size:.85rem}
</style></head><body>
<div class="hdr"><h1>World Rules</h1><p>世界规则库</p><div class="stats">"#);

    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (_, _, cat, _) in all {
        *counts.entry(format!("{}", cat)).or_insert(0) += 1;
    }
    let total = all.len();
    html.push_str(&format!("<div class=\"sn\"><b>{}</b><span>规则总数</span></div>", total));
    for (cat, count) in &counts {
        html.push_str(&format!("<div class=\"sn\"><b>{}</b><span>{}</span></div>", count, cat));
    }

    html.push_str("</div></div><div class=\"box\">");
    html.push_str("<input type=\"text\" class=\"sbox\" id=\"q\" placeholder=\"搜索规则...\" oninput=\"f()\">\n");

    let mut groups: std::collections::HashMap<String, Vec<&(String, RuleMetadata, RuleCategory, String)>> = std::collections::HashMap::new();
    for item in all {
        let key = format!("{}", item.2);
        groups.entry(key).or_default().push(item);
    }

    for (cat, items) in &groups {
        let icon = match cat.split('/').next().unwrap_or("") {
            "Games" => "\u{1f3ae}",
            "Sports" => "\u{1f3c3}",
            "Social" => "\u{1f91d}",
            "Science" => "\u{1f52c}",
            "Law" => "\u{2696}",
            "Health" => "\u{1f3e5}",
            _ => "\u{1f4cb}",
        };
        html.push_str(&format!("<div class=\"cat\"><h2>{} {} ({})</h2><div class=\"grid\">", icon, cat, items.len()));
        for (_, meta, _, _) in items {
            let tags_html: String = meta.tags.iter().map(|t| format!("<span class=\"tag\">{}</span>", t)).collect();
            let origin = meta.origin.as_deref().unwrap_or("");
            let ori = if origin.is_empty() { String::new() } else { format!("<span class=\"ori\">{}</span>", origin) };
            html.push_str(&format!(
                "<div class=\"card\" data-n=\"{}\" data-d=\"{}\"><h3>{}</h3><p>{}</p><div class=\"tags\">{}{}</div></div>",
                meta.name.to_lowercase(), meta.description.to_lowercase(),
                meta.name, meta.description, tags_html, ori
            ));
        }
        html.push_str("</div></div>");
    }

    html.push_str("</div><div class=\"ft\"><p>Generated by World Rules v");
    html.push_str(env!("CARGO_PKG_VERSION"));
    html.push_str("</p></div><script>function f(){const q=document.getElementById('q').value.toLowerCase();document.querySelectorAll('.card').forEach(c=>{c.style.display=(c.dataset.n.includes(q)||c.dataset.d.includes(q))?'':'none'})}</script></body></html>");

    std::fs::write("world-rules.html", &html).unwrap();
    println!("已生成 world-rules.html ({} 条规则)", total);
}

fn export_markdown(all: &[(String, RuleMetadata, RuleCategory, String)]) {
    let mut md = String::from("# World Rules\n\n");
    md.push_str(&format!("共 {} 条规则\n\n", all.len()));
    let mut groups: std::collections::HashMap<String, Vec<&(String, RuleMetadata, RuleCategory, String)>> = std::collections::HashMap::new();
    for item in all {
        let key = format!("{}", item.2);
        groups.entry(key).or_default().push(item);
    }
    for (cat, items) in &groups {
        md.push_str(&format!("## {} ({})\n\n", cat, items.len()));
        for (_, meta, _, _) in items {
            let origin = meta.origin.as_deref().unwrap_or("");
            let tags = if meta.tags.is_empty() { String::new() } else { format!(" [{}]", meta.tags.join(", ")) };
            let ori = if origin.is_empty() { String::new() } else { format!(" ({})", origin) };
            md.push_str(&format!("- **{}**{}{}: {}\n", meta.name, ori, tags, meta.description));
        }
        md.push('\n');
    }
    std::fs::write("world-rules.md", &md).unwrap();
    println!("已生成 world-rules.md ({} 条规则)", all.len());
}

fn cmd_web() {
    let all = collect_all_rules();
    export_html(&all);
    println!("用浏览器打开 world-rules.html 即可浏览");
}

'''

content = content.replace('fn cmd_count()', export_fn + 'fn cmd_count()')

with open('src/bin/wr.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print('Added export and web commands')
