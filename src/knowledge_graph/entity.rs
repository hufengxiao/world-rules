//! 规则实体抽取模块
//!
//! 从规则定义和文本中抽取关键实体，包括概念、动作、条件、结果等。

use std::collections::HashMap;

/// 实体类型
///
/// 定义知识图谱中的实体类别。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::EntityType;
///
/// assert_eq!(EntityType::Concept.to_string(), "概念");
/// assert!(EntityType::Action.is_action());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EntityType {
    /// 概念实体（规则中涉及的名词概念）
    Concept,
    /// 动作实体（规则中涉及的行为动作）
    Action,
    /// 条件实体（规则中的前提条件）
    Condition,
    /// 结果实体（规则中产生的后果）
    Result,
    /// 主体实体（规则适用的对象）
    Subject,
    /// 客体实体（规则作用的目标）
    Object,
    /// 时间实体（规则中的时间要素）
    Time,
    /// 地点实体（规则中的地点要素）
    Location,
    /// 数量实体（规则中的数量限制）
    Quantity,
    /// 属性实体（规则中的属性特征）
    Attribute,
}

impl EntityType {
    /// 检查是否为动作类型
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::EntityType;
    ///
    /// assert!(EntityType::Action.is_action());
    /// assert!(!EntityType::Concept.is_action());
    /// ```
    pub fn is_action(&self) -> bool {
        matches!(self, Self::Action)
    }

    /// 检查是否为条件或结果类型
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::EntityType;
    ///
    /// assert!(EntityType::Condition.is_conditional());
    /// assert!(EntityType::Result.is_conditional());
    /// assert!(!EntityType::Concept.is_conditional());
    /// ```
    pub fn is_conditional(&self) -> bool {
        matches!(self, Self::Condition | Self::Result)
    }

    /// 获取类型显示名称
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::EntityType;
    ///
    /// assert_eq!(EntityType::Concept.display_name(), "概念");
    /// ```
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Concept => "概念",
            Self::Action => "动作",
            Self::Condition => "条件",
            Self::Result => "结果",
            Self::Subject => "主体",
            Self::Object => "客体",
            Self::Time => "时间",
            Self::Location => "地点",
            Self::Quantity => "数量",
            Self::Attribute => "属性",
        }
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// 知识图谱实体
///
/// 表示从规则中抽取的一个实体节点。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::{Entity, EntityType};
///
/// let entity = Entity::new("球员", EntityType::Subject)
///     .with_confidence(0.95)
///     .with_source("足球规则第1条");
///
/// assert_eq!(entity.name, "球员");
/// assert_eq!(entity.entity_type, EntityType::Subject);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    /// 实体唯一标识
    pub id: String,
    /// 实体名称
    pub name: String,
    /// 实体类型
    pub entity_type: EntityType,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f64,
    /// 实体来源（规则名称或文本）
    pub source: Option<String>,
    /// 实体属性
    pub attributes: HashMap<String, String>,
    /// 实体同义词
    pub synonyms: Vec<String>,
}

impl Entity {
    /// 创建新实体
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{Entity, EntityType};
    ///
    /// let entity = Entity::new("犯规", EntityType::Action);
    /// assert_eq!(entity.name, "犯规");
    /// ```
    pub fn new(name: impl Into<String>, entity_type: EntityType) -> Self {
        let name = name.into();
        let id = format!("{}_{}", entity_type.display_name(), name);
        Self {
            id,
            name,
            entity_type,
            confidence: 1.0,
            source: None,
            attributes: HashMap::new(),
            synonyms: Vec::new(),
        }
    }

