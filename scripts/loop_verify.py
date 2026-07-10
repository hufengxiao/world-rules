#!/usr/bin/env python3
"""World Rules CI 验证脚本 - Loop Engineering gate"""
import subprocess
import sys

def run_cmd(cmd):
    """运行命令并返回结果"""
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd="D:/Projects/world-rules")
    return result.returncode == 0, result.stdout, result.stderr

def main():
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print(" World Rules CI Verification")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    
    checks = [
        ("cargo test --quiet", "单元测试 (1298 tests)"),
        ("cargo clippy -- -D warnings", "Clippy 检查"),
        ("cargo fmt --all -- --check", "格式检查"),
    ]
    
    all_pass = True
    for cmd, name in checks:
        ok, out, err = run_cmd(cmd)
        if ok:
            print(f"  ✅ {name}")
        else:
            print(f"  ❌ {name}")
            if err:
                print(f"     Error: {err[:200]}")
            all_pass = False
    
    if all_pass:
        print("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
        print(" ✅ All checks passed — ready to commit")
        print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
        sys.exit(0)
    else:
        print("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
        print(" ❌ Verification failed — fix before committing")
        print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
        sys.exit(1)

if __name__ == "__main__":
    main()