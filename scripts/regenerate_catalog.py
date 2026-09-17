#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""用 wr list --json 重新生成 world-rules.md 规则目录文档"""
import json, subprocess, sys
from collections import defaultdict

out = subprocess.run(["./target/debug/wr", "list", "--json"],
                     capture_output=True, text=True, cwd="/root/world-rules")
if out.returncode != 0:
    print("wr list 失败:", out.stderr); sys.exit(1)
rules = json.loads(out.stdout)

CAT_ORDER = {"games":"游戏规则","sports":"体育规则","social":"社交礼仪",
             "science":"科学定律","law":"法律法规","health":"健康规则"}
groups = defaultdict(list)
for r in rules:
    groups[r["category"]].append(r)

total = len(rules)
lines = ["# World Rules", "", f"共 {total} 条规则", ""]
order = ["games","sports","social","science","law","health"]
for cat in order:
    if cat not in groups: continue
    items = sorted(groups[cat], key=lambda x: x["name"])
    label = CAT_ORDER.get(cat, cat)
    lines.append(f"# {label} ({len(items)})"); lines.append("")
    for r in items:
        name = r["name"]; origin = r.get("origin","")
        tags = ", ".join(r.get("tags", []))
        desc = r.get("description", "").strip()
        meta = f" ({origin})" if origin else ""
        tagbit = f" [{tags}]" if tags else ""
        lines.append(f"## {name}{meta}{tagbit}")
        if desc:
            lines.append("")
            lines.append(f"- **{name}**: {desc}")
        lines.append("")
    lines.append("")

md = "\n".join(lines)
with open("/root/world-rules/world-rules.md", "w", encoding="utf-8") as f:
    f.write(md)
print(f"OK: 共 {total} 条规则, 写入 {md.count(chr(10))} 行, {len(md)} 字符")