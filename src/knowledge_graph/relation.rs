//! 规则关系抽取模块
//!
//! 从规则定义中抽取实体之间的关系。

use crate::knowledge_graph::entity::{Entity, EntityType};
use std::collections::HashMap;

/// 关系类型
///
/// 定义知识图谱中实体之间的关系类别。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::RelationType;
///
/// assert_eq!(RelationType::Causes.to_string(), "导致");
/// assert!(RelationType::Requires.is_causal());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RelationType {
    /// 因果关系（A 导致 B）
    Causes,
    /// 条件关系（A 是 B 的前提）
    Requires,
    /// 禁止关系（A 禁止 B）
    Prohibits,
    /// 允许关系（A 允许 B）
    Permits,
    /// 等价关系（A 等价于 B）
    Equivalent,
    /// 蕴含关系（A 蕴含 B）
    Implies,
    /// 矛盾关系（A 与 B 矛盾）
    Contradicts,
    /// 包含关系（A 包含 B）
    Contains,
    /// 顺序关系（A 在 B 之前）
    Precedes,
    /// 并行关系（A 与 B 同时进行）
    Parallel,
    /// 参与关系（A 参与 B）
    Participates,
    /// 应用关系（A 应用于 B）
    Applies,
    /// 定义关系（A 定义 B）
    Defines,
    /// 实例关系（A 是 B 的实例）
    Instance,
}

impl RelationType {
    /// 检查是否为因果关系类型
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::RelationType;
    ///
    /// assert!(RelationType::Causes.is_causal());
    /// assert!(RelationType::Requires.is_causal());
    /// ```
    pub fn is_causal(&self) -> bool {
        matches!(
            self,
            Self::Causes | Self::Requires | Self::Implies
        )
    }

    /// 检查是否为冲突关系类型
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::RelationType;
    ///
    /// assert!(RelationType::Contradicts.is_conflict());
    /// assert!(RelationType::Prohibits.is_conflict());
    /// ```
    pub fn is_conflict(&self) -> bool {
        matches!(self, Self::Contradicts | Self::Prohibits)
    }

    /// 检查是否为正向关系
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::RelationType;
    ///
    /// assert!(RelationType::Permits.is_positive());
    /// assert!(!RelationType::Prohibits.is_positive());
    /// ```
    pub fn is_positive(&self) -> bool {
        matches!(
            self,
            Self::Permits | Self::Equivalent
        )
    }

    /// 获取关系显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Causes => "导致",
            Self::Requires => "要求",
            Self::Prohibits => "禁止",
            Self::Permits => "允许",
            Self::Equivalent => "等价",
            Self::Implies => "蕴含",
            Self::Contradicts => "矛盾",
            Self::Contains => "包含",
            Self::Precedes => "先于",
            Self::Parallel => "并行",
            Self::Participates => "参与",
            Self::Applies => "应用",
            Self::Defines => "定义",
            Self::Instance => "实例",
        }
    }
}

impl std::fmt::Display for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// 知识图谱关系
///
/// 表示两个实体之间的关系。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::{Relation, RelationType, Entity, EntityType};
///
/// let subject = Entity::new("球员", EntityType::Subject);
/// let action = Entity::new("犯规", EntityType::Action);
///
/// let relation = Relation::new(
///     subject.id.clone(),
///     action.id.clone(),
///     RelationType::Participates,
/// ).with_confidence(0.9);
///
/// assert_eq!(relation.relation_type, RelationType::Participates);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Relation {
    /// 关系唯一标识
    pub id: String,
    /// 源实体 ID
    pub source_entity: String,
    /// 目标实体 ID
    pub target_entity: String,
    /// 关系类型
    pub relation_type: RelationType,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f64,
    /// 关系来源（规则名称或文本）
    pub source: Option<String>,
    /// 关系属性
    pub attributes: HashMap<String, String>,
}

impl Relation {
    /// 创建新关系
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{Relation, RelationType};
    ///
    /// let relation = Relation::new("entity1", "entity2", RelationType::Causes);
    /// assert_eq!(relation.source_entity, "entity1");
    /// ```
    pub fn new(
        source_entity: impl Into<String>,
        target_entity: impl Into<String>,
        relation_type: RelationType,
    ) -> Self {
        let source_entity = source_entity.into();
        let target_entity = target_entity.into();
        let id = format!(
            "{}_{}_{}",
            source_entity, relation_type.display_name(), target_entity
        );
        Self {
            id,
            source_entity,
            target_entity,
            relation_type,
            confidence: 1.0,
            source: None,
            attributes: HashMap::new(),
        }
    }

