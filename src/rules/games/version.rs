//! 游戏规则版本管理系统
//!
//! 本模块提供游戏规则的版本控制功能，支持版本追踪、兼容性检查、版本历史等。
//!
//! # 设计目标
//!
//! - **语义化版本**: 遵循 semver 规范 (MAJOR.MINOR.PATCH)
//! - **兼容性检查**: 自动判断版本间的兼容性
//! - **版本历史**: 记录规则演变历史
//! - **规则迁移**: 支持从旧版本迁移到新版本
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::version::*;
//!
//! // 创建版本号
//! let v1 = GameVersion::new(1, 0, 0);
//! let v2 = GameVersion::new(1, 1, 0);
//!
//! // 版本比较
//! assert!(v2 > v1);
//! assert!(v1.is_compatible_with(&v2));
//!
//! // 创建版本历史
//! let mut history = VersionHistory::new("围棋规则");
//! history.add_version(v1, "初始版本");
//! history.add_version(v2, "增加贴目规则");
//! ```

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

/// 游戏规则版本号
///
/// 遵循语义化版本规范 (SemVer 2.0.0)。
///
/// # 版本号规则
///
/// - **MAJOR**: 不兼容的 API 修改
/// - **MINOR**: 向后兼容的功能性新增
/// - **PATCH**: 向后兼容的问题修正
///
/// # Examples
///
/// ```
/// use world_rules::rules::games::version::GameVersion;
///
/// let v = GameVersion::new(2, 1, 3);
/// assert_eq!(v.major(), 2);
/// assert_eq!(v.minor(), 1);
/// assert_eq!(v.patch(), 3);
/// assert_eq!(v.to_string(), "2.1.3");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GameVersion {
    /// 主版本号
    major: u32,
    /// 次版本号
    minor: u32,
    /// 补丁版本号
    patch: u32,
}

impl GameVersion {
    /// 创建新的版本号
    ///
    /// # Examples
    ///
    /// ```
    /// use world_rules::rules::games::version::GameVersion;
    ///
    /// let v = GameVersion::new(1, 2, 3);
    /// assert_eq!(v.major(), 1);
    /// assert_eq!(v.minor(), 2);
    /// assert_eq!(v.patch(), 3);
    /// ```
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// 创建初始版本 (0.1.0)
    pub fn initial() -> Self {
        Self::new(0, 1, 0)
    }

    /// 获取主版本号
    pub fn major(&self) -> u32 {
        self.major
    }

    /// 获取次版本号
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// 获取补丁版本号
    pub fn patch(&self) -> u32 {
        self.patch
    }

    /// 从字符串解析版本号
    ///
    /// # Examples
    ///
    /// ```
    /// use world_rules::rules::games::version::GameVersion;
    ///
    /// let v = GameVersion::parse("2.1.0").unwrap();
    /// assert_eq!(v.major(), 2);
    /// assert_eq!(v.minor(), 1);
    /// assert_eq!(v.patch(), 0);
    /// ```
    pub fn parse(s: &str) -> Result<Self, VersionError> {
        let parts: Vec<&str> = s.trim().split('.').collect();
        if parts.len() != 3 {
            return Err(VersionError::InvalidFormat(s.to_string()));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| VersionError::InvalidFormat(s.to_string()))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| VersionError::InvalidFormat(s.to_string()))?;
        let patch = parts[2]
            .parse()
            .map_err(|_| VersionError::InvalidFormat(s.to_string()))?;