    /// 设置置信度
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{Entity, EntityType};
    ///
    /// let entity = Entity::new("犯规", EntityType::Action)
    ///     .with_confidence(0.85);
    /// assert_eq!(entity.confidence, 0.85);
    /// ```
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// 设置来源
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{Entity, EntityType};
    ///
    /// let entity = Entity::new("犯规", EntityType::Action)
    ///     .with_source("足球规则");
    /// assert_eq!(entity.source, Some("足球规则".to_string()));
    /// ```
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// 添加属性
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{Entity, EntityType};
    ///
    /// let entity = Entity::new("球员", EntityType::Subject)
    ///     .with_attribute("数量", "11人");
    /// assert_eq!(entity.attributes.get("数量"), Some(&"11人".to_string()));
    /// ```
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// 添加同义词
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{Entity, EntityType};
    ///
    /// let entity = Entity::new("球员", EntityType::Subject)
    ///     .with_synonyms(vec!["运动员", "选手"]);
    /// assert_eq!(entity.synonyms.len(), 2);
    /// ```
    pub fn with_synonyms(mut self, synonyms: Vec<&str>) -> Self {
        self.synonyms = synonyms.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// 检查是否匹配给定名称（包括同义词）
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{Entity, EntityType};
    ///
    /// let entity = Entity::new("球员", EntityType::Subject)
    ///     .with_synonyms(vec!["运动员"]);
    /// assert!(entity.matches("球员"));
    /// assert!(entity.matches("运动员"));
    /// assert!(!entity.matches("裁判"));
    /// ```
    pub fn matches(&self, name: &str) -> bool {
        self.name == name || self.synonyms.iter().any(|s| s == name)
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} (置信度: {:.2})",
            self.entity_type, self.name, self.confidence
        )
    }
}

/// 实体抽取配置
#[derive(Debug, Clone)]
pub struct ExtractorConfig {
    /// 是否启用同义词识别
    pub enable_synonyms: bool,
    /// 最小置信度阈值
    pub min_confidence: f64,
    /// 是否提取数量实体
    pub extract_quantities: bool,
    /// 是否提取时间实体
    pub extract_time: bool,
    /// 是否提取地点实体
    pub extract_location: bool,
}

impl Default for ExtractorConfig {
    fn default() -> Self {
        Self {
            enable_synonyms: true,
            min_confidence: 0.5,
            extract_quantities: true,
            extract_time: true,
            extract_location: true,
        }
    }
}

/// 实体抽取器
///
/// 从文本中抽取规则实体。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::EntityExtractor;
///
/// let extractor = EntityExtractor::new();
/// let entities = extractor.extract("球员在比赛中犯规会被出示黄牌");
/// assert!(!entities.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct EntityExtractor {
    /// 配置
    config: ExtractorConfig,
    /// 动作词库
    action_words: Vec<String>,
    /// 条件词库
    condition_words: Vec<String>,
    /// 结果词库
    result_words: Vec<String>,
    /// 时间词库
    time_words: Vec<String>,
    /// 地点词库
    location_words: Vec<String>,
}

