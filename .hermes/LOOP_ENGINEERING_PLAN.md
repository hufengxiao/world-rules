# World Rules Loop Engineering 实施方案

## 项目现状分析

### 已完成
- v2.0.0 发布，1098+ 条规则
- 1298 个测试全通过，clippy 零警告
- CI/CD 流水线完成（GitHub Actions）
- 1149 个源文件，739 个 simple_rule! 宏定义

### 待完成（ROADMAP.md）
- M3: API 文档（rustdoc + 示例）
- M4: 基准测试（criterion 性能基准）
- M5: 属性测试（proptest 核心逻辑）

### 规则扩充（ROADMAP_EXPANSION.md）
- E1-E19: 目标 1098 → 2000+ 规则
- 已完成 E20 (v2.0.0 发布)

---

## Loop Engineering 核心概念

> "I don't prompt Claude anymore. I have loops running that prompt Claude and figuring out what to do. My job is to write loops."
> — Boris Cherny, Head of Claude Code

**五大支柱**：
1. **Automations** — 定时触发（cron jobs, git hooks）
2. **Worktrees** — 并行 agent 的隔离环境
3. **Skills** — 可复用的知识文件
4. **Plugins** — 与真实世界交互的工具
5. **Sub-Agents** — 一个提议，另一个验证

**循环模式**：
```
┌─────────┐     ┌──────────┐     ┌────────┐     ┌────────┐     ┌──────────┐
│  发现    │ ──→ │   执行    │ ──→ │  验证   │ ──→ │  记录   │ ──→ │   循环   │
│Discover │     │ Execute  │     │Verify  │     │ Record │     │   Loop  │
└─────────┘     └──────────┘     └────────┘     └────────┘     └──────────┘
     ↑                                                              │
     └──────────────────────────────────────────────────────────────┘
```

---

## 实施方案对比

### 方案 A：GSD Core（官方工具）

**优点**：
- 官方维护，67 skills + 35+ agents + 16 hooks
- 标准化流程，社区支持
- `.planning/` 目录结构完整

**缺点**：
- 需要安装 npx 包
- 流程较复杂，学习曲线
- 可能需要适应现有 ROADMAP 格式

**安装**：
```bash
npx @opengsd/gsd-core --hermes --local
```

**生成的结构**：
```
.planning/
├── PROJECT.md      # 项目上下文
├── ROADMAP.md      # 阶段任务列表
├── STATE.md        # 当前状态
├── config.json     # 工作流配置
└── phases/
    └── N/PLAN.md   # 详细阶段计划
```

**GSD 命令**：
- `/gsd-new-project` — 初始化项目规划
- `/gsd-plan-phase N` — 创建阶段计划
- `/gsd-execute-phase N` — 执行阶段任务
- `/gsd-verify-work` — 验证完成工作
- `/gsd-ship` — 创建 PR / 发布

---

### 方案 B：自定义 Loop（轻量版）

**优点**：
- 轻量级，直接使用现有 ROADMAP
- 类似 quick-translate 已验证的模式
- 完全可控，易于调试

**缺点**：
- 需要手动维护脚本
- 功能相对简单

**核心组件**：

#### 1. loop.py（任务发现脚本）