        Ok(Self::new(major, minor, patch))
    }

    /// 递增主版本号 (不兼容变更)
    pub fn bump_major(&self) -> Self {
        Self::new(self.major + 1, 0, 0)
    }

    /// 递增次版本号 (兼容功能新增)
    pub fn bump_minor(&self) -> Self {
        Self::new(self.major, self.minor + 1, 0)
    }

    /// 递增补丁版本号 (Bug 修复)
    pub fn bump_patch(&self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1)
    }

    /// 检查是否与另一个版本兼容
    ///
    /// 兼容性规则:
    /// - 主版本号相同则兼容
    /// - 主版本号为 0 时，次版本号也需相同
    ///
    /// # Examples
    ///
    /// ```
    /// use world_rules::rules::games::version::GameVersion;
    ///
    /// let v1 = GameVersion::new(1, 0, 0);
    /// let v2 = GameVersion::new(1, 5, 3);
    /// let v3 = GameVersion::new(2, 0, 0);
    ///
    /// assert!(v1.is_compatible_with(&v2)); // 相同主版本
    /// assert!(!v1.is_compatible_with(&v3)); // 不同主版本
    /// ```
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        if self.major == 0 && other.major == 0 {
            // 预发布版本，次版本号也需相同
            self.minor == other.minor
        } else {
            self.major == other.major && other.major > 0
        }
    }

    /// 检查是否是稳定版本 (主版本号 > 0)
    pub fn is_stable(&self) -> bool {
        self.major > 0
    }

    /// 检查是否是预发布版本 (主版本号 = 0)
    pub fn is_prerelease(&self) -> bool {
        self.major == 0
    }

    /// 比较版本优先级
    ///
    /// 返回:
    /// - `Ordering::Less`: self < other
    /// - `Ordering::Equal`: self == other
    /// - `Ordering::Greater`: self > other
    pub fn compare(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Equal => self.patch.cmp(&other.patch),
                ord => ord,
            },
            ord => ord,
        }
    }

    /// 检查版本是否在指定范围内
    ///
    /// # Examples
    ///
    /// ```
    /// use world_rules::rules::games::version::GameVersion;
    ///
    /// let v = GameVersion::new(1, 5, 0);
    /// let min = GameVersion::new(1, 0, 0);
    /// let max = GameVersion::new(2, 0, 0);
    ///
    /// assert!(v.in_range(&min, &max));
    /// ```
    pub fn in_range(&self, min: &Self, max: &Self) -> bool {
        self.compare(min) >= Ordering::Equal && self.compare(max) <= Ordering::Equal
    }
}

impl fmt::Display for GameVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for GameVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for GameVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

/// 版本变更类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChangeType {
    /// 不兼容的 API 变更
    Major,
    /// 向后兼容的功能新增
    Minor,
    /// 向后兼容的问题修复
    Patch,
}

impl fmt::Display for ChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChangeType::Major => write!(f, "主版本"),
            ChangeType::Minor => write!(f, "次版本"),
            ChangeType::Patch => write!(f, "补丁"),
        }
    }
}

/// 版本变更记录
#[derive(Debug, Clone)]
pub struct VersionChange {
    /// 变更前的版本
    pub from: GameVersion,
    /// 变更后的版本
    pub to: GameVersion,
    /// 变更类型
    pub change_type: ChangeType,
    /// 变更描述
    pub description: String,
    /// 变更时间戳
    pub timestamp: u64,
}

impl VersionChange {
    /// 创建版本变更记录
    pub fn new(from: GameVersion, to: GameVersion, description: impl Into<String>) -> Self {
        let change_type = Self::determine_change_type(&from, &to);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            from,
            to,
            change_type,
            description: description.into(),
            timestamp,
        }
    }

    /// 确定变更类型
    fn determine_change_type(from: &GameVersion, to: &GameVersion) -> ChangeType {
        if to.major != from.major {
            ChangeType::Major
        } else if to.minor != from.minor {
            ChangeType::Minor
        } else {
            ChangeType::Patch
        }
    }

    /// 检查是否是破坏性变更
    pub fn is_breaking(&self) -> bool {
        self.change_type == ChangeType::Major
    }
}

/// 版本历史记录
#[derive(Debug, Clone)]
pub struct VersionHistory {
    /// 规则名称
    name: String,
    /// 版本变更记录
    changes: Vec<VersionChange>,
    /// 当前版本
    current: GameVersion,
    /// 版本标签 (如 "latest", "stable", "legacy")
    tags: HashMap<String, GameVersion>,
}

