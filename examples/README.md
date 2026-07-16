# World Rules 示例代码

本目录包含多个示例程序，展示 `world_rules` 库的各种用法。

## 示例列表

### 基础示例

| 示例 | 描述 | 运行命令 |
|------|------|----------|
| `basic_usage.rs` | 基础使用入门 | `cargo run --example basic_usage` |
| `demo.rs` | 完整功能演示 | `cargo run --example demo` |
| `validate_games.rs` | 游戏验证示例 | `cargo run --example validate_games` |

### 进阶示例

| 示例 | 描述 | 运行命令 |
|------|------|----------|
| `advanced_usage.rs` | 进阶功能展示 | `cargo run --example advanced_usage` |
| `complete_app.rs` | 完整应用示例 | `cargo run --example complete_app` |

### 领域示例

| 示例 | 描述 | 运行命令 |
|------|------|----------|
| `law_rules_example.rs` | 法律规则使用 | `cargo run --example law_rules_example` |
| `sports_rules_example.rs` | 体育规则使用 | `cargo run --example sports_rules_example` |

### 迁移示例

| 示例 | 描述 | 运行命令 |
|------|------|----------|
| `migration_basic.rs` | v1.x → v2.x 基础迁移 | `cargo run --example migration_basic` |
| `migration_new_features.rs` | v2.x 新功能使用 | `cargo run --example migration_new_features` |
| `migration_batch_processing.rs` | 批量规则处理 | `cargo run --example migration_batch_processing` |

## 示例详情

### 1. 基础使用 (`basic_usage.rs`)

展示库的基础功能：
- 创建规则实例
- 查看规则元数据
- 执行规则验证
- 获取规则说明

```bash
cargo run --example basic_usage
```

### 2. 进阶使用 (`advanced_usage.rs`)

展示进阶功能：
- 规则集管理
- 性能检查系统
- 批量验证
- 规则对比
- 自定义元数据

```bash
cargo run --example advanced_usage
```

### 3. 完整应用 (`complete_app.rs`)

构建完整的规则验证应用：
- 规则注册表
- 批量测试
- 性能分析
- 统计报告

```bash
cargo run --example complete_app
```

### 4. 法律规则示例 (`law_rules_example.rs`)

展示法律规则库的使用：
- 民法规则（总则、物权、债权）
- 刑法规则（总则、犯罪构成、量刑）
- 劳动法规则
- 交通规则

```bash
cargo run --example law_rules_example
```

### 5. 体育规则示例 (`sports_rules_example.rs`)

展示体育规则库的使用：
- 球类运动（足球、篮球、排球等）
- 田径运动（短跑、跳跃、投掷）
- 水上运动（游泳、跳水、水球）
- 格斗与体操（拳击、柔道、跆拳道）

```bash
cargo run --example sports_rules_example
```

## 测试示例

所有示例代码都会在 CI 中自动测试。你也可以本地测试：

```bash
# 测试所有示例
cargo test --examples

# 测试单个示例
cargo test --example basic_usage
```

## 贡献示例

欢迎贡献新的示例！请遵循以下规范：

1. **文件命名**: `<category>_<name>_example.rs` 或 `<name>.rs`
2. **文档注释**: 每个示例文件开头必须有 `//!` 注释说明用途和运行方法
3. **代码组织**: 使用函数组织代码，`main()` 函数简洁明了
4. **输出清晰**: 使用清晰的输出格式，包含分隔线和状态标记

示例模板：

```rust
//! <示例名称> - <简短描述>
//!
//! <详细说明>
//!
//! 运行: cargo run --example <example_name>

use world_rules::prelude::*;

fn main() {
    println!("=== <示例标题> ===\n");

    // 示例代码...

    println!("\n✅ 示例完成！");
}
```

## 相关文档

- [API 文档](https://docs.rs/world_rules)
- [用户指南](../docs/guides/user-guide.md)
- [最佳实践](../docs/guides/best-practices.md)
- [FAQ](../docs/guides/faq.md)

## 许可证

MIT License