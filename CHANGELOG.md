# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2024-01-XX

### Added
- **Phase 12**: 刑法深度规则扩展
  - 刑法总则深度规则 (criminal_law_general_deep.rs)
  - 刑法分则深度规则 (criminal_law_specific_deep.rs)
  - 刑事诉讼程序深度规则 (criminal_procedure_deep.rs)
  - 犯罪学深度规则 (criminology_deep.rs)
  - 经济犯罪深度规则 (economic_crime_deep.rs)
  - 量刑指南深度规则 (sentencing_guideline_deep.rs)
  - 刑事证据规则深度规则 (criminal_evidence_deep.rs)
  - 刑事司法改革深度规则 (criminal_justice_reform_deep.rs)
  - 刑事被害人保护深度规则 (victim_protection_deep.rs)

- **Phase 13**: API 文档
  - 为所有 pub 类型添加 rustdoc 注释
  - 添加 # Examples 示例代码
  - 添加 # Safety 安全说明
  - 生成完整 API 文档

- **Phase 14**: 性能基准测试
  - 使用 criterion 框架编写性能基准
  - 规则验证性能基准 (law_bench.rs)
  - 规则解释性能基准 (mahjong_bench.rs)
  - 规则序列化性能基准 (poker_bench.rs, sudoku_bench.rs)

- **Phase 15**: 属性测试
  - 使用 proptest 框架编写属性测试
  - 规则生成器属性测试
  - 规则验证属性测试
  - 规则序列化属性测试
  - tests/proptest_law.rs

### Changed
- 完善 README.md 文档
  - 添加徽章 (Crates.io, Documentation, License, Rust)
  - 添加项目规模统计
  - 添加特性标志说明
  - 添加开发指南
  - 改进项目结构说明

### Fixed
- 修复文档格式问题

## [1.0.0] - 2024-01-01

### Added
- **Phase 1**: 项目初始化
  - 创建项目结构
  - 设置 Cargo.toml
  - 配置 CI/CD

- **Phase 2**: 核心规则框架
  - Rule trait 定义
  - RuleMetadata 元数据结构
  - RuleCategory 规则分类
  - simple_rule! 宏

- **Phase 3**: 民法规则
  - 合同法规则
  - 物权法规则
  - 侵权责任规则

- **Phase 4**: 刑法规则
  - 刑法总则规则
  - 刑法分则规则
  - 刑罚规则

- **Phase 5**: 商法规则
  - 公司法规则
  - 证券法规则
  - 破产法规则

- **Phase 6**: 经济法规则
  - 竞争法规则
  - 消费者保护法规则
  - 税法规则

- **Phase 7**: 行政法规则
  - 行政处罚规则
  - 行政许可规则
  - 行政复议规则

- **Phase 8**: 社会法规则
  - 劳动法规则
  - 社会保障法规则
  - 环境保护法规则

- **Phase 9**: 程序法规则
  - 民事诉讼程序规则
  - 刑事诉讼程序规则
  - 行政诉讼程序规则

- **Phase 10**: 知识产权规则
  - 著作权规则
  - 专利法规则
  - 商标法规则

- **Phase 11**: 国际法规则
  - 国际公法规则
  - 国际私法规则
  - 国际经济法规则

- **游戏规则** (400+ 条)
  - 25 种麻将变体规则
  - 7 种扑克游戏规则
  - 15 种棋类游戏规则
  - 10+ 种桌游规则

- **体育规则**
  - 足球规则
  - 篮球规则
  - 网球规则
  - 乒乓球规则
  - 羽毛球规则

- **社交礼仪**
  - 商务礼仪
  - 餐桌礼仪
  - 网络礼仪

- **科学定律**
  - 物理定律
  - 化学定律
  - 生物定律

- **健康规则**
  - 营养健康
  - 运动健康
  - 心理健康

- **CLI 工具**
  - wr list - 列出规则
  - wr show - 显示规则详情
  - wr stats - 统计信息
  - wr validate - 验证规则

- **文档**
  - API 文档
  - 规则目录 (RULES_CATALOG.md)
  - 开发路线图 (ROADMAP.md)

## [Unreleased]

### Added
- Phase 16: 发布准备
  - ✅ 完善 README.md
  - ✅ 添加 CHANGELOG.md
  - ✅ 添加 CONTRIBUTING.md
  - ✅ 添加 CODE_OF_CONDUCT.md
  - ✅ 添加 SECURITY.md
  - ✅ 添加自动发布工作流 (.github/workflows/publish.yml)
  - ✅ 添加 Dependabot 配置
  - ✅ 删除冗余的 release.yml 工作流
  - ✅ 添加发布脚本 (scripts/release.sh, scripts/release.ps1)

### Planned
- 更多游戏规则
- 更多法律规则
- 国际化支持
- WebAssembly 支持

---

**注**: 本项目采用 Loop Engineering 开发方法，每个 Phase 都经过规划、实现、测试、发布、反馈的完整循环。