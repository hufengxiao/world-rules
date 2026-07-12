# 贡献指南

感谢你对 World Rules 项目的关注！本文档将帮助你了解如何为项目做出贡献。

## 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发环境设置](#开发环境设置)
- [代码规范](#代码规范)
- [提交信息规范](#提交信息规范)
- [Pull Request 流程](#pull-request-流程)
- [添加新规则](#添加新规则)

## 行为准则

本项目采用贡献者公约作为行为准则。参与本项目即表示你同意遵守其条款。请阅读 [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) 了解详情。

## 如何贡献

### 报告 Bug

如果你发现了 bug，请通过 [GitHub Issues](https://github.com/hufengxiao/world-rules/issues) 提交报告。提交前请：

1. 搜索现有的 issues，避免重复报告
2. 使用 issue 模板填写详细信息
3. 提供可复现的示例代码

### 建议新功能

欢迎提出新功能建议！请：

1. 在 Issues 中详细描述功能需求
2. 说明功能的使用场景
3. 如果可能，提供实现思路

### 提交代码

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 进行修改
4. 确保测试通过 (`cargo test`)
5. 确保代码检查通过 (`cargo clippy -- -D warnings`)
6. 提交更改 (`git commit -m 'feat: add amazing feature'`)
7. 推送到分支 (`git push origin feature/amazing-feature`)
8. 创建 Pull Request

## 开发环境设置

### 系统要求

- Rust 1.70 或更高版本
- Git

### 安装步骤

```bash
# 克隆仓库
git clone https://github.com/hufengxiao/world-rules
cd world-rules

# 安装依赖
cargo build

# 运行测试
cargo test

# 运行代码检查
cargo clippy -- -D warnings

# 格式化代码
cargo fmt

# 生成文档
cargo doc --open
```

### 项目结构

```
world-rules/
├── src/
│   ├── lib.rs              # 库入口
│   ├── prelude.rs          # 预导入模块
│   └── rules/              # 规则实现
│       ├── core.rs         # 核心 trait 和宏
│       ├── games/          # 游戏规则
│       ├── sports/         # 体育规则
│       ├── social/         # 社交礼仪
│       ├── science/        # 科学定律
│       ├── law/            # 法律规则
│       └── health/         # 健康规则
├── tests/                  # 集成测试
├── benches/               # 性能基准测试
├── examples/              # 使用示例
└── docs/                  # 文档
```

## 代码规范

### Rust 代码风格

- 遵循 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 进行代码检查
- 所有公开 API 必须有文档注释

### 文档注释

```rust
/// 规则简要说明
///
/// # Examples
///
/// ```
/// use world_rules::prelude::*;
///
/// let rule = MyRule::new();
/// assert!(rule.validate("test").unwrap());
/// ```
///
/// # Notes
///
/// 一些额外的注意事项。
pub struct MyRule {
    // ...
}
```

### 测试规范

- 每个公开函数都应有测试
- 单元测试放在源文件中 (`#[cfg(test)] mod tests`)
- 集成测试放在 `tests/` 目录
- 属性测试使用 `proptest` 框架

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_rule() {
        let rule = MyRule::new();
        assert!(rule.validate("valid").unwrap());
        assert!(!rule.validate("invalid").unwrap());
    }
}
```

## 提交信息规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### 类型

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建/工具相关
- `perf`: 性能优化

### 示例

```
feat(law): 添加刑法量刑规则

添加刑法量刑指南规则，包括：
- 量刑原则
- 量刑情节
- 量刑步骤

Closes #123
```

## Pull Request 流程

1. **创建 PR**
   - 使用有意义的标题
   - 填写 PR 模板
   - 关联相关 Issue

2. **代码审查**
   - 等待维护者审查
   - 及时响应审查意见
   - 进行必要的修改

3. **CI 检查**
   - 确保所有测试通过
   - 确保代码检查通过
   - 确保文档构建成功

4. **合并**
   - 至少需要一个审查批准
   - 所有 CI 检查通过
   - 遵循 squash merge 或 merge commit

## 添加新规则

### 使用 simple_rule! 宏

最简单的方式是使用 `simple_rule!` 宏：

```rust
use world_rules::rules::core::{simple_rule, RuleMetadata, RuleCategory, Rule, RuleResult};

simple_rule!(
    MyGameRule,
    "我的游戏规则",
    RuleCategory::Games,
    "规则详细描述"
);
```

### 实现 Rule trait

对于复杂的规则，直接实现 `Rule` trait：

```rust
use world_rules::rules::core::{Rule, RuleMetadata, RuleCategory, RuleResult};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyRuleError {
    #[error("验证失败: {0}")]
    ValidationFailed(String),
}

pub struct MyComplexRule {
    metadata: RuleMetadata,
}

impl MyComplexRule {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata {
                name: "复杂规则".to_string(),
                description: "一个复杂的规则".to_string(),
                version: "1.0.0".to_string(),
                author: "Your Name".to_string(),
                tags: vec!["complex".to_string()],
            },
        }
    }
}

impl Rule for MyComplexRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Games
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        // 实现验证逻辑
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!("{}: {}", self.metadata.name, self.metadata.description)
    }
}
```

### 添加测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_complex_rule() {
        let rule = MyComplexRule::new();
        
        // 测试验证
        assert!(rule.validate("valid context").unwrap());
        assert!(!rule.validate("").unwrap());
        
        // 测试说明
        assert!(rule.explain().contains("复杂规则"));
    }
}
```

### 注册规则

在 `src/rules/<category>/mod.rs` 中添加导出：

```rust
mod my_complex_rule;
pub use my_complex_rule::MyComplexRule;
```

## 许可证

通过提交代码，你同意你的贡献将根据项目的 MIT 许可证进行许可。

## 联系方式

- GitHub Issues: [https://github.com/hufengxiao/world-rules/issues](https://github.com/hufengxiao/world-rules/issues)
- GitHub Discussions: [https://github.com/hufengxiao/world-rules/discussions](https://github.com/hufengxiao/world-rules/discussions)

---

再次感谢你的贡献！🎉