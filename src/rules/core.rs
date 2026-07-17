//! 核心规则定义

use std::collections::HashMap;

/// 三元组规则条目 (名称, 公式/分类, 描述)
pub type TitledItem = (&'static str, &'static str, &'static str);

/// 规则难度等级
///
/// 用于游戏规则难度分级系统。
///
/// # 示例
/// ```
/// use world_rules::rules::core::Difficulty;
///
/// assert!(Difficulty::Beginner < Difficulty::Easy);
/// assert_eq!(format!("{}", Difficulty::Hard), "困难");
/// ```
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Difficulty {
    /// 入门级 - 适合新手学习基本规则
    Beginner,
    /// 简单级 - 掌握基本策略即可参与
    Easy,
    /// 普通级 - 需要一定经验和策略
    #[default]
    Normal,
    /// 困难级 - 需要深入理解和高级策略
    Hard,
    /// 专家级 - 需要精通规则和复杂策略
    Expert,
    /// 大师级 - 最高难度，竞技级别
    Master,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Beginner => write!(f, "入门"),
            Self::Easy => write!(f, "简单"),
            Self::Normal => write!(f, "普通"),
            Self::Hard => write!(f, "困难"),
            Self::Expert => write!(f, "专家"),
            Self::Master => write!(f, "大师"),
        }
    }
}

/// 规则分类
///
/// 每个规则都属于一个大分类下的子分类。
/// 大分类通过枚举变体区分，子分类通过 `String` 字段区分。
///
/// # 示例
/// ```
/// use world_rules::rules::core::RuleCategory;
///
/// let cat = RuleCategory::games("mahjong");
/// assert_eq!(cat.to_string(), "Games/mahjong");
/// ```
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
    /// 创建游戏分类
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleCategory;
    ///
    /// let cat = RuleCategory::games("mahjong");
    /// assert!(matches!(cat, RuleCategory::Games(_)));
    /// ```
    pub fn games(name: impl Into<String>) -> Self {
        Self::Games(name.into())
    }

    /// 创建体育分类
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleCategory;
    ///
    /// let cat = RuleCategory::sports("football");
    /// assert!(matches!(cat, RuleCategory::Sports(_)));
    /// ```
    pub fn sports(name: impl Into<String>) -> Self {
        Self::Sports(name.into())
    }

    /// 创建社交礼仪分类
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleCategory;
    ///
    /// let cat = RuleCategory::social("dining");
    /// assert!(matches!(cat, RuleCategory::Social(_)));
    /// ```
    pub fn social(name: impl Into<String>) -> Self {
        Self::Social(name.into())
    }

    /// 创建科学分类
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleCategory;
    ///
    /// let cat = RuleCategory::science("physics");
    /// assert!(matches!(cat, RuleCategory::Science(_)));
    /// ```
    pub fn science(name: impl Into<String>) -> Self {
        Self::Science(name.into())
    }

    /// 创建法律分类
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleCategory;
    ///
    /// let cat = RuleCategory::law("contract");
    /// assert!(matches!(cat, RuleCategory::Law(_)));
    /// ```
    pub fn law(name: impl Into<String>) -> Self {
        Self::Law(name.into())
    }

    /// 创建健康分类
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleCategory;
    ///
    /// let cat = RuleCategory::health("nutrition");
    /// assert!(matches!(cat, RuleCategory::Health(_)));
    /// ```
    pub fn health(name: impl Into<String>) -> Self {
        Self::Health(name.into())
    }

    /// 创建自定义分类
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleCategory;
    ///
    /// let cat = RuleCategory::custom("custom", "my_rule");
    /// assert!(matches!(cat, RuleCategory::Custom(_)));
    /// assert_eq!(cat.to_string(), "Custom/custom/my_rule");
    /// ```
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
///
/// 包含规则的基本信息：名称、描述、版本、来源、标签和难度等级。
/// 通过 builder 模式构造。
///
/// # 示例
/// ```
/// use world_rules::rules::core::{RuleMetadata, Difficulty};
///
/// let meta = RuleMetadata::new("四川麻将", "血战到底规则")
///     .with_origin("四川")
///     .with_tags(vec!["麻将".into(), "地方变体".into()])
///     .with_difficulty(Difficulty::Hard);
/// assert_eq!(meta.name, "四川麻将");
/// assert_eq!(meta.difficulty, Difficulty::Hard);
/// ```
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
    /// 规则难度等级（默认 Normal）
    #[serde(default)]
    pub difficulty: Difficulty,
}

