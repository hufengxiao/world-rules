//! # 规则模板学习器
//!
//! 从现有规则代码中自动学习和提取模板模式。

use crate::ai::template::RuleTemplate;
use std::collections::HashMap;

/// 规则模式（从现有规则中提取）
#[derive(Debug, Clone)]
pub struct RulePattern {
    /// 规则名称模式
    pub name_pattern: String,

    /// 规则类别
    pub category: String,

    /// 结构特征
    pub struct_features: Vec<String>,

    /// 方法特征
    pub method_features: Vec<String>,

    /// 导入依赖
    pub imports: Vec<String>,

    /// 出现频率
    pub frequency: usize,
}

impl RulePattern {
    /// 创建新的规则模式
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            name_pattern: String::new(),
            category: category.into(),
            struct_features: Vec::new(),
            method_features: Vec::new(),
            imports: Vec::new(),
            frequency: 1,
        }
    }

    /// 转换为规则模板
    pub fn to_template(&self) -> RuleTemplate {
        let mut template =
            RuleTemplate::new(format!("{}_template", self.category)).with_category(&self.category);

        // 添加结构特征作为必需字段
        for feature in &self.struct_features {
            template = template.add_required_field(feature);
        }

        // 构建代码模板
        let code = self.generate_template_code();
        template.code_template = code;

        template
    }

    /// 生成模板代码
    fn generate_template_code(&self) -> String {
        let mut code = String::new();

        // 添加导入
        for import in &self.imports {
            code.push_str(&format!("{}\n", import));
        }

        if !self.imports.is_empty() {
            code.push('\n');
        }

        // 添加结构体定义
        code.push_str("/// {rule_description}\n#[derive(Debug, Clone)]\nstruct {rule_name} {\n");

        // 添加字段
        for feature in &self.struct_features {
            code.push_str(&format!("    {}: String,\n", feature));
        }

        code.push_str("}\n\n");

        // 添加 trait 实现
        code.push_str("impl Rule for {rule_name} {\n");

        for method in &self.method_features {
            if method.contains("metadata") {
                code.push_str("    fn metadata(&self) -> RuleMetadata {\n");
                code.push_str("        RuleMetadata {\n");
                code.push_str("            name: \"{rule_name}\".to_string(),\n");
                code.push_str("            version: \"1.0.0\".to_string(),\n");
                code.push_str("            description: \"{rule_description}\".to_string(),\n");
                code.push_str("        }\n");
                code.push_str("    }\n\n");
            } else if method.contains("category") {
                code.push_str("    fn category(&self) -> RuleCategory {\n");
                code.push_str(&format!("        RuleCategory::{}\n", self.category));
                code.push_str("    }\n\n");
            } else if method.contains("validate") {
                code.push_str("    fn validate(&self, context: &ValidateContext) -> Result<bool, RuleError> {\n");
                code.push_str("        // TODO: 实现验证逻辑\n");
                code.push_str("        Ok(true)\n");
                code.push_str("    }\n\n");
            } else if method.contains("explain") {
                code.push_str("    fn explain(&self) -> String {\n");
                code.push_str("        \"{rule_explanation}\".to_string()\n");
                code.push_str("    }\n");
            }
        }

        code.push_str("}\n");

        code
    }
}

/// 规则模板学习器
///
/// 从现有规则代码中学习模式和结构，自动生成模板。
///
/// # 示例
///
/// ```rust
/// use world_rules::ai::TemplateLearner;
///
/// let mut learner = TemplateLearner::new();
///
/// // 从规则代码学习
/// let code = r#"
/// struct PokerRule;
/// impl Rule for PokerRule {
///     fn category() -> RuleCategory::Games {}
/// }
/// "#;
///
/// let patterns = learner.learn_from_code(code);
/// println!("学习到 {} 个模式", patterns.len());
/// ```
pub struct TemplateLearner {
    /// 学习到的模式
    patterns: HashMap<String, RulePattern>,

    /// 学习统计
    stats: LearningStats,
}

/// 学习统计信息
#[derive(Debug, Clone, Default)]
pub struct LearningStats {
    /// 分析的规则数量
    pub rules_analyzed: usize,

    /// 学习的模式数量
    pub patterns_learned: usize,

    /// 按类别统计
    pub by_category: HashMap<String, usize>,