impl VersionHistory {
    /// 创建版本历史
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            changes: Vec::new(),
            current: GameVersion::initial(),
            tags: HashMap::new(),
        }
    }

    /// 获取规则名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 添加版本
    pub fn add_version(&mut self, version: GameVersion, description: impl Into<String>) {
        if version != self.current {
            let change = VersionChange::new(self.current, version, description);
            self.changes.push(change);
            self.current = version;
        }
    }

    /// 获取当前版本
    pub fn current(&self) -> GameVersion {
        self.current
    }

    /// 获取指定版本
    pub fn get_version(&self, version: &GameVersion) -> Option<&VersionChange> {
        self.changes.iter().find(|c| &c.to == version)
    }

    /// 获取所有版本
    pub fn all_versions(&self) -> Vec<GameVersion> {
        let mut versions: Vec<GameVersion> = self.changes.iter().map(|c| c.to).collect();
        if self.changes.is_empty() {
            versions.push(self.current);
        }
        versions
    }

    /// 获取版本数量
    pub fn version_count(&self) -> usize {
        self.changes.len()
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: impl Into<String>, version: GameVersion) {
        self.tags.insert(tag.into(), version);
    }

    /// 获取标签对应的版本
    pub fn get_tag(&self, tag: &str) -> Option<GameVersion> {
        self.tags.get(tag).copied()
    }

    /// 获取所有标签
    pub fn all_tags(&self) -> &HashMap<String, GameVersion> {
        &self.tags
    }

    /// 检查版本是否存在
    pub fn has_version(&self, version: &GameVersion) -> bool {
        self.changes.iter().any(|c| &c.to == version) || &self.current == version
    }

    /// 获取从指定版本到当前版本的变更历史
    pub fn changes_since(&self, from: &GameVersion) -> Vec<&VersionChange> {
        self.changes
            .iter()
            .filter(|c| &c.from >= from || &c.to >= from)
            .collect()
    }

    /// 获取最新稳定版本
    pub fn latest_stable(&self) -> Option<GameVersion> {
        self.get_tag("stable").or_else(|| {
            self.changes
                .iter()
                .rev()
                .find(|c| c.to.is_stable())
                .map(|c| c.to)
        })
    }
}

/// 版本兼容性规则
#[derive(Debug, Clone)]
pub struct CompatibilityRule {
    /// 规则名称
    pub name: String,
    /// 最小兼容版本
    pub min_version: GameVersion,
    /// 最大兼容版本 (包含)
    pub max_version: Option<GameVersion>,
    /// 是否向后兼容
    pub backward_compatible: bool,
}

impl CompatibilityRule {
    /// 创建兼容性规则
    pub fn new(
        name: impl Into<String>,
        min_version: GameVersion,
        max_version: Option<GameVersion>,
        backward_compatible: bool,
    ) -> Self {
        Self {
            name: name.into(),
            min_version,
            max_version,
            backward_compatible,
        }
    }

    /// 检查版本是否兼容
    pub fn is_compatible(&self, version: &GameVersion) -> bool {
        let min_check = version >= &self.min_version;
        let max_check = self.max_version.as_ref().is_none_or(|max| version <= max);
        min_check && max_check
    }
}

/// 迁移函数类型
type MigrationFn = Box<dyn Fn(&str) -> Result<String, String> + Send + Sync>;

/// 游戏规则版本管理器
///
/// 提供完整的版本管理功能，包括版本追踪、兼容性检查、迁移支持等。
pub struct VersionManager {
    /// 游戏名称
    game_name: String,
    /// 版本历史
    history: VersionHistory,
    /// 兼容性规则
    compatibility_rules: Vec<CompatibilityRule>,
    /// 迁移函数
    migrations: HashMap<String, MigrationFn>,
}

impl fmt::Debug for VersionManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VersionManager")
            .field("game_name", &self.game_name)
            .field("history", &self.history)
            .field("compatibility_rules", &self.compatibility_rules)
            .field("migrations_count", &self.migrations.len())
            .finish()
    }
}

impl VersionManager {
    /// 创建版本管理器
    pub fn new(game_name: impl Into<String>) -> Self {
        let name = game_name.into();
        Self {
            history: VersionHistory::new(&name),
            game_name: name,
            compatibility_rules: Vec::new(),
            migrations: HashMap::new(),
        }
    }

    /// 注册版本
    pub fn register_version(&mut self, version: GameVersion, description: impl Into<String>) {
        self.history.add_version(version, description);
    }

    /// 获取当前版本
    pub fn current_version(&self) -> GameVersion {
        self.history.current()
    }

    /// 检查版本是否存在
    pub fn has_version(&self, version: &GameVersion) -> bool {
        self.history.has_version(version)
    }

    /// 添加兼容性规则
    pub fn add_compatibility_rule(&mut self, rule: CompatibilityRule) {
        self.compatibility_rules.push(rule);
    }

    /// 检查版本兼容性
    pub fn check_compatibility(&self, version: &GameVersion) -> bool {
        if self.compatibility_rules.is_empty() {
            // 默认规则: 与当前版本主版本号相同
            version.is_compatible_with(&self.history.current())
        } else {
            self.compatibility_rules
                .iter()
                .all(|r| r.is_compatible(version))
        }
    }

    /// 获取版本历史
    pub fn history(&self) -> &VersionHistory {
        &self.history
    }

    /// 获取可变版本历史
    pub fn history_mut(&mut self) -> &mut VersionHistory {
        &mut self.history
    }

