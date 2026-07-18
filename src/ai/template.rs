//! # 规则模板系统
//!
//! 提供规则模板的存储、检索和学习功能。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 规则模板
///
/// 定义了规则的基本结构和元数据模板。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::RuleTemplate;
///
/// let template = RuleTemplate::new("law_rule")
///     .with_description("法律规则模板")
///     .with_category("law");
///
/// println!("模板名称: {}", template.name);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleTemplate {
    /// 模板名称
    pub name: String,

    /// 模板描述
    pub description: String,

    /// 目标规则类别
    pub category: String,

    /// 模板代码
    pub code_template: String,

    /// 必需字段
    pub required_fields: Vec<String>,

    /// 可选字段
    pub optional_fields: Vec<String>,

    /// 元数据
    pub metadata: HashMap<String, String>,

    /// 使用次数（用于学习）
    pub usage_count: usize,
}

impl RuleTemplate {
    /// 创建新的规则模板
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            category: String::new(),
            code_template: String::new(),
            required_fields: Vec::new(),
            optional_fields: Vec::new(),
            metadata: HashMap::new(),
            usage_count: 0,
        }
    }

    /// 设置描述
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// 设置类别
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// 设置代码模板
    pub fn with_code_template(mut self, template: impl Into<String>) -> Self {
        self.code_template = template.into();
        self
    }

    /// 添加必需字段
    pub fn add_required_field(mut self, field: impl Into<String>) -> Self {
        self.required_fields.push(field.into());
        self
    }

    /// 添加可选字段
    pub fn add_optional_field(mut self, field: impl Into<String>) -> Self {
        self.optional_fields.push(field.into());
        self
    }

    /// 增加使用次数
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }

    /// 应用模板生成代码
    ///
    /// # 参数
    ///
    /// - `params`: 参数映射
    ///
    /// # 返回
    ///
    /// 返回生成的代码或错误
    pub fn apply(&self, params: &HashMap<String, String>) -> Result<String, TemplateError> {
        // 检查必需字段
        for field in &self.required_fields {
            if !params.contains_key(field) {
                return Err(TemplateError::MissingField(field.clone()));
            }
        }

        // 简单实现：替换占位符
        let mut code = self.code_template.clone();
        for (key, value) in params {
            code = code.replace(&format!("{{{{{}}}}}", key), value);
        }

        Ok(code)
    }
}

/// 模板错误
#[derive(Debug, Clone, thiserror::Error)]
pub enum TemplateError {
    /// 缺少必需字段
    #[error("缺少必需字段: {0}")]
    MissingField(String),

    /// 模板未找到
    #[error("模板未找到: {0}")]
    NotFound(String),

    /// 应用失败
    #[error("模板应用失败: {0}")]
    ApplyError(String),
}

/// 模板库
///
/// 存储和管理规则模板。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::TemplateLibrary;
///
/// let mut library = TemplateLibrary::new();
///
/// // 添加模板
/// library.add_default_templates();
///
/// // 搜索模板
/// let templates = library.search("law");
/// println!("找到 {} 个法律模板", templates.len());
/// ```
pub struct TemplateLibrary {
    /// 模板存储
    templates: HashMap<String, RuleTemplate>,
}

impl TemplateLibrary {
    /// 创建新的模板库
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// 添加模板
    pub fn add(&mut self, template: RuleTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// 获取模板
    pub fn get(&self, name: &str) -> Option<&RuleTemplate> {
        self.templates.get(name)
    }

    /// 搜索模板
    pub fn search(&self, query: &str) -> Vec<&RuleTemplate> {
        self.templates
            .values()
            .filter(|t| {
                t.name.contains(query)
                    || t.description.contains(query)
                    || t.category.contains(query)
            })
            .collect()
    }

    /// 按类别获取模板
    pub fn by_category(&self, category: &str) -> Vec<&RuleTemplate> {
        self.templates
            .values()
            .filter(|t| t.category == category)
            .collect()
    }

    /// 获取所有模板
    pub fn all(&self) -> Vec<&RuleTemplate> {
        self.templates.values().collect()
    }

    /// 获取模板数量
    pub fn count(&self) -> usize {
        self.templates.len()
    }

    /// 添加默认模板
    pub fn add_default_templates(&mut self) {
        // 法律规则模板
        let law_template = RuleTemplate::new("basic_law_rule")
            .with_description("基础法律规则模板")
            .with_category("law")
            .with_code_template(
                r#"
/// {{rule_name}}
/// 
/// {{rule_description}}
#[derive(Debug, Clone)]
pub struct {{rule_name}} {
    // 规则数据
}

impl Rule for {{rule_name}} {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            name: "{{rule_name}}".to_string(),
            version: "1.0.0".to_string(),
            description: "{{rule_description}}".to_string(),
        }
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Law
    }

    fn validate(&self, context: &ValidateContext) -> Result<bool, RuleError> {
        // TODO: 实现验证逻辑
        Ok(true)
    }

    fn explain(&self) -> String {
        "{{rule_explanation}}".to_string()
    }
}
"#
                .to_string(),
            )
            .add_required_field("rule_name")
            .add_required_field("rule_description")
            .add_required_field("rule_explanation");