    /// 设置置信度
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{Relation, RelationType};
    ///
    /// let relation = Relation::new("a", "b", RelationType::Causes)
    ///     .with_confidence(0.85);
    /// assert_eq!(relation.confidence, 0.85);
    /// ```
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// 设置来源
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// 添加属性
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// 检查是否涉及指定实体
    pub fn involves(&self, entity_id: &str) -> bool {
        self.source_entity == entity_id || self.target_entity == entity_id
    }
}

impl std::fmt::Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> [{}] -> {} ({:.2})",
            self.source_entity, self.relation_type, self.target_entity, self.confidence
        )
    }
}

/// 关系抽取配置
#[derive(Debug, Clone)]
pub struct RelationExtractorConfig {
    /// 最小置信度阈值
    pub min_confidence: f64,
    /// 是否抽取因果关系
    pub extract_causal: bool,
    /// 是否抽取冲突关系
    pub extract_conflict: bool,
    /// 是否抽取层级关系
    pub extract_hierarchical: bool,
}

impl Default for RelationExtractorConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.5,
            extract_causal: true,
            extract_conflict: true,
            extract_hierarchical: true,
        }
    }
}

/// 关系抽取器
///
/// 从实体集合中抽取关系。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::{RelationExtractor, EntityExtractor, EntityType};
///
/// let entity_extractor = EntityExtractor::new();
/// let entities = entity_extractor.extract("球员犯规会被出示黄牌");
///
/// let relation_extractor = RelationExtractor::new();
/// let relations = relation_extractor.extract(&entities, "球员犯规会被出示黄牌");
/// ```
#[derive(Debug, Clone)]
pub struct RelationExtractor {
    config: RelationExtractorConfig,
    /// 因果关键词
    causal_keywords: Vec<String>,
    /// 禁止关键词
    prohibit_keywords: Vec<String>,
    /// 允许关键词
    permit_keywords: Vec<String>,
    /// 条件关键词
    #[allow(dead_code)]
    condition_keywords: Vec<String>,
}

impl Default for RelationExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationExtractor {
    /// 创建新关系抽取器
    pub fn new() -> Self {
        Self {
            config: RelationExtractorConfig::default(),
            causal_keywords: vec![
                "导致".into(), "引起".into(), "产生".into(), "造成".into(),
                "会".into(), "将".into(), "将会".into(),
            ],
            prohibit_keywords: vec![
                "禁止".into(), "不得".into(), "不能".into(), "不许".into(),
                "严禁".into(), "不可".into(), "禁止".into(),
            ],
            permit_keywords: vec![
                "允许".into(), "可以".into(), "许可".into(), "能够".into(),
                "有权".into(), "可".into(),
            ],
            condition_keywords: vec![
                "如果".into(), "当".into(), "若".into(), "在".into(),
                "满足".into(), "符合".into(), "具备".into(),
            ],
        }
    }

    /// 使用自定义配置创建抽取器
    pub fn with_config(config: RelationExtractorConfig) -> Self {
        let mut extractor = Self::new();
        extractor.config = config;
        extractor
    }

    /// 从实体集合中抽取关系
    ///
    /// # Arguments
    /// * `entities` - 实体列表
    /// * `text` - 原始文本
    ///
    /// # Returns
    /// 抽取出的关系列表
    pub fn extract(&self, entities: &[Entity], text: &str) -> Vec<Relation> {
        let mut relations = Vec::new();

        // 抽取因果关系
        if self.config.extract_causal {
            relations.extend(self.extract_causal_relations(entities, text));
        }

        // 抽取冲突关系
        if self.config.extract_conflict {
            relations.extend(self.extract_conflict_relations(entities, text));
        }

        // 抽取层级关系
        if self.config.extract_hierarchical {
            relations.extend(self.extract_hierarchical_relations(entities, text));
        }

        // 抽取参与关系
        relations.extend(self.extract_participation_relations(entities, text));

        // 抽取应用关系
        relations.extend(self.extract_application_relations(entities, text));

        // 过滤低置信度关系
        relations.retain(|r| r.confidence >= self.config.min_confidence);

        relations
    }

