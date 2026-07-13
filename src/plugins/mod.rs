//! 插件系统 - 外部规则包动态加载
//!
//! 允许第三方以 JSON 文件形式提供自定义规则。
//!
//! # 插件格式
//!
//! JSON 文件结构:
//! ```json
//! {
//!   "name": "自定义规则包",
//!   "version": "1.0.0",
//!   "rules": [
//!     {
//!       "name": "我的游戏规则",
//!       "description": "自定义游戏规则",
//!       "category": "games",
//!       "origin": "自定义",
//!       "tags": ["游戏", "自定义"],
//!       "sections": [
//!         { "title": "基本规则", "items": ["规则1", "规则2"] }
//!       ]
//!     }
//!   ]
//! }
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};
#[cfg(feature = "serde_json")]
use std::path::Path;

/// 插件清单 - 描述一个插件包的元数据和规则集合
///
/// # Examples
///
/// ```rust
/// use world_rules::plugins::PluginManifest;
///
/// let json = r#"{
///     "name": "我的插件",
///     "version": "1.0.0",
///     "rules": []
/// }"#;
/// let manifest: PluginManifest = serde_json::from_str(json).unwrap();
/// assert_eq!(manifest.name, "我的插件");
/// assert_eq!(manifest.version, "1.0.0");
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginManifest {
    /// 插件名称
    pub name: String,
    /// 插件版本（遵循 semver）
    pub version: String,
    /// 插件包含的规则定义列表
    pub rules: Vec<PluginRuleDef>,
}

/// 插件规则定义 - JSON 格式的规则配置
///
/// # Examples
///
/// ```rust
/// use world_rules::plugins::PluginRuleDef;
///
/// let json = r#"{
///     "name": "麻将规则",
///     "description": "四川麻将规则",
///     "category": "games",
///     "origin": "四川",
///     "tags": ["麻将", "四川"],
///     "sections": [
///         { "title": "基本规则", "items": ["规则1", "规则2"] }
///     ]
/// }"#;
/// let def: PluginRuleDef = serde_json::from_str(json).unwrap();
/// assert_eq!(def.name, "麻将规则");
/// assert_eq!(def.category, "games");
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginRuleDef {
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 规则分类（games, sports, social, science, law, health 或自定义）
    pub category: String,
    /// 规则来源（可选）
    pub origin: Option<String>,
    /// 标签列表（可选）
    pub tags: Option<Vec<String>>,
    /// 规则分组章节
    pub sections: Vec<PluginSection>,
}

/// 插件规则分组 - 规则的一个章节
///
/// # Examples
///
/// ```rust
/// use world_rules::plugins::PluginSection;
///
/// let json = r#"{
///     "title": "基本规则",
///     "items": ["第一条", "第二条"]
/// }"#;
/// let section: PluginSection = serde_json::from_str(json).unwrap();
/// assert_eq!(section.title, "基本规则");
/// assert_eq!(section.items.len(), 2);
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginSection {
    /// 章节标题
    pub title: String,
    /// 章节条目列表
    pub items: Vec<String>,
}

#[cfg(feature = "serde_json")]
/// 从 JSON 文件加载插件规则
///
/// # Arguments
///
/// * `path` - JSON 插件文件的路径
///
/// # Returns
///
/// 成功返回 `Ok(Vec<PluginRule>)`，失败返回 `Err(String)`
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use world_rules::plugins::load_plugin_from_file;
///
/// let path = Path::new("my_plugin.json");
/// let rules = load_plugin_from_file(path).unwrap();
/// println!("加载了 {} 条规则", rules.len());
/// ```
///
/// # Errors
///
/// 当文件不存在、无法读取或 JSON 格式无效时返回错误
pub fn load_plugin_from_file(path: &Path) -> Result<Vec<PluginRule>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("读取插件文件失败: {}", e))?;
    load_plugin_from_str(&content)
}

#[cfg(feature = "serde_json")]
/// 从 JSON 字符串加载插件规则
///
/// # Arguments
///
/// * `json` - JSON 格式的插件定义字符串
///
/// # Returns
///
/// 成功返回 `Ok(Vec<PluginRule>)`，失败返回 `Err(String)`
///
/// # Examples
///
/// ```rust
/// use world_rules::plugins::load_plugin_from_str;
///
/// let json = r#"{
///     "name": "测试插件",
///     "version": "1.0.0",
///     "rules": [
///         {
///             "name": "规则1",
///             "description": "测试规则",
///             "category": "games",
///             "sections": []
///         }
///     ]
/// }"#;
/// let rules = load_plugin_from_str(json).unwrap();
/// assert_eq!(rules.len(), 1);
/// ```
///
/// # Errors
///
/// 当 JSON 格式无效或缺少必需字段时返回错误
pub fn load_plugin_from_str(json: &str) -> Result<Vec<PluginRule>, String> {
    let manifest: PluginManifest =
        serde_json::from_str(json).map_err(|e| format!("解析插件 JSON 失败: {}", e))?;

    let mut rules = Vec::new();
    for def in manifest.rules {
        rules.push(PluginRule::from_def(def));
    }
    Ok(rules)
}