    /// 注册迁移函数
    pub fn register_migration<F>(
        &mut self,
        from_version: GameVersion,
        to_version: GameVersion,
        migration: F,
    ) where
        F: Fn(&str) -> Result<String, String> + Send + Sync + 'static,
    {
        let key = format!("{}_{}", from_version, to_version);
        self.migrations.insert(key, Box::new(migration));
    }

    /// 执行迁移
    pub fn migrate(
        &self,
        from: &GameVersion,
        to: &GameVersion,
        data: &str,
    ) -> Result<String, String> {
        let key = format!("{}_{}", from, to);
        match self.migrations.get(&key) {
            Some(migration) => migration(data),
            None => Err(format!("未找到从 {} 到 {} 的迁移路径", from, to)),
        }
    }

    /// 获取游戏名称
    pub fn game_name(&self) -> &str {
        &self.game_name
    }

    /// 获取所有版本
    pub fn all_versions(&self) -> Vec<GameVersion> {
        self.history.all_versions()
    }

    /// 获取版本数量
    pub fn version_count(&self) -> usize {
        self.history.version_count()
    }
}

/// 版本错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionError {
    /// 无效的版本格式
    InvalidFormat(String),
    /// 版本不存在
    NotFound(String),
    /// 不兼容的版本
    Incompatible(String),
    /// 迁移失败
    MigrationFailed(String),
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionError::InvalidFormat(s) => write!(f, "无效的版本格式: {}", s),
            VersionError::NotFound(s) => write!(f, "版本不存在: {}", s),
            VersionError::Incompatible(s) => write!(f, "不兼容的版本: {}", s),
            VersionError::MigrationFailed(s) => write!(f, "迁移失败: {}", s),
        }
    }
}

impl std::error::Error for VersionError {}

/// 版本范围
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionRange {
    /// 最小版本 (包含)
    pub min: GameVersion,
    /// 最大版本 (包含)
    pub max: Option<GameVersion>,
}

impl VersionRange {
    /// 创建版本范围
    pub fn new(min: GameVersion, max: Option<GameVersion>) -> Self {
        Self { min, max }
    }

    /// 从版本字符串创建范围
    ///
    /// 支持格式:
    /// - "1.0.0" - 精确版本
    /// - ">=1.0.0" - 最小版本
    /// - ">=1.0.0,<2.0.0" - 范围
    pub fn parse(s: &str) -> Result<Self, VersionError> {
        let s = s.trim();

        if let Some(rest) = s.strip_prefix(">=") {
            let rest = rest.trim();
            if let Some(comma_pos) = rest.find(',') {
                let min_str = rest[..comma_pos].trim();
                let max_str = rest[comma_pos + 1..].trim();
                let max_str = max_str.strip_prefix('<').unwrap_or(max_str).trim();

                let min = GameVersion::parse(min_str)?;
                let max = GameVersion::parse(max_str)?;
                Ok(Self::new(min, Some(max)))
            } else {
                let min = GameVersion::parse(rest)?;
                Ok(Self::new(min, None))
            }
        } else {
            // 精确版本
            let version = GameVersion::parse(s)?;
            Ok(Self::new(version, Some(version)))
        }
    }