impl RuleMetadata {
    /// 创建规则元数据
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleMetadata;
    ///
    /// let meta = RuleMetadata::new("四川麻将", "血战到底规则");
    /// assert_eq!(meta.name, "四川麻将");
    /// assert_eq!(meta.description, "血战到底规则");
    /// ```
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: "1.0.0".to_string(),
            origin: None,
            tags: Vec::new(),
            difficulty: Difficulty::default(),
        }
    }

    /// 设置规则版本
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleMetadata;
    ///
    /// let meta = RuleMetadata::new("规则", "描述").with_version("2.0.0");
    /// assert_eq!(meta.version, "2.0.0");
    /// ```
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// 设置规则来源/地区
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleMetadata;
    ///
    /// let meta = RuleMetadata::new("规则", "描述").with_origin("中国");
    /// assert_eq!(meta.origin, Some("中国".to_string()));
    /// ```
    pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    /// 设置规则标签
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::RuleMetadata;
    ///
    /// let meta = RuleMetadata::new("规则", "描述")
    ///     .with_tags(vec!["体育".into(), "球类".into()]);
    /// assert_eq!(meta.tags.len(), 2);
    /// ```
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// 设置规则难度等级
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleMetadata, Difficulty};
    ///
    /// let meta = RuleMetadata::new("围棋", "古老棋类游戏")
    ///     .with_difficulty(Difficulty::Expert);
    /// assert_eq!(meta.difficulty, Difficulty::Expert);
    /// ```
    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }
}

impl std::fmt::Display for RuleMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(origin) = &self.origin {
            write!(f, " ({})", origin)?;
        }
        if !self.tags.is_empty() {
            write!(f, " [{}]", self.tags.join(", "))?;
        }
        Ok(())
    }
}

/// 规则错误类型
///
/// 统一的错误枚举，用于规则验证和查询过程中的错误处理。
///
/// # Examples
/// ```
/// use world_rules::rules::core::RuleError;
///
/// let err = RuleError::RuleNotFound("麻将规则".to_string());
/// assert!(err.to_string().contains("麻将规则"));
/// ```
#[derive(Debug, thiserror::Error)]
pub enum RuleError {
    /// 规则不存在错误
    #[error("规则不存在: {0}")]
    RuleNotFound(String),

