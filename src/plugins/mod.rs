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

#[cfg(feature = "serde_json")]
use crate::rules::core::format_rule_sections;
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};
#[cfg(feature = "serde_json")]
use std::path::Path;

/// 插件元数据
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub rules: Vec<PluginRuleDef>,
}

/// 插件规则定义
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginRuleDef {
    pub name: String,
    pub description: String,
    pub category: String,
    pub origin: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sections: Vec<PluginSection>,
}

/// 插件规则分组
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginSection {
    pub title: String,
    pub items: Vec<String>,
}

#[cfg(feature = "serde_json")]
/// 从 JSON 文件加载插件规则
pub fn load_plugin_from_file(path: &Path) -> Result<Vec<PluginRule>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("读取插件文件失败: {}", e))?;
    load_plugin_from_str(&content)
}

#[cfg(feature = "serde_json")]
/// 从 JSON 字符串加载插件规则
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
/// 从目录加载所有插件
pub fn load_plugins_from_dir(dir: &Path) -> Vec<PluginRule> {
    let mut all_rules = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
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
pub struct PluginRule {
    metadata: RuleMetadata,
    category: RuleCategory,
    sections: Vec<(String, Vec<String>)>,
}

impl PluginRule {
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
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        self.category.clone()
    }

    fn validate(&self, _ctx: &crate::rules::core::ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

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