    /// 常见导入
    pub common_imports: HashMap<String, usize>,
}

impl TemplateLearner {
    /// 创建新的学习器
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            stats: LearningStats::default(),
        }
    }

    /// 从规则代码学习
    ///
    /// # 参数
    ///
    /// - `code`: 规则源代码
    ///
    /// # 返回
    ///
    /// 返回学习到的模式列表
    pub fn learn_from_code(&mut self, code: &str) -> Vec<String> {
        let mut learned = Vec::new();

        // 1. 提取类别
        let category = self.extract_category(code);

        // 2. 提取结构特征
        let struct_features = self.extract_struct_features(code);

        // 3. 提取方法特征
        let method_features = self.extract_method_features(code);

        // 4. 提取导入
        let imports = self.extract_imports(code);

        // 5. 创建或更新模式
        let pattern_key = format!("{}_pattern", category);

        if let Some(existing) = self.patterns.get_mut(&pattern_key) {
            existing.frequency += 1;

            // 合并特征（去重）
            for feature in &struct_features {
                if !existing.struct_features.contains(feature) {
                    existing.struct_features.push(feature.clone());
                }
            }

            for method in &method_features {
                if !existing.method_features.contains(method) {
                    existing.method_features.push(method.clone());
                }
            }

            for import in &imports {
                if !existing.imports.contains(import) {
                    existing.imports.push(import.clone());
                }
            }
        } else {
            let mut pattern = RulePattern::new(&category);
            pattern.struct_features = struct_features;
            pattern.method_features = method_features;
            pattern.imports = imports;

            self.patterns.insert(pattern_key.clone(), pattern);
        }

        // 更新统计
        self.stats.rules_analyzed += 1;
        *self.stats.by_category.entry(category.clone()).or_insert(0) += 1;

        learned.push(format!("学习到 {} 类别规则模式", category));
        learned
    }

    /// 批量学习
    pub fn learn_batch(&mut self, codes: &[&str]) -> Vec<String> {
        let mut results = Vec::new();
        for code in codes {
            results.extend(self.learn_from_code(code));
        }
        self.stats.patterns_learned = self.patterns.len();
        results
    }

    /// 获取所有模式
    pub fn get_patterns(&self) -> Vec<&RulePattern> {
        self.patterns.values().collect()
    }

    /// 获取指定类别的模式
    pub fn get_pattern(&self, category: &str) -> Option<&RulePattern> {
        let key = format!("{}_pattern", category);
        self.patterns.get(&key)
    }

    /// 将模式转换为模板
    pub fn patterns_to_templates(&self) -> Vec<RuleTemplate> {
        self.patterns.values().map(|p| p.to_template()).collect()
    }

    /// 获取学习统计
    pub fn get_stats(&self) -> &LearningStats {
        &self.stats
    }

    /// 重置学习器
    pub fn reset(&mut self) {
        self.patterns.clear();
        self.stats = LearningStats::default();
    }

    // ===== 辅助方法 =====

    /// 提取规则类别
    fn extract_category(&self, code: &str) -> String {
        if code.contains("RuleCategory::Law") {
            "Law"
        } else if code.contains("RuleCategory::Sports") {
            "Sports"
        } else if code.contains("RuleCategory::Games") {
            "Games"
        } else if code.contains("RuleCategory::Science") {
            "Science"
        } else if code.contains("RuleCategory::Social") {
            "Social"
        } else if code.contains("RuleCategory::Health") {
            "Health"
        } else {
            "Unknown"
        }
        .to_string()
    }

    /// 提取结构特征
    fn extract_struct_features(&self, code: &str) -> Vec<String> {
        let mut features = Vec::new();

        // 查找 struct 定义
        for line in code.lines() {
            let line = line.trim();
            if line.starts_with("struct ") {
                // 提取结构体名称
                if let Some(name) = line.strip_prefix("struct ") {
                    let name = name.split_whitespace().next().unwrap_or("");
                    if !name.is_empty() {
                        features.push(format!("struct_{}", name));
                    }
                }
            }

            // 查找字段
            if line.contains(": String") {
                let field = line.split(':').next().unwrap_or("").trim();
                if !field.is_empty() && !field.starts_with("//") {
                    features.push(field.to_string());
                }
            }
        }

        features
    }

    /// 提取方法特征
    fn extract_method_features(&self, code: &str) -> Vec<String> {
        let mut methods = Vec::new();

        // 处理可能包含多个方法的单行代码
        // 例如: "impl Rule { fn test() {} fn validate() {} }"

        // 首先检查是否在impl块内
        if let Some(impl_start) = code.find("impl ") {
            // 找到impl块的范围
            if let Some(block_start) = code[impl_start..].find('{') {
                let block_content_start = impl_start + block_start + 1;

                // 找到impl块的结束
                if let Some(block_end) = code[block_content_start..].find('}') {
                    let block_content = &code[block_content_start..block_content_start + block_end];

                    // 在impl块内容中查找所有fn定义
                    for part in block_content.split("fn ") {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }

                        // 提取方法名（到第一个'('为止）
                        let method = part.split('(').next().unwrap_or("").trim();
                        if !method.is_empty() && method != "{" {
                            methods.push(format!("fn_{}", method));
                        }
                    }
                }
            }
        }

        methods
    }

    /// 提取导入语句
    fn extract_imports(&self, code: &str) -> Vec<String> {
        let mut imports = Vec::new();

        for line in code.lines() {
            let line = line.trim();

            if line.starts_with("use ") {
                imports.push(line.to_string());
            }
        }

        imports
    }
}