    /// 规则验证失败错误
    #[error("规则验证失败: {0}")]
    ValidationError(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 不支持的操作错误
    #[error("不支持的操作: {0}")]
    UnsupportedOperation(String),

    /// 上下文类型不匹配错误
    #[error("上下文类型不匹配: 期望 {expected}, 实际 {actual}")]
    ContextMismatch {
        /// 期望的上下文类型
        expected: String,
        /// 实际的上下文类型
        actual: String,
    },
}

/// 规则操作结果类型
///
/// 使用 `Result<T, RuleError>` 作为规则操作的统一返回类型。
pub type RuleResult<T> = Result<T, RuleError>;

/// 验证上下文
///
/// 为不同游戏类型提供类型安全的验证上下文。
/// 使用枚举确保在编译时就能识别上下文类型，避免字符串解析错误。
///
/// # 示例
/// ```rust
/// use world_rules::rules::core::ValidateContext;
///
/// let ctx = ValidateContext::doudizhu_cards("3s 4h 5d");
/// assert!(matches!(ctx, ValidateContext::DouDiZhuCards(_)));
///
/// let ctx = ValidateContext::chess_move("车", "0,0", "0,5");
/// assert!(matches!(ctx, ValidateContext::ChessMove { .. }));
/// ```
#[derive(Debug, Clone)]
pub enum ValidateContext {
    /// 斗地主牌面 (如 "3s 4h 5d")
    DouDiZhuCards(String),
    /// 麻将牌面 (如 "1m 2m 3m")
    MahjongTiles(String),
    /// 扑克牌面 (如 "As Kh Qd")
    PokerCards(String),
    /// 象棋走法
    ChessMove {
        /// 棋子名称（如"车"、"马"）
        piece: String,
        /// 起始位置（坐标格式）
        from: String,
        /// 目标位置（坐标格式）
        to: String,
    },
    /// 五子棋棋盘 (坐标列表: (x, y, is_black))
    GomokuBoard(Vec<(usize, usize, bool)>),
    /// 通用上下文
    Generic(String),
}

impl Default for ValidateContext {
    fn default() -> Self {
        Self::Generic(String::new())
    }
}

impl ValidateContext {
    /// 创建斗地主牌面上下文
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::ValidateContext;
    ///
    /// let ctx = ValidateContext::doudizhu_cards("3s 4h 5d");
    /// assert!(matches!(ctx, ValidateContext::DouDiZhuCards(_)));
    /// ```
    pub fn doudizhu_cards(cards: impl Into<String>) -> Self {
        Self::DouDiZhuCards(cards.into())
    }

    /// 创建麻将牌面上下文
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::ValidateContext;
    ///
    /// let ctx = ValidateContext::mahjong_tiles("1m 2m 3m");
    /// assert!(matches!(ctx, ValidateContext::MahjongTiles(_)));
    /// ```
    pub fn mahjong_tiles(tiles: impl Into<String>) -> Self {
        Self::MahjongTiles(tiles.into())
    }

    /// 创建扑克牌面上下文
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::ValidateContext;
    ///
    /// let ctx = ValidateContext::poker_cards("As Kh Qd");
    /// assert!(matches!(ctx, ValidateContext::PokerCards(_)));
    /// ```
    pub fn poker_cards(cards: impl Into<String>) -> Self {
        Self::PokerCards(cards.into())
    }

    /// 创建象棋走法上下文
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::ValidateContext;
    ///
    /// let ctx = ValidateContext::chess_move("车", "0,0", "0,5");
    /// assert!(matches!(ctx, ValidateContext::ChessMove { .. }));
    /// ```
    pub fn chess_move(
        piece: impl Into<String>,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        Self::ChessMove {
            piece: piece.into(),
            from: from.into(),
            to: to.into(),
        }
    }

    /// 创建五子棋棋盘上下文
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::ValidateContext;
    ///
    /// let moves = vec![(0, 0, true), (1, 1, false)];
    /// let ctx = ValidateContext::gomoku_board(moves);
    /// assert!(matches!(ctx, ValidateContext::GomokuBoard(_)));
    /// ```
    pub fn gomoku_board(moves: Vec<(usize, usize, bool)>) -> Self {
        Self::GomokuBoard(moves)
    }

    /// 创建通用上下文
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::ValidateContext;
    ///
    /// let ctx = ValidateContext::generic("任意上下文");
    /// assert!(matches!(ctx, ValidateContext::Generic(_)));
    /// ```
    pub fn generic(context: impl Into<String>) -> Self {
        Self::Generic(context.into())
    }

    /// 获取上下文类型名称
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::ValidateContext;
    ///
    /// let ctx = ValidateContext::doudizhu_cards("3s 4h");
    /// assert_eq!(ctx.type_name(), "斗地主牌面");
    /// ```
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::DouDiZhuCards(_) => "斗地主牌面",
            Self::MahjongTiles(_) => "麻将牌面",
            Self::PokerCards(_) => "扑克牌面",
            Self::ChessMove { .. } => "象棋走法",
            Self::GomokuBoard(_) => "五子棋棋盘",
            Self::Generic(_) => "通用上下文",
        }
    }

    /// 尝试获取通用上下文字符串
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::ValidateContext;
    ///
    /// let ctx = ValidateContext::generic("内容");
    /// assert_eq!(ctx.as_generic_str(), Some("内容"));
    ///
    /// let ctx2 = ValidateContext::doudizhu_cards("3s");
    /// assert_eq!(ctx2.as_generic_str(), None);
    /// ```
    pub fn as_generic_str(&self) -> Option<&str> {
        match self {
            Self::Generic(s) => Some(s),
            _ => None,
        }
    }
}

