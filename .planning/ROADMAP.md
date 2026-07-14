# World-Rules 开发路线图

## Phase 1-11: 已完成
- ✅ Phase 1: 项目初始化
- ✅ Phase 2: 核心规则框架
- ✅ Phase 3: 民法规则
- ✅ Phase 4: 刑法规则
- ✅ Phase 5: 商法规则
- ✅ Phase 6: 经济法规则
- ✅ Phase 7: 行政法规则
- ✅ Phase 8: 社会法规则
- ✅ Phase 9: 程序法规则
- ✅ Phase 10: 知识产权规则
- ✅ Phase 11: 国际法规则

## Phase 12: 刑法深度规则扩展 ✅
- [x] 创建 criminal_law_general_deep.rs - 刑法总则深度规则
- [x] 创建 criminal_law_specific_deep.rs - 刑法分则深度规则
- [x] 创建 criminal_procedure_deep.rs - 刑事诉讼程序深度规则
- [x] 创建 criminology_deep.rs - 犯罪学深度规则
- [x] 创建 economic_crime_deep.rs - 经济犯罪深度规则
- [x] 创建 sentencing_guideline_deep.rs - 量刑指南深度规则
- [x] 创建 criminal_evidence_deep.rs - 刑事证据规则深度规则
- [x] 创建 criminal_justice_reform_deep.rs - 刑事司法改革深度规则
- [x] 创建 victim_protection_deep.rs - 刑事被害人保护深度规则
- [x] 更新 mod.rs 导出新模块

## Phase 13: API 文档 ✅
- [x] 为核心类型添加 rustdoc 注释
- [x] 添加 # Examples 示例代码
- [x] 添加 # Safety 安全说明
- [x] 生成 cargo doc

## Phase 14: 基准测试 ✅
- [x] 使用 criterion 编写性能基准
- [x] 规则验证性能基准
- [x] 规则解释性能基准
- [x] 规则序列化性能基准

## Phase 15: 属性测试 ✅
- [x] 使用 proptest 编写属性测试
- [x] 规则生成器属性测试
- [x] 规则验证属性测试
- [x] 规则序列化属性测试

## Phase 16: 发布准备 ✅ (自动化任务完成)
- [x] 完善 README.md
- [x] 添加 CHANGELOG.md
- [x] 添加 CONTRIBUTING.md
- [x] 添加 CODE_OF_CONDUCT.md
- [x] 添加 SECURITY.md
- [x] 添加自动发布工作流 (.github/workflows/publish.yml)
- [x] 删除冗余的 release.yml 工作流（功能已合并到 publish.yml）
- [x] 添加发布脚本 (scripts/release.sh, scripts/release.ps1)
- [x] 配置 GitHub Secrets (CRATES_IO_TOKEN) - 用户已配置
- [x] 创建 v2.0.0 tag 触发自动发布 - 已完成（测试通过，已发布到 crates.io）

## Phase 17: 测试编译错误修复 ✅
- [x] 修复 phase_18_rules.rs 测试编译错误 - ShortDeckRules 类型导入
- [x] 修复 phase_20_rules.rs 测试编译错误 - 字符串比较和 Option<String> 类型
- [x] 修复 phase_24_rules.rs 测试编译错误 - SkeletonIbsfDetailedRules 类型名称
- [x] 修复 phase_27_dining_rules.rs - 临时值生命周期问题
- [x] 修复 phase_27_other_rules.rs - 临时值生命周期问题
- [x] 修复 phase_30_rules.rs - 未使用变量警告
- [x] 修复 phase_19_rules.rs - 异构类型数组问题
- [x] 修复 phase_32_civil_rules.rs - 添加缺失的 CivilCode*DeepRules 导出
- [x] 修复 proptest_law.rs - catch_unwind 类型问题
- [x] 移除重复的 LaborDetailedRules 导入
- [x] 验证 CI 测试通过（cargo check --lib 和 cargo clippy --lib 通过）
- [x] 创建 v2.0.0 tag 并推送到远程，触发自动发布

## Phase 18: 体育规则深度扩展 ⬜
状态: In progress

### Plans
- [x] 创建 athletics_detailed.rs - 田径项目详细规则
- [ ] 创建 aquatics_detailed.rs - 水上项目详细规则
- [ ] 创建 gymnastics_detailed.rs - 体操项目详细规则
- [ ] 创建 combat_sports_detailed.rs - 格斗项目详细规则
- [ ] 创建 ball_games_detailed.rs - 球类运动详细规则
- [ ] 更新 mod.rs 导出新模块
- [ ] 添加相关测试用例

## Phase 19: 游戏规则增强 ⬜
状态: Not started

### Plans
- [ ] 添加棋类游戏规则（围棋、象棋变体）
- [ ] 添加卡牌游戏规则（UNO、万智牌基础规则）
- [ ] 添加桌面游戏规则（大富翁、卡坦岛基础规则）
- [ ] 创建游戏规则通用 trait
- [ ] 添加规则难度分级
- [ ] 更新测试覆盖

## Phase 20: API 稳定性增强 ⬜
状态: Not started

### Plans
- [ ] 审查所有 pub API 的稳定性
- [ ] 添加 #[non_exhaustive] 到关键枚举
- [ ] 实现版本兼容性检查
- [ ] 添加 API 弃用警告机制
- [ ] 创建 API 迁移指南
- [ ] 添加向后兼容性测试

## Phase 21: 性能优化 ⬜
状态: Not started

### Plans
- [ ] 分析基准测试结果，识别性能瓶颈
- [ ] 优化热点代码路径
- [ ] 减少不必要的内存分配
- [ ] 添加缓存机制（如适用）
- [ ] 并行化规则验证（如适用）
- [ ] 性能回归检测集成到 CI

## Phase 22: 文档完善 ⬜
状态: Not started

### Plans
- [ ] 完善 crate 级文档
- [ ] 添加架构设计文档
- [ ] 添加贡献者指南细化
- [ ] 添加最佳实践文档
- [ ] 创建规则编写教程
- [ ] 添加常见问题解答 (FAQ)

## Phase 23: 测试覆盖率提升 ⬜
状态: Not started

### Plans
- [ ] 添加代码覆盖率检测 (cargo tarpaulin)
- [ ] 补充边界条件测试
- [ ] 添加错误路径测试
- [ ] 添加集成测试场景
- [ ] 测试覆盖率目标: 80%+
- [ ] 覆盖率报告集成到 CI

## Phase 24: 国际化增强 ⬜
状态: Not started

### Plans
- [ ] 完善规则描述的英文翻译
- [ ] 添加规则名称的本地化支持
- [ ] 创建术语对照表
- [ ] 添加语言切换 API
- [ ] 完善多语言文档
- [ ] 添加翻译贡献指南

## Phase 25: 示例和教程 ⬜
状态: Not started

### Plans
- [ ] 添加基础使用示例
- [ ] 添加自定义规则示例
- [ ] 添加规则验证示例
- [ ] 添加 CLI 使用教程
- [ ] 添加 Web 集成示例
- [ ] 创建 Jupyter/笔记教程

---

## 统计信息
- **已完成 Phase**: 17
- **总 Phase 数**: 25
- **完成进度**: 68%