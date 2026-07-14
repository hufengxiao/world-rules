# LOOP Engineering 配置

> World-Rules 持续迭代开发系统

---

## 🔄 迭代策略

### 默认行为
- **模式**: 多轨道并行
- **优先级**: LAW > SPORT > QUALITY > GAME > DOC > INNOVATION
- **每次循环**: 1 个任务
- **验证**: cargo test + cargo clippy

### 任务选择算法
```
1. 检查当前 Phase 进度
2. 选择最高优先级轨道的下一个未完成任务
3. 如果当前 Phase 完成，启动下一个 Phase
4. 如果所有 Phase 完成，从储备池添加新任务
```

---

## 🚀 发布策略

### 版本规则
- **Major (X.0.0)**: API 破坏性变更
- **Minor (x.Y.0)**: 新功能添加
- **Patch (x.y.Z)**: Bug 修复

### 发布触发条件
- Minor: 完成一个完整 Phase
- Major: API 稳定版本发布
- Patch: 任何 bug 修复

---

## 📊 轨道配置

### LAW 轨道
```yaml
name: 法律规则
priority: 3
status: active
phase: 深度扩展
next_task: 民法深度规则细化
estimate: 2-3 周
```

### SPORT 轨道
```yaml
name: 体育规则
priority: 3
status: active
phase: 18
next_task: 体操项目详细规则
estimate: 1-2 周
```

### GAME 轨道
```yaml
name: 游戏规则
priority: 2
status: pending
phase: 基础完成
next_task: 游戏规则增强
estimate: 2 周
```

### QUALITY 轨道
```yaml
name: 质量保证
priority: 3
status: active
phase: 持续改进
next_task: 代码覆盖率检测
estimate: 持续
```

### DOC 轨道
```yaml
name: 文档系统
priority: 2
status: pending
phase: 持续完善
next_task: 架构设计文档
estimate: 持续
```

### INNOVATION 轨道
```yaml
name: 创新功能
priority: 1
status: pending
phase: 探索阶段
next_task: 规则市场 Web 界面
estimate: 待定
```

---

## 🎯 质量门禁

### 必须通过
```bash
cargo test --lib           # 所有测试通过
cargo clippy -- -D warnings # 零警告
cargo fmt -- --check       # 格式检查
```

### 建议通过
```bash
cargo doc --no-deps        # 文档生成
cargo audit                # 安全审计
cargo tarpaulin            # 覆盖率检测
```

---

## 📋 任务模板

### 规则模块任务
```markdown
- [ ] 创建 {module_name}.rs
- [ ] 实现 {功能描述}
- [ ] 添加单元测试
- [ ] 添加文档注释
- [ ] 更新 mod.rs 导出
- [ ] 运行测试验证
```

### 文档任务
```markdown
- [ ] 创建 {doc_name}.md
- [ ] 添加 {内容描述}
- [ ] 添加代码示例
- [ ] 更新相关文档链接
```

---

## 🔁 Cron 配置

### 测试运行 (每小时)
```yaml
schedule: "0 * * * *"
command: "cargo test --lib"
notify_on_complete: false
```

### 质量检查 (每天)
```yaml
schedule: "0 9 * * *"
command: "cargo clippy -- -D warnings && cargo fmt -- --check"
notify_on_complete: true
```

### 发布检查 (每周)
```yaml
schedule: "0 9 * * 1"
command: "scripts/check-release.sh"
notify_on_complete: true
```

---

## 📈 度量指标

### 代码质量
- 测试覆盖率: 目标 80%+
- clippy 警告: 目标 0
- 文档覆盖率: 目标 100%

### 开发效率
- 平均任务完成时间: < 1 小时
- 每周完成任务数: > 5
- 发布频率: 每月至少 1 次

---

## 🎨 颜色编码

- 🟢 活跃: 当前正在开发
- 🟡 待启动: 已规划，等待启动
- 🔵 已完成: 已完成并验证
- ⚪ 待定: 未来规划
- 🔴 阻塞: 存在问题需要解决

---

## 🔄 循环检查清单

每次循环开始前：
- [ ] 检查 STATE.md 当前位置
- [ ] 读取 ROADMAP.md 任务列表
- [ ] 选择下一个未完成任务
- [ ] 确认无阻塞项

每次循环结束后：
- [ ] 运行测试验证
- [ ] 更新 STATE.md 进度
- [ ] 更新 ROADMAP.md 任务状态
- [ ] git commit 提交变更
- [ ] 检查是否需要发布

---

## 📚 参考资源

- [ROADMAP.md](./ROADMAP.md) - 开发路线图
- [STATE.md](./STATE.md) - 当前状态
- [CHANGELOG.md](../CHANGELOG.md) - 变更历史
- [CONTRIBUTING.md](../CONTRIBUTING.md) - 贡献指南

---

> **LOOP Engineering**: 永不停止，持续迭代，精益求精