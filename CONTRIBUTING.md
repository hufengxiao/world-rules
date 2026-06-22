# 贡献指南

感谢你对 World Rules 项目的关注！

## 如何贡献

### 添加新规则

1. Fork 本仓库
2. 在对应分类目录下创建新的 `.rs` 文件
3. 使用 `simple_rule!` 宏定义规则结构体
4. 实现 `Rule` trait
5. 添加测试
6. 在 `mod.rs` 中注册新模块
7. 提交 Pull Request

### 规则文件模板

```rust
//! 我的规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MyRules,
    name: "我的规则",
    desc: "规则描述",
    origin: "中国",
    tags: ["分类", "标签"]
}

impl MyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["条目1", "条目2"]
    }
}

impl Rule for MyRules {
    fn metadata(&self) -> &RuleMetadata { &self.metadata }
    fn category(&self) -> RuleCategory { RuleCategory::games("my_game") }
    fn validate(&self, ctx: &ValidateContext) -> RuleResult<bool> { Ok(true) }
    fn explain(&self) -> String {
        format_rule_sections("我的规则", &[("分组", &self.section_0())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MyRules::new();
        assert!(!r.explain().is_empty());
    }
}
```

### 提交规范

- `feat:` 新功能/新规则
- `fix:` Bug 修复
- `docs:` 文档更新
- `test:` 测试相关
- `refactor:` 代码重构

### 代码质量

提交前请确保：

```bash
cargo fmt
cargo clippy
cargo test
```
