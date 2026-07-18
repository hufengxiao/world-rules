//! 规则推理引擎模块
//!
//! 基于知识图谱进行规则推理。

use crate::knowledge_graph::entity::{Entity, EntityType};
use crate::knowledge_graph::graph::KnowledgeGraph;
use crate::knowledge_graph::relation::{Relation, RelationType};
use std::collections::HashMap;

/// 推理规则
///
/// 定义推理引擎使用的规则。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::InferenceRule;
///
/// let rule = InferenceRule::new("传递规则", "如果 A 导致 B，B 导致 C，则 A 导致 C")
///     .with_confidence(0.9);
///
/// assert_eq!(rule.name, "传递规则");
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InferenceRule {
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 规则条件（模式匹配）
    pub conditions: Vec<InferenceCondition>,
    /// 规则结论
    pub conclusions: Vec<InferenceConclusion>,
    /// 规则置信度
    pub confidence: f64,
}

impl InferenceRule {
    /// 创建新推理规则
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            conditions: Vec::new(),
            conclusions: Vec::new(),
            confidence: 1.0,
        }
    }

    /// 设置置信度
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// 添加条件
    pub fn add_condition(&mut self, condition: InferenceCondition) {
        self.conditions.push(condition);
    }

    /// 添加结论
    pub fn add_conclusion(&mut self, conclusion: InferenceConclusion) {
        self.conclusions.push(conclusion);
    }
}

/// 推理条件
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InferenceCondition {
    /// 条件类型
    pub condition_type: ConditionType,
    /// 涉及的实体变量名
    pub variables: Vec<String>,
    /// 关系类型（用于关系匹配）
    pub relation_type: Option<RelationType>,
    /// 实体类型（用于实体匹配）
    pub entity_type: Option<EntityType>,
}

/// 条件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConditionType {
    /// 存在实体
    EntityExists,
    /// 存在关系
    RelationExists,
    /// 实体类型匹配
    EntityTypeMatch,
    /// 实体属性匹配
    AttributeMatch,
}

/// 推理结论
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InferenceConclusion {
    /// 结论类型
    pub conclusion_type: ConclusionType,
    /// 涉及的实体
    pub entities: Vec<String>,
    /// 推导出的关系类型
    pub relation_type: Option<RelationType>,
    /// 结论置信度调整因子
    pub confidence_factor: f64,
}

/// 结论类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConclusionType {
    /// 推导出新关系
    NewRelation,
    /// 推导出新实体
    NewEntity,
    /// 推导出属性
    NewAttribute,
    /// 验证成立
    Verified,
}

/// 推理结果
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::InferenceResult;
///
/// let result = InferenceResult::success("推导成功")
///     .with_confidence(0.85);
///
/// assert!(result.is_success());
/// assert_eq!(result.confidence, 0.85);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InferenceResult {
    /// 是否成功
    pub success: bool,
    /// 结果消息
    pub message: String,
    /// 置信度
    pub confidence: f64,
    /// 推导出的新关系
    inferred_relations: Vec<Relation>,
    /// 推导出的新实体
    inferred_entities: Vec<Entity>,
    /// 推导出的属性
    inferred_attributes: HashMap<String, HashMap<String, String>>,
    /// 推理路径
    inference_path: Vec<String>,
}

impl InferenceResult {
    /// 创建成功的推理结果
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            confidence: 1.0,
            inferred_relations: Vec::new(),
            inferred_entities: Vec::new(),
            inferred_attributes: HashMap::new(),
            inference_path: Vec::new(),
        }
    }

    /// 创建失败的推理结果
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            confidence: 0.0,
            inferred_relations: Vec::new(),
            inferred_entities: Vec::new(),
            inferred_attributes: HashMap::new(),
            inference_path: Vec::new(),
        }
    }

    /// 设置置信度
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// 添加推导出的关系
    pub fn add_inferred_relation(&mut self, relation: Relation) {
        self.inferred_relations.push(relation);
    }

    /// 添加推导出的实体
    pub fn add_inferred_entity(&mut self, entity: Entity) {
        self.inferred_entities.push(entity);
    }

    /// 添加推理路径步骤
    pub fn add_path_step(&mut self, step: impl Into<String>) {
        self.inference_path.push(step.into());
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// 获取推导出的关系
    pub fn get_inferred_relations(&self) -> &[Relation] {
        &self.inferred_relations
    }

    /// 获取推导出的实体
    pub fn get_inferred_entities(&self) -> &[Entity] {
        &self.inferred_entities
    }

    /// 获取推理路径
    pub fn get_inference_path(&self) -> &[String] {
        &self.inference_path
    }
}

