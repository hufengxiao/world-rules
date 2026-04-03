//! 核心规则定义

use std::collections::HashMap;

/// 规则分类
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}

impl std::fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleCategory::Games(name) => write!(f, "Games/{}", name),
            RuleCategory::Sports(name) => write!(f, "Sports/{}", name),
            RuleCategory::Social(name) => write!(f, "Social/{}", name),
            RuleCategory::Science(name) => write!(f, "Science/{}", name),
            RuleCategory::Law(name) => write!(f, "Law/{}", name),
            RuleCategory::Custom(name) => write!(f, "Custom/{}", name),
        }
    }
}

/// 规则元数据
#[derive(Debug, Clone)]
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
}