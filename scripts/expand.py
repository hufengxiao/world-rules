#!/usr/bin/env python3
"""规则批量生成器 - 用于 E1-E20 扩充里程碑"""
import os, sys

BASE = 'D:/Projects/world-rules/src/rules'

def to_camel(name):
    return ''.join(p.capitalize() for p in name.split('_'))

def write_mod(cat, name, cn, desc, origin, tags, sections):
    tag_s = ', '.join('"' + t + '"' for t in tags)
    S = to_camel(name) + 'Rules'
    meth = ''
    calls = ''
    for idx, (sn, items) in enumerate(sections):
        il = ', '.join('"' + i + '"' for i in items)
        meth += '    pub fn section_' + str(idx) + '(&self) -> Vec<&\'static str> { vec![' + il + '] }\n\n'
        calls += '            ("' + sn + '", &self.section_' + str(idx) + '()),\n'
    c = '//! ' + cn + '''
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ''' + S + ''', name: "''' + cn + '''", desc: "''' + desc + '''", origin: "''' + origin + '''", tags: [''' + tag_s + '''] }
impl ''' + S + ''' {
''' + meth + '''}
impl Rule for ''' + S + ''' {
    fn metadata(&self) -> &RuleMetadata { &self.metadata }
    fn category(&self) -> RuleCategory { RuleCategory::''' + cat + '''("''' + name + '''") }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> { Ok(true) }
    fn explain(&self) -> String { crate::rules::core::format_rule_sections("''' + cn + '''", &[''' + calls + ''']) }
}
#[cfg(test)]
mod tests { use super::*; #[test] fn test() { let r = ''' + S + '''::new(); assert!(!r.explain().is_empty()); } }
'''
    fp = os.path.join(BASE, cat, name + '.rs')
    os.makedirs(os.path.dirname(fp), exist_ok=True)
    with open(fp, 'w', encoding='utf-8') as f:
        f.write(c)

def run_batch(cat, items):
    for row in items:
        write_mod(cat, *row)
    return len(items)

def update_mod_rs(cat):
    """更新 mod.rs 注册新模块"""
    import re
    mod_path = os.path.join(BASE, cat, 'mod.rs')
    with open(mod_path, 'r', encoding='utf-8') as f:
        content = f.read()
    existing = set(re.findall(r'pub mod (\w+)', content))
    new_mods = ''
    new_uses = ''
    d = os.path.join(BASE, cat)
    for fn in sorted(os.listdir(d)):
        if fn.endswith('.rs') and fn != 'mod.rs':
            name = fn[:-3]
            if name not in existing:
                new_mods += 'pub mod ' + name + ';\n'
                struct = to_camel(name) + 'Rules'
                new_uses += 'pub use ' + name + '::' + struct + ';\n'
    if not new_mods:
        return 0
    lines = content.split('\n')
    last_mod = max(i for i, l in enumerate(lines) if l.strip().startswith('pub mod '))
    lines.insert(last_mod + 1, new_mods.rstrip())
    last_use = max(i for i, l in enumerate(lines) if l.strip().startswith('pub use '))
    for i in range(last_use, len(lines)):
        if lines[i].strip().endswith(';'):
            last_use = i
            break
    lines.insert(last_use + 1, new_uses.rstrip())
    with open(mod_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines))
    return len(new_mods.strip().split('\n'))
