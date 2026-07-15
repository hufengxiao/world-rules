# 依赖更新流程

> **Phase 27**: 依赖更新自动化  
> **状态**: 活跃  
> **更新时间**: 2026-07-16

---

## 📋 Dependabot 配置

### 自动更新策略

- **更新频率**: 每周一自动检查
- **更新时间**: UTC 06:00 (北京时间 14:00)
- **PR 限制**: Cargo 最多 5 个，GitHub Actions 最多 3 个
- **分支**: master

### 更新分组

1. **dev-dependencies**: 开发依赖一起更新
   - criterion
   - proptest

2. **serde-ecosystem**: Serde 生态一起更新
   - serde
   - serde_json

### 更新类型限制

- ✅ **Patch 更新**: 自动合并（如 1.0.1 → 1.0.2）
- ✅ **Minor 更新**: 自动创建 PR（如 1.0.0 → 1.1.0）
- ⚠️ **Major 更新**: 需要人工审查（如 1.0.0 → 2.0.0）

---

## 🔄 依赖更新流程

### 自动流程

```
Dependabot 检测更新
    ↓
创建 Pull Request
    ↓
CI 自动运行测试
    ↓
测试通过？
    ├─ 是 → 自动合并（patch/minor）
    └─ 否 → 通知人工审查
```

### 人工审查流程

```
收到更新通知
    ↓
查看变更内容
    ↓
本地测试验证
    ├─ cargo test
    ├─ cargo clippy
    └─ cargo doc
    ↓
测试通过？
    ├─ 是 → 批准合并
    └─ 否 → 评论说明问题
```

---

## 🧪 测试验证清单

每次依赖更新 PR 必须验证：

### 基础验证

- [ ] `cargo build --all` 编译通过
- [ ] `cargo test --all` 测试通过
- [ ] `cargo clippy -- -D warnings` 无警告
- [ ] `cargo fmt --all -- --check` 格式正确

### 扩展验证（重要更新）

- [ ] `cargo doc --no-deps` 文档编译通过
- [ ] `cargo bench` 性能测试通过
- [ ] `cargo audit` 安全审计通过
- [ ] 特性测试：`cargo test --features full`

### 兼容性验证（Major 更新）

- [ ] API 兼容性检查
- [ ] 破坏性变更分析
- [ ] 更新文档是否需要调整
- [ ] 下游用户影响评估

---

## 🔒 版本锁定策略

### Cargo.lock

- **状态**: 已提交到仓库
- **目的**: 确保构建可重复性
- **更新**: Dependabot 自动更新

### 版本范围

```toml
# 生产依赖：使用语义化版本范围
thiserror = "2.0"  # 允许 2.x 的任何版本
serde = { version = "1", features = ["derive"] }  # 允许 1.x 的任何版本
serde_json = "1"   # 允许 1.x 的任何版本

# 开发依赖：可以使用精确版本
criterion = "0.8"
proptest = "1.4"
```

### 安全更新优先级

1. **Critical**: 立即合并（自动）
2. **High**: 24 小时内合并（优先处理）
3. **Medium**: 72 小时内合并（正常流程）
4. **Low**: 下次定期更新时处理

---

## 📊 依赖更新监控

### 仪表板指标

| 指标 | 目标 | 当前 |
|------|------|------|
| 依赖更新延迟 | < 7 天 | ✅ 监控中 |
| 安全补丁延迟 | < 24 小时 | ✅ 监控中 |
| 依赖过期数量 | < 5 个 | ✅ 监控中 |
| 传递依赖数量 | 最小化 | ✅ 监控中 |

### 警报条件

- ⚠️ 依赖超过 30 天未更新
- ⚠️ 发现安全漏洞
- ⚠️ Major 版本更新可用

---

## 🛠️ 手动更新步骤

### 单个依赖更新

```bash
# 1. 检查可用更新
cargo outdated

# 2. 更新特定依赖
cargo update -p serde

# 3. 验证更新
cargo test
cargo clippy -- -D warnings

# 4. 提交变更
git add Cargo.lock
git commit -m "deps: update serde"
```

### 全部依赖更新

```bash
# 1. 更新所有依赖
cargo update

# 2. 验证更新
cargo test --all
cargo clippy --all -- -D warnings

# 3. 检查安全漏洞
cargo audit

# 4. 提交变更
git add Cargo.lock
git commit -m "deps: update all dependencies"
```

---

## 📝 依赖更新日志

### 2026-07-16

- ✅ 配置 Dependabot
- ✅ 设置每周自动检查
- ✅ 配置分组更新策略
- ✅ 添加自动测试验证

---

## 🔗 相关文件

- `.github/dependabot.yml` - Dependabot 配置
- `Cargo.lock` - 依赖锁定文件
- `Cargo.toml` - 依赖声明
- `DEPENDENCY_RISK.md` - 依赖风险评估

---

## 📚 参考资料

- [Dependabot 文档](https://docs.github.com/en/code-security/dependabot)
- [Cargo Book - Cargo.toml vs Cargo.lock](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [Semantic Versioning](https://semver.org/)

---

*最后更新: 2026-07-16*