impl std::fmt::Display for ValidateContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DouDiZhuCards(cards) => write!(f, "斗地主: {}", cards),
            Self::MahjongTiles(tiles) => write!(f, "麻将: {}", tiles),
            Self::PokerCards(cards) => write!(f, "扑克: {}", cards),
            Self::ChessMove { piece, from, to } => {
                write!(f, "象棋: {} {}->{}", piece, from, to)
            }
            Self::GomokuBoard(moves) => {
                write!(f, "五子棋: {} 步", moves.len())
            }
            Self::Generic(s) => write!(f, "{}", s),
        }
    }
}

/// 规则核心 trait
///
/// 所有规则都需要实现此 trait。提供了元数据查询、分类、验证和说明四个核心方法。
///
/// # 实现指南
///
/// 使用 `simple_rule!` 宏可以自动生成大部分实现。手动实现时：
/// - `metadata()` 返回规则的名称、描述等基本信息
/// - `category()` 返回规则所属的分类
/// - `validate()` 验证给定上下文是否符合规则（默认实现返回 `Ok(true)`）
/// - `explain()` 返回规则的详细说明文本
///
/// # 示例
/// ```rust
/// use world_rules::prelude::*;
///
/// let rule = SichuanMahjongRules::new();
/// assert!(!rule.metadata().name.is_empty());
/// assert!(!rule.explain().is_empty());
/// ```
pub trait Rule: Send + Sync {
    /// 获取规则元数据
    fn metadata(&self) -> &RuleMetadata;

    /// 获取规则分类
    fn category(&self) -> RuleCategory;

    /// 验证某个状态是否符合规则（默认实现：接受通用上下文即通过）
    fn validate(&self, context: &ValidateContext) -> RuleResult<bool> {
        // 默认实现：接受任何上下文
        let _ = context;
        Ok(true)
    }

    /// 获取规则的详细说明
    fn explain(&self) -> String {
        format!(
            "【{}】\n{}版本: {}\n来源: {}",
            self.metadata().name,
            self.metadata().description,
            self.metadata().version,
            self.metadata().origin.as_deref().unwrap_or("未知")
        )
    }
}

/// 规则集 - 包含一组相关规则
///
/// 管理一组相关的规则，支持按分类/标签/名称查询。
///
/// # 示例
/// ```
/// use world_rules::rules::core::{RuleSet, RuleCategory, RuleMetadata};
///
/// let mut rs = RuleSet::new("测试规则集".to_string(), RuleCategory::games("test"));
/// assert_eq!(rs.list_rules().len(), 0);
/// ```
pub struct RuleSet {
    /// 规则集元数据（名称、描述等）
    pub metadata: RuleMetadata,
    /// 规则集所属分类
    pub category: RuleCategory,
    /// 规则集合（按名称索引）
    pub rules: HashMap<String, Box<dyn Rule>>,
}

impl std::fmt::Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "【{}】{} ({} 条规则)",
            self.metadata.name,
            self.metadata.description,
            self.rules.len()
        )
    }
}

impl RuleSet {
    /// 创建新的规则集
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试规则集".to_string(), RuleCategory::games("test"));
    /// assert_eq!(rs.metadata.name, "测试规则集");
    /// ```
    pub fn new(name: String, category: RuleCategory) -> Self {
        Self {
            metadata: RuleMetadata::new(&name, format!("{} 规则集", name)),
            category,
            rules: HashMap::new(),
        }
    }

