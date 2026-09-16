#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
world-rules 批量规则生成器（最终版）
- 依据项目 simple_rule! + impl Rule + all_rules() 约定生成 .rs 文件
- 自动在 mod.rs 注册 pub mod / pub use / 并追加到 all_rules()
用法:
    python3 scripts/wrgen.py <cat> <struct> <name> <desc> <file.rs 相对路径>
    然后从 stdin 读入 sections 定义（JSON 数组）
输出 rustfmt 稳定的多行格式；建议生成后运行 cargo fmt 兜底。
"""
import sys, os, json, re

BASE = os.path.dirname(os.path.abspath(__file__)) + "/.."
RULES_DIR = os.path.join(BASE, "src", "rules")


def gen_file(struct, name, desc, origin, tags, category, sections):
    """sections: list of (title, method, [items])"""
    L = []
    L.append("//! %s\n//!\n//! %s" % (name, desc))
    L.append("")
    L.append("use crate::rules::core::{Rule, RuleCategory, RuleMetadata};")
    L.append("use crate::simple_rule;")
    L.append("")
    L.append("simple_rule! {")
    L.append("    struct: %s," % struct)
    L.append("    name: \"%s\"," % name)
    L.append("    desc: \"%s\"," % desc)
    L.append("    origin: \"%s\"," % origin)
    L.append("    tags: [%s]" % ", ".join('"%s"' % t for t in tags))
    L.append("}")
    L.append("")
    L.append("impl %s {" % struct)
    for title, method, items in sections:
        L.append("    /// %s" % title)
        L.append("    pub fn %s(&self) -> Vec<&'static str> {" % method)
        L.append("        vec![")
        for it in items:
            L.append("            \"%s\"," % it)
        L.append("        ]")
        L.append("    }")
        L.append("")
    L.append("}")
    L.append("")
    L.append("impl Rule for %s {" % struct)
    L.append("    fn metadata(&self) -> &RuleMetadata {")
    L.append("        &self.metadata")
    L.append("    }")
    L.append("")
    L.append("    fn category(&self) -> RuleCategory {")
    L.append("        %s" % category)
    L.append("    }")
    L.append("")
    L.append("    fn explain(&self) -> String {")
    L.append("        let parts = vec![")
    for title, method, _ in sections:
        L.append(
            "            format!(\n"
            '                "%s：\\\\n{}",\n' % title +
            "                self.%s()\n" % method +
            "                    .iter()\n"
            "                    .map(|s| format!(\"  • {}\", s))\n"
            "                    .collect::<Vec<_>>()\n"
            '                    .join("\\\\n")\n'
            "            ),"
        )
    L.append("        ];")
    L.append("        format!(\"【%s】\\n{}\", parts.join(\"\\n\\n\"))" % name)
    L.append("    }")
    L.append("}")
    L.append("")
    L.append("#[cfg(test)]")
    L.append("mod tests {")
    L.append("    use super::*;")
    L.append("    use crate::rules::core::ValidateContext;")
    L.append("")
    L.append("    #[test]")
    L.append("    fn test_%s_basic() {" % struct.lower())
    L.append("        let rules = %s::new();" % struct)
    L.append("        assert_eq!(rules.metadata().name, \"%s\");" % name)
    for _, method, _ in sections:
        L.append("        assert!(!rules.%s().is_empty());" % method)
    L.append("    }")
    L.append("")
    L.append("    #[test]")
    L.append("    fn test_%s_validation() {" % struct.lower())
    L.append("        let rules = %s::new();" % struct)
    L.append("        assert!(rules")
    L.append("            .validate(&ValidateContext::Generic(\"test\".to_string()))")
    L.append("            .is_ok());")
    L.append("        assert_eq!(rules.category(), %s);" % category)
    L.append("    }")
    L.append("")
    L.append("    #[test]")
    L.append("    fn test_%s_explain() {" % struct.lower())
    L.append("        let rules = %s::new();" % struct)
    L.append("        let e = rules.explain();")
    for title, _, _ in sections[:3]:
        L.append("        assert!(e.contains(\"%s\"));" % title)
    L.append("    }")
    L.append("}")
    return "\n".join(L) + "\n"


def register(cat, modname, struct):
    """在 cat 的 mod.rs 中注册 pub mod / pub use，并把 struct 追加进 all_rules()"""
    mod_path = os.path.join(RULES_DIR, cat, "mod.rs")
    with open(mod_path, "r", encoding="utf-8") as f:
        content = f.read()
    changed = False

    # 1) pub mod
    if "pub mod %s;" % modname not in content:
        # 插到现有 pub mod 块中，保持字母序
        mods = re.findall(r'^pub mod (\w+);', content, re.M)
        mods_sorted = sorted(mods + [modname])
        # 若文件已带 all_rules 在后面，我们应该把新 pub mod 插在前面字母序正确位置
        lines = content.split("\n")
        insert_idx = None
        # 找第一个 pub mod 行之后，保持字母序插入
        for i, ln in enumerate(lines):
            m = re.match(r'^pub mod \w+;', ln)
            if m and ln.strip() > ("pub mod %s;" % modname):
                insert_idx = i
                break
        if insert_idx is None:
            # 追加在所有 pub mod 之后、pub use 之前
            for i, ln in enumerate(lines):
                if ln.startswith("pub use"):
                    insert_idx = i
                    break
        if insert_idx is None:
            insert_idx = len(lines)
        lines.insert(insert_idx, "pub mod %s;" % modname)
        content = "\n".join(lines)
        changed = True

    # 2) pub use
    use_line = "pub use %s::%s;" % (modname, struct)
    if use_line not in content:
        use_lines = re.findall(r'^pub use \w+::\w+;', content, re.M)
        if use_lines:
            # 插到 pub use 块
            sorted_uses = sorted(use_lines + [use_line])
            # 直接追加在最后一个 pub use 之后
            lines = content.split("\n")
            last_use = None
            for i, ln in enumerate(lines):
                if ln.startswith("pub use "):
                    last_use = i
            lines.insert(last_use + 1, use_line)
            content = "\n".join(lines)
        else:
            content = content.rstrip() + "\n" + use_line + "\n"
        changed = True

    # 3) 追加进 all_rules() —— 在函数体内 let mut rules 之后插入
    if f"{struct}::new()" not in content:
        m = re.search(r'pub fn all_rules\(\)[^{]*\{\n(.*?)\n    \}',
                      content, re.DOTALL)
        anchor = re.search(r'let mut rules = Vec::new\(\);', content)
        if anchor:
            newblock = (
                "    {\n"
                "        let r = %s::new();\n"
                "        rules.push((\"%s\", r.metadata().clone(), r.category(), r.explain()));\n"
                "    }" % (struct, cat)
            )
            pos = anchor.end()
            content = content[:pos] + "\n" + newblock + content[pos:]
            changed = True
    if changed:
        with open(mod_path, "w", encoding="utf-8") as f:
            f.write(content)
        return True
    return False


def main():
    # 从 stdin 读取定义 JSON
    spec = json.load(sys.stdin)
    cat = spec["cat"]
    file_name = spec["file"]
    struct = spec["struct"]
    name = spec["name"]
    desc = spec["desc"]
    origin = spec.get("origin", "国际")
    tags = spec.get("tags", [])
    category = spec.get("category", 'RuleCategory::social("generic")')
    sections = [(s.get("title", s["method"]), s["method"], s["items"]) for s in spec["sections"]]

    content = gen_file(struct, name, desc, origin, tags, category, sections)
    out = os.path.join(RULES_DIR, cat, file_name + ".rs")
    if not os.path.isdir(os.path.dirname(out)):
        os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        f.write(content)

    mod_name = file_name.split("/")[0] if "/" in file_name else file_name
    register(cat, mod_name, struct)
    print("WROTE", out, "mod", mod_name)


if __name__ == "__main__":
    main()