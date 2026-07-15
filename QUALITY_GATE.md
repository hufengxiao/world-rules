# 代码质量门禁标准

> **Phase 28**: 代码质量门禁  
> **状态**: 活跃  
> **更新时间**: 2026-07-16

---

## 📊 质量门禁定义

### 必须通过的检查

| 类别 | 检查项 | 要求 | 阻断级别 |
|------|--------|------|----------|
| **编译** | `cargo build --all` | ✅ 通过 | 🔴 必须通过 |
| **测试** | `cargo test --all` | ✅ 100% 通过 | 🔴 必须通过 |
| **代码格式** | `cargo fmt --all -- --check` | ✅ 通过 | 🔴 必须通过 |
| **Clippy** | `cargo clippy -- -D warnings` | ✅ 零警告 | 🔴 必须通过 |
| **安全审计** | `cargo audit` | ✅ 无已知漏洞 | 🔴 必须通过 |
| **文档** | `cargo doc --no-deps` | ✅ 编译通过 | 🟡 建议通过 |
| **覆盖率** | `cargo tarpaulin` | ✅ ≥70% | 🟡 建议通过 |
| **性能** | `cargo bench` | ✅ 无回归 | 🟡 建议通过 |

---

## 🎯 质量指标

### 代码质量指标

```yaml
质量指标:
  代码覆盖率:
    目标: ≥ 70%
    警告: 60-70%
    失败: < 60%
  
  代码复杂度:
    圈复杂度: ≤ 15
    函数长度: ≤ 50 行
    文件长度: ≤ 500 行
  
  文档覆盖率:
    公开 API 文档: 100%
    示例代码: ≥ 80%
  
  Clippy 警告:
    允许: 0
    警告级别: deny
```

---

## 🚦 门禁流程

### Pull Request 流程

```
PR 创建
    ↓
自动触发 CI
    ↓
┌─────────────────────────────┐
│  必须通过的检查              │
│  ├─ 编译检查                │
│  ├─ 测试检查                │
│  ├─ 格式检查                │
│  ├─ Clippy 检查             │
│  └─ 安全审计                │
└─────────────────────────────┘
    ↓
所有检查通过？
    ├─ 是 → 允许合并
    └─ 否 → 阻止合并（红色 ❌）
```

### 合并保护规则

GitHub 分支保护规则：

- ✅ **必须通过 CI**：所有检查必须通过
- ✅ **必须通过审核**：至少 1 人审核通过（可选）
- ✅ **分支必须是最新的**：必须基于最新的 master
- ✅ **禁止强制推送**：保护分支历史

---

## 📈 质量趋势报告

### 自动生成报告

每个 CI 运行会生成质量报告：

```markdown
## 🔍 质量门禁报告

| 检查项 | 状态 | 详情 |
|--------|------|------|
| 编译 | ✅ | 通过 |
| 测试 | ✅ | 610/610 通过 |
| 格式 | ✅ | 通过 |
| Clippy | ✅ | 0 警告 |
| 安全 | ✅ | 0 漏洞 |
| 覆盖率 | ✅ | 75.3% |
| 文档 | ✅ | 通过 |
```

### 质量徽章

项目 README.md 中显示的质量徽章：

- ![Build Status](https://github.com/hufengxiao/world-rules/workflows/CI/badge.svg)
- ![Coverage](https://img.shields.io/endpoint?url=...)
- ![Security](https://img.shields.io/badge/security-audit%20passing-green)
- ![Clippy](https://img.shields.io/badge/clippy-passing-green)

---

## 🛡️ 质量保证措施

### 代码审查清单

人工审查时检查：

- [ ] 代码逻辑正确性
- [ ] 错误处理完整性
- [ ] 性能影响评估
- [ ] 安全性考虑
- [ ] 文档更新
- [ ] 测试覆盖

### 自动化检查

```yaml
自动化检查:
  每次提交:
    - cargo fmt --check
    - cargo clippy -- -D warnings
    - cargo test
    - cargo audit
  
  每日检查:
    - cargo outdated
    - cargo audit
    - coverage report
  
  每周检查:
    - 依赖更新检查
    - 性能基准对比
    - MIRI 内存安全检查
```

---

## 🎖️ 质量徽章配置

### GitHub Actions 徽章

在 README.md 中添加：

```markdown
# World-Rules

[![CI](https://github.com/hufengxiao/world-rules/workflows/CI/badge.svg)](https://github.com/hufengxiao/world-rules/actions)
[![Security Audit](https://github.com/hufengxiao/world-rules/workflows/Security%20Audit/badge.svg)](https://github.com/hufengxiao/world-rules/actions)
[![Coverage](https://img.shields.io/codecov/c/github/hufengxiao/world-rules)](https://codecov.io/gh/hufengxiao/world-rules)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
```

### 质量徽章标准

- 🟢 **绿色**: 所有检查通过
- 🟡 **黄色**: 部分检查警告
- 🔴 **红色**: 检查失败

---

## 📊 质量趋势追踪

### 历史趋势指标

追踪以下指标的变化趋势：

- 代码覆盖率趋势
- Clippy 警告数量
- 测试数量增长
- 编译时间变化
- 依赖数量变化

### 趋势报告生成

每周生成质量趋势报告：

```bash
# 生成覆盖率趋势
cargo tarpaulin --out Xml

# 生成代码统计
tokei --output json

# 生成依赖统计
cargo tree --depth 1
```

---

## 🚨 质量门禁失败处理

### 自动回复

当质量门禁失败时，自动添加评论：

```markdown
## ⚠️ 质量门禁检查失败

您的 Pull Request 未能通过质量门禁检查。请修复以下问题：

- [ ] 编译错误
- [ ] 测试失败
- [ ] 格式问题
- [ ] Clippy 警告
- [ ] 安全漏洞

修复后重新提交以触发新的检查。
```

### 失败原因分析

自动分析失败原因：

1. **编译错误**: 显示具体错误信息
2. **测试失败**: 显示失败的测试用例
3. **格式问题**: 显示需要修复的文件
4. **Clippy 警告**: 显示警告位置和修复建议
5. **安全漏洞**: 显示漏洞详情和修复版本

---

## ✅ 验收标准

- [x] 定义质量门禁标准
- [x] CI 集成质量门禁
- [x] 配置分支保护规则
- [x] 质量徽章配置
- [x] 质量趋势报告

---

## 🔗 相关文件

- `.github/workflows/ci.yml` - CI 配置
- `.github/workflows/security.yml` - 安全审计
- `.github/branch-protection.yml` - 分支保护规则
- `Cargo.toml` - 项目配置

---

*最后更新: 2026-07-16*