impl Default for EntityExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityExtractor {
    /// 创建新实体抽取器
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::EntityExtractor;
    ///
    /// let extractor = EntityExtractor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: ExtractorConfig::default(),
            action_words: vec![
                "犯规".into(),
                "得分".into(),
                "射门".into(),
                "传球".into(),
                "违规".into(),
                "处罚".into(),
                "判罚".into(),
                "执行".into(),
                "申请".into(),
                "提交".into(),
                "审核".into(),
                "批准".into(),
                "签订".into(),
                "终止".into(),
                "解除".into(),
                "履行".into(),
                "使用".into(),
            ],
            condition_words: vec![
                "如果".into(),
                "当".into(),
                "若".into(),
                "在...情况下".into(),
                "符合".into(),
                "满足".into(),
                "具备".into(),
                "达到".into(),
                "超过".into(),
                "低于".into(),
                "不满".into(),
                "超过".into(),
            ],
            result_words: vec![
                "则".into(),
                "那么".into(),
                "将会".into(),
                "必须".into(),
                "应当".into(),
                "需要".into(),
                "导致".into(),
                "产生".into(),
                "予以".into(),
                "给予".into(),
                "处以".into(),
                "判定".into(),
            ],
            time_words: vec![
                "比赛期间".into(),
                "赛程".into(),
                "赛季".into(),
                "全年".into(),
                "每年".into(),
                "每月".into(),
                "每日".into(),
                "工作日".into(),
                "节假日".into(),
                "期限内".into(),
                "有效期".into(),
                "时效".into(),
            ],
            location_words: vec![
                "场地".into(),
                "赛区".into(),
                "主场".into(),
                "客场".into(),
                "法院".into(),
                "仲裁庭".into(),
                "办公地点".into(),
            ],
        }
    }

    /// 使用自定义配置创建抽取器
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{EntityExtractor, ExtractorConfig};
    ///
    /// let config = ExtractorConfig {
    ///     min_confidence: 0.7,
    ///     ..Default::default()
    /// };
    /// let extractor = EntityExtractor::with_config(config);
    /// ```
    pub fn with_config(config: ExtractorConfig) -> Self {
        let mut extractor = Self::new();
        extractor.config = config;
        extractor
    }

    /// 从文本中抽取实体
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::EntityExtractor;
    ///
    /// let extractor = EntityExtractor::new();
    /// let entities = extractor.extract("球员在比赛中犯规会被出示黄牌");
    /// assert!(!entities.is_empty());
    /// ```
    pub fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();

        // 抽取动作实体
        entities.extend(self.extract_actions(text));

        // 抽取条件实体
        entities.extend(self.extract_conditions(text));

        // 抽取结果实体
        entities.extend(self.extract_results(text));

        // 抽取时间实体
        if self.config.extract_time {
            entities.extend(self.extract_time_entities(text));
        }

        // 抽取地点实体
        if self.config.extract_location {
            entities.extend(self.extract_locations(text));
        }

        // 抽取数量实体
        if self.config.extract_quantities {
            entities.extend(self.extract_quantities(text));
        }

        // 抽取主体和客体
        entities.extend(self.extract_subjects_objects(text));

        // 过滤低置信度实体
        entities.retain(|e| e.confidence >= self.config.min_confidence);

        entities
    }

    /// 抽取动作实体
    fn extract_actions(&self, text: &str) -> Vec<Entity> {
        self.action_words
            .iter()
            .filter_map(|word| {
                if text.contains(word) {
                    Some(
                        Entity::new(word, EntityType::Action)
                            .with_confidence(0.9)
                            .with_source(text),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    /// 抽取条件实体
    fn extract_conditions(&self, text: &str) -> Vec<Entity> {
        self.condition_words
            .iter()
            .filter_map(|word| {
                if text.contains(word) {
                    // 提取条件上下文
                    let condition_text = self.extract_context(text, word);
                    Some(
                        Entity::new(condition_text, EntityType::Condition)
                            .with_confidence(0.85)
                            .with_source(text),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    /// 抽取结果实体
    fn extract_results(&self, text: &str) -> Vec<Entity> {
        self.result_words
            .iter()
            .filter_map(|word| {
                if text.contains(word) {
                    let result_text = self.extract_context_after(text, word);
                    Some(
                        Entity::new(result_text, EntityType::Result)
                            .with_confidence(0.85)
                            .with_source(text),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    /// 抽取时间实体
    fn extract_time_entities(&self, text: &str) -> Vec<Entity> {
        self.time_words
            .iter()
            .filter_map(|word| {
                if text.contains(word) {
                    Some(
                        Entity::new(word, EntityType::Time)
                            .with_confidence(0.9)
                            .with_source(text),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    /// 抽取地点实体
    fn extract_locations(&self, text: &str) -> Vec<Entity> {
        self.location_words
            .iter()
            .filter_map(|word| {
                if text.contains(word) {
                    Some(
                        Entity::new(word, EntityType::Location)
                            .with_confidence(0.9)
                            .with_source(text),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    /// 抽取数量实体
    fn extract_quantities(&self, text: &str) -> Vec<Entity> {
        // 匹配数字 + 单位/量词的模式
        let quantity_pattern = regex::Regex::new(
            r"\d+(\.\d+)?\s*(人|次|个|件|年|月|日|分钟|秒|米|厘米|公斤|克|元|万|亿|%)",
        )
        .unwrap_or_else(|_| regex::Regex::new(r"\d+").unwrap());

        quantity_pattern
            .find_iter(text)
            .map(|m| {
                Entity::new(m.as_str(), EntityType::Quantity)
                    .with_confidence(0.95)
                    .with_source(text)
            })
            .collect()
    }

    /// 抽取主体和客体
    fn extract_subjects_objects(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();

        // 常见主体词
        let subject_words = [
            "球员",
            "运动员",
            "选手",
            "参赛者",
            "队员",
            "队长",
            "教练",
            "裁判",
            "法官",
            "当事人",
            "原告",
            "被告",
            "申请人",
            "被申请人",
            "公司",
            "企业",
            "组织",
            "机构",
            "个人",
            "公民",
            "法人",
        ];

        // 常见客体词
        let object_words = [
            "球",
            "比赛",
            "赛事",
            "合同",
            "协议",
            "文件",
            "证书",
            "许可证",
            "执照",
            "资格",
            "权利",
            "义务",
            "责任",
        ];

        for word in subject_words {
            if text.contains(word) {
                entities.push(
                    Entity::new(word, EntityType::Subject)
                        .with_confidence(0.85)
                        .with_source(text),
                );
            }
        }

        for word in object_words {
            if text.contains(word) {
                entities.push(
                    Entity::new(word, EntityType::Object)
                        .with_confidence(0.85)
                        .with_source(text),
                );
            }
        }

        entities
    }

    /// 提取关键词前后的上下文
    fn extract_context(&self, text: &str, keyword: &str) -> String {
        if let Some(pos) = text.find(keyword) {
            // 计算字节位置
            let start_byte = pos.saturating_sub(10);
            let end_byte = (pos + keyword.len() + 20).min(text.len());

            // 找到有效的字符边界
            let start_byte = text.floor_char_boundary(start_byte);
            let end_byte = text.floor_char_boundary(end_byte);

            text[start_byte..end_byte].to_string()
        } else {
            keyword.to_string()
        }
    }

    /// 提取关键词后的上下文
    fn extract_context_after(&self, text: &str, keyword: &str) -> String {
        if let Some(pos) = text.find(keyword) {
            let start = pos + keyword.len();
            let end = (start + 20).min(text.len());
            text[start..end].to_string()
        } else {
            keyword.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_type_display() {
        assert_eq!(EntityType::Concept.to_string(), "概念");
        assert_eq!(EntityType::Action.to_string(), "动作");
        assert!(EntityType::Action.is_action());
        assert!(EntityType::Condition.is_conditional());
    }

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new("球员", EntityType::Subject)
            .with_confidence(0.9)
            .with_source("足球规则")
            .with_attribute("数量", "11人");

        assert_eq!(entity.name, "球员");
        assert_eq!(entity.entity_type, EntityType::Subject);
        assert_eq!(entity.confidence, 0.9);
        assert_eq!(entity.source, Some("足球规则".to_string()));
        assert_eq!(entity.attributes.get("数量"), Some(&"11人".to_string()));
    }

    #[test]
    fn test_entity_matches() {
        let entity = Entity::new("球员", EntityType::Subject).with_synonyms(vec!["运动员", "选手"]);

        assert!(entity.matches("球员"));
        assert!(entity.matches("运动员"));
        assert!(entity.matches("选手"));
        assert!(!entity.matches("裁判"));
    }

    #[test]
    fn test_extractor_basic() {
        let extractor = EntityExtractor::new();
        let entities = extractor.extract("球员在比赛中犯规会被出示黄牌");

        assert!(!entities.is_empty());

        // 应该检测到动作
        let has_action = entities.iter().any(|e| e.entity_type == EntityType::Action);
        assert!(has_action);

        // 应该检测到主体
        let has_subject = entities
            .iter()
            .any(|e| e.entity_type == EntityType::Subject);
        assert!(has_subject);
    }

    #[test]
    fn test_extractor_with_config() {
        let config = ExtractorConfig {
            min_confidence: 0.9,
            extract_time: false,
            extract_location: false,
            ..Default::default()
        };
        let extractor = EntityExtractor::with_config(config);
        let entities = extractor.extract("球员在比赛期间犯规");

        // 不应该有时间实体
        let has_time = entities.iter().any(|e| e.entity_type == EntityType::Time);
        assert!(!has_time);
    }

    #[test]
    fn test_extract_quantities() {
        let extractor = EntityExtractor::new();
        let entities = extractor.extract("比赛时间为90分钟，每队11人");

        let quantities: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Quantity)
            .collect();

        assert!(!quantities.is_empty());
    }
}