impl std::fmt::Display for InferenceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "推理结果: {} (置信度: {:.2})",
            if self.success { "成功" } else { "失败" },
            self.confidence
        )
    }
}

/// 推理引擎配置
#[derive(Debug, Clone)]
pub struct InferenceConfig {
    /// 最大推理深度
    pub max_depth: usize,
    /// 最小置信度阈值
    pub min_confidence: f64,
    /// 是否启用传递推理
    pub enable_transitive: bool,
    /// 是否启用反向推理
    pub enable_backward: bool,
    /// 是否缓存结果
    pub enable_cache: bool,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            max_depth: 5,
            min_confidence: 0.5,
            enable_transitive: true,
            enable_backward: true,
            enable_cache: true,
        }
    }
}

/// 推理引擎
///
/// 基于知识图谱进行规则推理。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::{InferenceEngine, KnowledgeGraph};
///
/// let graph = KnowledgeGraph::new("测试图谱");
/// let engine = InferenceEngine::new(graph);
///
/// let result = engine.query("球员犯规");
/// ```
#[derive(Debug)]
pub struct InferenceEngine {
    /// 知识图谱
    graph: KnowledgeGraph,
    /// 推理规则
    rules: Vec<InferenceRule>,
    /// 配置
    config: InferenceConfig,
    /// 缓存
    cache: HashMap<String, InferenceResult>,
}

impl InferenceEngine {
    /// 创建新推理引擎
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{InferenceEngine, KnowledgeGraph};
    ///
    /// let graph = KnowledgeGraph::new("测试图谱");
    /// let engine = InferenceEngine::new(graph);
    /// ```
    pub fn new(graph: KnowledgeGraph) -> Self {
        Self {
            graph,
            rules: Self::default_rules(),
            config: InferenceConfig::default(),
            cache: HashMap::new(),
        }
    }

    /// 使用自定义配置创建引擎
    pub fn with_config(graph: KnowledgeGraph, config: InferenceConfig) -> Self {
        Self {
            graph,
            rules: Self::default_rules(),
            config,
            cache: HashMap::new(),
        }
    }

    /// 获取默认推理规则
    fn default_rules() -> Vec<InferenceRule> {
        vec![
            // 传递规则：如果 A -> B -> C，则 A -> C
            InferenceRule::new(
                "传递推理",
                "如果 A 导致 B，B 导致 C，则 A 导致 C",
            ),
            // 矛盾规则：如果 A 禁止 B，A 发生，则 B 不应该发生
            InferenceRule::new(
                "禁止推理",
                "如果 A 禁止 B，A 发生，则 B 不应该发生",
            ),
            // 逆否推理：如果 A -> B，则 !B -> !A
            InferenceRule::new(
                "逆否推理",
                "如果 A 蕴含 B，则非 B 蕴含非 A",
            ),
        ]
    }

    /// 添加推理规则
    pub fn add_rule(&mut self, rule: InferenceRule) {
        self.rules.push(rule);
    }