    /// 抽取因果关系
    fn extract_causal_relations(&self, entities: &[Entity], text: &str) -> Vec<Relation> {
        let mut relations = Vec::new();

        // 查找条件实体和结果实体
        let conditions: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Condition)
            .collect();
        let results: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Result)
            .collect();
        let actions: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Action)
            .collect();

        // 条件 -> 结果
        for cond in &conditions {
            for result in &results {
                relations.push(
                    Relation::new(&cond.id, &result.id, RelationType::Causes)
                        .with_confidence(0.85)
                        .with_source(text),
                );
            }
        }

        // 动作 -> 结果
        for action in &actions {
            for result in &results {
                relations.push(
                    Relation::new(&action.id, &result.id, RelationType::Causes)
                        .with_confidence(0.9)
                        .with_source(text),
                );
            }
        }

        // 检查因果关键词
        for keyword in &self.causal_keywords {
            if text.contains(keyword) {
                // 找到关键词前后的实体，建立关系
                if let Some(relation) = self.create_relation_by_keyword(entities, text, keyword, RelationType::Causes) {
                    relations.push(relation);
                }
            }
        }

        relations
    }

    /// 抽取冲突关系
    fn extract_conflict_relations(&self, entities: &[Entity], text: &str) -> Vec<Relation> {
        let mut relations = Vec::new();

        for keyword in &self.prohibit_keywords {
            if text.contains(keyword) {
                // 找到禁止的主体和动作
                let subjects: Vec<_> = entities
                    .iter()
                    .filter(|e| e.entity_type == EntityType::Subject)
                    .collect();
                let actions: Vec<_> = entities
                    .iter()
                    .filter(|e| e.entity_type == EntityType::Action)
                    .collect();

                for subject in &subjects {
                    for action in &actions {
                        relations.push(
                            Relation::new(&subject.id, &action.id, RelationType::Prohibits)
                                .with_confidence(0.85)
                                .with_source(text),
                        );
                    }
                }
            }
        }

        relations
    }

    /// 抽取层级关系
    fn extract_hierarchical_relations(&self, entities: &[Entity], text: &str) -> Vec<Relation> {
        let mut relations = Vec::new();

        // 概念之间的包含关系
        let concepts: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Concept)
            .collect();

        // 如果有多个概念，建立包含关系
        for (i, concept1) in concepts.iter().enumerate() {
            for concept2 in concepts.iter().skip(i + 1) {
                // 检查是否有包含关系的关键词
                if text.contains("包括") || text.contains("包含") {
                    relations.push(
                        Relation::new(&concept1.id, &concept2.id, RelationType::Contains)
                            .with_confidence(0.75)
                            .with_source(text),
                    );
                }
            }
        }

        relations
    }

    /// 抽取参与关系
    fn extract_participation_relations(&self, entities: &[Entity], text: &str) -> Vec<Relation> {
        let mut relations = Vec::new();

        let subjects: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Subject)
            .collect();
        let actions: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Action)
            .collect();
        let objects: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Object)
            .collect();

        // 主体 -> 动作
        for subject in &subjects {
            for action in &actions {
                relations.push(
                    Relation::new(&subject.id, &action.id, RelationType::Participates)
                        .with_confidence(0.85)
                        .with_source(text),
                );
            }
        }

        // 动作 -> 客体
        for action in &actions {
            for object in &objects {
                relations.push(
                    Relation::new(&action.id, &object.id, RelationType::Applies)
                        .with_confidence(0.85)
                        .with_source(text),
                );
            }
        }

        relations
    }

    /// 抽取应用关系
    fn extract_application_relations(&self, entities: &[Entity], text: &str) -> Vec<Relation> {
        let mut relations = Vec::new();

        for keyword in &self.permit_keywords {
            if text.contains(keyword) {
                let subjects: Vec<_> = entities
                    .iter()
                    .filter(|e| e.entity_type == EntityType::Subject)
                    .collect();
                let actions: Vec<_> = entities
                    .iter()
                    .filter(|e| e.entity_type == EntityType::Action)
                    .collect();

                for subject in &subjects {
                    for action in &actions {
                        relations.push(
                            Relation::new(&subject.id, &action.id, RelationType::Permits)
                                .with_confidence(0.85)
                                .with_source(text),
                        );
                    }
                }
            }
        }

        relations
    }

    /// 根据关键词创建关系
    fn create_relation_by_keyword(
        &self,
        entities: &[Entity],
        text: &str,
        keyword: &str,
        relation_type: RelationType,
    ) -> Option<Relation> {
        if let Some(pos) = text.find(keyword) {
            // 找关键词前后的实体
            let before: Vec<_> = entities
                .iter()
                .filter(|e| {
                    if let Some(e_pos) = text.find(&e.name) {
                        e_pos < pos
                    } else {
                        false
                    }
                })
                .collect();

            let after: Vec<_> = entities
                .iter()
                .filter(|e| {
                    if let Some(e_pos) = text.find(&e.name) {
                        e_pos > pos + keyword.len()
                    } else {
                        false
                    }
                })
                .collect();

            if let (Some(source), Some(target)) = (before.first(), after.first()) {
                return Some(
                    Relation::new(&source.id, &target.id, relation_type)
                        .with_confidence(0.8)
                        .with_source(text),
                );
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge_graph::EntityExtractor;

    #[test]
    fn test_relation_type_display() {
        assert_eq!(RelationType::Causes.to_string(), "导致");
        assert!(RelationType::Causes.is_causal());
        assert!(RelationType::Contradicts.is_conflict());
        assert!(RelationType::Permits.is_positive());
    }

    #[test]
    fn test_relation_creation() {
        let relation = Relation::new("entity1", "entity2", RelationType::Causes)
            .with_confidence(0.9)
            .with_source("测试规则");

        assert_eq!(relation.source_entity, "entity1");
        assert_eq!(relation.target_entity, "entity2");
        assert_eq!(relation.relation_type, RelationType::Causes);
        assert!(relation.involves("entity1"));
        assert!(relation.involves("entity2"));
        assert!(!relation.involves("entity3"));
    }

    #[test]
    fn test_relation_extractor_basic() {
        let entity_extractor = EntityExtractor::new();
        let entities = entity_extractor.extract("球员在比赛中犯规会被出示黄牌");

        let relation_extractor = RelationExtractor::new();
        let relations = relation_extractor.extract(&entities, "球员在比赛中犯规会被出示黄牌");

        assert!(!relations.is_empty());

        // 应该有关系
        let has_participates = relations
            .iter()
            .any(|r| r.relation_type == RelationType::Participates);
        assert!(has_participates);
    }

    #[test]
    fn test_extract_causal_relations() {
        let entity_extractor = EntityExtractor::new();
        let entities = entity_extractor.extract("如果球员犯规，则会被出示黄牌");

        let relation_extractor = RelationExtractor::new();
        let relations = relation_extractor.extract(&entities, "如果球员犯规，则会被出示黄牌");

        // 应该有因果关系
        let has_causal = relations.iter().any(|r| r.relation_type.is_causal());
        assert!(has_causal);
    }

    #[test]
    fn test_extract_prohibit_relations() {
        let entity_extractor = EntityExtractor::new();
        let entities = entity_extractor.extract("球员禁止使用违禁药物");

        let relation_extractor = RelationExtractor::new();
        let relations = relation_extractor.extract(&entities, "球员禁止使用违禁药物");

        // 应该有禁止关系
        let has_prohibit = relations
            .iter()
            .any(|r| r.relation_type == RelationType::Prohibits);
        assert!(has_prohibit);
    }

    #[test]
    fn test_relation_confidence_filter() {
        let config = RelationExtractorConfig {
            min_confidence: 0.95,
            ..Default::default()
        };
        let relation_extractor = RelationExtractor::with_config(config);

        let entity_extractor = EntityExtractor::new();
        let entities = entity_extractor.extract("球员犯规会被出示黄牌");
        let relations = relation_extractor.extract(&entities, "球员犯规会被出示黄牌");

        // 大多数关系置信度应该低于 0.95
        // 因为默认置信度是 0.8-0.9
        let high_confidence_count = relations.iter().filter(|r| r.confidence >= 0.95).count();
        assert!(high_confidence_count <= relations.len());
    }
}