impl Default for TemplateLearner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learner_creation() {
        let learner = TemplateLearner::new();
        assert_eq!(learner.stats.rules_analyzed, 0);
    }

    #[test]
    fn test_learn_from_code() {
        let mut learner = TemplateLearner::new();

        let code = r#"
use world_rules::rules::{Rule, RuleCategory};

struct PokerRule {
    name: String,
}

impl Rule for PokerRule {
    fn category(&self) -> RuleCategory {
        RuleCategory::Games
    }
}
"#;

        let learned = learner.learn_from_code(code);
        assert!(!learned.is_empty());
        assert_eq!(learner.stats.rules_analyzed, 1);
    }

    #[test]
    fn test_extract_category() {
        let learner = TemplateLearner::new();

        assert_eq!(learner.extract_category("RuleCategory::Law"), "Law");
        assert_eq!(learner.extract_category("RuleCategory::Games"), "Games");
    }

    #[test]
    fn test_extract_struct_features() {
        let learner = TemplateLearner::new();

        let code = "struct Test { field: String }";
        let features = learner.extract_struct_features(code);

        assert!(!features.is_empty());
    }

    #[test]
    fn test_extract_method_features() {
        let learner = TemplateLearner::new();

        let code = "impl Rule { fn test() {} fn validate() {} }";
        let methods = learner.extract_method_features(code);

        assert!(!methods.is_empty());
    }

    #[test]
    fn test_pattern_to_template() {
        let mut pattern = RulePattern::new("Games");
        pattern.method_features = vec!["fn_metadata".to_string()];

        let template = pattern.to_template();
        assert_eq!(template.category, "Games");
    }

    #[test]
    fn test_learn_batch() {
        let mut learner = TemplateLearner::new();

        let codes = vec![
            "impl Rule for A { fn category() -> RuleCategory::Games }",
            "impl Rule for B { fn category() -> RuleCategory::Law }",
        ];

        let results = learner.learn_batch(&codes);
        assert_eq!(results.len(), 2);
        assert_eq!(learner.stats.rules_analyzed, 2);
    }

    #[test]
    fn test_get_patterns() {
        let mut learner = TemplateLearner::new();

        let code = "impl Rule { fn category() -> RuleCategory::Games }";
        learner.learn_from_code(code);

        let patterns = learner.get_patterns();
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_patterns_to_templates() {
        let mut learner = TemplateLearner::new();

        let code = "impl Rule { fn category() -> RuleCategory::Games }";
        learner.learn_from_code(code);

        let templates = learner.patterns_to_templates();
        assert!(!templates.is_empty());
    }

    #[test]
    fn test_learner_reset() {
        let mut learner = TemplateLearner::new();

        learner.learn_from_code("impl Rule { fn category() -> RuleCategory::Games }");
        assert!(learner.stats.rules_analyzed > 0);

        learner.reset();
        assert_eq!(learner.stats.rules_analyzed, 0);
    }
}