    /// 执行查询
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{InferenceEngine, KnowledgeGraph, Entity, EntityType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试图谱");
    /// graph.add_entity(Entity::new("球员", EntityType::Subject));
    ///
    /// let engine = InferenceEngine::new(graph);
    /// let result = engine.query("球员");
    /// ```
    pub fn query(&self, query_text: &str) -> InferenceResult {
        // 检查缓存
        if self.config.enable_cache {
            if let Some(result) = self.cache.get(query_text) {
                return result.clone();
            }
        }

        // 执行推理
        

        self.execute_inference(query_text)
    }

    /// 执行推理过程
    fn execute_inference(&self, query_text: &str) -> InferenceResult {
        let mut result = InferenceResult::success(format!("查询: {}", query_text));

        // 在图谱中查找匹配的实体
        let matching_entities = self.find_matching_entities(query_text);

        if matching_entities.is_empty() {
            return InferenceResult::failure(format!("未找到匹配实体: {}", query_text));
        }

        result.add_path_step(format!("找到匹配实体: {:?}", matching_entities));

        // 对每个匹配的实体进行推理
        for entity_name in &matching_entities {
            self.infer_from_entity(entity_name, &mut result, 0);
        }

        result
    }

    /// 查找匹配的实体
    fn find_matching_entities(&self, query_text: &str) -> Vec<String> {
        let mut matching = Vec::new();

        for node in self.graph.get_all_nodes() {
            if node.name.contains(query_text) || query_text.contains(&node.name) {
                matching.push(node.name.clone());
            }
        }

        matching
    }

    /// 从实体出发进行推理
    fn infer_from_entity(
        &self,
        entity_name: &str,
        result: &mut InferenceResult,
        depth: usize,
    ) {
        if depth >= self.config.max_depth {
            return;
        }

        // 获取实体的出边
        // 尝试不同类型的边
        for node in self.graph.get_all_nodes() {
            if node.name == entity_name {
                for edge in self.graph.get_out_edges(&node.id) {
                    result.add_path_step(format!(
                        "发现关系: {} -> {} -> {}",
                        entity_name, edge.relation_type, edge.target
                    ));

                    // 如果是因果关系，进行传递推理
                    if edge.relation_type.is_causal() && self.config.enable_transitive {
                        self.transitive_inference(edge, result, depth);
                    }

                    // 如果是冲突关系，进行冲突推理
                    if edge.relation_type.is_conflict() {
                        self.conflict_inference(edge, result);
                    }
                }
            }
        }
    }

    /// 传递推理
    fn transitive_inference(
        &self,
        edge: &crate::knowledge_graph::graph::GraphEdge,
        result: &mut InferenceResult,
        depth: usize,
    ) {
        // 查找从 edge.target 开始的后续关系
        for next_edge in self.graph.get_out_edges(&edge.target) {
            if next_edge.relation_type == edge.relation_type {
                // 推导出新关系
                let inferred = Relation::new(
                    &edge.source,
                    &next_edge.target,
                    edge.relation_type,
                )
                .with_confidence(edge.confidence * 0.9);

                result.add_inferred_relation(inferred);
                result.add_path_step(format!(
                    "传递推理: {} -> {} -> {} 蕴含 {} -> {}",
                    edge.source, edge.relation_type, edge.target,
                    edge.source, next_edge.target
                ));

                // 递归推理
                self.infer_from_entity(&next_edge.target.clone(), result, depth + 1);
            }
        }
    }

    /// 冲突推理
    fn conflict_inference(
        &self,
        edge: &crate::knowledge_graph::graph::GraphEdge,
        result: &mut InferenceResult,
    ) {
        result.add_path_step(format!(
            "发现冲突: {} 禁止 {}",
            edge.source, edge.target
        ));

        // 查找是否有违反冲突的情况
        for other_edge in self.graph.get_out_edges(&edge.source) {
            if other_edge.target == edge.target
                && other_edge.relation_type == RelationType::Permits
            {
                result.add_path_step(format!(
                    "警告: 检测到矛盾 - {} 既禁止又允许 {}",
                    edge.source, edge.target
                ));
            }
        }
    }

    /// 验证规则一致性
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{InferenceEngine, KnowledgeGraph};
    ///
    /// let graph = KnowledgeGraph::new("测试图谱");
    /// let engine = InferenceEngine::new(graph);
    /// let result = engine.validate_consistency();
    /// ```
    pub fn validate_consistency(&self) -> InferenceResult {
        let mut result = InferenceResult::success("一致性检查完成");

        for edge in self.graph.get_all_edges() {
            for other_edge in self.graph.get_all_edges() {
                // 检查同一对实体之间是否有冲突关系
                if edge.source == other_edge.source && edge.target == other_edge.target
                    && edge.relation_type.is_conflict() && other_edge.relation_type.is_positive() {
                        result.add_path_step(format!(
                            "冲突: {} 和 {} 对同一对实体 {} -> {}",
                            edge.relation_type, other_edge.relation_type,
                            edge.source, edge.target
                        ));
                    }
            }
        }

        result
    }

    /// 获取知识图谱引用
    pub fn get_graph(&self) -> &KnowledgeGraph {
        &self.graph
    }

    /// 获取知识图谱可变引用
    pub fn get_graph_mut(&mut self) -> &mut KnowledgeGraph {
        &mut self.graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_rule() {
        let rule = InferenceRule::new("测试规则", "这是一个测试规则")
            .with_confidence(0.9);

        assert_eq!(rule.name, "测试规则");
        assert_eq!(rule.description, "这是一个测试规则");
        assert_eq!(rule.confidence, 0.9);
    }

    #[test]
    fn test_inference_result() {
        let result = InferenceResult::success("推理成功")
            .with_confidence(0.85);

        assert!(result.is_success());
        assert_eq!(result.confidence, 0.85);
        assert_eq!(result.message, "推理成功");

        let failure = InferenceResult::failure("推理失败");
        assert!(!failure.is_success());
    }

    #[test]
    fn test_inference_result_additions() {
        let mut result = InferenceResult::success("测试");

        result.add_path_step("步骤1");
        result.add_path_step("步骤2");

        let path = result.get_inference_path();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], "步骤1");
        assert_eq!(path[1], "步骤2");
    }