    /// 设置规则集描述
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("规则集".to_string(), RuleCategory::games("test"))
    ///     .with_description("自定义描述");
    /// assert_eq!(rs.metadata.description, "自定义描述");
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.metadata.description = description.into();
        self
    }

    /// 添加规则到规则集
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory, Rule, RuleMetadata};
    ///
    /// struct MyRule { meta: RuleMetadata }
    /// impl Rule for MyRule {
    ///     fn metadata(&self) -> &RuleMetadata { &self.meta }
    ///     fn category(&self) -> RuleCategory { RuleCategory::games("test") }
    /// }
    ///
    /// let mut rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// rs.add_rule(MyRule { meta: RuleMetadata::new("规则", "描述") });
    /// assert_eq!(rs.len(), 1);
    /// ```
    pub fn add_rule<R: Rule + 'static>(&mut self, rule: R) {
        let name = rule.metadata().name.clone();
        self.rules.insert(name, Box::new(rule));
    }

    /// 获取规则数量
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// assert_eq!(rs.len(), 0);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// 是否为空
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// assert!(rs.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// 按名称获取规则
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// assert!(rs.get_rule("不存在").is_none());
    /// ```
    #[must_use]
    pub fn get_rule(&self, name: &str) -> Option<&dyn Rule> {
        self.rules.get(name).map(|b| b.as_ref())
    }

    /// 列出所有规则名称
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// let names = rs.list_rules();
    /// assert!(names.is_empty());
    /// ```
    #[must_use]
    pub fn list_rules(&self) -> Vec<&str> {
        self.rules.keys().map(|s| s.as_str()).collect()
    }

    /// 导出为 Markdown 格式
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// let md = rs.to_markdown();
    /// assert!(md.contains("测试"));
    /// ```
    pub fn to_markdown(&self) -> String {
        let mut md = format!(
            "# {}\n\n{}\n\n",
            self.metadata.name, self.metadata.description
        );

        for (name, rule) in &self.rules {
            md.push_str(&format!("## {}\n\n{}\n\n", name, rule.explain()));
        }

        md
    }

    /// 按分类过滤规则
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// let filtered = rs.filter_by_category(&RuleCategory::games("test"));
    /// assert!(filtered.is_empty());
    /// ```
    pub fn filter_by_category(&self, category: &RuleCategory) -> Vec<&str> {
        self.rules
            .iter()
            .filter(|(_, rule)| rule.category() == *category)
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// 按标签过滤规则
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// let filtered = rs.filter_by_tag("体育");
    /// assert!(filtered.is_empty());
    /// ```
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&str> {
        self.rules
            .iter()
            .filter(|(_, rule)| rule.metadata().tags.iter().any(|t| t == tag))
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// 按名称模糊搜索
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// let results = rs.search("关键词");
    /// assert!(results.is_empty());
    /// ```
    pub fn search(&self, query: &str) -> Vec<&str> {
        let query_lower = query.to_lowercase();
        self.rules
            .iter()
            .filter(|(name, rule)| {
                name.to_lowercase().contains(&query_lower)
                    || rule.metadata().name.to_lowercase().contains(&query_lower)
                    || rule
                        .metadata()
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
            })
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// 按来源/地区过滤
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// let filtered = rs.filter_by_origin("中国");
    /// assert!(filtered.is_empty());
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// let snapshot = rs.metadata_snapshot();
    /// assert!(snapshot.is_empty());
    /// ```
    pub fn metadata_snapshot(&self) -> Vec<(&str, &RuleMetadata)> {
        self.rules
            .iter()
            .map(|(name, rule)| (name.as_str(), rule.metadata()))
            .collect()
    }

    /// 统计各分类的规则数量
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleSet, RuleCategory};
    ///
    /// let rs = RuleSet::new("测试".to_string(), RuleCategory::games("test"));
    /// let counts = rs.count_by_category();
    /// assert!(counts.is_empty());
    /// ```
    pub fn count_by_category(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        for rule in self.rules.values() {
            let key = format!("{}", rule.category());
            *counts.entry(key).or_insert(0) += 1;
        }
        counts
    }
}

/// 规则验证器 - 统一校验规则完整性
///
/// 提供规则元数据和规则本身的完整性验证功能。
///
/// # Examples
/// ```
/// use world_rules::rules::core::{RuleValidator, RuleMetadata};
///
/// let meta = RuleMetadata::new("测试规则", "测试描述");
/// let issues = RuleValidator::validate_metadata(&meta);
/// assert!(issues.is_empty());
/// ```
pub struct RuleValidator;

impl RuleValidator {
    /// 验证规则元数据完整性
    ///
    /// 检查规则元数据的必填字段是否完整。
    ///
    /// # Arguments
    ///
    /// * `meta` - 要验证的规则元数据
    ///
    /// # Returns
    ///
    /// 返回一个包含所有问题描述的 Vec。如果元数据完整，返回空 Vec。
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleValidator, RuleMetadata};
    ///
    /// // 完整的元数据
    /// let meta = RuleMetadata::new("规则名称", "规则描述");
    /// let issues = RuleValidator::validate_metadata(&meta);
    /// assert!(issues.is_empty());
    ///
    /// // 缺失名称的元数据
    /// let bad_meta = RuleMetadata::new("", "描述");
    /// let issues = RuleValidator::validate_metadata(&bad_meta);
    /// assert!(issues.iter().any(|i| i.contains("名称")));
    /// ```
    pub fn validate_metadata(meta: &RuleMetadata) -> Vec<String> {
        let mut issues = Vec::new();
        if meta.name.is_empty() {
            issues.push("规则名称为空".to_string());
        }
        if meta.description.is_empty() {
            issues.push("规则描述为空".to_string());
        }
        if meta.version.is_empty() {
            issues.push("规则版本为空".to_string());
        }
        issues
    }

    /// 验证规则完整性（元数据 + explain）
    ///
    /// 检查规则的元数据和说明是否完整。
    ///
    /// # Arguments
    ///
    /// * `rule` - 要验证的规则（实现了 Rule trait）
    ///
    /// # Returns
    ///
    /// 返回一个包含所有问题描述的 Vec。如果规则完整，返回空 Vec。
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleValidator, Rule, RuleMetadata, RuleCategory};
    ///
    /// struct TestRule { meta: RuleMetadata }
    /// impl Rule for TestRule {
    ///     fn metadata(&self) -> &RuleMetadata { &self.meta }
    ///     fn category(&self) -> RuleCategory { RuleCategory::games("test") }
    /// }
    ///
    /// let rule = TestRule { meta: RuleMetadata::new("测试", "描述") };
    /// let issues = RuleValidator::validate_rule(&rule);
    /// assert!(issues.is_empty());
    /// ```
    pub fn validate_rule(rule: &dyn Rule) -> Vec<String> {
        let mut issues = Self::validate_metadata(rule.metadata());
        let explain = rule.explain();
        if explain.is_empty() {
            issues.push("explain() 返回空".to_string());
        }
        issues
    }

    /// 批量验证规则集
    ///
    /// 对一组规则进行完整性验证。
    ///
    /// # Arguments
    ///
    /// * `rules` - 要验证的规则列表
    ///
    /// # Returns
    ///
    /// 返回一个包含规则名称和对应问题列表的 Vec。
    /// 只包含有问题的规则，完整的规则不会出现在结果中。
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::core::{RuleValidator, Rule, RuleMetadata, RuleCategory};
    ///
    /// struct GoodRule { meta: RuleMetadata }
    /// impl Rule for GoodRule {
    ///     fn metadata(&self) -> &RuleMetadata { &self.meta }
    ///     fn category(&self) -> RuleCategory { RuleCategory::games("test") }
    /// }
    ///
    /// let rule = GoodRule { meta: RuleMetadata::new("完整规则", "描述") };
    /// let rules: Vec<&dyn Rule> = vec![&rule];
    /// let issues = RuleValidator::validate_ruleset(&rules);
    /// assert!(issues.is_empty()); // 完整规则不在结果中
    /// ```
    pub fn validate_ruleset(rules: &[&dyn Rule]) -> Vec<(String, Vec<String>)> {
        rules
            .iter()
            .map(|r| {
                let issues = Self::validate_rule(*r);
                (r.metadata().name.clone(), issues)
            })
            .filter(|(_, issues)| !issues.is_empty())
            .collect()
    }
}