```python
#!/usr/bin/env python3
"""World Rules 自动开发循环"""
import re
import sys
from pathlib import Path

ROADMAP = Path(__file__).parent.parent / "ROADMAP.md"
EXPANSION = Path(__file__).parent.parent / "ROADMAP_EXPANSION.md"
TASK_FILE = Path(__file__).parent.parent / ".planning" / "current-task.md"

def parse_roadmap():
    """解析 ROADMAP.md，提取任务"""
    content = ROADMAP.read_text(encoding="utf-8")
    
    # 找到未完成的 milestone
    milestones = re.findall(r'### (M\d+): ([^\n]+)\n(- \[[ x]\][^\n]+\n)+', content)
    
    tasks = []
    for m_id, m_name, task_block in milestones:
        # 检查 milestone 状态
        if "✅" in m_name or "已完成" in m_name:
            continue
        
        # 找到未完成的子任务
        pending = re.findall(r'- \[ \] ([^\n]+)', task_block)
        for task in pending:
            tasks.append({
                "milestone": m_id,
                "milestone_name": m_name.strip(),
                "task": task.strip()
            })
    
    return tasks

def find_next_task():
    """找到下一个待执行任务"""
    tasks = parse_roadmap()
    if not tasks:
        return None
    
    # 优先级：M3 > M4 > M5（按 ROADMAP 顺序）
    return tasks[0]

def generate_task_prompt(task):
    """生成任务提示"""
    prompt = f"""你是 world-rules 项目的自动开发代理。

当前任务：{task['milestone']} - {task['milestone_name']}
具体任务：{task['task']}

执行步骤：
1. 分析任务需求
2. 实现相关代码/文档
3. 运行测试验证：cargo test && cargo clippy
4. 提交代码：git add -A && git commit -m "loop: {task['task'][:50]}"
5. 更新 ROADMAP.md：将对应任务标记为 [x]

约束：
- 所有 pub 类型必须有 rustdoc 注释
- 测试必须通过才能提交
- 不要破坏现有 API
- 中文注释优先
"""
    return prompt

def main():
    if "--status" in sys.argv:
        tasks = parse_roadmap()
        print(f"待完成任务数: {len(tasks)}")
        for t in tasks[:5]:
            print(f"  - {t['milestone']}: {t['task']}")
        return
    
    if "--dry-run" in sys.argv:
        task = find_next_task()
        if task:
            print(generate_task_prompt(task))
        else:
            print("所有任务已完成")
        return
    
    # 正常模式：生成任务文件
    task = find_next_task()
    if not task:
        print("所有任务已完成，循环结束")
        return
    
    # 确保 .planning 目录存在
    TASK_FILE.parent.mkdir(exist_ok=True)
    
    prompt = generate_task_prompt(task)
    TASK_FILE.write_text(prompt, encoding="utf-8")
    print(f"生成任务: {task['milestone']} - {task['task']}")

if __name__ == "__main__":
    main()
```

#### 2. Hermes Cron Job

```python
# 创建自动循环 job
cronjob(
    action="create",
    name="world-rules-loop",
    schedule="every 30m",
    workdir="D:\\Projects\\world-rules",
    prompt="""你是 world-rules 项目的自动开发代理。

执行步骤：
1. 运行 python scripts/loop.py --status 查看任务状态
2. 如果有未完成任务，运行 python scripts/loop.py 获取当前任务
3. 阅读生成的 .planning/current-task.md
4. 执行任务内容
5. 运行 cargo test && cargo clippy 验证
6. 如果通过，git commit 并更新 ROADMAP.md
7. 否则，回滚并报告问题

约束：
- 测试必须通过才能提交
- 保持 ROADMAP.md 任务状态同步
- 一次只处理一个任务
""",
    enabled_toolsets=["file", "terminal"],
    deliver="local"  # 只保存，不推送
)
```

#### 3. 验证脚本（ci_test.py）

```python
#!/usr/bin/env python3
"""CI 验证脚本"""
import subprocess
import sys

def run_cmd(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.returncode == 0, result.stdout, result.stderr

def main():
    checks = [
        ("cargo test", "单元测试"),
        ("cargo clippy -- -D warnings", "Clippy 检查"),
        ("cargo fmt --all -- --check", "格式检查"),
    ]
    
    all_pass = True
    for cmd, name in checks:
        ok, out, err = run_cmd(cmd)
        if ok:
            print(f"✅ {name} 通过")
        else:
            print(f"❌ {name} 失败")
            print(err)
            all_pass = False
    
    sys.exit(0 if all_pass else 1)

if __name__ == "__main__":
    main()
```

---

## 推荐实施步骤

### 如果选择方案 A（GSD Core）：

1. 安装 GSD Core
   ```bash
   npx @opengsd/gsd-core --hermes --local
   ```

2. 初始化项目
   ```
   /gsd-new-project
   ```

3. 规划阶段
   ```
   /gsd-plan-phase 3  # M3: API 文档
   ```

4. 创建循环 cron
   参考 autonomous-dev-loop skill

### 如果选择方案 B（自定义）：

1. 创建 `scripts/loop.py`
2. 创建 `.planning/current-task.md`
3. 创建 Hermes cron job
4. 监控执行日志

---

## quick-translate 参考案例

已有成功案例：
- 项目：C:\Users\hufen\projects\quick-translate
- Cron job：`ad104c9489b3`（已暂停）
- 结构：`.planning/ROADMAP.md` + `loop.py` + `ci_test.py`
- 结果：Phase 1-4 自主完成

---

## 注意事项

1. **ROADMAP 必须同步** - 任务完成后立即标记 `[x]`
2. **测试必须通过** - 否则不要提交
3. **Cron 不能交互** - 任务提示必须自包含
4. **保持节奏紧凑** - 一次一个任务，完整循环
5. **用户偏好** - "不好用就删不要半成品"