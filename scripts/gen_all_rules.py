#!/usr/bin/env python3
"""Generate all_rules() functions for each category mod.rs.
V5: Handles sub-module qualified paths correctly."""
import re
import os
import sys

CATEGORIES = ['games', 'sports', 'social', 'science', 'law', 'health']

def extract_struct_names_from_file(filepath):
    """Extract struct names that implement Rule from a file."""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    basename = os.path.basename(filepath)
    if basename in ('mod.rs', 'core.rs'):
        return []
    structs = []
    for m in re.finditer(r'simple_rule!\s*\{[^}]*struct:\s*(\w+)', content, re.DOTALL):
        structs.append(m.group(1))
    for m in re.finditer(r'impl Rule for (\w+)', content):
        name = m.group(1)
        if name not in structs:
            structs.append(name)
    return structs

def get_exported_structs(mod_path):
    """Get all struct names exported by pub use in mod.rs."""
    with open(mod_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    exported = {}  # struct_name -> True
    has_wildcard = False
    
    for m in re.finditer(r'pub use [\w:]+::(\w+);', content):
        exported[m.group(1)] = True
    for m in re.finditer(r'pub use [\w:]+::\{([^}]+)\}', content, re.DOTALL):
        for name in re.findall(r'(\w+)', m.group(1)):
            exported[name] = True
    if re.search(r'pub use [\w:]+::\*;', content):
        has_wildcard = True
    
    return exported, has_wildcard

def get_submodules(mod_path):
    """Get list of pub mod sub-modules."""
    with open(mod_path, 'r', encoding='utf-8') as f:
        content = f.read()
    return re.findall(r'pub mod (\w+);', content)

def collect_rules_for_category(cat, rules_dir):
    """Collect all rule struct names for a category, with correct paths."""
    cat_dir = os.path.join(rules_dir, cat)
    mod_path = os.path.join(cat_dir, 'mod.rs')
    if not os.path.exists(mod_path):
        return []
    
    exported, has_wildcard = get_exported_structs(mod_path)
    submodules = set(get_submodules(mod_path))
    
    result = []  # (struct_name, qualified_ref)
    seen = set()
    
    for root, dirs, files in os.walk(cat_dir):
        for fname in sorted(files):
            if not fname.endswith('.rs') or fname == 'mod.rs':
                continue
            filepath = os.path.join(root, fname)
            
            # Determine the relative path from cat_dir
            rel = os.path.relpath(filepath, cat_dir).replace('\\', '/')
            parts = rel.replace('.rs', '').split('/')
            
            for sname in extract_struct_names_from_file(filepath):
                if sname in seen:
                    continue
                
                if sname in exported or has_wildcard:
                    # Directly accessible in category mod scope
                    result.append((sname, sname))
                    seen.add(sname)
                elif len(parts) > 1 and parts[0] in submodules:
                    # In a sub-module, use qualified path
                    qual = parts[0] + '::' + sname
                    result.append((sname, qual))
                    seen.add(sname)
                # else: not exported, skip
    
    return sorted(result, key=lambda x: x[0])

def generate_all_rules_function(cat, rules):
    """Generate the all_rules() function text."""
    lines = []
    lines.append(f"pub fn all_rules() -> Vec<(&'static str, crate::rules::core::RuleMetadata, crate::rules::core::RuleCategory)> {{")
    lines.append(f"    use crate::rules::core::Rule;")
    lines.append(f"    let mut rules = Vec::new();")
    for sname, qual in rules:
        lines.append(f"    {{ let r = {qual}::new(); rules.push((\"{cat}\", r.metadata().clone(), r.category())); }}")
    lines.append(f"    rules")
    lines.append(f"}}")
    return '\n'.join(lines)

def main():
    rules_dir = sys.argv[1] if len(sys.argv) > 1 else 'src/rules'
    
    for cat in CATEGORIES:
        rules = collect_rules_for_category(cat, rules_dir)
        if not rules:
            print(f"  {cat}: no rules found")
            continue
        
        func_text = generate_all_rules_function(cat, rules)
        
        mod_path = os.path.join(rules_dir, cat, 'mod.rs')
        with open(mod_path, 'r', encoding='utf-8') as f:
            mod_content = f.read()
        
        # Remove old all_rules()
        mod_content = re.sub(r'\npub fn all_rules\(\)[^\n]*\{.*?\n\}\n', '\n', mod_content, flags=re.DOTALL)
        mod_content = mod_content.rstrip() + '\n\n' + func_text + '\n'
        
        with open(mod_path, 'w', encoding='utf-8') as f:
            f.write(mod_content)
        
        print(f"  {cat}: {len(rules)} rules registered")

if __name__ == '__main__':
    main()