        self.add(law_template);

        // 体育规则模板
        let sports_template = RuleTemplate::new("basic_sports_rule")
            .with_description("基础体育规则模板")
            .with_category("sports")
            .with_code_template(
                r#"
/// {{rule_name}}
/// 
/// {{rule_description}}
#[derive(Debug, Clone)]
pub struct {{rule_name}} {
    // 规则数据
}

impl Rule for {{rule_name}} {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            name: "{{rule_name}}".to_string(),
            version: "1.0.0".to_string(),
            description: "{{rule_description}}".to_string(),
        }
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Sports
    }

    fn validate(&self, context: &ValidateContext) -> Result<bool, RuleError> {
        // TODO: 实现验证逻辑
        Ok(true)
    }

    fn explain(&self) -> String {
        "{{rule_explanation}}".to_string()
    }
}
"#
                .to_string(),
            )
            .add_required_field("rule_name")
            .add_required_field("rule_description")
            .add_required_field("rule_explanation");

        self.add(sports_template);

        // 游戏规则模板
        let game_template = RuleTemplate::new("basic_game_rule")
            .with_description("基础游戏规则模板")
            .with_category("games")
            .with_code_template(
                r#"
/// {{rule_name}}
/// 
/// {{rule_description}}
#[derive(Debug, Clone)]
pub struct {{rule_name}} {
    // 规则数据
}

impl Rule for {{rule_name}} {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            name: "{{rule_name}}".to_string(),
            version: "1.0.0".to_string(),
            description: "{{rule_description}}".to_string(),
        }
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Games
    }

    fn validate(&self, context: &ValidateContext) -> Result<bool, RuleError> {
        // TODO: 实现验证逻辑
        Ok(true)
    }

    fn explain(&self) -> String {
        "{{rule_explanation}}".to_string()
    }
}
"#
                .to_string(),
            )
            .add_required_field("rule_name")
            .add_required_field("rule_description")
            .add_required_field("rule_explanation");

        self.add(game_template);
    }

    /// 从现有规则学习模板
    ///
    /// 分析现有规则的结构和模式，提取新的模板。
    pub fn learn_from_rules(&mut self, rules: &[&str]) -> Vec<String> {
        let mut learned = Vec::new();

        for rule_code in rules {
            // 简单实现：提取结构特征
            if rule_code.contains("impl Rule") {
                let category = if rule_code.contains("RuleCategory::Law") {
                    "law"
                } else if rule_code.contains("RuleCategory::Sports") {
                    "sports"
                } else if rule_code.contains("RuleCategory::Games") {
                    "games"
                } else {
                    "unknown"
                };

                learned.push(format!("学习到 {} 类别规则", category));
            }
        }

        learned
    }
}

impl Default for TemplateLibrary {
    fn default() -> Self {
        let mut library = Self::new();
        library.add_default_templates();
        library
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_template_creation() {
        let template = RuleTemplate::new("test_template")
            .with_description("测试模板")
            .with_category("test");

        assert_eq!(template.name, "test_template");
        assert_eq!(template.description, "测试模板");
        assert_eq!(template.category, "test");
    }

    #[test]
    fn test_template_apply() {
        let template = RuleTemplate::new("test")
            .with_code_template("Hello, {{name}}!")
            .add_required_field("name");

        let mut params = HashMap::new();
        params.insert("name".to_string(), "World".to_string());

        let result = template.apply(&params).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_template_missing_field() {
        let template = RuleTemplate::new("test")
            .with_code_template("Hello, {name}!")
            .add_required_field("name");

        let params = HashMap::new();
        let result = template.apply(&params);

        assert!(result.is_err());
    }

    #[test]
    fn test_template_library() {
        let mut library = TemplateLibrary::new();
        library.add_default_templates();

        assert!(library.count() >= 3);
    }

    #[test]
    fn test_library_search() {
        let library = TemplateLibrary::default();

        let results = library.search("law");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_library_by_category() {
        let library = TemplateLibrary::default();

        let results = library.by_category("law");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_template_usage_count() {
        let mut template = RuleTemplate::new("test");
        assert_eq!(template.usage_count, 0);

        template.increment_usage();
        assert_eq!(template.usage_count, 1);
    }

    #[test]
    fn test_learn_from_rules() {
        let mut library = TemplateLibrary::new();

        let rules = vec![
            "impl Rule for Test { fn category() -> RuleCategory::Law }",
            "impl Rule for Test2 { fn category() -> RuleCategory::Sports }",
        ];

        let learned = library.learn_from_rules(&rules);
        assert!(!learned.is_empty());
    }
}