/// 格式化规则说明
///
/// 将多个规则章节格式化为统一的字符串格式
///
/// # Examples
///
/// ```rust,ignore
/// let text = format_rule_sections("我的规则", &[
///     ("场地规格", &self.court_specifications()),
///     ("技术动作", &self.techniques()),
/// ]);
/// ```
pub fn format_rule_sections<T: AsRef<str>>(title: &str, sections: &[(&str, &Vec<T>)]) -> String {
    let mut result = format!("【{}】", title);
    for (section_name, items) in sections {
        result.push_str(&format!("\n\n{}:\n", section_name));
        for item in *items {
            result.push_str(&format!("  • {}\n", item.as_ref()));
        }
    }
    result
}

/// 格式化三元组规则说明
///
/// 用于科学定律等 (名称, 公式/分类, 描述) 结构
pub fn format_titled_sections(title: &str, sections: &[(&str, &Vec<TitledItem>)]) -> String {
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
/// 自动生成: 结构体定义、new()、Default、Rule trait 实现、测试
///
/// 基础用法 (仅生成 struct + new + Default):
/// ```ignore
/// simple_rule! {
///     struct: MyRules,
///     name: "我的规则",
///     desc: "规则描述",
///     origin: "中国",
///     tags: ["体育", "格斗"]
/// }
/// ```
///
/// 完整用法 (自动生成 Rule trait + explain + 测试):
/// ```ignore
/// simple_rule! {
///     struct: MyRules,
///     name: "我的规则",
///     desc: "规则描述",
///     origin: "中国",
///     tags: ["体育", "格斗"],
///     category: RuleCategory::sports("my_sport"),
///     sections: [("基础规则", section_0), ("得分规则", section_1)]
/// }
/// ```
#[macro_export]
macro_rules! simple_rule {
    // 完整模式: 包含 category + sections，自动生成 Rule trait
    (
        struct: $name:ident,
        name: $display_name:expr,
        desc: $desc:expr,
        origin: $origin:expr,
        tags: [ $( $tag:expr ),* ],
        category: $category:expr,
        sections: [ $( ($section_title:expr, $section_fn:ident) ),* ] $(,)?
    ) => {
        #[doc = $display_name]
        #[doc = $desc]
        pub struct $name {
            metadata: $crate::rules::core::RuleMetadata,
        }

        impl $name {
            /// 创建新的规则实例
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

        impl $crate::rules::core::Rule for $name {
            fn metadata(&self) -> &$crate::rules::core::RuleMetadata {
                &self.metadata
            }
            fn category(&self) -> $crate::rules::core::RuleCategory {
                $category
            }
            fn validate(&self, ctx: &$crate::rules::core::ValidateContext) -> $crate::rules::core::RuleResult<bool> {
                // 默认实现：接受任何上下文
                let _ = ctx;
                Ok(true)
            }
            fn explain(&self) -> String {
                $crate::rules::core::format_rule_sections(
                    $display_name,
                    &[
                        $( ($section_title, &self.$section_fn()) ),*
                    ],
                )
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test() {
                use $crate::rules::core::Rule as _;
                let r = $name::new();
                assert!(!r.metadata().name.is_empty());
                assert!(!r.explain().is_empty());
            }
        }
    };

    // 基础模式: 仅 struct + new + Default (向后兼容)
    (
        struct: $name:ident,
        name: $display_name:expr,
        desc: $desc:expr,
        origin: $origin:expr,
        tags: [ $( $tag:expr ),* ] $(,)?
    ) => {
        #[doc = $display_name]
        #[doc = $desc]
        pub struct $name {
            metadata: $crate::rules::core::RuleMetadata,
        }

        impl $name {
            /// 创建新的规则实例
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
        fn metadata(&self) -> &RuleMetadata {
            &self.meta
        }
        fn category(&self) -> RuleCategory {
            self.cat.clone()
        }
    }

    fn make_ruleset() -> RuleSet {
        let mut rs = RuleSet::new("测试规则集".to_string(), RuleCategory::games("test"));
        rs.add_rule(MockRule {
            meta: RuleMetadata::new("足球规则", "足球比赛规则")
                .with_origin("国际")
                .with_tags(vec!["体育".into(), "球类".into()]),
            cat: RuleCategory::sports("football"),
        });
        rs.add_rule(MockRule {
            meta: RuleMetadata::new("篮球规则", "篮球比赛规则")
                .with_origin("美国")
                .with_tags(vec!["体育".into(), "球类".into()]),
            cat: RuleCategory::sports("basketball"),
        });
        rs.add_rule(MockRule {
            meta: RuleMetadata::new("合同法", "合同法律规则")
                .with_origin("中国")
                .with_tags(vec!["法律".into(), "民法".into()]),
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
        assert_eq!(
            format!("{}", RuleCategory::games("mahjong")),
            "Games/mahjong"
        );
        assert_eq!(
            format!("{}", RuleCategory::sports("football")),
            "Sports/football"
        );
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

#[cfg(test)]
mod validator_tests {
    use super::*;

    #[test]
    fn test_validate_metadata_ok() {
        let meta = RuleMetadata::new("测试规则", "描述");
        let issues = RuleValidator::validate_metadata(&meta);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_validate_metadata_empty_name() {
        let meta = RuleMetadata::new("", "描述");
        let issues = RuleValidator::validate_metadata(&meta);
        assert!(issues.iter().any(|i| i.contains("名称")));
    }

    #[test]
    fn test_validate_metadata_empty_desc() {
        let meta = RuleMetadata::new("名称", "");
        let issues = RuleValidator::validate_metadata(&meta);
        assert!(issues.iter().any(|i| i.contains("描述")));
    }
}

#[cfg(test)]
mod difficulty_tests {
    use super::*;

    #[test]
    fn test_difficulty_ordering() {
        assert!(Difficulty::Beginner < Difficulty::Easy);
        assert!(Difficulty::Easy < Difficulty::Normal);
        assert!(Difficulty::Normal < Difficulty::Hard);
        assert!(Difficulty::Hard < Difficulty::Expert);
        assert!(Difficulty::Expert < Difficulty::Master);
    }

    #[test]
    fn test_difficulty_default() {
        let diff = Difficulty::default();
        assert_eq!(diff, Difficulty::Normal);
    }

    #[test]
    fn test_difficulty_display() {
        assert_eq!(format!("{}", Difficulty::Beginner), "入门");
        assert_eq!(format!("{}", Difficulty::Easy), "简单");
        assert_eq!(format!("{}", Difficulty::Normal), "普通");
        assert_eq!(format!("{}", Difficulty::Hard), "困难");
        assert_eq!(format!("{}", Difficulty::Expert), "专家");
        assert_eq!(format!("{}", Difficulty::Master), "大师");
    }

    #[test]
    fn test_metadata_with_difficulty() {
        let meta = RuleMetadata::new("围棋", "古老棋类游戏").with_difficulty(Difficulty::Expert);
        assert_eq!(meta.difficulty, Difficulty::Expert);
    }

    #[test]
    fn test_metadata_default_difficulty() {
        let meta = RuleMetadata::new("测试", "描述");
        assert_eq!(meta.difficulty, Difficulty::Normal);
    }

    #[test]
    fn test_difficulty_serde() {
        let diff = Difficulty::Hard;
        let json = serde_json::to_string(&diff).unwrap();
        assert!(json.contains("Hard"));
        let parsed: Difficulty = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Difficulty::Hard);
    }
}
