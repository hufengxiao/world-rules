#!/usr/bin/env python3
"""批量生成规则模块 - 科学+法律+体育追加"""
import os

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
    c = '''//! ''' + cn + '''
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ''' + S + ''', name: "''' + cn + '''", desc: "''' + desc + '''", origin: "''' + origin + '''", tags: [''' + tag_s + '''] }
impl ''' + S + ''' {
''' + meth + '''}
impl Rule for ''' + S + ''' {
    fn metadata(&self) -> &RuleMetadata { &self.metadata }
    fn category(&self) -> RuleCategory { RuleCategory::''' + cat + '''("''' + name + '''") }
    fn validate(&self, ctx: &str) -> RuleResult<bool> { Ok(!ctx.is_empty()) }
    fn explain(&self) -> String { format_rule_sections("''' + cn + '''", &[''' + calls + ''']) }
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
    print(f'{cat}: +{len(items)}')
