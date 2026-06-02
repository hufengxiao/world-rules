//! 核心规则定义

use std::collections::HashMap;

/// 规则分类
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RuleCategory {
    /// 游戏规则
    Games(String),
    /// 体育规则
    Sports(String),
    /// 社交礼仪
    Social(String),
    /// 科学规则
    Science(String),
    /// 法律法规
    Law(String),
    /// 健康规则
    Health(String),
    /// 自定义分类
    Custom(String),
}

impl RuleCategory {
    pub fn games(name: impl Into<String>) -> Self {
        Self::Games(name.into())
    }

    pub fn sports(name: impl Into<String>) -> Self {
        Self::Sports(name.into())
    }

    pub fn social(name: impl Into<String>) -> Self {
        Self::Social(name.into())
    }

    pub fn science(name: impl Into<String>) -> Self {
        Self::Science(name.into())
    }

    pub fn law(name: impl Into<String>) -> Self {
        Self::Law(name.into())
    }

    pub fn health(name: impl Into<String>) -> Self {
        Self::Health(name.into())
    }

    pub fn custom(category: impl Into<String>, name: impl Into<String>) -> Self {
        Self::Custom(format!("{}/{}", category.into(), name.into()))
    }
}

impl std::fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleCategory::Games(name) => write!(f, "Games/{}", name),
            RuleCategory::Sports(name) => write!(f, "Sports/{}", name),
            RuleCategory::Social(name) => write!(f, "Social/{}", name),
            RuleCategory::Science(name) => write!(f, "Science/{}", name),
            RuleCategory::Law(name) => write!(f, "Law/{}", name),
            RuleCategory::Health(name) => write!(f, "Health/{}", name),
            RuleCategory::Custom(name) => write!(f, "Custom/{}", name),
        }
    }
}

/// 规则元数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuleMetadata {
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 规则版本
    pub version: String,
    /// 规则来源/地区
    pub origin: Option<String>,
    /// 标签
    pub tags: Vec<String>,
}

impl RuleMetadata {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: "1.0.0".to_string(),
            origin: None,
            tags: Vec::new(),
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// 规则错误类型
#[derive(Debug, thiserror::Error)]
pub enum RuleError {
    #[error("规则不存在: {0}")]
    RuleNotFound(String),

    #[error("规则验证失败: {0}")]
    ValidationError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("不支持的操作: {0}")]
    UnsupportedOperation(String),
}

pub type RuleResult<T> = Result<T, RuleError>;

/// 规则核心 trait
///
/// 所有规则都需要实现此 trait
pub trait Rule: Send + Sync {
    /// 获取规则元数据
    fn metadata(&self) -> &RuleMetadata;

    /// 获取规则分类
    fn category(&self) -> RuleCategory;

    /// 验证某个状态是否符合规则
    fn validate(&self, context: &str) -> RuleResult<bool>;

    /// 获取规则的详细说明
    fn explain(&self) -> String {
        format!(
            "【{}】\n{}\n版本: {}\n来源: {}",
            self.metadata().name,
            self.metadata().description,
            self.metadata().version,
            self.metadata().origin.as_deref().unwrap_or("未知")
        )
    }
}

/// 规则集 - 包含一组相关规则
pub struct RuleSet {
    pub metadata: RuleMetadata,
    pub category: RuleCategory,
    pub rules: HashMap<String, Box<dyn Rule>>,
}

impl RuleSet {
    pub fn new(name: String, category: RuleCategory) -> Self {
        Self {
            metadata: RuleMetadata::new(&name, format!("{} 规则集", name)),
            category,
            rules: HashMap::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.metadata.description = description.into();
        self
    }

    pub fn add_rule<R: Rule + 'static>(&mut self, rule: R) {
        let name = rule.metadata().name.clone();
        self.rules.insert(name, Box::new(rule));
    }

    pub fn get_rule(&self, name: &str) -> Option<&Box<dyn Rule>> {
        self.rules.get(name)
    }

