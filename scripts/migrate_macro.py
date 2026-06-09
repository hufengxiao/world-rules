#!/usr/bin/env python3
"""Migrate simple_rule! + manual Rule impl files to the new full macro syntax.
V2: handles non-standard method names (not just section_N)."""
import re
import os
import sys

def extract_simple_rule_args(content):
    """Extract arguments from simple_rule! { ... } call."""
    pattern = r'simple_rule!\s*\{([^}]+)\}'
    m = re.search(pattern, content, re.DOTALL)
    if not m:
        return None
    body = m.group(1)
    args = {}
    sm = re.search(r'struct:\s*(\w+)', body)
    if sm: args['struct'] = sm.group(1)
    nm = re.search(r'name:\s*"([^"]*)"', body)
    if nm: args['name'] = nm.group(1)
    dm = re.search(r'desc:\s*"([^"]*)"', body)
    if dm: args['desc'] = dm.group(1)
    om = re.search(r'origin:\s*"([^"]*)"', body)
    if om: args['origin'] = om.group(1)
    tm = re.search(r'tags:\s*\[([^\]]*)\]', body)
    if tm:
        args['tags'] = re.findall(r'"([^"]*)"', tm.group(1))
    return args

def extract_category(content, struct_name):
    """Extract category from impl Rule for X { fn category() ... }"""
    pattern = r'fn category\(&self\)\s*->\s*RuleCategory\s*\{\s*(RuleCategory::\w+\([^)]+\))\s*\}'
    m = re.search(pattern, content, re.DOTALL)
    return m.group(1) if m else None

def extract_sections_from_explain(content):
    """Extract section names and function names from explain() method."""
    explain_pattern = r'fn explain\(&self\)\s*->\s*String\s*\{(.*?)\n\s*\}'
    m = re.search(explain_pattern, content, re.DOTALL)
    if not m:
        return None, []
    explain_body = m.group(1)
    title_m = re.search(r'format_rule_sections\(\s*"([^"]*)"', explain_body)
    title = title_m.group(1) if title_m else None
    sections = re.findall(r'\("([^"]*)",\s*&self\.(\w+)\(\)\)', explain_body)
    return title, sections

def extract_method_bodies(content, method_names):
    """Extract method definitions by name from impl blocks."""
    methods = []
    for name in method_names:
        # Match the full method definition
        pattern = rf'(pub fn {name}\(&self\)\s*->\s*Vec<&\'static str>\s*\{{.*?\}})'
        m = re.search(pattern, content, re.DOTALL)
        if m:
            methods.append((name, m.group(1).strip()))
    return methods

def extract_doc_comment(content):
    """Extract the //! doc comment."""
    lines = []
    for line in content.split('\n'):
        stripped = line.strip().replace('\r', '')
        if stripped.startswith('//!'):
            lines.append(stripped)
        elif lines:
            break
    return '\n'.join(lines)

def migrate_file(filepath):
    """Migrate a single file to the new macro syntax."""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    if 'simple_rule!' not in content:
        return None, "SKIP: no simple_rule!"
    if 'impl Rule for' not in content:
        return None, "SKIP: no manual Rule impl"
    
    args = extract_simple_rule_args(content)
    if not args or 'struct' not in args:
        return None, "SKIP: can't parse simple_rule! args"
    
    struct_name = args['struct']
    category = extract_category(content, struct_name)
    if not category:
        return None, f"SKIP: can't extract category for {struct_name}"
    
    title, sections = extract_sections_from_explain(content)
    
    # Get the method names we need to keep
    method_names = [fn_name for _, fn_name in sections]
    section_methods = extract_method_bodies(content, method_names)
    
    # Build new file
    doc = extract_doc_comment(content)
    tags = args.get('tags', [])
    tags_str = ', '.join(f'"{t}"' for t in tags)
    
    if sections:
        section_list = ', '.join(f'("{name}", {fn})' for name, fn in sections)
    else:
        section_list = ''
    
    new_content = f"""{doc}
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {{
    struct: {struct_name},
    name: "{args.get('name', '')}",
    desc: "{args.get('desc', '')}",
    origin: "{args.get('origin', '')}",
    tags: [{tags_str}],
    category: {category},
    sections: [{section_list}]
}}
"""
    
    # Add section method definitions if they exist
    if section_methods:
        new_content += f"\nimpl {struct_name} {{\n"
        for fn_name, fn_body in section_methods:
            # Indent properly
            new_content += f"    {fn_body}\n\n"
        new_content = new_content.rstrip() + "\n}\n"
    
    return new_content, f"OK: {struct_name}"

def main():
    src_dir = sys.argv[1] if len(sys.argv) > 1 else 'src/rules'
    
    results = {'ok': 0, 'skip': 0, 'error': 0}
    
    for root, dirs, files in os.walk(src_dir):
        for fname in sorted(files):
            if not fname.endswith('.rs') or fname == 'mod.rs':
                continue
            filepath = os.path.join(root, fname)
            try:
                new_content, msg = migrate_file(filepath)
                if new_content is None:
                    results['skip'] += 1
                else:
                    results['ok'] += 1
                    with open(filepath, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"  MIGRATED: {filepath}")
            except Exception as e:
                results['error'] += 1
                print(f"  ERROR: {filepath}: {e}")
    
    print(f"\nResults: {results['ok']} migrated, {results['skip']} skipped, {results['error']} errors")

if __name__ == '__main__':
    main()
