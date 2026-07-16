# World Rules 教程代码仓库

本目录包含 World Rules 的完整教程代码示例，帮助你快速掌握各种集成场景。

## 📚 教程列表

| 教程 | 文件 | 难度 | 描述 |
|------|------|------|------|
| Web 应用集成 | [web_integration.rs](./web_integration.rs) | ⭐⭐⭐ | Actix Web、Axum、Warp 集成示例 |
| CLI 工具集成 | [cli_integration.rs](./cli_integration.rs) | ⭐⭐ | 命令行工具开发示例 |
| 数据库集成 | [database_integration.rs](./database_integration.rs) | ⭐⭐⭐ | SQLite、PostgreSQL 集成示例 |
| 微服务集成 | [microservice_integration.rs](./microservice_integration.rs) | ⭐⭐⭐⭐ | 微服务架构集成示例 |
| 完整应用 | [complete_application.rs](./complete_application.rs) | ⭐⭐⭐⭐ | 企业级应用示例 |

## 🚀 快速开始

### 运行单个教程

```bash
# Web 应用集成示例
cargo run --example web_integration

# CLI 工具集成示例
cargo run --example cli_integration

# 数据库集成示例
cargo run --example database_integration
```

### 查看教程代码

所有教程代码都可以直接查看和编辑：

```bash
# 使用你喜欢的编辑器
code tutorials/web_integration.rs
vim tutorials/cli_integration.rs
```

## 📖 教程详细说明

### 1. Web 应用集成

演示如何将 World Rules 集成到流行的 Rust Web 框架：

- **Actix Web**: 最流行的 Rust Web 框架
- **Axum**: Tower 生态系统的新星
- **Warp**: 函数式路由设计

关键概念：
- 使用 `Arc` 共享规则实例
- 异步请求处理
- JSON 序列化/反序列化
- RESTful API 设计

### 2. CLI 工具集成

演示如何构建强大的命令行工具：

- 使用 `clap` 定义命令和参数
- 彩色输出和表格显示
- 进度指示器
- 多命令支持

关键概念：
- 命令行参数解析
- 用户友好的输出格式
- 错误处理和提示

### 3. 数据库集成

演示如何与数据库系统集成：

- 基本 CRUD 操作
- 批量操作和事务
- 连接池管理
- 数据迁移

关键概念：
- 规则持久化存储
- 查询优化
- 事务处理

### 4. 微服务集成

演示微服务架构中的集成：

- 服务拆分策略
- 服务间通信
- API Gateway 设计
- 配置管理

关键概念：
- 服务边界划分
- gRPC/REST API
- 配置中心

### 5. 完整应用示例

综合示例，展示企业级应用开发：

- 合规管理系统
- 规则评估系统
- 报告生成系统

关键概念：
- 业务逻辑组织
- 错误处理策略
- 测试最佳实践

## 🛠️ 环境要求

### 基础要求

- Rust 1.70+
- Cargo

### 可选依赖

根据需要添加以下依赖：

```toml
# Web 框架
actix-web = "4"
axum = "0.7"
warp = "0.3"

# CLI 工具
clap = { version = "4", features = ["derive"] }
colored = "2"
indicatif = "0.17"

# 数据库
rusqlite = "0.30"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 📚 相关文档

- [快速入门教程](../docs/TUTORIAL_QUICKSTART.md)
- [深入理解教程](../docs/TUTORIAL_DEEP_DIVE.md)
- [高级特性教程](../docs/TUTORIAL_ADVANCED.md)
- [集成应用教程](../docs/TUTORIAL_INTEGRATION.md)
- [最佳实践](../docs/BEST_PRACTICES.md)

## 🤝 贡献

欢迎提交新的教程示例！

1. Fork 项目
2. 创建你的教程分支 (`git checkout -b tutorial/my-tutorial`)
3. 提交更改 (`git commit -am 'Add: 新教程'`)
4. 推送到分支 (`git push origin tutorial/my-tutorial`)
5. 创建 Pull Request

## 📝 教程编写指南

新教程应遵循以下规范：

### 代码规范

```rust
//! 教程标题
//! 
//! 简短描述
//! 
//! 运行方式:
//! ```bash
//! cargo run --example tutorial_name
//! ```

// 导入部分
use std::sync::Arc;

/// 演示函数命名以 demo_ 开头
fn demo_feature() {
    // 实现代码
}

/// 主函数
fn main() {
    // 打印标题
    // 调用演示函数
    // 打印总结
}
```

### 文档规范

- 每个教程有独立的 README 说明
- 代码包含详细的中文注释
- 使用 `///` 文档注释说明公共接口

### 结构规范

```
tutorials/
├── README.md                    # 本文件
├── web_integration.rs          # Web 集成
├── cli_integration.rs          # CLI 集成
├── database_integration.rs     # 数据库集成
├── microservice_integration.rs # 微服务集成
└── complete_application.rs     # 完整应用
```

## ❓ 常见问题

### Q: 如何选择 Web 框架？

A: 推荐选择：
- **Actix Web**: 成熟稳定，社区活跃
- **Axum**: 现代设计，与 Tower 生态集成
- **Warp**: 函数式风格，轻量级

### Q: 数据库集成需要什么？

A: 最小需求：
- SQLite: 本地应用，嵌入式
- PostgreSQL: 生产环境，高并发

### Q: CLI 工具如何测试？

A: 推荐使用 `assert_cmd` 和 `predicates` 库：

```rust
use assert_cmd::Command;

#[test]
fn test_cli() {
    let mut cmd = Command::cargo_bin("my-cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success();
}
```

## 📄 许可证

MIT License