    pub fn list_rules(&self) -> Vec<&str> {
        self.rules.keys().map(|s| s.as_str()).collect()
    }

    /// 导出为 Markdown 格式
    pub fn to_markdown(&self) -> String {
        let mut md = format!("# {}\n\n{}\n\n", self.metadata.name, self.metadata.description);

        for (name, rule) in &self.rules {
            md.push_str(&format!("## {}\n\n{}\n\n", name, rule.explain()));
        }

        md
    }

    /// 按分类过滤规则
    pub fn filter_by_category(&self, category: &RuleCategory) -> Vec<&str> {
        self.rules
            .iter()
            .filter(|(_, rule)| rule.category() == *category)
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// 按标签过滤规则
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&str> {
        self.rules
            .iter()
            .filter(|(_, rule)| rule.metadata().tags.iter().any(|t| t == tag))
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// 按名称模糊搜索
    pub fn search(&self, query: &str) -> Vec<&str> {
        let query_lower = query.to_lowercase();
        self.rules
            .iter()
            .filter(|(name, rule)| {
                name.to_lowercase().contains(&query_lower)
                    || rule.metadata().name.to_lowercase().contains(&query_lower)
                    || rule.metadata().description.to_lowercase().contains(&query_lower)
            })
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// 按来源/地区过滤
    pub fn filter_by_origin(&self, origin: &str) -> Vec<&str> {
        self.rules
            .iter()
            .filter(|(_, rule)| {
                rule.metadata()
                    .origin
                    .as_deref()
                    .map(|o| o == origin)
                    .unwrap_or(false)
            })
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// 获取所有规则的元数据快照（可序列化）
    pub fn metadata_snapshot(&self) -> Vec<(&str, &RuleMetadata)> {
        self.rules
            .iter()
            .map(|(name, rule)| (name.as_str(), rule.metadata()))
            .collect()
    }

    /// 统计各分类的规则数量
    pub fn count_by_category(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        for (_, rule) in &self.rules {
            let key = format!("{}", rule.category());
            *counts.entry(key).or_insert(0) += 1;
        }
        counts
    }
}

/// 格式化规则说明的辅助函数
///
/// 将多个 "区块名 + 条目列表" 格式化为统一的说明文本
/// 用法:
/// ```ignore
/// let text = format_rule_sections("我的规则", &[
///     ("场地规格", &self.court_specifications()),
///     ("技术动作", &self.techniques()),
/// ]);
/// ```
pub fn format_rule_sections(title: &str, sections: &[(&str, &Vec<&'static str>)]) -> String {
    let mut result = format!("【{}】", title);
    for (section_name, items) in sections {
        result.push_str(&format!("\n\n{}:\n", section_name));
        for item in *items {
            result.push_str(&format!("  • {}\n", item));
        }
    }
    result
}

/// 格式化三元组规则说明
///
/// 用于科学定律等 (名称, 公式/分类, 描述) 结构
pub fn format_titled_sections(title: &str, sections: &[(&str, &Vec<(&'static str, &'static str, &'static str)>)]) -> String {
    let mut result = format!("【{}】", title);
    for (section_name, items) in sections {
        result.push_str(&format!("\n\n{}:\n", section_name));
        for (name, formula, desc) in *items {
            result.push_str(&format!("  ▶ {}: {} - {}\n", name, formula, desc));
        }
    }
    result
}

/// 生成规则模块的样板代码宏
///
/// 自动生成: 结构体定义、new()、Default
/// 不包含 Rule trait 实现 — 模块需自行编写（可自定义 explain）
///
/// 用法:
/// ```ignore
/// simple_rule! {
///     struct: MyRules,
///     name: "我的规则",
///     desc: "规则描述",
///     origin: "中国",
///     tags: ["体育", "格斗"]
/// }
/// ```
#[macro_export]
macro_rules! simple_rule {
    (
        struct: $name:ident,
        name: $display_name:expr,
        desc: $desc:expr,
        origin: $origin:expr,
        tags: [ $( $tag:expr ),* ] $(,)?
    ) => {
        pub struct $name {
            metadata: $crate::rules::core::RuleMetadata,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    metadata: $crate::rules::core::RuleMetadata::new(
                        $display_name,
                        $desc,
                    )
                    .with_origin($origin)
                    .with_tags(vec![ $( $tag.into() ),* ]),
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试用的简单规则实现
    struct MockRule {
        meta: RuleMetadata,
        cat: RuleCategory,
    }

    impl Rule for MockRule {
        fn metadata(&self) -> &RuleMetadata { &self.meta }
        fn category(&self) -> RuleCategory { self.cat.clone() }
        fn validate(&self, ctx: &str) -> RuleResult<bool> { Ok(!ctx.is_empty()) }
    }

    fn make_ruleset() -> RuleSet {
        let mut rs = RuleSet::new("测试规则集".to_string(), RuleCategory::games("test"));
        rs.add_rule(MockRule {
            meta: RuleMetadata::new("足球规则", "足球比赛规则").with_origin("国际").with_tags(vec!["体育".into(), "球类".into()]),
            cat: RuleCategory::sports("football"),
        });
        rs.add_rule(MockRule {
            meta: RuleMetadata::new("篮球规则", "篮球比赛规则").with_origin("美国").with_tags(vec!["体育".into(), "球类".into()]),
            cat: RuleCategory::sports("basketball"),
        });
        rs.add_rule(MockRule {
            meta: RuleMetadata::new("合同法", "合同法律规则").with_origin("中国").with_tags(vec!["法律".into(), "民法".into()]),
            cat: RuleCategory::law("contract"),
        });
        rs
    }

    #[test]
    fn test_filter_by_tag() {
        let rs = make_ruleset();
        let ball_games = rs.filter_by_tag("球类");
        assert_eq!(ball_games.len(), 2);
        assert!(ball_games.contains(&"足球规则"));
        assert!(ball_games.contains(&"篮球规则"));
    }

    #[test]
    fn test_filter_by_origin() {
        let rs = make_ruleset();
        let china = rs.filter_by_origin("中国");
        assert_eq!(china.len(), 1);
        assert_eq!(china[0], "合同法");
    }

    #[test]
    fn test_search() {
        let rs = make_ruleset();
        let results = rs.search("篮球");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "篮球规则");
    }

    #[test]
    fn test_search_by_description() {
        let rs = make_ruleset();
        let results = rs.search("比赛");
        assert_eq!(results.len(), 2); // 足球和篮球的描述都含"比赛"
    }

    #[test]
    fn test_count_by_category() {
        let rs = make_ruleset();
        let counts = rs.count_by_category();
        assert_eq!(counts.get("Sports/football"), Some(&1));
        assert_eq!(counts.get("Sports/basketball"), Some(&1));
        assert_eq!(counts.get("Law/contract"), Some(&1));
    }

    #[test]
    fn test_metadata_snapshot() {
        let rs = make_ruleset();
        let snapshot = rs.metadata_snapshot();
        assert_eq!(snapshot.len(), 3);
    }

    #[test]
    fn test_rule_category_display() {
        assert_eq!(format!("{}", RuleCategory::games("mahjong")), "Games/mahjong");
        assert_eq!(format!("{}", RuleCategory::sports("football")), "Sports/football");
        assert_eq!(format!("{}", RuleCategory::law("traffic")), "Law/traffic");
    }

    #[test]
    fn test_rule_metadata_serde() {
        let meta = RuleMetadata::new("测试", "描述").with_origin("中国");
        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("测试"));
        assert!(json.contains("中国"));
    }

    #[test]
    fn test_format_rule_sections() {
        let items = vec!["条目1", "条目2"];
        let text = format_rule_sections("标题", &[("章节", &items)]);
        assert!(text.contains("标题"));
        assert!(text.contains("章节"));
        assert!(text.contains("条目1"));
    }
}