#[cfg(feature = "serde_json")]
/// 从目录加载所有插件（遍历所有 .json 文件）
///
/// # Arguments
///
/// * `dir` - 包含插件 JSON 文件的目录路径
///
/// # Returns
///
/// 返回所有成功加载的插件规则列表（失败的文件会被忽略并打印警告）
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use world_rules::plugins::load_plugins_from_dir;
/// use world_rules::rules::core::Rule;
///
/// let dir = Path::new("./plugins");
/// let rules = load_plugins_from_dir(dir);
/// for rule in &rules {
///     println!("规则: {}", rule.metadata().name);
/// }
/// ```
///
/// # Note
///
/// - 只加载 `.json` 扩展名的文件
/// - 加载失败的文件会打印错误消息到 stderr，但不会中断整个加载过程
/// - 返回的规则列表可能为空（如果目录不存在或没有有效的 JSON 文件）
pub fn load_plugins_from_dir(dir: &Path) -> Vec<PluginRule> {
    let mut all_rules = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                match load_plugin_from_file(&path) {
                    Ok(rules) => all_rules.extend(rules),
                    Err(e) => eprintln!("加载插件 {:?} 失败: {}", path, e),
                }
            }
        }
    }
    all_rules
}

/// 动态加载的插件规则
///
/// 通过 JSON 文件定义并动态加载的规则实现，实现 [`Rule`] trait。
///
/// # Examples
///
/// ```rust,ignore
/// use world_rules::plugins::load_plugin_from_str;
/// use world_rules::rules::core::Rule;
///
/// let json = r#"{
///     "name": "测试插件",
///     "version": "1.0.0",
///     "rules": [
///         {
///             "name": "自定义规则",
///             "description": "这是一个自定义规则",
///             "category": "games",
///             "sections": [
///                 { "title": "基本规则", "items": ["条目1", "条目2"] }
///             ]
///         }
///     ]
/// }"#;
/// let rules = load_plugin_from_str(json).unwrap();
/// let rule = &rules[0];
/// assert_eq!(rule.metadata().name, "自定义规则");
/// println!("{}", rule.explain()); // 输出格式化规则说明
/// ```
pub struct PluginRule {
    metadata: RuleMetadata,
    category: RuleCategory,
    sections: Vec<(String, Vec<String>)>,
}

impl PluginRule {
    /// 从 JSON 定义创建插件规则实例
    ///
    /// # Arguments
    ///
    /// * `def` - 插件规则定义（从 JSON 解析而来）
    ///
    /// # Note
    ///
    /// 此方法是内部实现细节，用户应使用 [`load_plugin_from_str`] 或 [`load_plugin_from_file`]
    #[cfg(feature = "serde_json")]
    fn from_def(def: PluginRuleDef) -> Self {
        let tags = def.tags.unwrap_or_default();
        let origin = def.origin.unwrap_or_else(|| "插件".to_string());
        let category = match def.category.as_str() {
            "games" => RuleCategory::games("plugin"),
            "sports" => RuleCategory::sports("plugin"),
            "social" => RuleCategory::social("plugin"),
            "science" => RuleCategory::science("plugin"),
            "law" => RuleCategory::law("plugin"),
            "health" => RuleCategory::health("plugin"),
            _ => RuleCategory::custom("plugin", &def.category),
        };
        let sections: Vec<(String, Vec<String>)> = def
            .sections
            .into_iter()
            .map(|s| (s.title, s.items))
            .collect();

        Self {
            metadata: RuleMetadata::new(&def.name, &def.description)
                .with_origin(&origin)
                .with_tags(tags),
            category,
            sections,
        }
    }
}

impl Rule for PluginRule {
    /// 返回规则的元数据（名称、描述、来源、标签等）
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    /// 返回规则的分类
    fn category(&self) -> RuleCategory {
        self.category.clone()
    }

    /// 验证规则（插件规则默认返回 Ok(true)）
    fn validate(&self, _ctx: &crate::rules::core::ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    /// 生成规则的可读说明
    fn explain(&self) -> String {
        let mut result = format!("【{}】", self.metadata.name);
        for (title, items) in &self.sections {
            result.push_str(&format!("\n\n{}:", title));
            for item in items {
                result.push_str(&format!("\n  • {}", item));
            }
        }
        result
    }
}

#[cfg(all(test, feature = "serde_json"))]
mod tests {
    use super::*;

    #[test]
    fn test_load_plugin_from_str() {
        let json = r#"{
            "name": "测试插件",
            "version": "1.0.0",
            "rules": [
                {
                    "name": "测试规则",
                    "description": "一个测试规则",
                    "category": "games",
                    "origin": "测试",
                    "tags": ["测试"],
                    "sections": [
                        { "title": "规则", "items": ["条目1", "条目2"] }
                    ]
                }
            ]
        }"#;
        let rules = load_plugin_from_str(json).unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].metadata().name, "测试规则");
        assert!(!rules[0].explain().is_empty());
    }

    #[test]
    fn test_plugin_category() {
        let json = r#"{
            "name": "测试",
            "version": "1.0.0",
            "rules": [{
                "name": "规则",
                "description": "描述",
                "category": "sports",
                "sections": []
            }]
        }"#;
        let rules = load_plugin_from_str(json).unwrap();
        assert!(matches!(rules[0].category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_invalid_json() {
        let result = load_plugin_from_str("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_rules() {
        let json = r#"{"name": "空", "version": "1.0", "rules": []}"#;
        let rules = load_plugin_from_str(json).unwrap();
        assert!(rules.is_empty());
    }
}