    /// 检查版本是否在范围内
    pub fn contains(&self, version: &GameVersion) -> bool {
        version >= &self.min && self.max.as_ref().is_none_or(|max| version <= max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_creation() {
        let v = GameVersion::new(1, 2, 3);
        assert_eq!(v.major(), 1);
        assert_eq!(v.minor(), 2);
        assert_eq!(v.patch(), 3);
    }

    #[test]
    fn test_version_parse() {
        let v = GameVersion::parse("2.1.0").unwrap();
        assert_eq!(v.major(), 2);
        assert_eq!(v.minor(), 1);
        assert_eq!(v.patch(), 0);
    }

    #[test]
    fn test_version_parse_invalid() {
        assert!(GameVersion::parse("invalid").is_err());
        assert!(GameVersion::parse("1.2").is_err());
        assert!(GameVersion::parse("1.2.3.4").is_err());
    }

    #[test]
    fn test_version_display() {
        let v = GameVersion::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_ordering() {
        let v1 = GameVersion::new(1, 0, 0);
        let v2 = GameVersion::new(1, 1, 0);
        let v3 = GameVersion::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

    #[test]
    fn test_version_bump() {
        let v = GameVersion::new(1, 2, 3);
        assert_eq!(v.bump_major(), GameVersion::new(2, 0, 0));
        assert_eq!(v.bump_minor(), GameVersion::new(1, 3, 0));
        assert_eq!(v.bump_patch(), GameVersion::new(1, 2, 4));
    }

    #[test]
    fn test_version_compatibility() {
        let v1 = GameVersion::new(1, 0, 0);
        let v2 = GameVersion::new(1, 5, 0);
        let v3 = GameVersion::new(2, 0, 0);

        assert!(v1.is_compatible_with(&v2));
        assert!(!v1.is_compatible_with(&v3));
    }

    #[test]
    fn test_prerelease_compatibility() {
        let v1 = GameVersion::new(0, 1, 0);
        let v2 = GameVersion::new(0, 1, 5);
        let v3 = GameVersion::new(0, 2, 0);

        assert!(v1.is_compatible_with(&v2));
        assert!(!v1.is_compatible_with(&v3));
    }

    #[test]
    fn test_version_range() {
        let range = VersionRange::parse(">=1.0.0,<2.0.0").unwrap();

        assert!(!range.contains(&GameVersion::new(0, 9, 9)));
        assert!(range.contains(&GameVersion::new(1, 0, 0)));
        assert!(range.contains(&GameVersion::new(1, 5, 0)));
        assert!(range.contains(&GameVersion::new(2, 0, 0)));
        assert!(!range.contains(&GameVersion::new(2, 0, 1)));
    }

    #[test]
    fn test_version_history() {
        let mut history = VersionHistory::new("围棋规则");

        let v1 = GameVersion::new(1, 0, 0);
        let v2 = GameVersion::new(1, 1, 0);

        history.add_version(v1, "初始版本");
        history.add_version(v2, "增加贴目规则");

        assert_eq!(history.current(), v2);
        assert_eq!(history.version_count(), 2);
        assert!(history.has_version(&v1));
        assert!(history.has_version(&v2));
    }

    #[test]
    fn test_version_history_tags() {
        let mut history = VersionHistory::new("围棋规则");

        let v1 = GameVersion::new(1, 0, 0);
        let v2 = GameVersion::new(2, 0, 0);

        history.add_version(v1, "稳定版");
        history.add_version(v2, "新版本");

        history.add_tag("stable", v1);
        history.add_tag("latest", v2);

        assert_eq!(history.get_tag("stable"), Some(v1));
        assert_eq!(history.get_tag("latest"), Some(v2));
    }

    #[test]
    fn test_version_manager() {
        let mut manager = VersionManager::new("围棋");

        let v1 = GameVersion::new(1, 0, 0);
        let v2 = GameVersion::new(1, 1, 0);

        manager.register_version(v1, "初始版本");
        manager.register_version(v2, "功能增强");

        assert_eq!(manager.current_version(), v2);
        assert_eq!(manager.version_count(), 2);
    }

    #[test]
    fn test_compatibility_rule() {
        let rule = CompatibilityRule::new(
            "围棋规则兼容",
            GameVersion::new(1, 0, 0),
            Some(GameVersion::new(2, 0, 0)),
            true,
        );

        assert!(rule.is_compatible(&GameVersion::new(1, 5, 0)));
        assert!(!rule.is_compatible(&GameVersion::new(0, 9, 0)));
        assert!(!rule.is_compatible(&GameVersion::new(2, 1, 0)));
    }

    #[test]
    fn test_version_stable() {
        let v1 = GameVersion::new(0, 5, 0);
        let v2 = GameVersion::new(1, 0, 0);

        assert!(v1.is_prerelease());
        assert!(!v1.is_stable());
        assert!(v2.is_stable());
        assert!(!v2.is_prerelease());
    }

    #[test]
    fn test_version_in_range() {
        let v = GameVersion::new(1, 5, 0);
        let min = GameVersion::new(1, 0, 0);
        let max = GameVersion::new(2, 0, 0);

        assert!(v.in_range(&min, &max));
        assert!(!GameVersion::new(0, 9, 0).in_range(&min, &max));
        assert!(!GameVersion::new(2, 1, 0).in_range(&min, &max));
    }

    #[test]
    fn test_version_change() {
        let v1 = GameVersion::new(1, 0, 0);
        let v2 = GameVersion::new(1, 1, 0);

        let change = VersionChange::new(v1, v2, "新增功能");

        assert_eq!(change.change_type, ChangeType::Minor);
        assert!(!change.is_breaking());
    }

    #[test]
    fn test_version_change_breaking() {
        let v1 = GameVersion::new(1, 0, 0);
        let v2 = GameVersion::new(2, 0, 0);

        let change = VersionChange::new(v1, v2, "不兼容更新");

        assert_eq!(change.change_type, ChangeType::Major);
        assert!(change.is_breaking());
    }
}