    #[test]
    fn test_inference_engine_creation() {
        let graph = KnowledgeGraph::new("测试图谱");
        let engine = InferenceEngine::new(graph);

        assert_eq!(engine.get_graph().name, "测试图谱");
    }

    #[test]
    fn test_query_empty_graph() {
        let graph = KnowledgeGraph::new("测试图谱");
        let engine = InferenceEngine::new(graph);

        let result = engine.query("不存在的实体");
        assert!(!result.is_success());
    }

    #[test]
    fn test_query_with_entities() {
        let mut graph = KnowledgeGraph::new("测试图谱");
        graph.add_entity(Entity::new("球员", EntityType::Subject));

        let engine = InferenceEngine::new(graph);

        let result = engine.query("球员");
        assert!(result.is_success());
    }

    #[test]
    fn test_validate_consistency() {
        let graph = KnowledgeGraph::new("测试图谱");
        let engine = InferenceEngine::new(graph);

        let result = engine.validate_consistency();
        assert!(result.is_success());
    }

    #[test]
    fn test_inference_with_relations() {
        let mut graph = KnowledgeGraph::new("测试图谱");

        graph.add_entity(Entity::new("A", EntityType::Subject));
        graph.add_entity(Entity::new("B", EntityType::Object));
        graph.add_entity(Entity::new("C", EntityType::Result));

        graph.add_relation(Relation::new("主体_A", "客体_B", RelationType::Causes));
        graph.add_relation(Relation::new("客体_B", "结果_C", RelationType::Causes));

        let engine = InferenceEngine::new(graph);

        let result = engine.query("A");
        assert!(result.is_success());
        assert!(!result.get_inference_path().is_empty());
    }

    #[test]
    fn test_config_customization() {
        let config = InferenceConfig {
            max_depth: 10,
            min_confidence: 0.8,
            enable_transitive: false,
            ..Default::default()
        };

        let graph = KnowledgeGraph::new("测试图谱");
        let engine = InferenceEngine::with_config(graph, config);

        // 配置应该生效
        assert_eq!(engine.config.max_depth, 10);
        assert_eq!(engine.config.min_confidence, 0.8);
        assert!(!engine.config.enable_transitive);
    }
}