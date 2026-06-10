# 贡献指南

欢迎为世界规则库贡献规则！

## 快速开始

1. Fork 本仓库
2. 创建分支: `git checkout -b add-xxx-rules`
3. 使用 `simple_rule!` 宏创建规则模块
4. 添加测试
5. 提交 PR

## 使用 simple_rule! 宏

```rust
use crate::rules::core::{simple_rule, RuleCategory};

simple_rule! {
    struct: MyGameRules,
    name: "我的游戏规则",
    desc: "游戏规则描述",
    origin: "中国",
    tags: ["游戏", "我的游戏"],
    category: RuleCategory::games("my_game"),
    sections: [
        ("基本规则", basic_rules),
        ("得分规则", scoring_rules)
    ]
}

impl MyGameRules {
    fn basic_rules(&self) -> Vec<&'static str> {
        vec!["规则1", "规则2"]
    }

    fn scoring_rules(&self) -> Vec<&'static str> {
        vec!["得分1", "得分2"]
    }
}
```

## 规则分类

- `RuleCategory::games(name)` — 游戏规则
- `RuleCategory::sports(name)` — 体育规则
- `RuleCategory::social(name)` — 社交礼仪
- `RuleCategory::science(name)` — 科学定律
- `RuleCategory::law(name)` — 法律法规
- `RuleCategory::health(name)` — 健康规则

## 代码规范

- 运行 `cargo fmt` 格式化
- 运行 `cargo clippy --all-features` 检查
- 运行 `cargo test --all-features` 确保测试通过
- 每个规则模块至少一个测试

## 提交规范

```
feat(scope): 描述

- 改动1
- 改动2
```

scope: games/sports/social/science/law/health/cli/core
